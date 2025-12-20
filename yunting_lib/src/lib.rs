use std::time::SystemTime;

use crate::error::RequestError;
use crate::model::{ProvinceResponse, RadioInfo, RadioListResponse};

const KEY: &str = "f0fc4c668392f9f9a447e48584c214ee";

pub mod config;
mod crypto;
mod error;
mod model;
mod windows_util;

pub fn get_app_folder(game_name: &str) -> Option<std::path::PathBuf> {
    let my_documents = windows_util::SpecialFolder::MyDocuments
        .get()
        .or_else(|| std::env::home_dir().map(|p| p.join("Documents")))?;

    Some(my_documents.join(game_name))
}

pub(crate) fn get_current_unix_epoch() -> u128 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time should go forward")
        .as_millis()
}

pub fn format_live_streams(radios: &[RadioInfo]) -> String {
    // TODO: handle m3u8 streams
    let radios = radios
        .iter()
        .filter(|radio| !radio.mp3_play_url_high.contains("m3u8"))
        .filter(|radio| !radio.mp3_play_url_high.is_empty())
        .collect::<Vec<_>>();
    let mut s = r#"SiiNunit
{
live_stream_def : _nameless.204.d25f.7590 {
 stream_data: "#
        .to_string();
    s.push_str(&radios.len().to_string());
    s.push('\n');
    for (i, radio) in radios.iter().enumerate() {
        s.push_str(&format!(
            " stream_data[{}]: \"{}|{}|Radio|ZHO|192|0\"\n",
            i, radio.mp3_play_url_high, radio.title
        ));
    }
    s.push_str(
        r#"}
}"#,
    );
    s
}

fn create_headers(sign: &str) -> reqwest::header::HeaderMap {
    let timestamp = get_current_unix_epoch();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Sign", sign.parse().unwrap());
    headers.insert("Timestamp", timestamp.to_string().parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("equipmentId", "0000".parse().unwrap());
    headers.insert("platformCode", "WEB".parse().unwrap());
    headers
}

pub async fn get_radio_list(province_code: u64) -> Result<RadioListResponse, RequestError> {
    let timestamp = get_current_unix_epoch();
    let sign = crypto::md5hash(&format!(
        "categoryId=0&provinceCode={}&timestamp={}&key={}",
        province_code, timestamp, KEY
    ));
    let headers = create_headers(&sign);

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()?;

    Ok(client
        .get(format!(
            "https://ytmsout.radio.cn/web/appBroadcast/list?categoryId=0&provinceCode={}",
            province_code
        ))
        .send()
        .await?
        .json()
        .await?)
}

pub async fn list_all_provinces() -> Result<ProvinceResponse, RequestError> {
    let timestamp = get_current_unix_epoch();
    let sign = crypto::md5hash(&format!("timestamp={}&key={}", timestamp, KEY));
    let headers = create_headers(&sign);

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()?;
    Ok(client
        .get("https://ytmsout.radio.cn/web/appProvince/list/all")
        .send()
        .await?
        .json()
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_radio_list() {
        let response = get_radio_list(110000).await.unwrap();
        assert!(response.data.is_some());
    }
}
