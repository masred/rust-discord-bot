mod commands;
mod config;

use commands::slap::slap;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_secrets::SecretStore;
use shuttle_serenity::ShuttleSerenity;

#[derive(Clone)]
struct Data {
    conf: config::Config,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    let conf = config::Config::new(secret_store).await;
    let data = Data { conf: conf.clone() };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![slap()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data.clone())
            })
        })
        .build();

    let client = ClientBuilder::new(conf.discord.token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
