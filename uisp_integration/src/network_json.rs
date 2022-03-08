use crate::topology::{LqAccessPoint, LqSite};
use anyhow::Result;

pub struct NetworkNode {
    pub name: String,
    pub download_bandwidth_mbps: usize,
    pub upload_bandwidth_mbps: usize,
    pub children: Vec<NetworkNode>,
}

impl NetworkNode {
    pub fn from_lq_site(site: &LqSite) -> Self {
        let mut result = NetworkNode {
            name: site.name.clone(),
            download_bandwidth_mbps: 1000,
            upload_bandwidth_mbps: 1000,
            children: Vec::new(),
        };

        for ap in site.access_points.iter() {
            result.children.push(NetworkNode::from_lq_ap(ap.1));
        }

        for cs in site.children.iter() {
            result.children.push(NetworkNode::from_lq_site(cs));
        }

        result
    }

    fn from_lq_ap(ap: &LqAccessPoint) -> Self {
        Self {
            name: ap.name.clone(),
            download_bandwidth_mbps: 1000,
            upload_bandwidth_mbps: 1000,
            children: Vec::new(),
        }
    }

    fn to_json(&self, base: usize, add_comma: bool) -> String {
        // This is messy because the network.json format isn't very strong-type friendly.
        // Serde-json didn't want to go with the free-from look.
        // Doing my best to match https://github.com/rchac/LibreQoS/blob/main/v1.1/network.json
        let mut js = String::new();
        if base == 0 {
            js += &pad_line_add_eol(base, "{");
        }

        js += &pad_line_add_eol(base + 1, &format!("\"{}\":", self.name));
        js += &pad_line_add_eol(base + 1, "{");
        js += &pad_line_add_eol(
            base + 1,
            &format!(
                "\"downloadBandwidthMbps\":{},",
                self.download_bandwidth_mbps
            ),
        );
        if self.children.is_empty() {
            js += &pad_line_add_eol(
                base + 1,
                &format!("\"uploadBandwidthMbps\":{}", self.upload_bandwidth_mbps),
            );
        } else {
            js += &pad_line_add_eol(
                base + 1,
                &format!("\"uploadBandwidthMbps\":{},", self.upload_bandwidth_mbps),
            );
        }
        if !self.children.is_empty() {
            js += &pad_line_add_eol(base + 1, "\"children\":");
            js += &pad_line_add_eol(base + 2, "{");
            let len = self.children.len();
            for (i, c) in self.children.iter().enumerate() {
                js += &c.to_json(base + 3, i < len - 1);
            }
            js += &pad_line_add_eol(base + 2, "}");
        }
        if add_comma {
            js += &pad_line_add_eol(base + 1, "},");
        } else {
            js += &pad_line_add_eol(base + 1, "}");
        }

        if base == 0 {
            js += &pad_line_add_eol(base, "}");
        }
        js
    }

    pub fn write_to_file(&self) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        let mut f = File::create("network.json")?;
        f.write_all(self.to_json(0, false).as_bytes())?;
        Ok(())
    }
}

fn pad_line_add_eol(tabs: usize, line: &str) -> String {
    let mut spacing = String::new();
    for _ in 0..tabs {
        spacing += "   ";
    }
    format!("{spacing}{line}\n")
}
