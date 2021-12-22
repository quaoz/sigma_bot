use crate::Lavalink;
use serenity::{
	client::Context,
	framework::standard::{macros::command, CommandResult},
	model::channel::Message,
	builder::{CreateEmbed, CreateEmbedAuthor}
};

#[command]
#[num_args(0)]
#[description("Displays the currently playing track")]
#[usage("queue")]
#[example("queue")]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let lava_client = data.get::<Lavalink>().unwrap().clone();

	// Create the embed
	let mut embed = CreateEmbed::default();
	let mut field = String::from("");
	embed.title("Up Next:");

	if let Some(node) = lava_client.nodes().await.get(&msg.guild_id.unwrap().0) {
		let mut counter = 1;
		for track in &node.queue {
			let length = track.track.info.as_ref().unwrap().length;
			let duration = format!("{}:{}", (length.div_floor(60000)), &(length % 60000).to_string()[..2]);
			field.push_str(&*format!("{}. {} - {}\n", counter, track.track.info.as_ref().unwrap().title, duration));
			counter += 1;
		}
	} else {
		msg.reply(&ctx.http, "Failed to get queue.").await?;
	}

	embed.description(&field);

	// Send the message
	msg.channel_id
			.send_message(&ctx, |f| {
				f.content("").embed(|e| {
					e.0 = embed.0;
					e
				})
			})
			.await
			.unwrap();

	Ok(())
}
