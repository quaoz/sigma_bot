use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::process::Command;

use crate::ShardManagerContainer;

#[command]
#[num_args(0)]
#[aliases(reboot)]
#[sub_commands(restart_bot, restart_lavalink)]
#[description("Restarts the bot and lavalink")]
#[usage("restart")]
#[example("restart")]
async fn restart(ctx: &Context, msg: &Message) -> CommandResult {
	let mut response = if Command::new("./Lavalink.sh")
		.arg("-r")
		.output()
		.expect("Failed to run command")
		.status
		.success()
	{
		String::from("Restarting Lavalink, ")
	} else {
		String::from("Failed to restart Lavalink, ")
	};

	let data = ctx.data.read().await;

	if let Some(manager) = data.get::<ShardManagerContainer>() {
		if Command::new("./Bot.sh")
			.arg("-s")
			.output()
			.expect("Failed to run command")
			.status
			.success()
		{
			response.push_str("restarting myself!");
			msg.reply(ctx, response).await?;
			manager.lock().await.shutdown_all().await;
		} else {
			response.push_str("failed to restart myself.");
			msg.reply(ctx, response).await?;
		}
	} else {
		msg.reply(ctx, "There was a problem getting the shard manager.").await?;
	}

	Ok(())
}

#[command("bot")]
#[num_args(0)]
#[description("Restarts the bot")]
#[usage("restart bot")]
#[example("restart bot")]
async fn restart_bot(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;

	if let Some(manager) = data.get::<ShardManagerContainer>() {
		if Command::new("./Bot.sh")
			.arg("-s")
			.output()
			.expect("Failed to run command")
			.status
			.success()
		{
			msg.reply(ctx, "Restarting!").await?;
			manager.lock().await.shutdown_all().await;
		} else {
			msg.reply(ctx, "Failed to execute restart command.").await?;
		}
	} else {
		msg.reply(ctx, "There was a problem getting the shard manager.").await?;
	}

	Ok(())
}

#[command("lavalink")]
#[num_args(0)]
#[aliases(ll)]
#[description("Restarts Lavalink")]
#[usage("restart lavalink")]
#[example("restart lavalink")]
async fn restart_lavalink(ctx: &Context, msg: &Message) -> CommandResult {
	if Command::new("./Lavalink.sh")
		.arg("-r")
		.output()
		.expect("Failed to run command")
		.status
		.success()
	{
		msg.reply(ctx, "Restarting Lavalink!").await?;
	} else {
		msg.reply(ctx, "Failed to restart Lavalink.").await?;
	}

	Ok(())
}
