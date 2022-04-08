use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("info")]
async fn about(context: &Context, message: &Message) -> CommandResult {
    let current_user = context.cache.current_user().await;

    let version = env!("CARGO_PKG_VERSION").to_string();
    let bot_owner = context.http.get_current_application_info().await?.owner.tag();
    let bot_name = &current_user.name;
    let bot_avatar = &current_user.avatar_url().unwrap();
    let num_shards = context.cache.shard_count().await;
    let num_channels = context.cache.guild_channel_count().await;
    let num_users = context.cache.user_count().await;

    let about_fields = vec![
        ("Version", version, true),
        ("Owner", bot_owner, true),
        ("Shards", num_shards.to_string(), true),
        ("Channels", num_channels.to_string(), true),
        ("Users", num_users.to_string(), true),
    ];

    message
        .channel_id
        .send_message(&context, |message| {
            message.embed(|embed| {
                embed.title(format!("**{}**", bot_name));
                embed.url("https://github.com/quaoz/sigma_bot");
                embed.thumbnail(bot_avatar);
                embed.fields(about_fields);
                embed.footer(|footer| footer.text("Written with Rust & serenity."));
                embed
            })
        })
        .await?;

    Ok(())
}