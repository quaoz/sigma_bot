pub mod commands;

use crate::commands::*;
use serenity::{
	async_trait,
	client::{bridge::gateway::ShardManager, Client, Context, EventHandler},
	framework::{
		standard::{macros::hook, CommandResult},
		StandardFramework,
	},
	http::Http,
	model::{channel::Message, event::ResumedEvent, gateway::Ready},
	prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};

use lavalink_rs::{gateway::*, model::*, LavalinkClient};
use songbird::SerenityInit;

use tracing::{debug, error, info, instrument};

struct Lavalink;

impl TypeMapKey for Lavalink {
	type Value = LavalinkClient;
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
	type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;
struct LavalinkHandler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, _: Context, ready: Ready) {
		info!("Connected as {}", ready.user.name);
	}

	#[instrument(skip(self, _ctx))]
	async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
		debug!("Resumed; trace: {:?}", resume.trace);
	}
}

#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {
	async fn track_start(&self, _client: LavalinkClient, event: TrackStart) {
		info!("Track started!\nGuild: {}", event.guild_id);
	}
	async fn track_finish(&self, _client: LavalinkClient, event: TrackFinish) {
		info!("Track finished!\nGuild: {}", event.guild_id);
	}
}

#[hook]
#[instrument]
async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
	info!("Got command '{}' by user '{}'", command_name, msg.author.name);
	true
}

#[hook]
#[instrument]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
	match command_result {
		Err(why) => info!("Command '{}' returned error {:?} => {}", command_name, why, why),
		_ => (),
	}
}

#[tokio::main]
#[instrument]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Load the environmental variables
	dotenv::dotenv().expect("Failed to load .env file");

	env::set_var("RUST_LOG", "info,lavalink_rs=debug");

	// Initialize the logger to use environment variables
	tracing_subscriber::fmt::init();

	let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

	let http = Http::new_with_token(&token);

	// Fetch your bots owners and id
	let (owners, bot_id) = match http.get_current_application_info().await {
		Ok(info) => {
			let mut owners = HashSet::new();
			owners.insert(info.owner.id);

			(owners, info.id)
		}
		Err(why) => panic!("Could not access application info: {:?}", why),
	};

	// Create the framework
	let framework = StandardFramework::new()
		.configure(|c| c.owners(owners).on_mention(Some(bot_id)).prefix("~"))
		.before(before)
		.after(after)
		.group(&ADMIN_GROUP)
		.group(&INFO_GROUP)
		.group(&MATHS_GROUP)
		.group(&WORDS_GROUP)
		.group(&VOICE_GROUP);

	let mut client = Client::builder(&token)
		.framework(framework)
		.event_handler(Handler)
		.register_songbird()
		.await
		.expect("Err creating client");

	let lava_client = LavalinkClient::builder(bot_id)
		.set_host("127.0.0.1")
		.set_password(env::var("LAVALINK_PASSWORD").unwrap_or_else(|_| "youshallnotpass".to_string()))
		.build(LavalinkHandler)
		.await?;

	{
		let mut data = client.data.write().await;
		data.insert::<ShardManagerContainer>(client.shard_manager.clone());
		data.insert::<Lavalink>(lava_client);
	}

	let shard_manager = client.shard_manager.clone();

	tokio::spawn(async move {
		tokio::signal::ctrl_c()
			.await
			.expect("Could not register ctrl+c handler");
		shard_manager.lock().await.shutdown_all().await;
	});

	if let Err(why) = client.start_autosharded().await {
		error!("Client error: {:?}", why);
	}

	Ok(())
}
