use crate::{
    unms::{nms_request_get_vec, DataLink, Device, Site},
    Keys,
};
use anyhow::Result;

#[derive(Debug)]
pub struct LqClientSite {
    pub id: String,
    pub name: String,
    pub download: usize,
    pub upload: usize,
    pub devices: Vec<LqClientDevice>,
}

#[derive(Debug, Clone)]
pub struct LqClientDevice {
    pub id: String,
    pub hostname: String,
    pub mac: String,
    pub model: String,
    pub ip: String,
    pub access_point_id: String,
    pub access_point_name: String,
    pub parent_site_id: String,
    pub parent_site_name: String,
}

pub async fn build_clients(keys: &Keys) -> Result<Vec<LqClientSite>> {
    let (key, url) = keys.uisp();
    let sites: Vec<Site> = nms_request_get_vec::<Site>("sites?type=client", key, url).await?;
    let mut active_sites: Vec<LqClientSite> = sites
        .iter()
        .filter(|s| s.is_active())
        .filter_map(|s| s.as_lq_client_site())
        .collect();

    for client_site in active_sites.iter_mut() {
        println!(
            "Devices in {} ({}/{})",
            client_site.name, client_site.download, client_site.upload
        );
        //println!("{:?}", nms_request_get_text(&format!("devices?siteId={}", client_site.id), key, url).await?);
        let mut devices: Vec<LqClientDevice> =
            nms_request_get_vec::<Device>(&format!("devices?siteId={}", client_site.id), key, url)
                .await?
                .iter()
                .filter_map(|c| c.as_lq_client_device())
                .collect();
        for device in devices.iter_mut() {
            lookup_data_link(device, key, url).await?;
        }
        client_site.devices = devices;
    }
    Ok(active_sites)
}

pub async fn lookup_data_link(device: &mut LqClientDevice, key: &str, url: &str) -> Result<()> {
    if !device.access_point_id.is_empty() {
        return Ok(()); // Bail out because it already has an AP
    }
    let links =
        nms_request_get_vec::<DataLink>(&format!("data-links/device/{}", device.id), key, url)
            .await?;
    for link in links.iter() {
        device.access_point_id = link.from.device.identification.id.clone();
        device.access_point_name = link.from.device.identification.name.clone();
    }

    Ok(())
}
