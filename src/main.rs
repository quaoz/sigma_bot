pub mod commands;

use crate::{commands::*};
use serenity::{
	client::bridge::gateway::ShardManager,
	framework::standard::{
		macros::{hook},
		StandardFramework
	},
	model::{channel::Message, event::ResumedEvent, gateway::Ready},
	http::Http,
	prelude::*,
	async_trait,
};
use std::{
	collections::HashSet,
	env,
	sync::Arc
};

use tracing::{debug, error, info, instrument};


pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
	type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

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

#[hook]
#[instrument]
async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
	info!("Got command '{}' by user '{}'", command_name, msg.author.name);
	true
}

#[tokio::main]
#[instrument]
async fn main() {
	// Load the environmental variables
	dotenv::dotenv().expect("Failed to load .env file");

	// Initialize the logger to use environment variables
	tracing_subscriber::fmt::init();

	let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

	let http = Http::new_with_token(&token);

	// Fetch your bots owners and id
	let (owners, _bot_id) = match http.get_current_application_info().await {
		Ok(info) => {
			let mut owners = HashSet::new();
			owners.insert(info.owner.id);

			(owners, info.id)
		},
		Err(why) => panic!("Could not access application info: {:?}", why),
	};

	// Create the framework
	let framework = StandardFramework::new()
			.configure(|c| c.owners(owners).on_mention(Some(bot_id)).prefix("~"))
			.before(before)
			.group(&ADMIN_GROUP)
			.group(&INFO_GROUP)
			.group(&MATHS_GROUP)
			.group(&WORDS_GROUP);

	let mut client = Client::builder(&token)
			.framework(framework)
			.event_handler(Handler)
			.await
			.expect("Err creating client");

	{
		let mut data = client.data.write().await;
		data.insert::<ShardManagerContainer>(client.shard_manager.clone());
	}

	let shard_manager = client.shard_manager.clone();

	tokio::spawn(async move {
		tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
		shard_manager.lock().await.shutdown_all().await;
	});

	if let Err(why) = client.start_autosharded().await {
		error!("Client error: {:?}", why);
	}
}
