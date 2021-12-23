use serenity::builder::CreateEmbed;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

#[command]
#[num_args(1)]
#[aliases(def)]
#[description("Defines the given word")]
#[usage("define <word>")]
#[example("define word")]
async fn define(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let word = args.single::<String>()?;

	// Reads the token and creates the request url
	let token = env::var("DICTIONARY_TOKEN").expect("Expected a token in the environment");
	let url = format!(
		"https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}?key={}",
		&word, token
	);

	// Gets a json response from the url
	let json: serde_json::Value = reqwest::get(&url).await?.json().await?;

	let entries = json.as_array().unwrap();
	// Get the first entry
	let mut defs: Vec<&Vec<serde_json::Value>> =
		vec![json.get(0).unwrap().get("shortdef").unwrap().as_array().unwrap()];

	// Get the remaining entries
	for i in 1..entries.len() {
		let value = format!("{}:{}", &word, &i + 1);
		let pattern = value.as_str();

		// Check that the entry is not a similar term
		if entries
			.get(i)
			.unwrap()
			.get("meta")
			.unwrap()
			.get("id")
			.unwrap()
			.as_str()
			.unwrap() == pattern
		{
			defs.push(entries.get(i).unwrap().get("shortdef").unwrap().as_array().unwrap());
		} else {
			break;
		}
	}

	// Create the embed
	let mut embed = CreateEmbed::default();
	embed.title(format!("The definition of {} is:\n", &word));

	let mut entry_number = 1;
	// Iterate through the definitions
	for def in defs {
		let mut field = String::from("");
		let mut index = 1;

		// Iterate through each definitions items
		for item in def {
			// Capitalise the first letter of each item
			let mut v: Vec<char> = item.as_str().unwrap().chars().collect();
			v[0] = v[0].to_uppercase().next().unwrap();
			let capitalised: String = v.into_iter().collect();

			field.push_str(&*format!("{}. {}\n\n", index, capitalised));
			index += 1;
		}

		// Get the word type
		let word_type = entries
			.get(entry_number - 1)
			.unwrap()
			.get("fl")
			.unwrap()
			.as_str()
			.unwrap();

		// Add the field to the embed
		embed.field(&*format!("\nEntry {} ({}):\n", entry_number, &word_type), field, true);
		entry_number += 1;
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
