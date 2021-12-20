use serenity::builder::{CreateEmbed, CreateEmbedAuthor};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

#[command]
#[aliases(def)]
#[description = "Defines the given words"]
async fn define(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let word = args.single::<String>()?;

    let token = env::var("DICTIONARY_TOKEN").expect("Expected a token in the environment");
    let url = format!(
        "https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}?key={}",
        &word, token
    );
    let json: serde_json::Value = reqwest::get(&url).await?.json().await?;

    let entries = json.as_array().unwrap();
    let mut defs: Vec<&Vec<serde_json::Value>> = vec![json
        .get(0)
        .unwrap()
        .get("shortdef")
        .unwrap()
        .as_array()
        .unwrap()];

    for i in 1..entries.len() {
        let value = format!("{}:{}", &word, &i + 1);
        let pattern = value.as_str();

        if entries
            .get(i)
            .unwrap()
            .get("meta")
            .unwrap()
            .get("id")
            .unwrap()
            .as_str()
            .unwrap()
            .starts_with(&pattern)
        {
            defs.push(
                entries
                    .get(i)
                    .unwrap()
                    .get("shortdef")
                    .unwrap()
                    .as_array()
                    .unwrap(),
            );
        } else {
            break;
        }
    }

    let mut embed = CreateEmbed::default();
    embed.title(format!("The definition of {} is:\n", &word));

    let mut entry_number = 1;
    for def in defs {
        let mut field = String::from("");
        let mut index = 1;

        for item in def {
            let mut v: Vec<char> = item.as_str().unwrap().chars().collect();
            v[0] = v[0].to_uppercase().nth(0).unwrap();
            let capitalised: String = v.into_iter().collect();

            field.push_str(&*format!("{}. {}\n", index, capitalised));
            index += 1;
        }

        embed.field(&*format!("\nEntry {}:\n", entry_number), field, true);
        entry_number += 1;
    }

    let mut auth = CreateEmbedAuthor::default();
    auth.name(&msg.author.name);
    auth.url(
        &msg.author
            .avatar_url()
            .unwrap_or(String::from(&msg.author.default_avatar_url())),
    );
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
