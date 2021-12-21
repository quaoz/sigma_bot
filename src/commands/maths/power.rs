use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases(pow, raise, "^")]
#[description = "Raises one number to the power of another"]
pub async fn power(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let number_one = args.single::<f64>()?;
	let number_two = args.single::<f64>()?;

	let product = f64::powf(number_one, number_two);
	msg.reply(&ctx.http, product).await?;
	Ok(())
}
