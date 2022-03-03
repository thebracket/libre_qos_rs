use anyhow::Result;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

/// Key store structure
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Keys {
    nms_key: String,
    nms_url: String,
}

impl Keys {
    pub fn load() -> Result<Self> {
        let f = File::open("keys.ron").unwrap();
        let mut keys: Self = from_reader(f)?;
        if keys.nms_url.ends_with("/") {
            keys.nms_url = format!("{}nms/api/v2.1", keys.nms_url);
        } else {
            keys.nms_url = format!("{}/nms/api/v2.1", keys.nms_url);
        }
        Ok(keys)
    }

    pub fn uisp(&self) -> (&str, &str) {
        (&self.nms_key, &self.nms_url)
    }
}
