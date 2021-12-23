use crate::Lavalink;
use serenity::{
	client::Context,
	framework::standard::{macros::command, CommandResult},
	model::channel::Message,
};

#[command("now playing")]
#[num_args(0)]
#[aliases(playing, np)]
#[description("Displays the currently playing track")]
#[usage("now_playing")]
#[example("now_playing")]
async fn now_playing(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let lava_client = data.get::<Lavalink>().unwrap().clone();

	if let Some(node) = lava_client.nodes().await.get(&msg.guild_id.unwrap().0) {
		if let Some(track) = &node.now_playing {
			msg.reply(
				&ctx.http,
				format!("Now Playing: {}", track.track.info.as_ref().unwrap().title),
			)
			.await?;
		} else {
			msg.reply(&ctx.http, "Nothing is playing at the moment.").await?;
		}
	} else {
		msg.reply(&ctx.http, "Nothing is playing at the moment.").await?;
	}

	Ok(())
}
