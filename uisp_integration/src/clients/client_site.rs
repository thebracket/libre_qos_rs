use super::LqClientDevice;

#[derive(Debug, Clone)]
pub struct LqClientSite {
    pub id: String,
    pub name: String,
    pub download: usize,
    pub upload: usize,
    pub devices: Vec<LqClientDevice>,
}
