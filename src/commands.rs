use serenity::framework::standard::macros::group;

pub mod admin;
pub use admin::*;
#[group]
#[commands(quit)]
struct Admin;

pub mod maths;
pub use maths::*;
#[group]
#[commands(multiply, divide, add, subtract, random, power)]
struct Maths;

pub mod info;
pub use info::*;
#[group]
#[commands(ping)]
struct Info;

pub mod words;
pub use words::*;
#[group]
#[commands(define)]
struct Words;

pub mod voice;
pub use voice::*;
#[group]
#[only_in(guilds)]
#[commands(join, leave, play, now_playing, skip)]
struct Voice;
