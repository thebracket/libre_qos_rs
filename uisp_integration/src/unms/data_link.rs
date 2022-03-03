use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DataLink {
    pub id: String,
    pub from: DataLinkFrom,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DataLinkFrom {
    pub device: DataLinkDevice,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DataLinkDevice {
    pub identification: DataLinkDeviceIdentification,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DataLinkDeviceIdentification {
    pub id: String,
    pub name: String,
}
