use regex::Regex;
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
    let resp = reqwest::get(format!("http://libgen.is/search.php?req={}", title.replace(' ', "+"))).await?.text().await?;
    let md5 = Regex::new("md5=(.*?)'").unwrap().captures(resp.as_str()).unwrap().get(1).unwrap().as_str();

    let json: serde_json::Value = reqwest::get(format!("http://libgen.is/json.php?fields=ipfs_cid,author,title,publisher,year,extension&ids={}", &md5)).await?.json().await?;
    let dl_url = format!("https://dweb.link/ipfs/{}?filename={}%20-%20{}-{}%20%28{}%29.{}", sanitise(&json, "ipfs_cid"), sanitise(&json, "author"), sanitise(&json, "title"), sanitise(&json, "publisher"), sanitise(&json, "year"), sanitise(&json, "extension"));

    println!("{}", title);

    // Send the message
    msg.channel_id
        .send_message(&ctx, |f| {
            f.content("")
                .embed(|e| {
                    e.title(format!("LibGen results for \"{}\":\n", &title))
                        .field(get_field(&json, "title"), dl_url, false);
                    e
                })
        })
        .await
        .unwrap();

    Ok(())
}

fn sanitise(json: &serde_json::Value, value: &str) -> String {
   get_field(json, value).to_lowercase().replace(' ', "%20").replace(':', "_")
}

fn get_field(json: &serde_json::Value, value: &str) -> String {
    json.get(0).unwrap().get(value).unwrap().to_string()
}






















