use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[min_args(1)]
#[aliases(times, "*")]
#[description = "Multiplies several numbers"]
#[usage("multiply <numbers>")]
#[example("multiply 236 41 101")]
pub async fn multiply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let mut product = 1f64;

	for arg in args.iter::<f64>() {
		product *= arg.unwrap();
	}

	msg.reply(&ctx.http, product).await?;
	Ok(())
}
