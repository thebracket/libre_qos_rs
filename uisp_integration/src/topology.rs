use crate::{
    clients::LqClientDevice,
    unms::{nms_request_get_vec, Keys, Site},
};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LqSite {
    pub id: String,
    pub name: String,
    pub parent: Option<String>,
    pub children: Vec<LqSite>,
    pub access_points: HashMap<String, LqAccessPoint>,
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

#[derive(Debug, Clone)]
pub struct LqAccessPoint {
    pub name: String,
    pub clients: Vec<LqClientDevice>,
}

pub async fn build_site_tree(sites: &HashMap<String, LqSite>) -> Result<LqSite> {
    let mut root = sites
        .iter()
        .find(|s| s.1.parent.is_none())
        .unwrap()
        .1
        .clone();
    root.take_children(&sites);
    Ok(root)
}

pub async fn build_site_list(keys: &Keys) -> Result<HashMap<String, LqSite>> {
    let (key, url) = keys.uisp();
    let sites = nms_request_get_vec::<Site>("sites?type=site", key, url)
        .await?
        .iter()
        .filter_map(|s| s.as_lq_site())
        .map(|s| (s.id.clone(), s))
        .collect::<HashMap<String, LqSite>>();
    Ok(sites)
}
