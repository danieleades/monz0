use monzo::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ClientConfig {
    access_token: String,
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

impl Into<Client> for ClientConfig {
    fn into(self) -> Client {
        Client::new(self.access_token, self.client_id, self.client_secret, self.refresh_token)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Configuration {
    client_config: ClientConfig,
    #[serde(flatten)]
    app_config: AppConfig,
}

impl Configuration {
    pub fn load() -> std::io::Result<Self> {
        confy::load("monz0")
    }

    pub fn save(&self) -> std::io::Result<()> {
        confy::store("monz0", self)
    }
    pub fn into_parts(self) -> (Client, AppConfig) {
        (self.client_config.into(), self.app_config)
    }
}
