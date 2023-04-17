use std::sync::Arc;

use tokio::{
    select,
    sync::{mpsc, oneshot},
};

use crate::rpc::{self, magicbox, mediafilefind, rpclogin::User, Client};

pub enum CameraCommand {
    Logout(oneshot::Sender<Result<(), rpc::Error>>),
}

pub struct CameraState {
    pub user: User,
    pub client: Client,
}

pub struct Camera {
    pub id: i64,
    rpc_tx: mpsc::Sender<oneshot::Sender<Result<rpc::RequestBuilder, rpc::Error>>>,
    cmd_tx: mpsc::Sender<CameraCommand>,
}

impl Camera {
    async fn send_command(state: &mut CameraState, cmd: CameraCommand) {
        match cmd {
            CameraCommand::Logout(tx) => {
                tx.send(User::logout(&mut state.client).await).ok();
            }
        };
    }

    async fn send_rpc(
        state: &mut CameraState,
        rpc_tx: oneshot::Sender<Result<rpc::RequestBuilder, rpc::Error>>,
    ) {
        rpc_tx
            .send(
                state
                    .user
                    .keep_alive_or_login(&mut state.client)
                    .await
                    .map(|_| state.client.rpc()),
            )
            .ok();
    }

    pub fn new(id: i64, mut state: CameraState) -> Camera {
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<CameraCommand>(1);
        let (rpc_tx, mut rpc_rx) =
            mpsc::channel::<oneshot::Sender<Result<rpc::RequestBuilder, rpc::Error>>>(1);

        tokio::spawn(async move {
            loop {
                select! {
                    biased;
                    cmd = cmd_rx.recv() =>{
                        if let Some(cmd) = cmd {
                            Self::send_command(&mut state, cmd).await;
                        } else {
                            break;
                        }
                    }
                    rpc_tx = rpc_rx.recv() => {
                        if let Some(rpc_tx) = rpc_tx {
                            Self::send_rpc(&mut state, rpc_tx).await;
                        } else {
                            break;
                        }
                    }
                }
            }
        });

        return Camera { id, rpc_tx, cmd_tx };
    }

    pub async fn rpc(&self) -> Result<rpc::RequestBuilder, rpc::Error> {
        let (tx, rx) = oneshot::channel();
        self.rpc_tx
            .send(tx)
            .await
            .map_err(|_| rpc::Error::NoData("Camera thread dead".to_string()))?;
        match rx
            .await
            .map_err(|_| rpc::Error::NoData("Camera thread dead".to_string()))
        {
            Ok(res) => res,
            Err(err) => Err(err),
        }
    }

    pub async fn logout(&self) -> Result<(), rpc::Error> {
        let (tx, rx) = oneshot::channel();
        self.cmd_tx
            .send(CameraCommand::Logout(tx))
            .await
            .map_err(|_| rpc::Error::NoData("Camera thread dead".to_string()))?;
        rx.await
            .map_err(|_| rpc::Error::NoData("Camera thread dead".to_string()))?
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
    pub async fn get(cam: &Camera) -> Result<CameraDetail, rpc::Error> {
        Ok(CameraDetail {
            sn: magicbox::get_serial_no(cam.rpc().await?).await.ok(),
            device_class: magicbox::get_device_class(cam.rpc().await?).await.ok(),
            device_type: magicbox::get_device_type(cam.rpc().await?).await.ok(),
            hardware_version: magicbox::get_hardware_version(cam.rpc().await?).await.ok(),
            market_area: magicbox::get_market_area(cam.rpc().await?).await.ok(),
            process_info: magicbox::get_process_info(cam.rpc().await?).await.ok(),
            vendor: magicbox::get_vendor(cam.rpc().await?).await.ok(),
        })
    }
}

pub type CameraSoftwareVersion = magicbox::GetSoftwareVersion;

impl CameraSoftwareVersion {
    pub async fn get(cam: &Camera) -> Result<magicbox::GetSoftwareVersion, rpc::Error> {
        magicbox::get_software_version(cam.rpc().await?).await
    }
}

pub struct CameraStore {
    cams: Vec<Arc<Camera>>,
}

impl CameraStore {
    pub fn add(mut self, cam: Camera) -> Self {
        self.cams.push(Arc::new(cam));
        self
    }

    pub fn get(&self, id: i64) -> Option<Arc<Camera>> {
        for cam in self.cams.iter() {
            if cam.id == id {
                return Some(Arc::clone(cam));
            }
        }
        None
    }

    pub async fn clear(mut self) -> Self {
        for cam in self.cams.iter() {
            _ = cam.logout().await;
        }
        self.cams.clear();
        self
    }
}

pub struct FindNextFileInfoStream<'a> {
    cam: &'a Camera,
    object: i64,
    pub error: Option<rpc::Error>,
    count: i32,
    closed: bool,
}

impl FindNextFileInfoStream<'_> {
    pub async fn new(
        cam: &Camera,
        condition: mediafilefind::Condition,
    ) -> Result<FindNextFileInfoStream, rpc::Error> {
        let object = mediafilefind::create(cam.rpc().await?).await?;

        let closed = match mediafilefind::find_file(cam.rpc().await?, object, condition).await {
            Ok(o) => !o,
            Err(rpc::Error::NoData(_)) => true,
            Err(err) => return Err(err),
        };

        Ok(FindNextFileInfoStream {
            cam,
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

        let rpc = match self.cam.rpc().await {
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
        let rpc = match self.cam.rpc().await {
            Ok(o) => o,
            Err(_) => return,
        };
        _ = mediafilefind::close(rpc, self.object).await;
        let rpc = match self.cam.rpc().await {
            Ok(o) => o,
            Err(_) => return,
        };
        _ = mediafilefind::destroy(rpc, self.object).await;

        self.closed = true;
    }
}
