use regex::Regex;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[min_args(1)]
#[aliases(libgen, lg)]
#[description("Searches libgen for the given book")]
#[usage("access <title>")]
#[example("access word")]
async fn access(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let title = args.rest();

	// Mirrors: libgen.rs, libgen.is, libgen.st
	let resp = reqwest::get(format!("http://libgen.is/search.php?req={}", title.replace(' ', "+")))
		.await?
		.text()
		.await?;

	let md5 = Regex::new("md5=(.*?)'").unwrap().captures(resp.as_str()).unwrap();

	let mut embed = CreateEmbed::default();
	embed.title(format!("LibGen results for \"{}\":\n", &title));

	for i in 0..md5.len() {
		let json: serde_json::Value = reqwest::get(format!(
			"http://libgen.is/json.php?fields=ipfs_cid,author,title,publisher,year,extension&ids={}",
			&md5.get(i).unwrap().as_str()
		))
		.await?
		.json()
		.await?;

		let dl_url = format!(
			"https://dweb.link/ipfs/{}?filename={}%20-%20{}-{}%20%28{}%29.{}",
			sanitise(i, &json, "ipfs_cid"),
			sanitise(i, &json, "author"),
			sanitise(i, &json, "title"),
			sanitise(i, &json, "publisher"),
			sanitise(i, &json, "year"),
			sanitise(i, &json, "extension")
		);

		embed.field(
			format!(
				"{} ({}) by {} ({})",
				get_field(i, &json, "title"),
				get_field(i, &json, "extension"),
				get_field(i, &json, "author"),
				get_field(i, &json, "year")
			),
			dl_url,
			true,
		);
	}

	// Send the message
	msg.channel_id
		.send_message(&ctx, |f| {
			f.content("").embed(|e| {
				e.0 = embed.0;
				e
			})
		})
		.await
		.unwrap();

	Ok(())
}

fn sanitise(index: usize, json: &serde_json::Value, value: &str) -> String {
	urlencoding::encode(&*get_field(index, json, value).to_lowercase()).to_string()
}

fn get_field(index: usize, json: &serde_json::Value, value: &str) -> String {
	json.get(index.to_string()).unwrap().get(value).unwrap().to_string()
}
