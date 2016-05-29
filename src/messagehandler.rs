pub trait DiscordMessageHandler {
    fn handle_message(&self, message: &str);
}

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
    fn handle_message(&self, message: &str) {
        println!("Message was: {}", message);
    }
}
