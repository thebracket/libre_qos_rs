use std::collections::HashMap;

use crate::clients::LqClientSite;
use crate::topology::LqSite;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Site {
    pub id: String,
    pub identification: Option<SiteId>,
    pub description: Option<Description>,
    pub qos: Option<Qos>,
}

impl Site {
    pub fn as_lq_site(&self, sites_csv: &HashMap<String, (usize, usize)>) -> Option<LqSite> {
        if !self.is_active() {
            return None;
        }
        let mut result = None;

        if let Some(ident) = &self.identification {
            if let Some(status) = &ident.status {
                if status != "active" {
                    return None;
                }
            }

            let parent: Option<String> = if let Some(parent) = &ident.parent {
                if let Some(parent) = &parent.id {
                    Some(parent.clone())
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(name) = &ident.name {
                let (download_mbps, upload_mbps) = if let Some(site_info) = sites_csv.get(name) {
                    (site_info.0, site_info.1)
                } else {
                    (1_000, 1_000)
                };
                result = Some(LqSite {
                    id: self.id.clone(),
                    name: name.clone(),
                    parent,
                    children: Vec::new(),
                    access_points: HashMap::new(),
                    download_mbps,
                    upload_mbps,
                });
            }
        }
        result
    }

    pub fn as_lq_client_site(&self) -> Option<LqClientSite> {
        let mut result = None;

        if let Some(ident) = &self.identification {
            if let Some(qos) = &self.qos {
                if let Some(name) = &ident.name {
                    result = Some(LqClientSite {
                        id: self.id.clone(),
                        name: name.clone(),
                        download: qos.downloadSpeed.unwrap_or(0),
                        upload: qos.uploadSpeed.unwrap_or(0),
                        devices: Vec::new(),
                    });
                }
            } else {
                println!("Rejected - no QoS");
            }
        }
        result
    }

    pub fn is_active(&self) -> bool {
        if let Some(id) = &self.identification {
            if let Some(status) = &id.status {
                if status == "active" {
                    return true;
                }
            }
        }
        false
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct SiteParent {
    pub id: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct SiteId {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub site_type: Option<String>,
    pub parent: Option<SiteParent>,
    pub status: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint {
    pub id: Option<String>,
    pub name: Option<String>,
    pub parentId: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Description {
    pub location: Option<Location>,
    pub height: Option<f64>,
    pub endpoints: Option<Vec<Endpoint>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Qos {
    pub enabled: bool,
    pub downloadSpeed: Option<usize>,
    pub uploadSpeed: Option<usize>,
}
