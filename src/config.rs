#[deriving(Clone)]
pub struct Configuration{
    pub address: String,
    pub port: u16,
    pub max_players: uint,
    pub server_name: String,
    pub server_motd: String,
    pub is_public: String,
    pub salt: String,
    pub heartbeat_interval: i64
}
