use std::ops::AddAssign;

pub struct CreateCamera {
    pub ip: String,
    pub username: String,
    pub password: String,
}

pub struct UpdateCamera {
    pub id: i64,
    pub ip: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub struct Camera {
    pub id: i64,
    pub ip: String,
    pub username: String,
    pub password: String,
}

#[derive(Default, Debug)]
pub struct CameraScanResult {
    pub upserted: u64,
    pub deleted: u64,
}

impl AddAssign for CameraScanResult {
    fn add_assign(&mut self, rhs: Self) {
        self.upserted += rhs.upserted;
        self.deleted += rhs.deleted;
    }
}
