use crate::{
    clients::{LqClientDevice, LqClientSite},
    unms::{nms_request_get_vec, Keys, Site},
};
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::{collections::HashMap, io::Read, path::Path};

#[derive(Debug, Clone)]
pub struct LqSite {
    pub id: String,
    pub name: String,
    pub parent: Option<String>,
    pub children: Vec<LqSite>,
    pub access_points: HashMap<String, LqAccessPoint>,
    pub download_mbps: usize,
    pub upload_mbps: usize,
}

impl LqSite {
    pub fn take_children(&mut self, sites: &HashMap<String, LqSite>) {
        sites
            .iter()
            .filter(|s| s.1.parent.is_some() && s.1.parent.as_ref().unwrap() == &self.id)
            .for_each(|s| {
                let mut child = s.1.clone();
                child.take_children(sites);
                self.children.push(child)
            });
    }
}

#[derive(Debug, Clone)]
pub struct LqAccessPoint {
    pub name: String,
    pub download_mbps: usize,
    pub upload_mbps: usize,
    pub clients: Vec<LqClientDevice>,
}

pub async fn build_site_tree(sites: &HashMap<String, LqSite>) -> Result<LqSite> {
    let mut root = sites
        .iter()
        .find(|s| s.1.parent.is_none())
        .unwrap()
        .1
        .clone();
    root.take_children(&sites);
    Ok(root)
}

fn load_sites_csv() -> Result<HashMap<String, (usize, usize)>> {
    let path = Path::new("Sites.csv");
    if path.exists() {
        let mut result = HashMap::<String, (usize, usize)>::new();
        let mut data = String::new();
        let mut f = File::open(path)?;
        f.read_to_string(&mut data)?;
        data.split("\n").skip(1).for_each(|line| {
            if !line.trim().is_empty() {
                let cols = line.split(",").collect::<Vec<&str>>();
                let name = cols[0].trim();
                let download = cols[1].trim();
                let upload = cols[2].trim();
                result.insert(
                    name.to_string(),
                    (download.parse().unwrap(), upload.parse().unwrap()),
                );
            }
        });
        Ok(result)
    } else {
        Ok(HashMap::new())
    }
}

fn load_aps_csv() -> Result<HashMap<String, (usize, usize)>> {
    let path = Path::new("AccessPoints.csv");
    if path.exists() {
        let mut result = HashMap::<String, (usize, usize)>::new();
        let mut data = String::new();
        let mut f = File::open(path)?;
        f.read_to_string(&mut data)?;
        data.split("\n").skip(1).for_each(|line| {
            if !line.trim().is_empty() {
                let cols = line.split(",").collect::<Vec<&str>>();
                let name = cols[0].trim();
                let download = cols[1].trim();
                let upload = cols[2].trim();
                result.insert(
                    name.to_string(),
                    (download.parse().unwrap(), upload.parse().unwrap()),
                );
            }
        });
        Ok(result)
    } else {
        Ok(HashMap::new())
    }
}

pub async fn build_site_list(keys: &Keys) -> Result<HashMap<String, LqSite>> {
    let sites_csv = load_sites_csv()?;
    let (key, url) = keys.uisp();
    let sites = nms_request_get_vec::<Site>("sites?type=site", key, url)
        .await?
        .iter()
        .filter_map(|s| s.as_lq_site(&sites_csv))
        .map(|s| (s.id.clone(), s))
        .collect::<HashMap<String, LqSite>>();
    Ok(sites)
}

pub async fn build_topology(
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
            } else {
                if let Some(site) = network_sites.get_mut(&cpe.parent_site_id) {
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
    let mut network_map = build_site_tree(&network_sites).await?;
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
