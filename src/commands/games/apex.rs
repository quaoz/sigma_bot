use regex::Regex;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[num_args(0)]
#[aliases(apex)]
#[description("Displays the current apex map")]
#[usage("map")]
#[example("map")]
async fn map(ctx: &Context, msg: &Message) -> CommandResult {
	let client = reqwest::Client::builder().build()?;
	let resp = client
		.get("https://apexlegendsstatus.com/current-map")
		.send()
		.await?
		.text()
		.await?;

	// Questionable but it works for now
	let pubs_map_reg =
		Regex::new("<h1 style=\"font-weight: 600; margin-bottom: 5px;\">Battle Royale: (.*?)</h1>").unwrap();
	let next_pubs_map_reg =
		Regex::new("<h5 style=\"margin-top: 0px;\">Next map is <b>(.*?)</b>, from (.*?) to (.*?) UTC</h5>").unwrap();

	let pubs_map = pubs_map_reg.captures(resp.as_str()).unwrap().get(1).unwrap().as_str();
	let next_pubs_map = next_pubs_map_reg.captures(resp.as_str()).unwrap();

	let content = format!(
		"The current map is {}, the next map is {} from {} to {}",
		pubs_map,
		next_pubs_map.get(1).unwrap().as_str(),
		next_pubs_map.get(2).unwrap().as_str(),
		next_pubs_map.get(3).unwrap().as_str()
	);
	msg.reply(&ctx.http, content).await?;

	Ok(())
}
