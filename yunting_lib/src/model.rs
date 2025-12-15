use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,

    #[serde(rename = "extInfo")]
    pub ext_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvinceInfo {
    pub province_name: String,
    pub province_code: u64,
}

pub type ProvinceResponse = Response<Vec<ProvinceInfo>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioInfo {
    pub content_id: String,
    pub title: String,
    pub subtitle: String,
    pub image: String,
    pub play_url_low: String,
    pub mp3_play_url_low: String,
    pub mp3_play_url_high: String,
    pub play_url_multi: String,
}

pub type RadioListResponse = Response<Vec<RadioInfo>>;
