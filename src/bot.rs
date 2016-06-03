extern crate regex;
extern crate discord;


use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

use database::Database;
use self::regex::Regex;
use std::collections::HashMap;

use self::discord::Discord;
use self::discord::model::Event;

use messagehandler::DiscordMessageHandler;

pub struct Bot {
    client: discord::Discord,
    conn: discord::Connection,
    database: Database,
    handlers: HashMap<String, Box<DiscordMessageHandler>>
}

impl Bot {
    fn get_token() -> Result<String, String> {
        let mut f = match File::open("key.txt") {
    		Ok(file) => file,
    		Err(err) => return Err(err.description().to_string()),
    	};

    	let mut buffer = String::new();

    	match f.read_to_string(&mut buffer) {
    		Ok(size) if size > 0 => Ok(buffer[0..(size-1)].to_string()),
    		Ok(_) => Err("No key in file!".to_string()),
    		Err(err) => Err(err.description().to_string()),
    	}
    }

    pub fn incoming_message(&mut self, msg: &discord::model::Message) -> Option<String> {
    	println!("{} says: {}", msg.author.name, msg.content);
        lazy_static! {
            static ref KEYWORD: Regex = Regex::new(r"^!([:word:]+)").unwrap();
        }

        // Is a command?
        let key = match KEYWORD.captures(&msg.content) {
            Some(key) => key.at(1).unwrap(),
            None => return None
        };
        /*
        if let Some(res) = self.handlers.get(key)
        .and_then(|handler| handler.handle_message(&msg.content, &msg.author.name, &mut self.database)) {
            return Some(res);
        };*/
        let handler = self.handlers.get(key);
        let res = match handler {
            Some(h) => h.handle_message(&msg.content, &msg.author.name, &mut self.database),
            None => None
        };

        if let Some(r) = res {
            return Some(r);
        }
        println!("No handler matched or got nothing back");
    	if msg.content == "!test" {
    		return Some("This is a reply to the test.".to_string());
    	} else if msg.author.name == "vakz" && msg.content == "!quit" {
    		println!("Quitting.");
    	}
    	None
    }

    pub fn add_handler<T: DiscordMessageHandler + 'static>(&mut self, keyword: String, handler: T) {
        self.handlers.insert(keyword, Box::new(handler));
    }

    pub fn new() -> Bot {
        let key = match Bot::get_token() {
    		Ok(key) => key,
    		Err(_) => panic!("Found no token!"),
    	};

    	let mut db = match Database::new() {
    		Ok(d) => d,
    		Err(err) => panic!(err)
    	};

    	let discord = Discord::from_bot_token(&key).expect("login failed");
        let (mut connection, _) = discord.connect().expect("connect failed");

        Bot {
            client: discord,
            conn: connection,
            database: db,
            handlers: HashMap::new()
        }
    }

    pub fn run(mut self) {
        println!("Running...");
        loop {
    		match self.conn.recv_event() {
    			Ok(Event::MessageCreate(message)) => {
    				self.incoming_message(&message)
    				.and_then(|s| self.client.send_message(&message.channel_id, &s, "", false).ok());
    			},
    			Ok(_) => {},
    			Err(discord::Error::Closed(code, body)) => {
    				println!("Gateway closed on us with code {:?}: {}", code, String::from_utf8_lossy(&body));
    				break;
    			},
    			Err(err) => println!("Receive error: {:?}", err)
    		}
    	}

        self.client.logout().expect("logout failed");
    }
}
