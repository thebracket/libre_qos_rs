mod clients;
mod network_json;
mod topology;
mod unms;
use anyhow::Result;
use clients::write_shaper_csv;
use network_json::NetworkNode;
use topology::build_topology;
use unms::*;

#[tokio::main]
async fn main() -> Result<()> {
    let keys = Keys::load()?;
    let mut network_sites = topology::build_site_list(&keys).await?;
    let clients = clients::build_clients(&keys).await?;
    write_shaper_csv(&clients)?;
    let network_map = build_topology(&clients, &mut network_sites).await?;
    let network_json_data = NetworkNode::from_lq_site(&network_map);
    network_json_data.write_to_file()?;

    // Complete
    Ok(())
}
