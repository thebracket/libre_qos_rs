use super::LqAccessPoint;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LqSite {
    pub id: String,
    pub name: String,
    pub parent: Option<String>,
    pub children: Vec<LqSite>,
    pub access_points: HashMap<String, LqAccessPoint>,
    pub download_mbps: usize,
    pub upload_mbps: usize,
}

impl LqSite {
    pub fn take_children(&mut self, sites: &HashMap<String, LqSite>) {
        sites
            .iter()
            .filter(|s| s.1.parent.is_some() && s.1.parent.as_ref().unwrap() == &self.id)
            .for_each(|s| {
                let mut child = s.1.clone();
                child.take_children(sites);
                self.children.push(child)
            });
    }
}
