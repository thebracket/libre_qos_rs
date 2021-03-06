use crate::clients::LqClientDevice;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Device {
    pub identification: DeviceIdentification,
    pub ipAddress: Option<String>,
    pub attributes: Option<DeviceAttributes>,
    pub mode: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DeviceIdentification {
    pub id: String,
    pub hostname: Option<String>,
    pub mac: Option<String>,
    pub model: Option<String>,
    pub role: Option<String>,
    pub site: Option<DeviceSite>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DeviceSite {
    pub id: String,
    pub parent: Option<DeviceParent>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DeviceParent {
    pub id: String,
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DeviceAttributes {
    pub ssid: Option<String>,
    pub apDevice: Option<DeviceAccessPoint>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DeviceAccessPoint {
    pub id: String,
    pub name: String,
}

impl Device {
    pub fn as_lq_client_device(&self, upload: usize, download: usize) -> Option<LqClientDevice> {
        let mut result = None;
        let mut is_access_point = false;
        let mut is_bridge = false;
        if let Some(role) = &self.identification.role {
            if role == "ap" {
                is_access_point = true;
            }
        }
        if let Some(mode) = &self.mode {
            if mode == "bridge" {
                is_bridge = true;
            }
        }
        if let Some(ip) = &self.ipAddress {
            let mut access_point_id = String::new();
            let mut access_point_name = String::new();

            if let Some(attr) = &self.attributes {
                if let Some(ap) = &attr.apDevice {
                    access_point_id = ap.id.clone();
                    access_point_name = ap.name.replace(",", "_");
                }
            }

            let mut parent_site_id = String::new();
            let mut parent_site_name = String::new();

            if let Some(site) = &self.identification.site {
                if let Some(parent) = &site.parent {
                    parent_site_id = parent.id.clone();
                    parent_site_name = parent.name.clone();
                }
            }

            let hostname = if let Some(hn) = &self.identification.hostname {
                if hn.to_lowercase().contains("medusa") {
                    is_bridge = true;
                }
                hn.replace(",", "_")
            } else {
                String::new()
            };

            if let Some(model) = &self.identification.model {
                if model.to_lowercase().contains("pmp450") {
                    is_bridge = true;
                }
            }

            result = Some(LqClientDevice {
                id: self.identification.id.clone(),
                hostname,
                mac: self.identification.mac.clone().unwrap_or_default(),
                ip: ip.clone(),
                model: self.identification.model.clone().unwrap_or_default(),
                access_point_id,
                access_point_name,
                parent_site_id,
                parent_site_name,
                upload,
                download,
                is_access_point,
                is_bridge,
            });
        }
        result
    }
}
