mod clients;
mod network_json;
mod topology;
mod unms;
use std::time::Instant;

use anyhow::Result;
use clients::write_shaper_csv;
use network_json::NetworkNode;
use tokio::join;
use topology::build_topology;
use unms::*;

/// Connects to uISP and downloads all sites, devices and data-links.
/// Please ensure that you setup `keys.ron` correctly, or this won't work.
async fn pre_load_uisp() -> Result<(Vec<Site>, Vec<Device>, Vec<DataLink>)> {
    let keys = Keys::load()?;
    let (key, url) = keys.uisp();
    let sites_future = nms_request_get_vec::<Site>("sites", key, url);
    let devices_future = nms_request_get_vec::<Device>("devices?authorized=true", key, url);
    let data_links_future = nms_request_get_vec::<DataLink>("data-links", key, url);

    let (sites, devices, data_links) = join!(sites_future, devices_future, data_links_future);
    Ok((sites?, devices?, data_links?))
}

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    println!("Fetching sites, devices and data links from uISP.");
    let start_fetch = Instant::now();
    let (all_sites, all_devices, all_data_links) = pre_load_uisp().await?;
    println!("Fetched all uISP data in {:?}", start_fetch.elapsed());

    let mut network_sites = topology::build_site_list(&all_sites, &all_devices, &all_data_links)?;
    let clients = clients::build_clients(&all_sites, &all_devices, &all_data_links)?;
    write_shaper_csv(&clients)?;
    let network_map = build_topology(&clients, &mut network_sites)?;
    let network_json_data = NetworkNode::from_lq_site(&network_map);
    network_json_data.write_to_file()?;

    // Complete
    println!("Completed topology rebuild in {:?}", start.elapsed());
    Ok(())
}
