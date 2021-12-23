use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::process::Command;

use crate::ShardManagerContainer;

#[command]
#[num_args(0)]
#[aliases(kill)]
#[sub_commands(update_bot, update_lavalink)]
#[description("Updates the bot and lavalink")]
#[usage("update")]
#[example("update")]
async fn update(ctx: &Context, msg: &Message) -> CommandResult {
	let mut response = if Command::new("./Lavalink.sh")
		.arg("-u")
		.output()
		.expect("Failed to run command")
		.status
		.success()
	{
		String::from("Updating Lavalink, ")
	} else {
		String::from("Failed to update Lavalink, ")
	};

	let data = ctx.data.read().await;

	if let Some(manager) = data.get::<ShardManagerContainer>() {
		manager.lock().await.shutdown_all().await;

		if Command::new("./Bot.sh")
			.arg("-u")
			.output()
			.expect("Failed to run command")
			.status
			.success()
		{
			response.push_str("updating myself!")
		} else {
			response.push_str("failed to update myself.")
		}
		msg.reply(ctx, response).await?;
	} else {
		msg.reply(ctx, "There was a problem getting the shard manager.").await?;
	}

	Ok(())
}

#[command("bot")]
#[num_args(0)]
#[description("Updates the bot")]
#[usage("update bot")]
#[example("update bot")]
async fn update_bot(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;

	if let Some(manager) = data.get::<ShardManagerContainer>() {
		manager.lock().await.shutdown_all().await;

		if Command::new("./Bot.sh")
			.arg("-u")
			.output()
			.expect("Failed to run command")
			.status
			.success()
		{
			msg.reply(ctx, "Updating!").await?;
		} else {
			msg.reply(ctx, "Failed to execute update command.").await?;
		}
	} else {
		msg.reply(ctx, "There was a problem getting the shard manager.").await?;
	}

	Ok(())
}

#[command("lavalink")]
#[num_args(0)]
#[aliases(ll)]
#[description("Updates Lavalink")]
#[usage("update lavalink")]
#[example("update lavalink")]
async fn update_lavalink(ctx: &Context, msg: &Message) -> CommandResult {
	if Command::new("./Lavalink.sh")
		.arg("-u")
		.output()
		.expect("Failed to run command")
		.status
		.success()
	{
		msg.reply(ctx, "Updating Lavalink!").await?;
	} else {
		msg.reply(ctx, "Failed to update Lavalink").await?;
	}

	Ok(())
}
