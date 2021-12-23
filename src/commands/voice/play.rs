use crate::Lavalink;
use serenity::{
	client::Context,
	framework::standard::{macros::command, Args, CommandResult},
	model::channel::Message,
};
use tracing::error;

#[command]
#[min_args(1)]
#[description("Searches for and plays the query")]
#[usage("play <query>")]
#[example("play lost in the world by kanye west")]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let query = args.message().to_string();

	let guild_id = match ctx.cache.guild_channel(msg.channel_id).await {
		Some(channel) => channel.guild_id,
		None => {
			msg.channel_id.say(&ctx.http, "Error finding channel info.").await?;

			return Ok(());
		}
	};

	let lava_client = {
		let data = ctx.data.read().await;
		data.get::<Lavalink>().unwrap().clone()
	};

	let manager = songbird::get(ctx).await.unwrap().clone();

	if let Some(_handler) = manager.get(guild_id) {
		let query_information = lava_client.auto_search_tracks(&query).await?;

		if query_information.tracks.is_empty() {
			msg.reply(&ctx, "Could not find any video of the search query.").await?;
			return Ok(());
		}

		if let Err(why) = &lava_client
			.play(guild_id, query_information.tracks[0].clone())
			.queue()
			.await
		{
			error!("{}", why);
			return Ok(());
		};
		msg.reply(
			&ctx.http,
			format!(
				"Added to queue: {}",
				query_information.tracks[0].info.as_ref().unwrap().title
			),
		)
		.await?;
	} else {
		msg.reply(&ctx.http, "Use `~join` first.").await?;
	}

	Ok(())
}
