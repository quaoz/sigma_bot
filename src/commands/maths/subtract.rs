use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[min_args(1)]
#[aliases(sub, minus, "-")]
#[description = "Subtracts several numbers"]
#[usage("subtract <numbers>")]
#[example("subtract 54 3 92")]
pub async fn subtract(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let mut product = args.single::<f64>()?;

	for _ in 1..args.len() {
		product -= args.single::<f64>()?;
	}

	msg.reply(&ctx.http, product).await?;
	Ok(())
}
