use dahua_rpc_derive::{config_table, ConfigTable};
use serde::{Deserialize, Serialize};

#[config_table]
#[derive(Serialize, Deserialize, Debug, ConfigTable)]
pub struct General {
    #[serde(rename = "LocalNo")]
    pub local_no: i32,
    #[serde(rename = "LockLoginEnable")]
    pub lock_login_enable: bool,
    #[serde(rename = "LockLoginTimes")]
    pub lock_login_times: i32,
    #[serde(rename = "LoginFailLockTime")]
    pub login_fail_lock_time: i32,
    #[serde(rename = "MachineName")]
    pub machine_name: String,
    #[serde(rename = "MaxOnlineTime")]
    pub max_online_time: i32,
}

#[config_table]
#[derive(Serialize, Deserialize, Debug, ConfigTable)]
pub struct NTP {
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Enable")]
    pub enable: bool,
    #[serde(rename = "Port")]
    pub port: i32,
    #[serde(rename = "TimeZone")]
    pub time_zone: i32,
    #[serde(rename = "TimeZoneDesc")]
    pub time_zone_desc: String,
    #[serde(rename = "UpdatePeriod")]
    pub update_period: i32,
}

#[config_table]
#[derive(Serialize, Deserialize, Debug, ConfigTable)]
pub struct VideoInMode(Vec<_VideoInMode>);

#[config_table]
#[derive(Serialize, Deserialize, Debug)]
pub struct _VideoInMode {
    #[serde(rename = "Config")]
    pub config: Vec<i32>,
    #[serde(rename = "Mode")]
    pub mode: i32,
    #[serde(rename = "TimeSection")]
    pub time_section: Vec<Vec<String>>,
}

#[config_table]
#[derive(Serialize, Deserialize, Debug, ConfigTable)]
pub struct Email {
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Anonymous")]
    pub anonymous: bool,
    #[serde(rename = "AttachEnable")]
    pub attach_enable: bool,
    #[serde(rename = "Authentication")]
    pub authentication: bool,
    // #[serde(rename = "CustomTitle")]
    // pub custom_title: Vec<Value>,
    #[serde(rename = "Enable")]
    pub enable: bool,
    #[serde(rename = "HealthReport")]
    pub health_report: _EmailHealthReport,
    #[serde(rename = "OnlyAttachment")]
    pub only_attachment: bool,
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "Port")]
    pub port: i32,
    #[serde(rename = "Receivers")]
    pub receivers: Vec<String>,
    #[serde(rename = "SendAddress")]
    pub send_address: String,
    #[serde(rename = "SendInterv")]
    pub send_interv: i32,
    #[serde(rename = "SslEnable")]
    pub ssl_enable: bool,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "TlsEnable")]
    pub tls_enable: bool,
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[config_table]
#[derive(Serialize, Deserialize, Debug)]
pub struct _EmailHealthReport {
    #[serde(rename = "Enable")]
    pub enable: bool,
    #[serde(rename = "Interval")]
    pub interval: i32,
}

#[config_table]
#[derive(Serialize, Deserialize, Debug, ConfigTable)]
pub struct Locales {
    #[serde(rename = "DSTEnable")]
    pub dst_enable: bool,
    #[serde(rename = "DSTEnd")]
    pub dst_end: _LocalesDSTEnd,
    #[serde(rename = "DSTStart")]
    pub dst_start: _LocalesDSTStart,
    #[serde(rename = "TimeFormat")]
    pub time_format: String,
}

#[config_table]
#[derive(Serialize, Deserialize, Debug)]
pub struct _LocalesDSTEnd {
    #[serde(rename = "Day")]
    pub day: i32,
    #[serde(rename = "Hour")]
    pub hour: i32,
    #[serde(rename = "Minute")]
    pub minute: i32,
    #[serde(rename = "Month")]
    pub month: i32,
    #[serde(rename = "Week")]
    pub week: i32,
    #[serde(rename = "Year")]
    pub year: i32,
}

#[config_table]
#[derive(Serialize, Deserialize, Debug)]
pub struct _LocalesDSTStart {
    #[serde(rename = "Day")]
    pub day: i32,
    #[serde(rename = "Hour")]
    pub hour: i32,
    #[serde(rename = "Minute")]
    pub minute: i32,
    #[serde(rename = "Month")]
    pub month: i32,
    #[serde(rename = "Week")]
    pub week: i32,
    #[serde(rename = "Year")]
    pub year: i32,
}

#[config_table]
#[derive(Serialize, Deserialize, Debug, ConfigTable)]
pub struct DisableLinkage {
    #[serde(rename = "Enable")]
    pub enable: bool,
}

#[config_table]
#[derive(Serialize, Deserialize, Debug, ConfigTable)]
pub struct DisableLinkageTimeSection {
    #[serde(rename = "Enable")]
    pub enable: bool,
    #[serde(rename = "TimeSection")]
    pub time_section: Vec<Vec<String>>,
}

#[config_table]
#[derive(Serialize, Deserialize, Debug, ConfigTable)]
pub struct DisableEmailLinkage {
    #[serde(rename = "Enable")]
    pub enable: bool,
    #[serde(rename = "Name")]
    pub name: String,
}
