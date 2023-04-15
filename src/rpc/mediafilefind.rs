use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{
    utils::{de_string_to_date_time, se_date_time_to_string},
    Client, Error, RequestBuilder,
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
    pub start_time: DateTime<chrono::Utc>,
    #[serde(rename = "EndTime", serialize_with = "se_date_time_to_string")]
    pub end_time: DateTime<chrono::Utc>,
    #[serde(rename = "Flags")]
    pub flags: Vec<&'static str>,
}

#[derive(Deserialize, Debug)]
pub struct FindNextFileInfo {
    #[serde(rename = "Channel")]
    pub channel: i32,
    #[serde(rename = "StartTime", deserialize_with = "de_string_to_date_time")]
    pub start_time: DateTime<chrono::Utc>,
    #[serde(rename = "EndTime", deserialize_with = "de_string_to_date_time")]
    pub end_time: DateTime<chrono::Utc>,
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

#[derive(Deserialize, Debug)]
pub struct FindNextFile {
    pub found: i32,
    pub infos: Option<Vec<FindNextFileInfo>>,
}

#[derive(Deserialize, Debug)]
struct GetCount {
    count: i32,
}

pub fn create(rpc: RequestBuilder) -> Result<i64, Error> {
    rpc.method("mediaFileFind.factory.create")
        .send::<Value>()?
        .result_number()
}

pub fn find_file(rpc: RequestBuilder, object: i64, condition: Condition) -> Result<bool, Error> {
    rpc.method("mediaFileFind.findFile")
        .params(json!({
            "condition": condition,
        }))
        .object(object)
        .send::<Value>()?
        .result()
}

pub fn find_next_file(rpc: RequestBuilder, object: i64, count: i32) -> Result<FindNextFile, Error> {
    rpc.method("mediaFileFind.findNextFile")
        .params(json!({
            "count": count,
        }))
        .object(object)
        .send::<FindNextFile>()?
        .params()
}

pub fn get_count(rpc: RequestBuilder, object: i64) -> Result<i32, Error> {
    rpc.method("mediaFileFind.getCount")
        .object(object)
        .send::<GetCount>()?
        .params_as(|p, _| p.count)
}

pub fn close(rpc: RequestBuilder, object: i64) -> Result<bool, Error> {
    rpc.method("mediaFileFind.close")
        .object(object)
        .send::<Value>()?
        .result()
}

pub fn destroy(rpc: RequestBuilder, object: i64) -> Result<bool, Error> {
    rpc.method("mediaFileFind.destroy")
        .object(object)
        .send::<Value>()?
        .result()
}

impl Condition {
    pub fn new(start_time: DateTime<chrono::Utc>, end_time: DateTime<chrono::Utc>) -> Condition {
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

pub struct FindNextFileInfoIterator<'a> {
    client: &'a mut Client,
    object: i64,
    pub error: Option<Error>,
    count: i32,
    closed: bool,
}

pub fn find_next_file_info_iterator(
    client: &mut Client,
    condition: Condition,
) -> Result<FindNextFileInfoIterator, Error> {
    let object = create(client.rpc())?;
    find_file(client.rpc(), object, condition)?;
    Ok(FindNextFileInfoIterator {
        client,
        object,
        error: None,
        count: 64,
        closed: false,
    })
}

impl Iterator for FindNextFileInfoIterator<'_> {
    type Item = Vec<FindNextFileInfo>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.closed {
            return None;
        }

        match find_next_file(self.client.rpc(), self.object, self.count) {
            Ok(FindNextFile {
                found,
                infos: Some(infos),
            }) => {
                if found < self.count {
                    self.close();
                }
                Some(infos)
            }
            res => {
                self.close();
                if let Err(err) = res {
                    self.error = Some(err);
                }
                None
            }
        }
    }
}

impl FindNextFileInfoIterator<'_> {
    fn close(&mut self) {
        _ = close(self.client.rpc(), self.object);
        _ = destroy(self.client.rpc(), self.object);
        self.closed = true;
    }
}
