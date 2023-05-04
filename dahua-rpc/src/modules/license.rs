use serde::{Deserialize, Serialize};

use crate::{Error, RequestBuilder};

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    #[serde(rename = "AbroadInfo")]
    pub abroad_info: String,
    #[serde(rename = "AllType")]
    pub all_type: bool,
    #[serde(rename = "DigitChannel")]
    pub digit_channel: u32,
    #[serde(rename = "EffectiveDays")]
    pub effective_days: u32,
    #[serde(rename = "EffectiveTime")]
    pub effective_time: u32,
    #[serde(rename = "LicenseID")]
    pub license_id: u32,
    #[serde(rename = "ProductType")]
    pub product_type: String,
    #[serde(rename = "Status")]
    pub status: u32,
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoContainer {
    #[serde(rename = "Info")]
    pub info: Info,
}

pub async fn get_license_info(rpc: RequestBuilder) -> Result<Vec<InfoContainer>, Error> {
    rpc.method("License.getLicenseInfo")
        .send::<Vec<InfoContainer>>()
        .await?
        .params()
}
