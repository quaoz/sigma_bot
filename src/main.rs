#![feature(int_roundings)]

pub mod commands;

use crate::commands::*;
use serenity::{
	async_trait,
	client::{bridge::gateway::ShardManager, Client, Context, EventHandler},
	framework::{
		standard::{
			macros::{hook},
			Args, CommandResult,
		},
		StandardFramework,
	},
	http::Http,
	model::{channel::Message, event::ResumedEvent, gateway::Ready},
	prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};
use std::collections::HashMap;

use lavalink_rs::{gateway::*, model::*, LavalinkClient};
use serenity::model::channel::Reaction;
use serenity::model::channel::ReactionType;
use serenity::model::id::ChannelId;
use serenity::model::id::MessageId;
use songbird::SerenityInit;
use tokio::sync::Mutex;

use tracing::{debug, error, info, instrument};

struct Lavalink;

impl TypeMapKey for Lavalink {
	type Value = LavalinkClient;
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
	type Value = Arc<Mutex<ShardManager>>;
}

enum ReactionEvent<'a> {
	Reaction(&'a Reaction),
	RemoveAll(ChannelId, MessageId),
}

struct PollsKey;

impl TypeMapKey for PollsKey {
	type Value = Arc<Mutex<PollsMap>>;
}

type PollsMap = HashMap<(ChannelId, MessageId), Poll>;

struct Poll {
	pub question: String,
	pub answers: Vec<String>,
	pub answerers: Vec<usize>,
}

fn render_message(poll: &Poll) -> String {
	let mut message_text = format!("**Poll:** {}\n", poll.question);
	let total_answerers = poll.answerers.iter().sum::<usize>();

	for (i, (answer, &num)) in poll.answers.iter().zip(poll.answerers.iter()).enumerate() {
		let emoji = std::char::from_u32('🇦' as u32 + i as u32).expect("Failed to format emoji");
		message_text.push(emoji);

		if total_answerers > 0 {
			let percent = num as f64 / total_answerers as f64 * 100.;
			message_text.push_str(&format!(" {:.0}%", percent));
		}

		message_text.push(' ');
		message_text.push_str(answer);
		message_text.push_str(&format!(" ({} votes)", num));
		message_text.push('\n');
	}

	message_text
}

macro_rules! perform_reaction {
	(($ctx:expr, $reaction_event:expr) $body:expr) => {
		use ReactionEvent::*;

		// Discard if it's our own reaction.
		if let Reaction(r) = $reaction_event {
			if r.user_id == Some($ctx.cache.current_user_id().await) {
				return;
			}
		}

		let key = match $reaction_event {
			Reaction(r) => (r.channel_id, r.message_id),
			RemoveAll(c, m) => (c, m),
		};

		// Try to get poll for the given message otherwise return
		{
			let poll_data = $ctx.data.read().await;
			let poll_map = poll_data
				.get::<PollsKey>()
				.expect("Failed to retrieve polls map!")
				.lock()
				.await;
			if !poll_map.contains_key(&key) {
				debug!("Message not in polls map, ignoring");
				return;
			}
		}

		let mut poll_data = $ctx.data.write().await;
		let mut poll_map = poll_data
			.get_mut::<PollsKey>()
			.expect("Failed to retrieve polls map!")
			.lock()
			.await;
		let poll = match poll_map.get_mut(&key) {
			None => {
				debug!("Failed to get poll for {:?}", key);
				return;
			}
			Some(poll) => poll,
		};

		// nudges Rust towards the right type :)
		fn get_f<F: FnOnce(&mut Poll, Option<usize>)>(f: F) -> F {
			f
		}
		let f = get_f($body);

		match $reaction_event {
			Reaction(r) => match r.emoji {
				ReactionType::Unicode(ref s) => {
					let c = s.chars().nth(0).unwrap();
					let end_char = std::char::from_u32('🇦' as u32 + poll.answers.len() as u32 - 1)
						.expect("Failed to format emoji");
					if c < '🇦' || c > end_char {
						debug!("Emoji is not regional indicator or is not in range, ignoring");
						return;
					}
					let number = (c as u32 - '🇦' as u32) as usize;

					f(poll, Some(number));
				}
				_ => {
					debug!("Unknown emoji in reaction, ignoring");
					return;
				}
			},
			RemoveAll(..) => f(poll, None),
		}

		let content = render_message(&poll);
		key.0
			.edit_message(&$ctx.http, key.1, |edit| edit.content(&content))
			.await
			.expect("Failed to edit message");
	};
}

struct Handler;
struct LavalinkHandler;

#[async_trait]
impl EventHandler for Handler {
	async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
		perform_reaction! { (ctx, ReactionEvent::Reaction(&add_reaction)) |poll, number| {
		  poll.answerers[number.unwrap()] += 1;
		}}
	}

	async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {
		perform_reaction! { (ctx, ReactionEvent::Reaction(&removed_reaction)) |poll, number| {
		  poll.answerers[number.unwrap()] -= 1;
		}}
	}

	async fn reaction_remove_all(&self, ctx: Context, channel_id: ChannelId, removed_from_message_id: MessageId) {
		perform_reaction! { (ctx, ReactionEvent::RemoveAll(channel_id, removed_from_message_id)) |poll, _| {
		  for answers in poll.answerers.iter_mut() {
			*answers = 0;
		  }
		}}
	}

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
		info!("Track started! Guild: {}", event.guild_id);
	}
	async fn track_finish(&self, _client: LavalinkClient, event: TrackFinish) {
		info!("Track finished! Guild: {}", event.guild_id);
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
	if let Err(why) = command_result {
		info!("Command '{}' returned error {:?} => {}", command_name, why, why)
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
			if let Some(team) = info.team {
				owners.insert(team.owner_user_id);
			} else {
				owners.insert(info.owner.id);
			}
			match http.get_current_user().await {
				Ok(bot_id) => (owners, bot_id.id),
				Err(why) => panic!("Could not access the bot id: {:?}", why),
			}
		}
		Err(why) => panic!("Could not access application info: {:?}", why),
	};

	// Create the framework
	let framework = StandardFramework::new()
		.configure(|c| {
			c.owners(owners)
				.on_mention(Some(bot_id))
				.with_whitespace(true)
				.case_insensitivity(true)
				.no_dm_prefix(true)
				.prefix("~")
		})
		.before(before)
		.after(after)
		.group(&ADMIN_GROUP)
		.group(&INFO_GROUP)
		.group(&MATHS_GROUP)
		.group(&WORDS_GROUP)
		.group(&VOICE_GROUP)
		.group(&GAMES_GROUP);

	let mut client = Client::builder(&token)
		.framework(framework)
		.event_handler(Handler)
		.register_songbird()
		.await
		.expect("Err creating client");

	let lava_client = LavalinkClient::builder(bot_id)
		.set_host("127.0.0.1")
		.set_port(2333)
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
