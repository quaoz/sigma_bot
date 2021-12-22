use crate::Lavalink;
use serenity::{
	client::Context,
	framework::standard::{macros::command, CommandResult},
	model::channel::Message,
};

#[command]
#[num_args(0)]
#[aliases(next)]
#[description("Skips the current track")]
#[usage("skip")]
#[example("skip")]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let lava_client = data.get::<Lavalink>().unwrap().clone();

	if let Some(track) = lava_client.skip(msg.guild_id.unwrap()).await {
		msg.reply(ctx, format!("Skipped: {}", track.track.info.as_ref().unwrap().title))
			.await?;
	} else {
		msg.reply(&ctx.http, "Nothing to skip.").await?;
	}

	Ok(())
}
