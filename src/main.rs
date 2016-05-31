extern crate discord;

mod messagehandler;

use discord::Discord;
use discord::model::Event;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn get_token() -> Result<String, String> {
	let mut f = match File::open("key.txt") {
		Ok(file) => file,
		Err(err) => return Err(err.description().to_string()),
	};

	let mut buffer = String::new();

	match f.read_to_string(&mut buffer) {
		Ok(size) => {
			if size > 0 { Ok(buffer[0..(size-1)].to_string()) }
			else { Err("No key in file!".to_string()) }
		},
		Err(err) => Err(err.description().to_string()),
	}
}

fn main() {
	// Log in to Discord using the email and password in the environment
	let key = match get_token() {
		Ok(key) => key,
		Err(_) => panic!("Found no token!"),
	};

	let discord = Discord::from_bot_token(&key).expect("login failed");

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
