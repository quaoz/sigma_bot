use crate::Lavalink;
use serenity::{
	client::Context,
	framework::standard::{macros::command, CommandResult},
	model::channel::Message,
};

#[command]
#[num_args(0)]
#[description("Pauses the current track")]
#[usage("pause")]
#[example("pause")]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let lava_client = data.get::<Lavalink>().unwrap().clone();

	if let Err(_why) = lava_client.pause(msg.guild_id.unwrap()).await {
		msg.reply(&ctx.http, "Nothing to pause.").await?;
	} else {
		msg.reply(&ctx.http, "Paused").await?;
	}

	Ok(())
}
