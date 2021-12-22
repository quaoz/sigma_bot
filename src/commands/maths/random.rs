use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::{mem::swap, ops::Add};

use rand::Rng;

#[command]
#[min_args(1)]
#[max_args(2)]
#[aliases(rand, rnd)]
#[description = "Generates a random number between two bounds"]
#[usage("rand <bound> <optional: bound>")]
#[example("rand 10 100")]
pub async fn random(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let mut min = 0f64;

	if args.len() == 2 {
		min = args.single::<f64>()?;
	}

	let mut max = args.single::<f64>()?;

	if min > max {
		swap(&mut min, &mut max);
	}

	let product = rand::thread_rng().gen_range(min..max.add(1f64)).floor();

	msg.reply(&ctx.http, product).await?;
	Ok(())
}
