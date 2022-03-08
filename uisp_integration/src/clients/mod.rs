mod client_site;
use std::collections::HashMap;

pub use client_site::*;
mod client_device;
pub use client_device::*;
mod csv;
use crate::{
    topology::LqSite,
    unms::{DataLink, Device, Site},
};
use anyhow::Result;
pub use csv::*;

fn lookup_data_link(device: &mut LqClientDevice, all_data_links: &[DataLink]) -> Result<()> {
    //if !device.access_point_id.is_empty() {
    //    return Ok(()); // Bail out because it already has an AP
    //}
    all_data_links
        .iter()
        .filter(|l| {
            l.from.device.identification.id == device.id
                || l.to.device.identification.id == device.id
        })
        .for_each(|link| {
            if link.from.device.identification.id != device.id {
                device.access_point_id = link.from.device.identification.id.clone();
                device.access_point_name = link.from.device.identification.name.clone();
            } else {
                device.access_point_id = link.to.device.identification.id.clone();
                device.access_point_name = link.to.device.identification.name.clone();
            }
        });

    Ok(())
}

fn active_clients(all_sites: &[Site]) -> Vec<LqClientSite> {
    all_sites
        .iter()
        .filter(|s| {
            if let Some(id) = &s.identification {
                if let Some(site_type) = &id.site_type {
                    if site_type == "endpoint" {
                        return true;
                    }
                }
            }
            false
        })
        .filter(|s| s.is_active())
        .filter_map(|s| s.as_lq_client_site())
        .collect()
}

/// The easy case: the client site has one device present, in router mode (or unspecified)
pub fn single_entry_clients(
    all_sites: &[Site],
    all_devices: &[Device],
    all_data_links: &[DataLink],
) -> Result<Vec<LqClientSite>> {
    let mut result = Vec::<LqClientSite>::new();
    active_clients(all_sites).iter().for_each(|client_site| {
        let devices: Vec<LqClientDevice> = all_devices
            .iter()
            .filter(|d| {
                if let Some(site) = &d.identification.site {
                    site.id == client_site.id
                } else {
                    false
                }
            })
            .filter_map(|c| c.as_lq_client_device(client_site.upload, client_site.download))
            .collect();

        if devices.len() == 1 {
            let mut cs = client_site.clone();
            let mut device = devices[0].clone();
            let _ = lookup_data_link(&mut device, all_data_links);
            cs.devices.push(device);
            result.push(cs);
        }
    });
    Ok(result)
}

pub fn complex_clients(
    all_sites: &[Site],
    all_devices: &[Device],
    all_data_links: &[DataLink],
    network_sites: &mut HashMap<String, LqSite>,
) -> Result<Vec<LqClientSite>> {
    let mut result = Vec::<LqClientSite>::new();

    active_clients(all_sites).iter().for_each(|client_site| {
        let mut devices: Vec<LqClientDevice> = all_devices
            .iter()
            .filter(|d| {
                if let Some(site) = &d.identification.site {
                    site.id == client_site.id
                } else {
                    false
                }
            })
            .filter_map(|c| c.as_lq_client_device(client_site.upload, client_site.download))
            .collect();

        if devices.len() > 1 {
            let local_access_points: Vec<String> = devices
                .iter()
                .filter(|d| d.is_access_point)
                .map(|d| d.id.clone())
                .collect();
            devices.iter_mut().for_each(|d| {
                let _ = lookup_data_link(d, all_data_links);

                // Identify in-site relays
                if local_access_points
                    .iter()
                    .any(|ap| d.access_point_id == **ap)
                {
                    d.parent_site_id = client_site.id.clone();
                    d.parent_site_name = client_site.name.clone();
                }

                // Identify lazy parentage (no data link, but in site)
                if d.access_point_id.is_empty() {
                    d.parent_site_id = client_site.id.clone();
                    d.parent_site_name = client_site.name.clone();
                }
            });

            let mut externals = HashMap::new();
            devices
                .iter()
                .filter(|d| d.parent_site_id != client_site.id)
                .for_each(|d| {
                    externals.insert(
                        d.parent_site_id.clone(),
                        (
                            d.parent_site_id.clone(),
                            d.parent_site_name.clone(),
                            d.access_point_id.clone(),
                            d.access_point_name.clone(),
                        ),
                    );
                });
            let n_external_links = externals.len();

            if n_external_links == 0 {
                println!("Orphan: {}", client_site.name);
                let mut cs = client_site.clone();
                let mut device = devices[0].clone();
                let _ = lookup_data_link(&mut device, all_data_links);
                cs.devices.push(device);
                result.push(cs);
            } else if n_external_links == 1 {
                devices.retain(|d| !d.is_access_point);
                devices.retain(|d| !d.is_bridge);
                if devices.len() == 1 {
                    let (pid, pn, apid, apn) = externals.iter().next().unwrap().1;
                    for device in devices.iter_mut() {
                        device.access_point_id = apid.clone();
                        device.access_point_name = apn.clone();
                        device.parent_site_id = pid.clone();
                        device.parent_site_name = pn.clone();
                    }
                    let mut cs = client_site.clone();
                    let mut device = devices[0].clone();
                    let _ = lookup_data_link(&mut device, all_data_links);
                    cs.devices.push(device);
                    result.push(cs);
                } else {
                    // Need to create a new network topology site so as to share bandwidth
                    // with all items in it
                    //println!("\nExternal links: {}", n_external_links);
                    //println!("{:#?}\n", devices);
                    network_sites.insert(
                        client_site.id.clone(),
                        LqSite {
                            id: client_site.id.clone(),
                            name: client_site.name.clone(),
                            download_mbps: client_site.download / 1_000_000,
                            upload_mbps: client_site.upload / 1_000_000,
                            children: Vec::new(),
                            access_points: HashMap::new(),
                            parent: Some(externals.iter().next().unwrap().0.clone()),
                        },
                    );

                    let mut cs = client_site.clone();
                    for device in devices.iter() {
                        let mut d = device.clone();
                        d.parent_site_id = client_site.id.clone();
                        d.parent_site_name = client_site.name.clone();
                        cs.devices.push(d);
                    }
                    result.push(cs);
                }
            } else {
                println!("\nReally tough site: {}", client_site.name);
                println!("External links: {}", n_external_links);
                println!("{:#?}", externals);
            }
        }
    });

    Ok(result)
}

pub fn create_network_infrastructure(
    sites: &HashMap<String, LqSite>,
    all_devices: &[Device],
) -> Result<Vec<LqClientSite>> {
    let mut result = Vec::new();

    for (i, (_, site)) in sites.iter().enumerate() {
        let mut ls = LqClientSite {
            id: format!("inf{i}"),
            name: format!("{}Infrastructure", site.name),
            download: 1_000_000_000_000,
            upload: 1_000_000_000_000,
            devices: Vec::new(),
        };

        // Find devices in this site
        let mut devices: Vec<LqClientDevice> = all_devices
            .iter()
            .filter(|d| {
                if let Some(s) = &d.identification.site {
                    site.id == s.id
                } else {
                    false
                }
            })
            .filter_map(|c| c.as_lq_client_device(ls.upload, ls.download))
            .collect();

        for d in devices.iter_mut() {
            d.access_point_id = format!("infap{i}");
            d.access_point_name = format!("{}Infrastructure", site.name);
            d.parent_site_id = site.id.clone();
            d.parent_site_name = site.name.clone();
        }

        ls.devices = devices;
        result.push(ls);
    }

    Ok(result)
}
