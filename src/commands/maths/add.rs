use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases(plus, "+")]
#[description = "Adds several numbers"]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let mut product = 1f64;

	for arg in args.iter::<f64>() {
		product += arg.unwrap();
	}

	msg.reply(&ctx.http, product).await?;
	Ok(())
}
