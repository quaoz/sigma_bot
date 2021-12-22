use crate::Lavalink;
use serenity::{
	client::Context,
	framework::standard::{macros::command, CommandResult},
	model::channel::Message,
};

#[command]
#[num_args(0)]
#[description("Stops the player")]
#[usage("stop")]
#[example("stop")]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let lava_client = data.get::<Lavalink>().unwrap().clone();

	if let Err(_why) = lava_client.stop(msg.guild_id.unwrap()).await {
		msg.reply(&ctx.http, "Unable to stop.").await?;
	} else {
		msg.reply(&ctx.http, "Stopped").await?;
	}

	Ok(())
}
