use crate::unms::{DataLink, Device, Site};
use std::collections::HashSet;

pub fn promote_client_sites_with_links_to_other_sites(
    all_sites: &[Site],
    all_devices: &[Device],
    all_data_links: &[DataLink],
) -> HashSet<String> {
    let mut to_promote = HashSet::<String>::new();

    all_sites
        .iter()
        .filter(|s| {
            if let Some(id) = &s.identification {
                if let Some(site_type) = &id.site_type {
                    if site_type == "endpoint" {
                        return true;
                    }
                }
            }
            false
        })
        .filter(|s| s.is_active())
        .filter_map(|c| c.as_lq_client_site())
        .for_each(|client_site| {
            all_devices
                .iter()
                .filter(|d| {
                    if let Some(site) = &d.identification.site {
                        if site.id == client_site.id {
                            return true;
                        }
                    }
                    false
                })
                .filter_map(|c| c.as_lq_client_device(0, 0))
                .for_each(|device| {
                    all_data_links
                        .iter()
                        .filter(|l| l.from.device.identification.id == device.id)
                        .for_each(|dl| {
                            if let Some(dls) = &dl.to.site {
                                let site_id = &dls.identification;
                                if let Some(remote) = all_sites.iter().find(|s| s.id == site_id.id)
                                {
                                    if let Some(id) = &remote.identification {
                                        if let Some(t) = &id.site_type {
                                            if t == "endpoint" {
                                                to_promote.insert(site_id.id.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        });
                });
        });
    to_promote
}
