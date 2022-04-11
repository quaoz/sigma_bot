use chrono::Duration;
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

	let now = chrono::Utc::now().time();
	let start_time = next_pubs_map.get(2).unwrap().as_str().split(':').collect::<Vec<&str>>();
	let mut time = chrono::NaiveTime::from_hms(start_time[0].parse().unwrap(), start_time[1].parse().unwrap(), 0) - now;

	if time.num_minutes().is_negative() {
		time = Duration::hours(24) + time;
	}

	let hours = time.num_hours();
	let minutes = time.num_minutes() - hours * 60;
	let mut time_to_string = String::new();

	if hours != 0 {
		if minutes != 0 {
			time_to_string.push_str(&*format!("{} hours and {} minutes", hours, minutes))
		} else {
			time_to_string.push_str(&*format!("{} hours", hours));
		}
	} else if minutes != 0 {
		time_to_string.push_str(&*format!("{} minutes", minutes))
	}

	let content = format!(
		"The current map is {} for {}, the next map is {} from {} to {} (UTC)",
		pubs_map,
		time_to_string,
		next_pubs_map.get(1).unwrap().as_str(),
		next_pubs_map.get(2).unwrap().as_str(),
		next_pubs_map.get(3).unwrap().as_str()
	);
	msg.reply(&ctx.http, content).await?;

	Ok(())
}
