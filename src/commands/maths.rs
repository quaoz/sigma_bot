use std::ops::Add;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use rand::Rng;

#[command]
#[aliases(times, x, "*")]
#[description = "Multiplies several numbers"]
pub async fn multiply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let mut product = 1f64;

	for arg in args.iter::<f64>() {
		product *= arg.unwrap();
	}

	msg.reply(&ctx.http, product).await?;
	Ok(())
}

#[command]
#[aliases(div, "/")]
#[description = "Divides several numbers"]
pub async fn divide(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let mut product = 1f64;

	for arg in args.iter::<f64>() {
		product /= arg.unwrap();
	}

	msg.reply(&ctx.http, product).await?;
	Ok(())
}

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

#[command]
#[aliases(rand, rng, rnd)]
#[description = "Generates a random number between two parameters"]
pub async fn random(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let mut min = 0f64;

	if args.len() == 2 {
		min = args.single::<f64>()?;
	}

	let mut max = args.single::<f64>()?;

	if min > max {
		let tmp = min;
		min = max;
		max = tmp;
	}

	let product = rand::thread_rng().gen_range(min..max.add(1f64)).floor();

	msg.reply(&ctx.http, product).await?;
	Ok(())
}
