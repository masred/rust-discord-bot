use anyhow::Context;
use shuttle_secrets::SecretStore;

#[derive(Clone)]
pub struct Discord {
    pub token: String,
}

#[derive(Clone)]
pub struct Tenor {
    pub endpoint: String,
}

#[derive(Clone)]
pub struct Config {
    pub discord: Discord,
    pub tenor: Tenor,
}

impl Config {
    pub async fn new(secret_store: SecretStore) -> Config {
        let discord_token = secret_store
            .get("DISCORD_TOKEN")
            .context("'DISCORD_TOKEN' was not found")
            .unwrap();
        let tenor_endpoint = secret_store
            .get("TENOR_ENDPOINT")
            .context("'TENOR_ENDPOINT' was not found")
            .unwrap();

        Config {
            discord: Discord {
                token: discord_token,
            },
            tenor: Tenor {
                endpoint: tenor_endpoint,
            },
        }
    }
}
