use std::rand::{task_rng, Rng};

#[deriving(Clone)]
pub struct Configuration{
    pub address: String,
    pub port: u16,
    pub max_players: uint,
    pub server_name: String,
    pub server_motd: String,
    pub is_public: String,
    pub online_mode: bool,
    pub salt: String,
    pub heartbeat_interval: i64
}

impl Configuration{
    pub fn get_default_config() -> Configuration{
        return Configuration{
            address: "0.0.0.0".to_string(),
            port: 25565,
            max_players: 20,
            server_name: "RustOreBeta".to_string(),
            server_motd: "A Minecraft classic server written in Rust".to_string(),
            is_public: "True".to_string(),
            online_mode: false,
            salt: task_rng().gen_ascii_chars().take(16).collect(),
            heartbeat_interval: 45
        };
    }
}