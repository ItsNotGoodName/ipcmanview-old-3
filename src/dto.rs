use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateCamera {
    pub ip: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateCamera {
    pub id: i64,
    pub ip: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}
