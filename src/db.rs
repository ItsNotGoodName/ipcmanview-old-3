use sqlx::{Acquire, SqliteConnection};

use crate::core;

use crate::rpc::{
    self, magicbox,
    rpclogin::{self, Manager},
};

struct Camera {
    ip: String,
    username: String,
    password: String,
}

pub async fn camera_manager_get(
    pool: &mut SqliteConnection,
    id: i64,
    agent: ureq::Agent,
) -> Result<core::Camera, sqlx::Error> {
    let camera = sqlx::query_as!(
        Camera,
        r#"
        SELECT ip, username, password FROM cameras WHERE id = ?
        "#,
        id,
    )
    .fetch_one(pool)
    .await?;

    let man = rpclogin::Manager::new(rpc::Client::new(camera.ip, agent))
        .username(camera.username)
        .password(camera.password)
        .unlock();

    Ok(core::Camera { id, man })
}

pub async fn camera_add(
    pool: &mut SqliteConnection,
    man: Manager,
) -> Result<core::Camera, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let id = sqlx::query!(
        r#"
        INSERT INTO cameras
        (ip, username, password)
        VALUES
        (?1, ?2, ?3);
        
        "#,
        man.client.ip,
        man.username,
        man.password,
    )
    .execute(&mut *tx)
    .await?
    .last_insert_rowid();

    sqlx::query!(
        r#"
        INSERT INTO camera_details
        (id)
        VALUES
        (?1)
        "#,
        id
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO camera_software_version
        (id)
        VALUES
        (?1)
        "#,
        id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(core::Camera { id, man })
}

pub async fn camera_detail_update(
    pool: &mut SqliteConnection,
    cam: &mut core::Camera,
) -> Result<(), sqlx::Error> {
    let sn = cam.man.rpc(|rpc| magicbox::get_serial_no(rpc)).ok();
    let device_class = cam.man.rpc(|rpc| magicbox::get_device_class(rpc)).ok();
    let device_type = cam.man.rpc(|rpc| magicbox::get_device_type(rpc)).ok();
    let hardware_version = cam.man.rpc(|rpc| magicbox::get_hardware_version(rpc)).ok();
    let market_area = cam.man.rpc(|rpc| magicbox::get_market_area(rpc)).ok();
    let process_info = cam.man.rpc(|rpc| magicbox::get_process_info(rpc)).ok();
    let vendor = cam.man.rpc(|rpc| magicbox::get_vendor(rpc)).ok();

    sqlx::query!(
        r#"
        UPDATE camera_details SET 
        sn = coalesce(?2, sn),
        device_class = coalesce(?3, device_class),
        device_type = coalesce(?4, device_type),
        hardware_version = coalesce(?5, hardware_version),
        market_area = coalesce(?6, market_area),
        process_info = coalesce(?7, process_info),
        vendor = coalesce(?8, vendor)
        WHERE id = ?1
        "#,
        cam.id,
        sn,
        device_class,
        device_type,
        hardware_version,
        market_area,
        process_info,
        vendor
    )
    .execute(pool)
    .await
    .map(|_| ())
}

pub async fn camera_software_version_update(
    pool: &mut SqliteConnection,
    cam: &mut core::Camera,
) -> Result<(), sqlx::Error> {
    let res = if let Ok(res) = cam.man.rpc(|rpc| magicbox::get_software_version(rpc)) {
        res
    } else {
        return Ok(());
    };

    sqlx::query!(
        r#"
        UPDATE camera_software_version SET 
        build = ?2,
        build_date = ?3,
        security_base_line_version = ?4,
        version = ?5,
        web_version = ?6
        WHERE id = ?1
        "#,
        cam.id,
        res.build,
        res.build_date,
        res.security_base_line_version,
        res.version,
        res.web_version
    )
    .execute(pool)
    .await
    .map(|_| ())
}
