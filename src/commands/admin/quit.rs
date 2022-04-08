use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::process::Command;

use crate::ShardManagerContainer;

#[command]
#[num_args(0)]
#[aliases(kill)]
#[sub_commands(quit_bot, quit_lavalink)]
#[description("Quits the bot and lavalink")]
#[usage("quit")]
#[example("quit")]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
	let mut response = if Command::new("./Lavalink.sh")
		.arg("-k")
		.output()
		.expect("Failed to run command")
		.status
		.success()
	{
		String::from("Quit Lavalink, ")
	} else {
		String::from("Failed to quit Lavalink, ")
	};

	let data = ctx.data.read().await;

	if let Some(manager) = data.get::<ShardManagerContainer>() {
		response.push_str("shutting down!");
		msg.reply(ctx, response).await?;
		manager.lock().await.shutdown_all().await;
	} else {
		response.push_str("problem getting the shard manager.");
		msg.reply(ctx, response).await?;
	}

	Ok(())
}

#[command("bot")]
#[num_args(0)]
#[description("Quits the bot")]
#[usage("quit bot")]
#[example("quit bot")]
async fn quit_bot(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;

	if let Some(manager) = data.get::<ShardManagerContainer>() {
		msg.reply(ctx, "Shutting down!").await?;
		manager.lock().await.shutdown_all().await;
	} else {
		msg.reply(ctx, "Problem getting the shard manager.").await?;
	}

	Ok(())
}

#[command("lavalink")]
#[num_args(0)]
#[aliases(ll)]
#[description("Quits Lavalink")]
#[usage("quit lavalink")]
#[example("quit lavalink")]
async fn quit_lavalink(ctx: &Context, msg: &Message) -> CommandResult {
	if Command::new("./Lavalink.sh")
		.arg("-k")
		.output()
		.expect("Failed to run command")
		.status
		.success()
	{
		msg.reply(ctx, "Quit Lavalink!").await?;
	} else {
		msg.reply(ctx, "Failed to quit Lavalink").await?;
	}

	Ok(())
}
