pub trait DiscordMessageHandler {
    fn handle_message(&self, message: &str) -> String;
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
    fn handle_message(&self, message: &str) -> String {
        println!("Message was: {}", message);
        "FileHandler was called!".to_string()
    }
}
