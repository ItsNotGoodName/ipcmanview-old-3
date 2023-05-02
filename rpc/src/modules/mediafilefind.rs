use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::super::{
    utils::{
        de_null_array_to_string_vec, de_string_to_date_time, parse_file_path_tags,
        se_date_time_to_string,
    },
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
            types: vec!["dav", "jpg"],
            order: ConditionOrder::Ascent,
            redundant: "Exclusion",
            events: None,
            start_time,
            end_time,
            flags: vec!["Timing", "Event", "Event", "Manual"],
        }
    }

    pub fn video(mut self) -> Condition {
        self.types = vec!["dav"];
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
    #[serde(
        default,
        rename = "Flags",
        deserialize_with = "de_null_array_to_string_vec"
    )]
    pub flags: Vec<String>,
    #[serde(
        default,
        rename = "Events",
        deserialize_with = "de_null_array_to_string_vec"
    )]
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
        let mut tag_seeds = parse_file_path_tags(&self.file_path).into_iter().skip(2);

        let mut type_seed = 0;
        for c in self.r#type.chars() {
            type_seed += c as u32;
        }

        let ns = ((Self::to_num(tag_seeds.next()) + Self::to_num(tag_seeds.next()) + type_seed)
            % 500)
            * 1_000_000;

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
    Ok(rpc
        .method("mediaFileFind.factory.create")
        .send::<Value>()
        .await?
        .result_number())
}

pub async fn find_file(
    rpc: RequestBuilder,
    object: i64,
    condition: Condition,
) -> Result<bool, Error> {
    Ok(rpc
        .method("mediaFileFind.findFile")
        .params(json!({
            "condition": condition,
        }))
        .object(object)
        .send::<Value>()
        .await?
        .result())
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
        .params_map(|p, _| p.count)
}

pub async fn close(rpc: RequestBuilder, object: i64) -> Result<bool, Error> {
    Ok(rpc
        .method("mediaFileFind.close")
        .object(object)
        .send::<Value>()
        .await?
        .result())
}

pub async fn destroy(rpc: RequestBuilder, object: i64) -> Result<bool, Error> {
    Ok(rpc
        .method("mediaFileFind.destroy")
        .object(object)
        .send::<Value>()
        .await?
        .result())
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    fn new_info(
        file_path: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        r#type: &str,
    ) -> FindNextFileInfo {
        FindNextFileInfo {
            channel: 0,
            start_time,
            end_time,
            length: 0,
            r#type: r#type.to_string(),
            file_path: file_path.to_string(),
            duration: Some(0),
            disk: 0,
            video_stream: "".to_string(),
            flags: vec![],
            events: vec![],
            cluster: Some(0),
            partition: Some(0),
            pic_index: Some(0),
            repeat: Some(0),
            work_dir: Some("".to_string()),
            work_dir_sn: Some(0),
        }
    }

    #[test]
    fn it_find_next_file_info_unique_time() {
        let start_time = Utc::now();
        let end_time = start_time + Duration::seconds(5);

        let neq = [
            (
                new_info(
                    "/mnt/sd/2023-04-09/001/jpg/07/12/04[M][0@0][0][].jpg",
                    start_time,
                    end_time,
                    "jpg",
                ),
                new_info(
                    "/mnt/sd/2023-04-09/001/jpg/07/12/04[M][0@0][0][1].jpg",
                    start_time,
                    end_time,
                    "jpg",
                ),
            ),
            (
                new_info(
                    "/mnt/sd/2023-04-09/001/jpg/07/12/04[M][0@0][0][].jpg",
                    start_time,
                    end_time,
                    "dav",
                ),
                new_info(
                    "/mnt/sd/2023-04-09/001/jpg/07/12/04[M][0@0][0][].jpg",
                    start_time,
                    end_time,
                    "jpg",
                ),
            ),
        ];

        for (input, output) in neq.into_iter() {
            assert_ne!(input.unique_time(), output.unique_time());
        }

        let eq = [(
            new_info(
                "/mnt/sd/2023-04-09/001/jpg/07/12/04[M][0@0][0][].jpg",
                start_time,
                end_time,
                "jpg",
            ),
            new_info(
                "/mnt/sd/2023-04-09/001/jpg/07/12/04[M][0@0][0][].jpg",
                start_time,
                end_time,
                "jpg",
            ),
        )];

        for (input, output) in eq.into_iter() {
            assert_eq!(input.unique_time(), output.unique_time());
        }
    }
}
