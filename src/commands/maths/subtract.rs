use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases(sub, minus, "-")]
#[description = "Subtracts several numbers"]
pub async fn subtract(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let mut product = 1f64;

	for arg in args.iter::<f64>() {
		product -= arg.unwrap();
	}

	msg.reply(&ctx.http, product).await?;
	Ok(())
}
