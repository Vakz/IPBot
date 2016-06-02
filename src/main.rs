mod bot;
mod messagehandler;
mod database;

use bot::Bot;

fn main() {
	let mut bot = Bot::new();
	bot.add_handler("file".to_string(), messagehandler::FileHandler{});
	bot.run();
}
