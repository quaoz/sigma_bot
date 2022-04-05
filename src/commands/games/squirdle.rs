use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[num_args(0)]
#[description("Displays the daily squirdle")]
#[usage("squirdle")]
#[example("squirdle")]
async fn squirdle(ctx: &Context, msg: &Message) -> CommandResult {
	// Gets a json response from the url
	let resp = reqwest::get("https://squirdle.fireblend.com/data/daily.json").await?;
	let json: serde_json::Value = resp.json().await?;

	// Gets the current date in the format YYYY-MM-DD
	let date = chrono::Local::now().format("%Y-%m-%d").to_string();

	let content = format!(
		"The daily squirdle for {} is ||{}||",
		&date,
		json[&date].get(1).unwrap()
	);
	msg.reply(&ctx.http, content).await?;

	Ok(())
}
