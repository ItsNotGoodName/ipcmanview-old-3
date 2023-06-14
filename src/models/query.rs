use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::models::{CameraFileQuery, CameraFileQueryCursor, CameraFileQueryFilter};

impl CameraFileQueryCursor {
    fn from_(cursor: &str) -> Result<(i64, DateTime<Utc>)> {
        let (first, second) = cursor.split_once("_").context("no seperator")?;
        let id: i64 = first.parse()?;
        let tsms: i64 = second.parse()?;
        let time = NaiveDateTime::from_timestamp_millis(tsms)
            .context("invalid time")?
            .and_local_timezone(Utc)
            .earliest()
            .context("parse time")?;

        Ok((id, time))
    }

    pub fn from(cursor: &str) -> Result<(i64, DateTime<Utc>)> {
        Self::from_(cursor).with_context(|| format!("Invalid cursor {cursor}."))
    }

    pub fn to(id: i64, time: DateTime<Utc>) -> String {
        format!("{id}_{time}", time = time.timestamp_millis())
    }
}

impl CameraFileQueryFilter {
    pub fn new() -> Self {
        CameraFileQueryFilter {
            start: None,
            end: None,
            camera_ids: vec![],
            kinds: vec![],
            events: vec![],
        }
    }

    pub fn kinds(mut self, kinds: Vec<String>) -> Self {
        self.kinds = kinds;
        self
    }

    pub fn events(mut self, events: Vec<String>) -> Self {
        self.events = events;
        self
    }

    pub fn camera_ids(mut self, camera_ids: Vec<i64>) -> Self {
        self.camera_ids = camera_ids;
        self
    }

    pub fn start(mut self, start: Option<DateTime<Utc>>) -> Self {
        self.start = start;
        self
    }

    pub fn end(mut self, end: Option<DateTime<Utc>>) -> Self {
        self.end = end;
        self
    }
}

impl<'a> CameraFileQuery<'a> {
    pub fn new(filter: &'a CameraFileQueryFilter) -> Self {
        CameraFileQuery {
            cursor: CameraFileQueryCursor::None,
            limit: 25,
            filter,
        }
    }

    pub fn maybe_limit(mut self, limit: Option<i32>) -> Self {
        if let Some(limit) = limit {
            self.limit = if limit > 100 {
                100
            } else if limit < 10 {
                10
            } else {
                limit
            };
        }
        self
    }

    pub fn maybe_after(mut self, cursor: Option<String>) -> Result<Self> {
        if let Some(cursor) = cursor {
            if !cursor.is_empty() {
                self.cursor = CameraFileQueryCursor::After(CameraFileQueryCursor::from(&cursor)?);
            }
        }
        Ok(self)
    }

    pub fn maybe_before(mut self, cursor: Option<String>) -> Result<Self> {
        if let Some(cursor) = cursor {
            if !cursor.is_empty() {
                self.cursor = CameraFileQueryCursor::Before(CameraFileQueryCursor::from(&cursor)?);
            }
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;

    use super::*;

    #[test]
    fn it_file_query_cursor() {
        let (id, time) = (4, Utc::now().with_nanosecond(0).unwrap());

        assert_eq!(
            CameraFileQueryCursor::from(&CameraFileQueryCursor::to(id, time)).unwrap(),
            (id, time)
        );
    }
}
