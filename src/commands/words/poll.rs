use crate::util::poll_utils::{render_message, Poll, PollsKey};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn poll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let question = args.single_quoted::<String>()?;
	let answers = args
		.quoted()
		.iter::<String>()
		.filter_map(|x| x.ok())
		.collect::<Vec<_>>();

	let answers_len = answers.len();
	let poll = Poll {
		question,
		answerers: vec![0; answers_len],
		answers,
	};

	let message_text = render_message(&poll);
	let emojis = (0..answers_len)
		.map(|i| std::char::from_u32('🇦' as u32 + i as u32).expect("Failed to format emoji"))
		.collect::<Vec<_>>();

	let poll_msg = msg.channel_id.say(&ctx.http, &message_text).await?;

	for &emoji in &emojis {
		poll_msg
			.react(&ctx.http, ReactionType::Unicode(emoji.to_string()))
			.await?;
	}

	let mut poll_data = ctx.data.write().await;

	let poll_map = poll_data.get_mut::<PollsKey>().expect("Failed to retrieve polls map!");

	poll_map.lock().await.insert((msg.channel_id, poll_msg.id), poll);

	Ok(())
}
