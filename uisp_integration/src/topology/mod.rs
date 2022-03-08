mod site;
pub use site::*;
mod access_point;
pub use access_point::*;
mod csv;
use crate::unms::Keys;
use crate::{clients::LqClientSite, unms::Site};
use anyhow::Result;
pub use csv::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub fn build_site_tree(sites: &HashMap<String, LqSite>) -> Result<LqSite> {
    let keys = Keys::load()?;
    let root_name = keys.root();
    let mut root = sites
        .iter()
        .find(|s| s.1.name == root_name)
        .unwrap()
        .1
        .clone();
    root.take_children(sites);
    Ok(root)
}

pub fn build_site_list(all_sites: &[Site]) -> Result<HashMap<String, LqSite>> {
    let sites_csv = load_sites_csv()?;
    let sites = all_sites
        .iter()
        .filter(|s| {
            if let Some(id) = &s.identification {
                if let Some(site_type) = &id.site_type {
                    if site_type == "site" {
                        return true;
                    }
                }
            }
            false
        })
        .filter_map(|s| s.as_lq_site(&sites_csv))
        .map(|s| (s.id.clone(), s))
        .collect::<HashMap<String, LqSite>>();
    Ok(sites)
}

pub fn build_topology(
    clients: &[LqClientSite],
    network_sites: &mut HashMap<String, LqSite>,
) -> Result<LqSite> {
    let access_points_csv = load_aps_csv()?;
    let mut parentless = Vec::new();
    for client in clients.iter() {
        for cpe in client.devices.iter() {
            let mut no_parent = false;
            if cpe.parent_site_id.is_empty() {
                no_parent = true;
            } else if let Some(site) = network_sites.get_mut(&cpe.parent_site_id) {
                let access_point = if cpe.access_point_name.is_empty() {
                    format!("{}-NoAP", cpe.parent_site_name.replace(",", "_"))
                } else {
                    cpe.access_point_name.clone()
                };

                if let Some(ap) = site.access_points.get_mut(&access_point) {
                    ap.clients.push(cpe.clone());
                } else {
                    let (download_mbps, upload_mbps) =
                        if let Some(ap_info) = access_points_csv.get(&access_point) {
                            (ap_info.0, ap_info.1)
                        } else {
                            (1_000, 1_000)
                        };
                    site.access_points.insert(
                        access_point.clone(),
                        LqAccessPoint {
                            name: access_point.clone(),
                            clients: vec![cpe.clone()],
                            download_mbps,
                            upload_mbps,
                        },
                    );
                }
            } else {
                no_parent = true;
            }

            if no_parent {
                parentless.push(cpe.clone());
            }
        }
    }

    // Save "AccessPoints.csv", and "Sites.csv"
    let mut acsv = "AP,Download,Upload\n".to_string();
    let mut scsv = "Site,Download,Upload\n".to_string();
    for (_, site) in network_sites.iter() {
        for (_, ap) in site.access_points.iter() {
            acsv += &format!("{},{},{}\n", ap.name, ap.download_mbps, ap.upload_mbps);
        }
        scsv += &format!(
            "{},{},{}\n",
            site.name, site.download_mbps, site.upload_mbps
        );
    }
    let mut f = File::create("AccessPoints.csv")?;
    f.write_all(acsv.as_bytes())?;
    let mut f = File::create("Sites.csv")?;
    f.write_all(scsv.as_bytes())?;

    // Save "Parentless.csv"
    let mut pcsv = "Hostname\n".to_string();
    for p in parentless.iter() {
        pcsv += &format!("{}\n", p.hostname);
    }
    let mut f = File::create("Parentless.csv")?;
    f.write_all(pcsv.as_bytes())?;

    // Overall topology
    let mut network_map = build_site_tree(network_sites)?;
    network_map.access_points.insert(
        "0".to_string(),
        LqAccessPoint {
            name: "Unparented".to_string(),
            clients: parentless,
            download_mbps: 1_000,
            upload_mbps: 1_000,
        },
    );
    Ok(network_map)
}
