use std::sync::Arc;

use tokio::sync::Mutex;

use anyhow::{bail, Result};
use rpc::{
    modules::{magicbox, mediafilefind},
    Client, Error, RequestBuilder, ResponseError, ResponseKind,
};

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

    pub async fn close(&self) {
        let mut client = self.client.lock().await;
        client.logout().await.ok();
        client.closed = true;
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

pub struct IpcSoftwareVersion {
    pub software: magicbox::GetSoftwareVersion,
}

impl IpcSoftwareVersion {
    pub async fn get(man: &IpcManager) -> Result<IpcSoftwareVersion, Error> {
        Ok(IpcSoftwareVersion {
            software: magicbox::get_software_version(man.rpc().await?).await?,
        })
    }
}

#[derive(Clone)]
pub struct IpcManagerStore(Arc<Mutex<Vec<IpcManager>>>);

impl IpcManagerStore {
    pub fn new() -> IpcManagerStore {
        IpcManagerStore(Arc::new(Mutex::new(vec![])))
    }

    pub async fn add(&self, man: IpcManager) -> Result<()> {
        let mut mans = self.0.lock().await;
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
        let mut mans = self.0.lock().await;
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

    pub async fn list(&self) -> Vec<IpcManager> {
        self.0.lock().await.clone()
    }

    pub async fn get(&self, id: i64) -> Result<IpcManager> {
        let mans = self.0.lock().await;
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
        let mut mans = self.0.lock().await;
        for man in mans.iter() {
            man.close().await;
        }

        *mans = vec![];
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
