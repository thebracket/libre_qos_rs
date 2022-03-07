use crate::clients::LqClientDevice;

#[derive(Debug, Clone)]
pub struct LqAccessPoint {
    pub name: String,
    pub download_mbps: usize,
    pub upload_mbps: usize,
    pub clients: Vec<LqClientDevice>,
}
