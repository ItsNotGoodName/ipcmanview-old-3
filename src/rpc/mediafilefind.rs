use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{
    utils::{de_string_to_date_time, parse_file_path_tags, se_date_time_to_string},
    Error, RequestBuilder,
};

#[derive(Serialize, Debug)]
pub enum ConditionOrder {
    Ascent,
    Descent,
}

#[derive(Serialize, Debug)]
pub struct Condition {
    #[serde(rename = "Channel")]
    pub channel: i32,
    #[serde(rename = "Dirs")]
    pub dirs: Vec<String>,
    #[serde(rename = "Types")]
    pub types: Vec<&'static str>,
    #[serde(rename = "Order")]
    pub order: ConditionOrder,
    #[serde(rename = "Redundant")]
    pub redundant: &'static str,
    #[serde(rename = "Events")]
    pub events: Option<Vec<&'static str>>,
    #[serde(rename = "StartTime", serialize_with = "se_date_time_to_string")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime", serialize_with = "se_date_time_to_string")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "Flags")]
    pub flags: Vec<&'static str>,
}

impl Condition {
    pub fn new(start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Condition {
        Condition {
            channel: 0,
            dirs: vec![],
            types: vec!["dav"],
            order: ConditionOrder::Ascent,
            redundant: "Exclusion",
            events: None,
            start_time,
            end_time,
            flags: vec![""],
        }
    }

    pub fn video(mut self) -> Condition {
        self.types = vec!["dav"];
        self.flags = vec!["Timing", "Event", "Event", "Manual"];
        self
    }

    pub fn picture(mut self) -> Condition {
        self.types = vec!["jpg"];
        self.flags = vec!["Timing", "Event", "Event"];
        self
    }
}

#[derive(Deserialize, Debug)]
pub struct FindNextFileInfo {
    #[serde(rename = "Channel")]
    pub channel: i32,
    #[serde(rename = "StartTime", deserialize_with = "de_string_to_date_time")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime", deserialize_with = "de_string_to_date_time")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "Length")]
    pub length: i32,
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "FilePath")]
    pub file_path: String,
    #[serde(rename = "Duration")]
    pub duration: Option<i32>,
    #[serde(rename = "Disk")]
    pub disk: i32,
    #[serde(rename = "VideoStream")]
    pub video_stream: String,
    #[serde(rename = "Flags")]
    pub flags: Vec<String>,
    #[serde(rename = "Events")]
    pub events: Vec<String>,
    #[serde(rename = "Cluster")]
    pub cluster: Option<i32>,
    #[serde(rename = "Partition")]
    pub partition: Option<i32>,
    #[serde(rename = "PicIndex")]
    pub pic_index: Option<i32>,
    #[serde(rename = "Repeat")]
    pub repeat: Option<i32>,
    #[serde(rename = "WorkDir")]
    pub work_dir: Option<String>,
    #[serde(rename = "WorkDirSN")]
    pub work_dir_sn: Option<i32>,
}

impl FindNextFileInfo {
    fn to_num(s: Option<&str>) -> u32 {
        match s {
            Some(s) => match s.parse() {
                Ok(o) => o,
                Err(_) => 0,
            },
            None => 0,
        }
    }

    pub fn unique_time(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        let mut tags = parse_file_path_tags(&self.file_path).into_iter().skip(2);

        let ns = ((Self::to_num(tags.next()) + Self::to_num(tags.next())) % 500) * 1_000_000;

        (
            self.start_time
                .with_nanosecond(ns)
                .unwrap_or_else(|| self.start_time),
            self.end_time
                .with_nanosecond(ns)
                .unwrap_or_else(|| self.end_time),
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct FindNextFile {
    pub found: i32,
    pub infos: Option<Vec<FindNextFileInfo>>,
}

#[derive(Deserialize, Debug)]
struct GetCount {
    count: i32,
}

pub async fn create(rpc: RequestBuilder) -> Result<i64, Error> {
    rpc.method("mediaFileFind.factory.create")
        .send::<Value>()
        .await?
        .result_number()
}

pub async fn find_file(
    rpc: RequestBuilder,
    object: i64,
    condition: Condition,
) -> Result<bool, Error> {
    rpc.method("mediaFileFind.findFile")
        .params(json!({
            "condition": condition,
        }))
        .object(object)
        .send::<Value>()
        .await?
        .result()
}

pub async fn find_next_file(
    rpc: RequestBuilder,
    object: i64,
    count: i32,
) -> Result<FindNextFile, Error> {
    rpc.method("mediaFileFind.findNextFile")
        .params(json!({
            "count": count,
        }))
        .object(object)
        .send::<FindNextFile>()
        .await?
        .params()
}

pub async fn get_count(rpc: RequestBuilder, object: i64) -> Result<i32, Error> {
    rpc.method("mediaFileFind.getCount")
        .object(object)
        .send::<GetCount>()
        .await?
        .params_as(|p, _| p.count)
}

pub async fn close(rpc: RequestBuilder, object: i64) -> Result<bool, Error> {
    rpc.method("mediaFileFind.close")
        .object(object)
        .send::<Value>()
        .await?
        .result()
}

pub async fn destroy(rpc: RequestBuilder, object: i64) -> Result<bool, Error> {
    rpc.method("mediaFileFind.destroy")
        .object(object)
        .send::<Value>()
        .await?
        .result()
}
