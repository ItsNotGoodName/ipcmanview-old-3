use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

use crate::models::{Cursor, QueryCameraFile};

use base64::Engine as _;

impl Cursor<'_> {
    fn from_(cursor: &str) -> Result<(i64, DateTime<Utc>)> {
        let cursor = String::from_utf8(base64::engine::general_purpose::STANDARD.decode(cursor)?)?;
        let (first, second) = cursor.split_once("_").context("no seperator")?;
        let id: i64 = first.parse()?;
        let time: DateTime<Utc> = DateTime::parse_from_rfc3339(&second)?.with_timezone(&Utc);
        Ok((id, time))
    }

    pub fn from(cursor: &str) -> Result<(i64, DateTime<Utc>)> {
        Self::from_(cursor).with_context(|| format!("invalid cursor: {cursor}"))
    }

    pub fn to(id: i64, time: DateTime<Utc>) -> String {
        base64::engine::general_purpose::STANDARD
            .encode(format!("{id}_{time}", time = time.to_rfc3339()))
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
    use super::*;

    #[test]
    fn it_file_query_cursor() {
        let (id, time) = (4, Utc::now());

        assert_eq!(Cursor::from(&Cursor::to(id, time)).unwrap(), (id, time));
    }
}
