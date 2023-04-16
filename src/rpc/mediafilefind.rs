use chrono::{DateTime, Timelike};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{
    rpclogin,
    utils::{self, de_string_to_date_time, se_date_time_to_string},
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

    pub fn unique_time(&self) -> (DateTime<chrono::Utc>, DateTime<chrono::Utc>) {
        let mut tags = utils::parse_file_path_tags(&self.file_path)
            .into_iter()
            .skip(2);

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

pub struct FindNextFileInfoStream<'a> {
    man: &'a mut rpclogin::Manager,
    object: i64,
    pub error: Option<Error>,
    count: i32,
    closed: bool,
}

pub async fn find_next_file_info_stream(
    man: &mut rpclogin::Manager,
    condition: Condition,
) -> Result<FindNextFileInfoStream, Error> {
    if let Err(e) = man.keep_alive_or_login().await {
        return Err(e);
    };

    let object = create(man.client.rpc()).await?;

    let closed = match find_file(man.client.rpc(), object, condition).await {
        Ok(o) => !o,
        Err(Error::NoData(_)) => true,
        Err(err) => return Err(err),
    };

    Ok(FindNextFileInfoStream {
        man,
        object,
        error: None,
        count: 64,
        closed,
    })
}

impl FindNextFileInfoStream<'_> {
    pub async fn next(&mut self) -> Option<Vec<FindNextFileInfo>> {
        if self.closed {
            return None;
        }

        if let Err(e) = self.man.keep_alive_or_login().await {
            self.error = Some(e);
            return None;
        };

        match find_next_file(self.man.client.rpc(), self.object, self.count).await {
            Ok(FindNextFile {
                found,
                infos: Some(infos),
            }) => {
                if found < self.count {
                    self.close().await;
                }
                Some(infos)
            }
            res => {
                self.close().await;
                if let Err(err) = res {
                    self.error = Some(err);
                }
                None
            }
        }
    }

    pub async fn close(&mut self) {
        if self.closed {
            return;
        }

        if let Err(e) = self.man.keep_alive_or_login().await {
            self.error = Some(e);
            return;
        };

        _ = close(self.man.client.rpc(), self.object).await;
        _ = destroy(self.man.client.rpc(), self.object).await;

        self.closed = true;
    }
}
