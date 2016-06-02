extern crate regex;

use database::Database;
use self::regex::Regex;
use std::fmt;

pub trait DiscordMessageHandler {
    fn handle_message(&self, message: &str, user: &str, db: &mut Database) -> Option<String>;
}
/*
pub struct ScheduleHandler {
    address: &'static str
}

impl ScheduleHandler {
     pub fn new() -> ScheduleHandler {
         ScheduleHandler {
             address: "http://something.com"
         }
     }
}

impl DiscordMessageHandler for ScheduleHandler {
    fn handle_message(&self, message: &str) -> String {
        println!("Message was: {}", message);
    }
}
*/
pub struct FileHandler {

}

impl DiscordMessageHandler for FileHandler {
    fn handle_message(&self, message: &str, user: &str, db: &mut Database) -> Option<String> {
        println!("FileHandler: Line was  \"{}\"", message);
        lazy_static! {
            static ref FIND: Regex = Regex::new("!file ([:word:]+)").unwrap();
        }

        let name = match FIND.captures(&message) {
            Some(name) => name.at(1).unwrap(),
            None => return None
        };

        println!("Searched for word \"{}\"", name);

        if let Some(file) = db.get_exact(name.to_string(), user.to_string()) {
            return Some(format!("Filename: {} | Destination: {} | Uploaded By: {} | At: {:?}",
                file.name, file.dest, file.user, file.time
            ));
        }

        None
    }
}
