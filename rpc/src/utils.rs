use base64::Engine as _;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{de, Deserialize, Deserializer, Serializer};

pub fn de_null_array_to_string_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_vec: Option<Vec<String>> = Option::deserialize(deserializer)?;
    Ok(opt_vec.unwrap_or_else(|| vec![]))
}

pub fn de_string_to_date_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::String(n) => {
            match from_timestamp(&n).map(|t| t.and_local_timezone(chrono::Local)) {
                Ok(chrono::LocalResult::Ambiguous(tz, _)) | Ok(chrono::LocalResult::Single(tz)) => {
                    Ok(tz.with_timezone(&Utc))
                }
                _ => Err(de::Error::custom("could not convert to local timezone")),
            }
        }
        _ => Err(de::Error::custom("expected string")),
    }
}

pub fn se_date_time_to_string<S>(
    date_time: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&to_timestamp(
        &date_time.with_timezone(&chrono::Local).naive_local(),
    ))
}

pub fn de_int_bool_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::Bool(b) => Ok(b as i64),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i)
            } else {
                Err(serde::de::Error::custom("invalid number value"))
            }
        }
        _ => Err(de::Error::custom("expected bool or number")),
    }
}

pub fn de_int_float_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::Number(n) => {
            if let Some(int_val) = n.as_i64() {
                Ok(int_val)
            } else if let Some(float_val) = n.as_f64() {
                Ok(float_val as i64)
            } else {
                Err(de::Error::custom("invalid number"))
            }
        }
        _ => Err(de::Error::custom("expected number")),
    }
}

pub fn de_number_string_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    match de::Deserialize::deserialize(deserializer)? {
        serde_json::Value::Number(n) => Ok(n.to_string()),
        serde_json::Value::String(s) => Ok(s),
        _ => Err(de::Error::custom("expected string or number")),
    }
}

#[derive(Deserialize, Debug)]
pub struct AuthParam {
    pub encryption: String,
    pub random: String,
    pub realm: String,
}

pub fn get_auth(username: &str, password: &str, param: &AuthParam) -> String {
    match param.encryption.as_str() {
        "Basic" => {
            base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", username, password))
        }
        "Default" => format!(
            "{:x}",
            md5::compute(format!(
                "{}:{}:{}",
                username,
                param.random,
                format!(
                    "{:x}",
                    md5::compute(format!("{}:{}:{}", username, param.realm, password))
                )
                .to_uppercase()
            ))
        )
        .to_uppercase(),
        _ => password.to_string(),
    }
}

pub fn parse_file_path_tags(file_path: &str) -> Vec<&str> {
    let file_path = if let Some(index) = file_path.rfind("/") {
        &file_path[index..]
    } else {
        file_path
    };

    let mut vec: Vec<&str> = vec![];

    for start_tag in file_path.split("[").skip(1) {
        if let Some(end) = start_tag.find("]") {
            vec.push(&start_tag[..end]);
        }
    }

    vec
}

pub fn from_timestamp(timestamp: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S")
}

pub fn to_timestamp(datetime: &NaiveDateTime) -> String {
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_get_auth() {
        let (username, password, params) = (
            "admin".to_string(),
            "123".to_string(),
            AuthParam {
                realm: "Login to a0c50bcd05b2f03d067e530d9bf069af".to_string(),
                random: "1172275829".to_string(),
                encryption: "Default".to_string(),
            },
        );

        assert_eq!(
            "2E9AD6D2DB08E0882F376A622BC76B9A",
            &get_auth(&username, &password, &params)
        )
    }

    #[test]
    fn it_parse_file_path_tags() {
        let data = [
            (
                "/mnt/sd/2023-04-09/001/jpg/07/12/04[M][0@0][0][].jpg",
                vec!["M", "0@0", "0", ""],
            ),
            ("04[M][0@0][0][].jpg", vec!["M", "0@0", "0", ""]),
            ("04M]0@0][0][].jpg", vec!["0", ""]),
            (
                "/mnt/dvr/mmc0p2_0/2023-04-09/0/jpg/09/44/34[M][0@0][7136][0].jpg",
                vec!["M", "0@0", "7136", "0"],
            ),
            (
                "/mnt/dvr/mmc0p2_0/2023-04-09/0/jpg/09/44/34[M][0@0][7136][0.jpg",
                vec!["M", "0@0", "7136"],
            ),
            (
                "/mnt/dvr/mmc0p2_0/2023-04-09/0/jpg/09/44/34M][0@0][7136].jpg",
                vec!["0@0", "7136"],
            ),
        ];

        for (input, output) in data.into_iter() {
            let tags = parse_file_path_tags(input);
            assert_eq!(tags, output);
        }
    }

    #[test]
    fn it_timestamp() {
        let data = [
            "2023-02-06 00:00:00",
            "2023-02-06 03:09:09",
            "2023-02-06 23:59:59",
        ];

        for i in data.into_iter() {
            assert_eq!(to_timestamp(&from_timestamp(i).unwrap()), i)
        }
    }
}
