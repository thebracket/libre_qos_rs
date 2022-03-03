mod clients;
mod topology;
mod unms;
mod network_json;
use anyhow::Result;
use network_json::NetworkNode;
use unms::*;

use crate::topology::LqAccessPoint;

#[tokio::main]
async fn main() -> Result<()> {
    let keys = Keys::load()?;
    let mut network_sites = topology::build_site_list(&keys).await?;
    let clients = clients::build_clients(&keys).await?;

    // Map APs to Sites
    let mut parentless = Vec::new();
    for client in clients.iter() {
        for cpe in client.devices.iter() {
            let mut no_parent = false;
            if cpe.parent_site_id.is_empty() {
                no_parent = true;
            } else {
                if let Some(site) = network_sites.get_mut(&cpe.parent_site_id) {
                    let access_point = if cpe.access_point_name.is_empty() {
                        format!("{}-NoAP", cpe.parent_site_name)
                    } else {
                        cpe.access_point_name.clone()
                    };

                    if let Some(ap) = site.access_points.get_mut(&access_point) {
                        ap.clients.push(cpe.clone());
                    } else {
                        site.access_points.insert(
                            access_point.clone(),
                            topology::LqAccessPoint {
                                name: access_point.clone(),
                                clients: vec![cpe.clone()],
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

    // Overall topology
    let mut network_map = topology::build_site_tree(&network_sites).await?;
    network_map.access_points.insert("0".to_string(), LqAccessPoint{
        name: "Unparented".to_string(),
        clients: parentless,
    });
    let network_json_data = NetworkNode::from_lq_site(&network_map);
    network_json_data.write_to_file()?;

    // Complete
    Ok(())
}
