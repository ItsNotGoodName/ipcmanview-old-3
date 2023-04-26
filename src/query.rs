use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::models::{Cursor, QueryCameraFile};

impl Cursor<'_> {
    fn from_(cursor: &str) -> Result<(i64, DateTime<Utc>)> {
        let (first, second) = cursor.split_once("_").context("no seperator")?;
        let id: i64 = first.parse()?;
        let tsms: i64 = second.parse()?;
        let time = match NaiveDateTime::from_timestamp_millis(tsms)
            .context("invalid time")?
            .and_local_timezone(Utc)
        {
            chrono::LocalResult::Ambiguous(tz, _) | chrono::LocalResult::Single(tz) => {
                Ok(tz.with_timezone(&Utc))
            }
            chrono::LocalResult::None => Err(anyhow!("parse time")),
        }?;

        Ok((id, time))
    }

    pub fn from(cursor: &str) -> Result<(i64, DateTime<Utc>)> {
        Self::from_(cursor).with_context(|| format!("invalid cursor: {cursor}"))
    }

    pub fn to(id: i64, time: DateTime<Utc>) -> String {
        format!("{id}_{time}", time = time.timestamp_millis())
    }
}

#[derive(Debug)]
pub struct QueryCameraFileBuilder<'a> {
    cursor: Cursor<'a>,
    limit: i32,
}

impl<'a> QueryCameraFileBuilder<'a> {
    pub fn new() -> QueryCameraFileBuilder<'a> {
        QueryCameraFileBuilder {
            cursor: Cursor::None,
            limit: 25,
        }
    }

    pub fn limit(mut self, limit: Option<i32>) -> Self {
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

    pub fn after(mut self, cursor: Option<&'a str>) -> Self {
        if let Some(cursor) = cursor {
            if !cursor.is_empty() {
                self.cursor = Cursor::After(cursor);
            }
        }
        self
    }

    pub fn before(mut self, cursor: Option<&'a str>) -> Self {
        if let Some(cursor) = cursor {
            if !cursor.is_empty() {
                self.cursor = Cursor::Before(cursor);
            }
        }
        self
    }

    pub fn build(self) -> QueryCameraFile<'a> {
        QueryCameraFile {
            cursor: self.cursor,
            limit: self.limit,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;

    use super::*;

    #[test]
    fn it_file_query_cursor() {
        let (id, time) = (4, Utc::now().with_nanosecond(0).unwrap());

        assert_eq!(Cursor::from(&Cursor::to(id, time)).unwrap(), (id, time));
    }
}
