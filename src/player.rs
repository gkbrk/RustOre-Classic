#[deriving(Clone)]
pub struct Player{
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub yaw: u8,
    pub pitch: u8,
    pub stream: TcpStream
}