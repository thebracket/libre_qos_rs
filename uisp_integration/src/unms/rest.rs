use serde::de::DeserializeOwned;

/// Submits a request to the UNMS API and returns the result as unprocessed text.
#[allow(dead_code)]
pub async fn nms_request_get_text(
    url: &str,
    key: &str,
    api: &str,
) -> Result<String, reqwest::Error> {
    let full_url = format!("{}/{}", api, url);
    let client = reqwest::Client::new();

    let res = client
        .get(&full_url)
        .header("'Content-Type", "application/json")
        .header("X-Auth-Token", key)
        .send()
        .await
        .unwrap();

    res.text().await
}

/// Submits a request to the UNMS API, returning a deserialized vector of type T.
#[allow(dead_code)]
pub async fn nms_request_get_vec<T>(
    url: &str,
    key: &str,
    api: &str,
) -> Result<Vec<T>, reqwest::Error>
where
    T: DeserializeOwned,
{
    let full_url = format!("{}/{}", api, url);
    //println!("{full_url}");
    let client = reqwest::Client::new();

    let res = client
        .get(&full_url)
        .header("'Content-Type", "application/json")
        .header("X-Auth-Token", key)
        .send()
        .await
        .unwrap();

    res.json::<Vec<T>>().await
}
