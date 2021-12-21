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
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let lava_client = data.get::<Lavalink>().unwrap().clone();

	if let Some(track) = lava_client.skip(msg.guild_id.unwrap()).await {
		check_msg(
			msg.channel_id
					.say(
						ctx,
						format!("Skipped: {}", track.track.info.as_ref().unwrap().title),
					)
					.await,
		);
	} else {
		check_msg(msg.channel_id.say(&ctx.http, "Nothing to skip.").await);
	}

	Ok(())
}
