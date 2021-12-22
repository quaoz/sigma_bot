use crate::Lavalink;
use serenity::{
	client::Context,
	framework::standard::{macros::command, CommandResult},
	model::{channel::Message, misc::Mentionable},
};

#[command]
#[num_args(0)]
#[aliases(summon)]
#[description("Join the voice channel the author is in")]
#[usage("join")]
#[example("join")]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
	let guild = msg.guild(&ctx.cache).await.unwrap();
	let guild_id = guild.id;

	let channel_id = guild
		.voice_states
		.get(&msg.author.id)
		.and_then(|voice_state| voice_state.channel_id);

	let connect_to = match channel_id {
		Some(channel) => channel,
		None => {
			msg.reply(&ctx.http, "Join a voice channel first.").await?;

			return Ok(());
		}
	};

	let manager = songbird::get(ctx).await.unwrap().clone();
	let (_, handler) = manager.join_gateway(guild_id, connect_to).await;

	match handler {
		Ok(connection_info) => {
			let data = ctx.data.read().await;
			let lava_client = data.get::<Lavalink>().unwrap().clone();
			lava_client.create_session_with_songbird(&connection_info).await?;

			msg.reply(&ctx.http, &format!("Joined {}", connect_to.mention()))
				.await?;
		}
		Err(why) => {
			msg.reply(&ctx.http, format!("Error joining the channel: {}", why))
				.await?;
		}
	}

	Ok(())
}
