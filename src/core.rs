use anyhow::{bail, Context, Result};
use tokio::{
    select,
    sync::{mpsc, oneshot},
};

use crate::rpc::{self, magicbox, mediafilefind, rpclogin, Client, ResponseError, ResponseKind};

pub struct CameraState {
    pub user: rpclogin::User,
    pub client: Client,
}

impl CameraState {
    pub async fn destroy(mut self) -> Result<(), rpc::Error> {
        rpclogin::User::logout(&mut self.client).await
    }
}

enum Command {
    Close(oneshot::Sender<CameraState>),
}

pub struct CameraManager {
    pub id: i64,
    rpc_tx: mpsc::Sender<oneshot::Sender<Result<rpc::RequestBuilder, rpc::Error>>>,
    cmd_tx: mpsc::Sender<Command>,
}

impl Clone for CameraManager {
    fn clone(&self) -> Self {
        CameraManager {
            id: self.id,
            rpc_tx: self.rpc_tx.clone(),
            cmd_tx: self.cmd_tx.clone(),
        }
    }
}

impl CameraManager {
    pub fn new(id: i64, mut state: CameraState) -> CameraManager {
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<Command>(1);
        let (rpc_tx, mut rpc_rx) =
            mpsc::channel::<oneshot::Sender<Result<rpc::RequestBuilder, rpc::Error>>>(1);

        tokio::spawn(async move {
            loop {
                select! {
                    biased;
                    cmd = cmd_rx.recv() =>{
                        if let Some(cmd) = cmd {
                            match cmd {
                                Command::Close(tx) => {
                                    if let Err(s) = tx.send(state) {
                                        state = s;
                                    } else {
                                        break
                                    }
                                }
                            };
                        } else {
                            break;
                        }
                    }
                    rpc_tx = rpc_rx.recv() => {
                        if let Some(rpc_tx) = rpc_tx {
                            rpc_tx.send(state.user.keep_alive_or_login(&mut state.client).await.map(|_| state.client.rpc())).ok();
                        } else {
                            break;
                        }
                    }
                }
            }
        });

        return CameraManager { id, rpc_tx, cmd_tx };
    }

    pub async fn rpc(&self) -> Result<rpc::RequestBuilder, rpc::Error> {
        let (tx, rx) = oneshot::channel();
        self.rpc_tx
            .send(tx)
            .await
            .map_err(|_| rpc::Error::Request("Camera thread dead".to_string()))?;
        rx.await
            .map_err(|_| rpc::Error::Request("Camera thread dead".to_string()))?
    }

    pub async fn close(self) -> Result<CameraState> {
        let (tx, rx) = oneshot::channel();
        self.cmd_tx.send(Command::Close(tx)).await.ok();
        rx.await.with_context(|| format!("Camera thread dead"))
    }

    pub async fn destroy(self) {
        if let Ok(state) = self.close().await {
            state.destroy().await.ok();
        }
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
    pub async fn get(man: &CameraManager) -> Result<CameraDetail, rpc::Error> {
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
    pub async fn get(man: &CameraManager) -> Result<magicbox::GetSoftwareVersion, rpc::Error> {
        magicbox::get_software_version(man.rpc().await?).await
    }
}

pub struct CameraManagerStore {
    mans: Vec<CameraManager>,
}

impl CameraManagerStore {
    pub fn new() -> CameraManagerStore {
        CameraManagerStore { mans: vec![] }
    }

    pub fn add(&mut self, man: CameraManager) -> Result<()> {
        for i in self.mans.iter() {
            if i.id == man.id {
                bail!("Duplicate camera id {}", man.id)
            }
        }
        self.mans.push(man);
        Ok(())
    }

    pub fn list(&self) -> Vec<CameraManager> {
        self.mans.clone()
    }

    pub fn get(&self, id: i64) -> Option<CameraManager> {
        for man in self.mans.iter() {
            if man.id == id {
                return Some(man.clone());
            }
        }
        None
    }

    pub async fn destroy(self) {
        for man in self.mans {
            man.destroy().await
        }
    }
}

pub struct CameraFileStream<'a> {
    man: &'a CameraManager,
    object: i64,
    pub error: Option<rpc::Error>,
    count: i32,
    closed: bool,
}

impl CameraFileStream<'_> {
    pub async fn new(
        man: &CameraManager,
        condition: mediafilefind::Condition,
    ) -> Result<CameraFileStream, rpc::Error> {
        let object = mediafilefind::create(man.rpc().await?).await?;

        let closed = match mediafilefind::find_file(man.rpc().await?, object, condition).await {
            Ok(o) => !o,
            Err(rpc::Error::Response(ResponseError {
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
