use super::Client;

impl Client {
    pub fn file_url(&self, file_path: &str) -> String {
        format!("http://{}/RPC_Loadfile{}", self.ip, file_path)
    }
}
