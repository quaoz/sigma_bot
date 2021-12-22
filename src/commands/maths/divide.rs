use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[min_args(1)]
#[aliases(div, "/")]
#[description("Divides several numbers")]
#[usage("divide <numbers>")]
#[example("divide 144 12 3")]
pub async fn divide(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let mut product = args.single::<f64>()?;

	for _ in 1..args.len() {
		product /= args.single::<f64>()?;
	}

	msg.reply(&ctx.http, product).await?;
	Ok(())
}
