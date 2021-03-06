use serenity::framework::standard::macros::group;

pub mod admin;
pub use admin::*;
#[group]
#[owners_only(true)]
#[commands(quit, restart, update)]
struct Admin;

pub mod maths;
pub use maths::*;
#[group]
#[commands(multiply, divide, add, subtract, random, power)]
struct Maths;

pub mod info;
pub use info::*;
#[group]
#[commands(ping, about)]
struct Info;

pub mod words;
pub use words::*;
#[group]
#[commands(define, synonyms, poll, libgen)]
struct Words;

pub mod games;
pub use games::*;
#[group]
#[commands(map, squirdle)]
struct Games;

pub mod voice;
pub use voice::*;
#[group]
#[only_in(guilds)]
#[commands(join, leave, play, now_playing, skip, pause, resume, volume, queue)]
struct Voice;
