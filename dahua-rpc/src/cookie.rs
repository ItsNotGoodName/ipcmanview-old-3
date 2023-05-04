use crate::Client;

impl Client {
    pub fn cookie_raw(&self) -> String {
        format!("WebClientSessionID={session}; DWebClientSessionID={session}; DhWebClientSessionID={session}", session=self.connection.session)
    }
}
