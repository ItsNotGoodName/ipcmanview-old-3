use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::models::{QueryCameraFile, QueryCameraFileCursor, QueryCameraFileFilter};

impl QueryCameraFileCursor {
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
        Self::from_(cursor).with_context(|| format!("invalid cursor: {cursor}"))
    }

    pub fn to(id: i64, time: DateTime<Utc>) -> String {
        format!("{id}_{time}", time = time.timestamp_millis())
    }
}

impl<'a> QueryCameraFileFilter<'a> {
    pub fn new() -> Self {
        QueryCameraFileFilter {
            begin: None,
            end: None,
            camera_ids: vec![],
            kinds: vec![],
        }
    }

    pub fn kinds(mut self, kinds: Vec<&'a str>) -> Self {
        self.kinds = kinds;
        self
    }

    pub fn camera_ids(mut self, camera_ids: Vec<i64>) -> Self {
        self.camera_ids = camera_ids;
        self
    }

    pub fn maybe_begin(mut self, begin: Option<&'a str>) -> Result<Self> {
        if let Some(begin) = begin {
            self.begin = Some(
                // TODO: move to FromParam rocket
                DateTime::parse_from_rfc3339(begin)
                    .map(|f| f.with_timezone(&Utc))
                    .context("parsing begin")?,
            );
        }
        Ok(self)
    }

    pub fn maybe_end(mut self, end: Option<&'a str>) -> Result<Self> {
        if let Some(end) = end {
            self.end = Some(
                // TODO: move to FromParam rocket
                DateTime::parse_from_rfc3339(end)
                    .map(|f| f.with_timezone(&Utc))
                    .context("parsing end")?,
            );
        }
        Ok(self)
    }
}

#[derive(Debug)]
pub struct QueryCameraFileBuilder<'a>(QueryCameraFile<'a>);

impl<'a> QueryCameraFile<'a> {
    pub fn new(filter: &'a QueryCameraFileFilter) -> Self {
        QueryCameraFile {
            cursor: QueryCameraFileCursor::None,
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

    pub fn maybe_after(mut self, cursor: Option<&'a str>) -> Result<Self> {
        if let Some(cursor) = cursor {
            if !cursor.is_empty() {
                self.cursor = QueryCameraFileCursor::After(QueryCameraFileCursor::from(cursor)?);
            }
        }
        Ok(self)
    }

    pub fn maybe_before(mut self, cursor: Option<&'a str>) -> Result<Self> {
        if let Some(cursor) = cursor {
            if !cursor.is_empty() {
                self.cursor = QueryCameraFileCursor::Before(QueryCameraFileCursor::from(cursor)?);
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
            QueryCameraFileCursor::from(&QueryCameraFileCursor::to(id, time)).unwrap(),
            (id, time)
        );
    }
}
