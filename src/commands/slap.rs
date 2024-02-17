use crate::{Context, Error};
use poise::{
    serenity_prelude::{self as serenity, builder, Mentionable},
    CreateReply,
};
use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize)]
struct TenorResponse {
    results: Vec<serde_json::Value>,
}

#[poise::command(slash_command)]
pub async fn slap(
    ctx: Context<'_>,
    #[description = "Who do you want to slap?"] user: serenity::User,
) -> Result<(), Error> {
    let response: TenorResponse = ureq::get(&ctx.data().conf.tenor.endpoint)
        .call()?
        .into_json()?;

    let random_number = rand::thread_rng().gen_range(0..25);
    let image_url = response.results[random_number]["media_formats"]["gif"]["url"].as_str();

    let embed = builder::CreateEmbed::default()
        .description(format!(
            "{} slapped {}. You deserves it! 😠",
            ctx.author().mention(),
            user.mention()
        ))
        .image(image_url.unwrap())
        .color(serenity::Color::RED);

    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;

    Ok(())
}
