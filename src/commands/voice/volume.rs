use crate::Lavalink;
use serenity::{
	client::Context,
	framework::standard::{macros::command, CommandResult, Args},
	model::channel::Message,
};

#[command]
#[num_args(1)]
#[aliases(vol)]
#[description("Sets the volume of the player")]
#[usage("volume <number>")]
#[example("volume 10")]
async fn volume(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let data = ctx.data.read().await;
	let lava_client = data.get::<Lavalink>().unwrap().clone();

	let volume = args.single::<u16>()?;

	if let Err(_why) = lava_client.volume(msg.guild_id.unwrap(), volume).await {
		msg.reply(&ctx.http, "Unable to change the volume").await?;
	} else {
		msg.reply(&ctx.http, format!("Set the volume to: {}", volume)).await?;
	}

	Ok(())
}
