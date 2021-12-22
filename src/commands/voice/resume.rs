use crate::Lavalink;
use serenity::{
	client::Context,
	framework::standard::{macros::command, CommandResult},
	model::channel::Message,
};

#[command]
#[num_args(0)]
#[description("Resumes the current track")]
#[usage("pause")]
#[example("pause")]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let lava_client = data.get::<Lavalink>().unwrap().clone();

	if let Err(_why) = lava_client.resume(msg.guild_id.unwrap()).await {
		msg.reply(&ctx.http, "Nothing to resume.").await?;
	} else {
		msg.reply(&ctx.http, "Resumed").await?;
	}

	Ok(())
}
