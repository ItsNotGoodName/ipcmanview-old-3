use super::Client;

impl Client {
    pub fn cookie(&self) -> String {
        format!("WebClientSessionID={session}; DWebClientSessionID={session}; DhWebClientSessionID={session}", session=self.state.session)
    }
}
