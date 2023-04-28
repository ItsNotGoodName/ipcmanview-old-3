use std::sync::Arc;

use anyhow::{bail, Context, Result};
use rpc::{
    modules::{magicbox, mediafilefind},
    reqwest, Client, Error, RequestBuilder, ResponseError, ResponseKind,
};
use tokio::sync::Mutex;

/// Turns rpc::ResponseError to Ok(None), Ok(o) to Ok(Some(o)) and, any other error to Err(rpc::Error)
fn maybe<T>(check: Result<T, Error>) -> Result<Option<T>, Error> {
    match check {
        Err(Error::Response(_)) => Ok(check.ok()),
        Ok(o) => Ok(Some(o)),
        Err(e) => Err(e),
    }
}

#[derive(Debug)]
pub struct IpcFile {
    pub cookie: String,
    pub url: String,
}

#[derive(Clone)]
pub struct IpcManager {
    pub id: i64,
    pub client: Arc<Mutex<Client>>,
}

impl IpcManager {
    pub fn new(id: i64, client: Client) -> IpcManager {
        IpcManager {
            id,
            client: Arc::new(Mutex::new(client)),
        }
    }

    pub async fn rpc(&self) -> Result<RequestBuilder, Error> {
        let mut client = self.client.lock().await;
        client.keep_alive_or_login().await.map(|_| client.rpc())
    }

    pub async fn file(&self, file_path: &str) -> Result<IpcFile, Error> {
        let mut client = self.client.lock().await;
        client.keep_alive_or_login().await?;

        Ok(IpcFile {
            cookie: client.cookie(),
            url: client.file_url(file_path),
        })
    }

    pub async fn close(&self) {
        let mut client = self.client.lock().await;
        client.logout().await;
        client.state = rpc::State::Error(rpc::LoginError::Closed)
    }
}

pub struct IpcDetail {
    pub sn: Option<String>,
    pub device_class: Option<String>,
    pub device_type: Option<String>,
    pub hardware_version: Option<String>,
    pub market_area: Option<String>,
    pub process_info: Option<String>,
    pub vendor: Option<String>,
}

impl IpcDetail {
    pub async fn get(man: &IpcManager) -> Result<IpcDetail, Error> {
        Ok(IpcDetail {
            sn: maybe(magicbox::get_serial_no(man.rpc().await?).await)?,
            device_class: maybe(magicbox::get_device_class(man.rpc().await?).await)?,
            device_type: maybe(magicbox::get_device_type(man.rpc().await?).await)?,
            hardware_version: maybe(magicbox::get_hardware_version(man.rpc().await?).await)?,
            market_area: maybe(magicbox::get_market_area(man.rpc().await?).await)?,
            process_info: maybe(magicbox::get_process_info(man.rpc().await?).await)?,
            vendor: maybe(magicbox::get_vendor(man.rpc().await?).await)?,
        })
    }
}

pub struct IpcSoftware(pub Option<magicbox::GetSoftwareVersion>);

impl IpcSoftware {
    pub async fn get(man: &IpcManager) -> Result<IpcSoftware, Error> {
        Ok(IpcSoftware(maybe(
            magicbox::get_software_version(man.rpc().await?).await,
        )?))
    }
}

pub struct IpcFileStream<'a> {
    man: &'a IpcManager,
    object: i64,
    pub error: Option<Error>,
    count: i32,
    closed: bool,
}

impl IpcFileStream<'_> {
    pub async fn new(
        man: &IpcManager,
        condition: mediafilefind::Condition,
    ) -> Result<IpcFileStream, Error> {
        let object = mediafilefind::create(man.rpc().await?).await?;

        let closed = match mediafilefind::find_file(man.rpc().await?, object, condition).await {
            Ok(o) => !o,
            Err(Error::Response(ResponseError {
                kind: ResponseKind::NoData,
                ..
            })) => true,
            Err(err) => return Err(err),
        };

        Ok(IpcFileStream {
            man,
            object,
            error: None,
            count: 64,
            closed,
        })
    }

    pub async fn next(&mut self) -> Option<Vec<mediafilefind::FindNextFileInfo>> {
        if self.closed {
            return None;
        }

        let rpc = match self.man.rpc().await {
            Ok(o) => o,
            Err(err) => {
                self.error = Some(err);
                self.close().await;
                return None;
            }
        };

        match mediafilefind::find_next_file(rpc, self.object, self.count).await {
            Ok(mediafilefind::FindNextFile {
                found,
                infos: Some(infos),
            }) => {
                if found < self.count {
                    self.close().await;
                }
                Some(infos)
            }
            res => {
                if let Err(err) = res {
                    self.error = Some(err);
                }
                self.close().await;
                None
            }
        }
    }

    pub async fn close(&mut self) {
        if self.closed {
            return;
        }
        let rpc = match self.man.rpc().await {
            Ok(o) => o,
            Err(_) => return,
        };
        mediafilefind::close(rpc, self.object).await.ok();
        let rpc = match self.man.rpc().await {
            Ok(o) => o,
            Err(_) => return,
        };
        mediafilefind::destroy(rpc, self.object).await.ok();
        self.closed = true;
    }
}

#[derive(Clone)]
pub struct IpcManagerStore {
    mans: Arc<Mutex<Vec<IpcManager>>>,
    client: reqwest::Client,
}

use crate::models::ICamera;

impl ICamera {
    pub fn to_camera_manager(self, client: reqwest::Client) -> IpcManager {
        IpcManager::new(
            self.id,
            rpc::Client::new(client, self.ip, self.username, self.password),
        )
    }
}

impl IpcManagerStore {
    pub async fn new(pool: &sqlx::SqlitePool) -> Result<IpcManagerStore> {
        let client = rpc::recommended_reqwest_client_builder()
            .build()
            .context("Failed to build reqwest client")?;
        let mans = ICamera::list(pool)
            .await?
            .into_iter()
            .map(|c| c.to_camera_manager(client.clone()))
            .collect();

        Ok(IpcManagerStore {
            mans: Arc::new(Mutex::new(mans)),
            client,
        })
    }

    pub async fn refresh(&self, pool: &sqlx::SqlitePool, id: i64) -> Result<()> {
        let mut mans = self.mans.lock().await;
        let icam = ICamera::find(pool, id).await?;
        let old = mans.iter().enumerate().find(|(_, old)| old.id == id);
        match (icam, old) {
            // Update
            (Some(icam), Some((idx, old))) => {
                old.close().await;
                mans[idx] = icam.to_camera_manager(self.client.clone());
            }
            // Add
            (Some(icam), None) => mans.push(icam.to_camera_manager(self.client.clone())),
            // Delete
            (None, Some((idx, _))) => {
                mans.remove(idx);
            }
            (None, None) => {}
        }

        Ok(())
    }

    pub async fn list(&self) -> Vec<IpcManager> {
        self.mans.lock().await.clone()
    }

    pub async fn get(&self, id: i64) -> Result<IpcManager> {
        let mans = self.mans.lock().await;
        for man in mans.iter() {
            if man.id == id {
                return Ok(man.clone());
            }
        }

        bail!("Failed to get ipc manager with id {}", id)
    }

    pub async fn reset(&self) {
        let mut mans = self.mans.lock().await;

        for man in mans.iter() {
            man.close().await;
        }

        *mans = vec![];
    }
}
