extern crate curl;

use curl::http;
use config::Configuration;
use std::io::timer;
use std::time::Duration;
use std::thread::Thread;

#[deriving(Clone)]
pub struct Heartbeat{
    config: Configuration
}

impl Heartbeat{
    pub fn new(config: Configuration) -> Heartbeat{
        return Heartbeat{
            config: config
        };
    }
    
    pub fn send_heartbeat(&self){
        let response = http::handle().get(format!("https://minecraft.net/heartbeat.jsp?port={}&max={}&name={}&public={}&version=7&salt={}&users=0", self.config.port, self.config.max_players, self.config.server_name.as_slice(), self.config.is_public.as_slice(), self.config.salt.as_slice())).exec().unwrap();
    }
    
    pub fn loop_blocking(&self){
        loop{
            self.send_heartbeat();
            println!("Sent heartbeat!");
            timer::sleep(Duration::seconds(self.config.heartbeat_interval));
        }
    }
    
    pub fn spawn_task(&self){
        let clone = self.clone();
        Thread::spawn(move || {
            clone.loop_blocking();
        }).detach();
    }
}
