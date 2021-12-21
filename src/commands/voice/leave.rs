use serenity::{
	client::{Context},
	framework::{
		standard::{
			macros::{command},
			CommandResult,
		},
	},
	model::{channel::Message},
};
use crate::{check_msg, Lavalink};

#[command]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
	let guild = msg.guild(&ctx.cache).await.unwrap();
	let guild_id = guild.id;

	let manager = songbird::get(ctx).await.unwrap().clone();
	let has_handler = manager.get(guild_id).is_some();

	if has_handler {
		if let Err(e) = manager.remove(guild_id).await {
			check_msg(
				msg.channel_id
						.say(&ctx.http, format!("Failed: {:?}", e))
						.await,
			);
		}

		{
			let data = ctx.data.read().await;
			let lava_client = data.get::<Lavalink>().unwrap().clone();
			lava_client.destroy(guild_id).await?;
		}

		check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);
	} else {
		check_msg(msg.reply(&ctx.http, "Not in a voice channel").await);
	}

	Ok(())
}
