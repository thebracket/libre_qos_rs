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
    pub upload: usize,
    pub download: usize,
}
