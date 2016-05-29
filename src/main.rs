extern crate discord;

mod messagehandler;

use discord::Discord;
use discord::model::Event;

fn main() {
	// Log in to Discord using the email and password in the environment
	let discord = Discord::from_bot_token("MTg2NTU3NTY3OTczOTgyMjA4.CizYBw.76v0_VOHfCNT5hU4_ojVVYgVVE0").expect("login failed");

	// Establish and use a websocket connection
	let (mut connection, _) = discord.connect().expect("connect failed");
	println!("Ready.");
	loop {
		match connection.recv_event() {
			Ok(Event::MessageCreate(message)) => {
				println!("{} says: {}", message.author.name, message.content);
				if message.content == "!test" {
					let _ = discord.send_message(&message.channel_id, "This is a reply to the test.", "", false);
				} else if message.author.name == "vakz" && message.content == "!quit" {
					println!("Quitting.");
					break
				}
			}
			Ok(_) => {}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed on us with code {:?}: {}", code, String::from_utf8_lossy(&body));
				break
			}
			Err(err) => println!("Receive error: {:?}", err)
		}
	}

	// Log out from the API
	discord.logout().expect("logout failed");
}


// let discord = Discord::from_bot_token("MTg2NTU3NTY3OTczOTgyMjA4.CizYBw.76v0_VOHfCNT5hU4_ojVVYgVVE0").expect("login failed");
