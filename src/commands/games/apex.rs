use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use reqwest;

#[command]
#[num_args(0)]
#[aliases(apex)]
#[description("Displays the current apex map")]
#[usage("map")]
#[example("map")]
async fn map(ctx: &Context, msg: &Message) -> CommandResult {
    let resp = reqwest::get("https://apexlegendsstatus.com/current-map").await;
    
    // TODO: fix this or do something cursed
    
    // let document = kuchiki::parse_html().one(resp.unwrap().text().await.unwrap());
    
    let content = format!(
        "My ping is: {}ms",
        chrono::Utc::now().timestamp_millis() - msg.timestamp.timestamp_millis()
    );
    msg.reply(&ctx.http, content).await?;

    Ok(())
}
