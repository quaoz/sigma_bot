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

	let md5_captures = Regex::new(r"index\.php\?md5=(.*?)'")
		.unwrap()
		.find_iter(resp.as_str())
		.collect::<Vec<_>>();

	// Sets results equal to the smaller value of md5_captures.len() and 5
	let results = std::cmp::min(md5_captures.len(), 5);

	let mut embed = CreateEmbed::default();
	embed.title(format!("LibGen results for \"{}\":\n", &title));

	for i in 0..results {
		let mut md5 = &md5_captures.get(i).unwrap().as_str()[14..&md5_captures.get(i).unwrap().as_str().len() - 1];

		let json: serde_json::Value = reqwest::get(format!(
			"http://libgen.is/json.php?fields=ipfs_cid,author,title,publisher,year,extension&ids={}",
			&md5
		))
		.await?
		.json()
		.await?;

		// Can be swapped for another ipfs gateway such as ipfs.io
		let dl_url = format!(
			"https://dweb.link/ipfs/{}?filename={}%20-%20{}-{}%20%28{}%29.{}",
			sanitise(&json, "ipfs_cid"),
			sanitise(&json, "author"),
			sanitise(&json, "title"),
			sanitise(&json, "publisher"),
			sanitise(&json, "year"),
			sanitise(&json, "extension")
		);

		embed.field(
			format!(
				"{} ({}) by {} ({})",
				get_field(&json, "title"),
				get_field(&json, "extension"),
				get_field(&json, "author"),
				get_field(&json, "year")
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

fn sanitise(json: &serde_json::Value, value: &str) -> String {
	urlencoding::encode(&*get_field(json, value).to_lowercase()).to_string()
}

fn get_field(json: &serde_json::Value, value: &str) -> String {
	json.get(0).unwrap().get(value).unwrap().to_string()
}
