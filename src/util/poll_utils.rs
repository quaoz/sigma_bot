use serenity::model::id::{ChannelId, MessageId};
use serenity::model::prelude::Reaction;
use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct PollsKey;

impl TypeMapKey for PollsKey {
	type Value = Arc<Mutex<PollsMap>>;
}

type PollsMap = HashMap<(ChannelId, MessageId), Poll>;

pub struct Poll {
	pub question: String,
	pub answers: Vec<String>,
	pub answerers: Vec<usize>,
}

pub enum ReactionEvent<'a> {
	Reaction(&'a Reaction),
	RemoveAll(ChannelId, MessageId),
}

pub fn render_message(poll: &Poll) -> String {
	let mut message_text = format!("**Poll:** {}\n\n", poll.question);
	//let total_answerers = poll.answerers.iter().sum::<usize>();

	for (i, (answer, &num)) in poll.answers.iter().zip(poll.answerers.iter()).enumerate() {
		let emoji = std::char::from_u32('🇦' as u32 + i as u32).expect("Failed to format emoji");
		message_text.push(emoji);

		//if total_answerers > 0 {
		//	let percent = num as f64 / total_answerers as f64 * 100.;
		//	message_text.push_str(&format!(" {:.0}%", percent));
		//}

		message_text.push(' ');
		message_text.push_str(answer);
		message_text.push_str(&format!(" ({} votes)", num));
		message_text.push('\n');
	}

	message_text
}
