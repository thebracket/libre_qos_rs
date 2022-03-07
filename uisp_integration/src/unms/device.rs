use crate::clients::LqClientDevice;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Device {
    pub identification: DeviceIdentification,
    pub ipAddress: Option<String>,
    pub attributes: Option<DeviceAttributes>,
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
        if let Some(role) = &self.identification.role {
            if role == "ap" {
                return None;
            }
        } else {
            return None;
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
                hn.replace(",", "_")
            } else {
                String::new()
            };

            result = Some(LqClientDevice {
                id: self.identification.id.clone(),
                hostname,
                mac: self.identification.mac.clone().unwrap_or(String::new()),
                ip: ip.clone(),
                model: self.identification.model.clone().unwrap_or(String::new()),
                access_point_id,
                access_point_name,
                parent_site_id,
                parent_site_name,
                upload,
                download,
            });
        }
        result
    }
}
