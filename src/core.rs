use std::sync::{Arc, Mutex};

use anyhow::{bail, Result};

use crate::rpc::{
    magicbox, mediafilefind, Client, Error, RequestBuilder, ResponseError, ResponseKind,
};

#[derive(Clone)]
pub struct CameraManager {
    pub id: i64,
    pub client: Arc<Mutex<Client>>,
}

impl CameraManager {
    pub fn new(id: i64, client: Client) -> CameraManager {
        CameraManager {
            id,
            client: Arc::new(Mutex::new(client)),
        }
    }

    pub async fn rpc(&self) -> Result<RequestBuilder, Error> {
        let mut client = self.client.lock().unwrap();
        client.keep_alive_or_login().await.map(|_| client.rpc())
    }

    pub async fn close(&self) {
        let mut client = self.client.lock().unwrap();
        client.logout().await.ok();
        client.closed = true;
    }
}

pub struct CameraDetail {
    pub sn: Option<String>,
    pub device_class: Option<String>,
    pub device_type: Option<String>,
    pub hardware_version: Option<String>,
    pub market_area: Option<String>,
    pub process_info: Option<String>,
    pub vendor: Option<String>,
}

impl CameraDetail {
    pub async fn get(man: &CameraManager) -> Result<CameraDetail, Error> {
        Ok(CameraDetail {
            sn: magicbox::get_serial_no(man.rpc().await?).await.ok(),
            device_class: magicbox::get_device_class(man.rpc().await?).await.ok(),
            device_type: magicbox::get_device_type(man.rpc().await?).await.ok(),
            hardware_version: magicbox::get_hardware_version(man.rpc().await?).await.ok(),
            market_area: magicbox::get_market_area(man.rpc().await?).await.ok(),
            process_info: magicbox::get_process_info(man.rpc().await?).await.ok(),
            vendor: magicbox::get_vendor(man.rpc().await?).await.ok(),
        })
    }
}

pub type CameraSoftwareVersion = magicbox::GetSoftwareVersion;

impl CameraSoftwareVersion {
    pub async fn get(man: &CameraManager) -> Result<magicbox::GetSoftwareVersion, Error> {
        magicbox::get_software_version(man.rpc().await?).await
    }
}

pub struct CameraManagerStore {
    mans: Mutex<Vec<CameraManager>>,
}

impl CameraManagerStore {
    pub fn new() -> CameraManagerStore {
        CameraManagerStore {
            mans: Mutex::new(vec![]),
        }
    }

    pub fn add(&self, man: CameraManager) -> Result<()> {
        let mut mans = self.mans.lock().unwrap();
        for old in mans.iter() {
            if old.id == man.id {
                bail!(
                    "Failed to add manager, already exists in store with camera {}",
                    old.id
                )
            }
        }

        mans.push(man);

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        let mut mans = self.mans.lock().unwrap();
        match mans.iter().enumerate().find(|(_, old)| old.id == id) {
            Some((idx, _)) => {
                mans.swap_remove(idx).close().await;
                Ok(())
            }
            None => {
                bail!(
                    "Failed to delete manager, not found in store with camera {}",
                    id
                )
            }
        }
    }

    pub fn list(&self) -> Vec<CameraManager> {
        self.mans.lock().unwrap().clone()
    }

    pub fn get(&self, id: i64) -> Result<CameraManager> {
        let mans = self.mans.lock().unwrap();
        for man in mans.iter() {
            if man.id == id {
                return Ok(man.clone());
            }
        }

        bail!(
            "Failed to get manager, not found in store with camera {}",
            id
        )
    }

    pub async fn reset(self) {
        let mut mans = self.mans.lock().unwrap();
        for man in mans.iter() {
            man.close().await;
        }

        *mans = vec![];
    }
}

pub struct CameraFileStream<'a> {
    man: &'a CameraManager,
    object: i64,
    pub error: Option<Error>,
    count: i32,
    closed: bool,
}

impl CameraFileStream<'_> {
    pub async fn new(
        man: &CameraManager,
        condition: mediafilefind::Condition,
    ) -> Result<CameraFileStream, Error> {
        let object = mediafilefind::create(man.rpc().await?).await?;

        let closed = match mediafilefind::find_file(man.rpc().await?, object, condition).await {
            Ok(o) => !o,
            Err(Error::Response(ResponseError {
                kind: ResponseKind::NoData,
                ..
            })) => true,
            Err(err) => return Err(err),
        };

        Ok(CameraFileStream {
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
        let rpc = match self.man.rpc().await {
            Ok(o) => o,
            Err(_) => return,
        };
        _ = mediafilefind::close(rpc, self.object).await;
        let rpc = match self.man.rpc().await {
            Ok(o) => o,
            Err(_) => return,
        };
        _ = mediafilefind::destroy(rpc, self.object).await;

        self.closed = true;
    }
}
