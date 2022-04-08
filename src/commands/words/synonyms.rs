use serenity::builder::CreateEmbed;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

#[command]
#[num_args(1)]
#[aliases(syns, thesaurus)]
#[description("Gives synonyms for the given word")]
#[usage("synonyms <word>")]
#[example("synonyms word")]
async fn synonyms(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let word = args.single::<String>()?;

	// Reads the token and creates the request url
	let token = env::var("THESAURUS_TOKEN").expect("Expected a token in the environment");
	let url = format!(
		"https://www.dictionaryapi.com/api/v3/references/thesaurus/json/{}?key={}",
		&word, token
	);

	// Gets a json response from the url
	let json: serde_json::Value = reqwest::get(&url).await?.json().await?;
	let entries = json.as_array().unwrap();

	let mut embed = CreateEmbed::default();
	embed.title(format!("Synonyms for {}:\n", &word));

	for entry in entries {
		let synonyms_list = entry
			.get("meta")
			.unwrap()
			.get("syns")
			.unwrap()
			.get(0)
			.unwrap()
			.as_array()
			.unwrap();
		let word_type = entry.get("fl").unwrap().as_str().unwrap();

		let mut v: Vec<char> = entry
			.get("shortdef")
			.unwrap()
			.get(0)
			.unwrap()
			.as_str()
			.unwrap()
			.chars()
			.collect();
		v[0] = v[0].to_uppercase().next().unwrap();
		let short_def: String = v.into_iter().collect();

		let mut synonyms = String::default();

		for synonym in synonyms_list {
			synonyms.push_str(&*format!("-{}\n", synonym.as_str().unwrap()));
		}

		embed.field(format!("{} ({})", short_def, word_type), synonyms, true);
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
