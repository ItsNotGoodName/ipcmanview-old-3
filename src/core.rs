use std::sync::{Arc, Mutex};

use crate::rpc::{self, magicbox, rpclogin::Manager};

pub struct Camera {
    pub id: i64,
    pub man: Mutex<Manager>,
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

impl Camera {
    pub async fn detail(&self) -> Result<CameraDetail, rpc::Error> {
        let mut man = self.man.lock().unwrap();
        man.keep_alive_or_login()?;
        let sn_rpc = man.client.rpc();
        let device_class_rpc = man.client.rpc();
        let device_type_rpc = man.client.rpc();
        let hardware_version_rpc = man.client.rpc();
        let market_area_rpc = man.client.rpc();
        let process_info_rpc = man.client.rpc();
        let vendor_rpc = man.client.rpc();
        drop(man);

        Ok(tokio::task::spawn_blocking(|| CameraDetail {
            sn: magicbox::get_serial_no(sn_rpc).ok(),
            device_class: magicbox::get_device_class(device_class_rpc).ok(),
            device_type: magicbox::get_device_type(device_type_rpc).ok(),
            hardware_version: magicbox::get_hardware_version(hardware_version_rpc).ok(),
            market_area: magicbox::get_market_area(market_area_rpc).ok(),
            process_info: magicbox::get_process_info(process_info_rpc).ok(),
            vendor: magicbox::get_vendor(vendor_rpc).ok(),
        })
        .await
        .unwrap())
    }

    pub async fn version(&self) -> Result<magicbox::GetSoftwareVersion, rpc::Error> {
        let mut man = self.man.lock().unwrap();
        man.keep_alive_or_login()?;
        let rpc = man.client.rpc();
        drop(man);

        tokio::task::spawn_blocking(|| magicbox::get_software_version(rpc))
            .await
            .unwrap()
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

    pub fn clear(mut self) -> Self {
        for cam in self.cams.iter() {
            if let Ok(mut cam) = cam.man.lock() {
                _ = cam.logout()
            }
        }
        self.cams.clear();
        self
    }
}
