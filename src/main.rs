mod bot;
mod messagehandler;
mod database;

use bot::Bot;

#[macro_use] extern crate lazy_static;

fn main() {
	let mut bot = Bot::new();
	bot.add_handler("file".to_string(), messagehandler::FileHandler{});
	bot.run();
}
