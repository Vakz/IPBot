extern crate regex;

use database::Database;
use self::regex::Regex;
use database::DBFile;

pub trait DiscordMessageHandler {
    fn handle_message(&self, message: &str, user: &str, db: &mut Database) -> String;
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
    fn handle_message(&self, message: &str, user: &str, db: &mut Database) -> String {
        println!("FileHandler: Line was  \"{}\"", message);
        lazy_static! {
            static ref FIND: Regex = Regex::new(r"^!file(?:\s(?P<name>[:word:]+))?(?:\s(?P<dest>[:word:]+))?$").unwrap();
        }

        if let Some(matches) = FIND.captures(&message) {
            let m = matches.iter().filter(|&x| x.is_some()).collect::<Vec<_>>();

            match m.len() {
                1 => FileHandler::by_username(user, db),

                2 => FileHandler::by_filename(user, matches.at(1).unwrap(), db),
                3 => FileHandler::insert(user, matches.at(1).unwrap(), matches.at(2).unwrap(), db),
                _ => "Incorrect usage. Should be: \"!file [filename [destination]]\"".to_string()
            }
        } else {
            "Should never get here".to_string()
        }
    }
}

impl FileHandler {
    fn file_to_string(file: &DBFile) -> String {
        println!("Called");
        format!("Filename: {} | Destination: {} | Uploaded By: {} | At: {:?}",
            file.name, file.dest, file.user, file.time)
    }

    fn by_username(user: &str, db: &mut Database) -> String {
        if let Some(res) = db.by_username(user.to_string()) {
            res.iter()
            .map(FileHandler::file_to_string)
            .collect::<Vec<_>>()
            .join("\n")
        } else {
            format!("No results found for user {}", user)
        }
    }

    fn by_filename(user: &str, filename: &str, db: &mut Database) -> String {
        db.get_exact(filename.to_string(), user.to_string())
        .as_ref()
        .map_or("You have uploded no file by that name".to_string(), FileHandler::file_to_string)
    }

    fn insert(user: &str, filename: &str, dest: &str, db: &mut Database) -> String {
        let file = DBFile { name: filename.to_string(), dest: dest.to_string(), user: user.to_string(), ..Default::default() };
        match db.insert(file) {
            Ok(_) => "File inserted successfully".to_string(),
            Err(err) => err.to_string()
        }
    }
}
