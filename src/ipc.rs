use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use dahua_rpc::{
    modules::{license, magicbox, mediafilefind},
    reqwest, Client, Error, RequestBuilder, ResponseError, ResponseKind,
};
use tokio::sync::{mpsc, oneshot, Mutex};

/// If the error is of type ResponseError then it will return the Default::default() of type T.
fn maybe<T>(check: Result<T, Error>) -> Result<T, Error>
where
    T: Default,
{
    match check {
        Err(Error::Response(_)) => Ok(Default::default()),
        Ok(o) => Ok(o),
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
        client.rpc().await
    }

    pub async fn file(&self, file_path: &str) -> Result<IpcFile, Error> {
        let mut client = self.client.lock().await;

        Ok(IpcFile {
            cookie: client.cookie().await?,
            url: client.file_url(file_path),
        })
    }

    pub async fn close(&self) {
        let mut client = self.client.lock().await;
        client.logout().await;
        client.state = dahua_rpc::State::Error(dahua_rpc::LoginError::Closed)
    }
}

pub struct IpcDetail {
    pub sn: String,
    pub device_class: String,
    pub device_type: String,
    pub hardware_version: String,
    pub market_area: String,
    pub process_info: String,
    pub vendor: String,
}

impl IpcDetail {
    pub async fn get(man: &IpcManager) -> Result<Self, Error> {
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

pub struct IpcSoftware(pub magicbox::GetSoftwareVersion);

impl IpcSoftware {
    pub async fn get(man: &IpcManager) -> Result<Self, Error> {
        Ok(IpcSoftware(maybe(
            magicbox::get_software_version(man.rpc().await?).await,
        )?))
    }
}

pub struct IpcLicenses(pub Vec<license::InfoContainer>);

impl IpcLicenses {
    pub async fn get(man: &IpcManager) -> Result<Self, Error> {
        Ok(IpcLicenses(maybe(
            license::get_license_info(man.rpc().await?).await,
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

use crate::models::ICamera;

impl From<(ICamera, reqwest::Client)> for IpcManager {
    fn from(value: (ICamera, reqwest::Client)) -> Self {
        IpcManager::new(
            value.0.id,
            dahua_rpc::Client::new(value.1, value.0.ip, value.0.username, value.0.password),
        )
    }
}

enum IpcStoreMessage {
    Get(i64, oneshot::Sender<Option<IpcManager>>),
    Refresh(i64),
    Shutdown(oneshot::Sender<()>),
}

struct IpcStoreActor {
    receiver: mpsc::Receiver<IpcStoreMessage>,
    mans: Vec<IpcManager>,
    client: reqwest::Client,
    pool: sqlx::SqlitePool,
}

impl IpcStoreActor {
    async fn new(
        receiver: mpsc::Receiver<IpcStoreMessage>,
        pool: sqlx::SqlitePool,
    ) -> Result<Self> {
        let client = dahua_rpc::recommended_reqwest_client_builder()
            .build()
            .context("Failed to build reqwest client.")?;

        let mans = ICamera::list(&pool)
            .await?
            .into_iter()
            .map(|c| IpcManager::from((c, client.clone())))
            .collect();

        Ok(IpcStoreActor {
            receiver,
            mans,
            client,
            pool,
        })
    }

    async fn handle_message(&mut self, msg: IpcStoreMessage) {
        match msg {
            IpcStoreMessage::Get(id, respond_to) => {
                for man in self.mans.iter() {
                    if man.id == id {
                        respond_to.send(Some(man.clone())).ok();
                        return;
                    }
                }

                respond_to.send(None).ok();
            }
            IpcStoreMessage::Refresh(id) => {
                let icam = match ICamera::find_optional(&self.pool, id).await {
                    Ok(o) => o,
                    Err(err) => {
                        tracing::error!("{err:?}");
                        return;
                    }
                };
                let old = self.mans.iter().enumerate().find(|(_, old)| old.id == id);

                match (icam, old) {
                    // Update
                    (Some(icam), Some((idx, old))) => {
                        old.close().await;
                        self.mans[idx] = IpcManager::from((icam, self.client.clone()));
                    }
                    // Add
                    (Some(icam), None) => self
                        .mans
                        .push(IpcManager::from((icam, self.client.clone()))),
                    // Delete
                    (None, Some((idx, old))) => {
                        old.close().await;
                        self.mans.remove(idx);
                    }
                    (None, None) => {}
                }
            }
            IpcStoreMessage::Shutdown(respond_to) => {
                for man in self.mans.iter() {
                    man.close().await;
                }

                self.receiver.close();

                respond_to.send(()).ok();
            }
        }
    }

    async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await
        }
    }
}

#[derive(Clone)]
pub struct IpcStore {
    sender: mpsc::Sender<IpcStoreMessage>,
}

impl IpcStore {
    pub async fn new(pool: sqlx::SqlitePool) -> Result<Self> {
        let (sender, receiver) = mpsc::channel(8);
        let actor = IpcStoreActor::new(receiver, pool).await?;
        tokio::spawn(actor.run());

        Ok(Self { sender })
    }

    pub async fn get_optional(&self, id: i64) -> Result<Option<IpcManager>> {
        let (send, recv) = oneshot::channel();
        let msg = IpcStoreMessage::Get(id, send);
        self.sender.send(msg).await.ok();
        recv.await.context("Ipc store is shutdonw.")
    }

    pub async fn get(&self, id: i64) -> Result<IpcManager> {
        self.get_optional(id)
            .await?
            .ok_or_else(|| anyhow!("Failed to find ipc manager with camera id {id}."))
    }

    pub async fn refresh(&self, id: i64) -> Result<()> {
        let msg = IpcStoreMessage::Refresh(id);
        self.sender
            .send(msg)
            .await
            .map_err(|_| anyhow!("Ipc store is shutdown."))
    }

    pub async fn shutdown(&self) {
        let (send, recv) = oneshot::channel();
        let msg = IpcStoreMessage::Shutdown(send);
        self.sender.send(msg).await.ok();
        recv.await.ok();
    }
}
