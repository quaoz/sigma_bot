use std::env;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases(def)]
#[description = "Defines the given words"]
async fn define(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let word = args.single::<String>()?;

	let token = env::var("DICTIONARY_TOKEN").expect("Expected a token in the environment");
	let url = format!("https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}?key={}", &word, token);

	let response = reqwest::get(url).await?.text().await?;
	let json: serde_json::Value = serde_json::from_str(&response).expect("JSON was not well-formatted");

	let defs = json.get(0).unwrap().get("shortdef").unwrap().as_array().unwrap();
	let mut def = String::from(format!("The definition of {} is:\n", word));

	let mut index = 1;
	for item in defs {
		let mut v: Vec<char> = item.as_str().unwrap().chars().collect();
		v[0] = v[0].to_uppercase().nth(0).unwrap();
		let capitalised: String = v.into_iter().collect();

		def.push_str(&*format!("{}. {}\n", index, capitalised ));
		index += 1;
	}

	msg.reply(&ctx.http, &def).await?;
	Ok(())
}
