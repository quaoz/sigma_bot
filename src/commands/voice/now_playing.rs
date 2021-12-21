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
#[aliases(np)]
async fn now_playing(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let lava_client = data.get::<Lavalink>().unwrap().clone();

	if let Some(node) = lava_client.nodes().await.get(&msg.guild_id.unwrap().0) {
		if let Some(track) = &node.now_playing {
			check_msg(
				msg.channel_id
						.say(
							&ctx.http,
							format!("Now Playing: {}", track.track.info.as_ref().unwrap().title),
						)
						.await,
			);
		} else {
			check_msg(
				msg.channel_id
						.say(&ctx.http, "Nothing is playing at the moment.")
						.await,
			);
		}
	} else {
		check_msg(
			msg.channel_id
					.say(&ctx.http, "Nothing is playing at the moment.")
					.await,
		);
	}

	Ok(())
}
