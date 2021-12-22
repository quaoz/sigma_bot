use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[num_args(0)]
#[aliases(latency)]
#[description("Displays the bots ping")]
#[usage("ping")]
#[example("ping")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
	let content = format!(
		"My ping is: {}ms",
		chrono::Utc::now().timestamp_millis() - msg.timestamp.timestamp_millis()
	);
	msg.reply(&ctx.http, content).await?;

	Ok(())
}
