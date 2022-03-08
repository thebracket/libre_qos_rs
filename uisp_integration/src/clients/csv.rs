use super::LqClientSite;
use anyhow::Result;

fn strip_ip(ip: &str) -> String {
    if ip.contains('/') {
        ip.split('/').next().unwrap().to_string()
    } else {
        ip.to_string()
    }
}

pub fn write_shaper_csv(clients: &[LqClientSite]) -> Result<()> {
    //let mut csv =
    //    "ID,AP,MAC,Hostname,IPv4,IPv6,Download Min,Upload Min, Download Max, Upload Max\n"
    //        .to_string();
    let mut csv = "deviceID, ParentNode, mac, hostname,ipv4, ipv6, downloadMin, uploadMin, downloadMax, uploadMax\n".to_string();
    clients.iter().for_each(|s| {
        s.devices.iter().for_each(|c| {
            // If QoS returned 0 for speed plan, change it to 1gbps.
            let dl = if c.download == 0 { 1_000 } else { c.download };
            let ul = if c.upload == 0 { 1_000 } else { c.upload };
            let dl_mbps = dl / 1_000_000; // Convert to Mbps
            let ul_mbps = ul / 1_000_000;

            let ap = if c.access_point_name.is_empty() {
                format!("{}-NoAP", s.name.replace(",", "_"))
            } else {
                c.access_point_name.to_string()
            };
            let hostname = &c.hostname;
            let ipv4 = strip_ip(&c.ip);
            let ipv6 = "";
            let device_id = c.id.clone();
            let mac = &c.mac;

            csv += &format!(
                "{device_id},{ap},{mac},{hostname},{ipv4},{ipv6},{:.2},{:.2},{},{}\n",
                dl_mbps as f32 / 4.0,
                ul_mbps as f32 / 4.0,
                dl_mbps,
                ul_mbps,
            );
        });
    });

    use std::fs::File;
    use std::io::Write;
    let mut f = File::create("Shaper.csv")?;
    f.write_all(csv.as_bytes())?;
    Ok(())
}
