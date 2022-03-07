mod client_site;
pub use client_site::*;
mod client_device;
pub use client_device::*;
mod csv;
use crate::unms::{DataLink, Device, Site};
use anyhow::Result;
pub use csv::*;

pub fn build_clients(
    all_sites: &[Site],
    all_devices: &[Device],
    all_data_links: &[DataLink],
) -> Result<Vec<LqClientSite>> {
    let mut active_sites: Vec<LqClientSite> = all_sites
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
        .collect();

    for client_site in active_sites.iter_mut() {
        let mut devices: Vec<LqClientDevice> = all_devices
            .iter()
            .filter(|d| {
                if let Some(site) = &d.identification.site {
                    if site.id == client_site.id {
                        return true;
                    }
                }
                false
            })
            .filter_map(|c| c.as_lq_client_device(client_site.upload, client_site.download))
            .collect();
        for device in devices.iter_mut() {
            lookup_data_link(device, all_data_links)?;
        }
        client_site.devices = devices;
    }
    Ok(active_sites)
}

pub fn lookup_data_link(device: &mut LqClientDevice, all_data_links: &[DataLink]) -> Result<()> {
    if !device.access_point_id.is_empty() {
        return Ok(()); // Bail out because it already has an AP
    }
    all_data_links
        .iter()
        .filter(|l| l.from.device.identification.id == device.id)
        .for_each(|link| {
            device.access_point_id = link.from.device.identification.id.clone();
            device.access_point_name = link.from.device.identification.name.clone();
        });

    Ok(())
}
