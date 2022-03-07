use anyhow::Result;
use std::{collections::HashMap, fs::File, io::Read, path::Path};

pub fn load_sites_csv() -> Result<HashMap<String, (usize, usize)>> {
    let path = Path::new("Sites.csv");
    if path.exists() {
        let mut result = HashMap::<String, (usize, usize)>::new();
        let mut data = String::new();
        let mut f = File::open(path)?;
        f.read_to_string(&mut data)?;
        data.split("\n").skip(1).for_each(|line| {
            if !line.trim().is_empty() {
                let cols = line.split(",").collect::<Vec<&str>>();
                let name = cols[0].trim();
                let download = cols[1].trim();
                let upload = cols[2].trim();
                result.insert(
                    name.to_string(),
                    (download.parse().unwrap(), upload.parse().unwrap()),
                );
            }
        });
        Ok(result)
    } else {
        Ok(HashMap::new())
    }
}

pub fn load_aps_csv() -> Result<HashMap<String, (usize, usize)>> {
    let path = Path::new("AccessPoints.csv");
    if path.exists() {
        let mut result = HashMap::<String, (usize, usize)>::new();
        let mut data = String::new();
        let mut f = File::open(path)?;
        f.read_to_string(&mut data)?;
        data.split("\n").skip(1).for_each(|line| {
            if !line.trim().is_empty() {
                let cols = line.split(",").collect::<Vec<&str>>();
                let name = cols[0].trim();
                let download = cols[1].trim();
                let upload = cols[2].trim();
                result.insert(
                    name.to_string(),
                    (download.parse().unwrap(), upload.parse().unwrap()),
                );
            }
        });
        Ok(result)
    } else {
        Ok(HashMap::new())
    }
}
