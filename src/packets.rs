use std::io::MemReader;

use std::io::net::tcp::TcpStream;

use config::Configuration;

use mc_string::MCString;


pub struct Packet{
    pub packet_id: u8,
    pub packet_len: uint,
    pub data: Vec<u8>
}

impl Packet{
    pub fn receive(mut conn: TcpStream) -> Packet{
        let packet_id = conn.read_byte().unwrap();
        
        let packet_len = match packet_id{
            0x00 => 130,
            0x05 => 8,
            0x08 => 9,
            0x0d => 65,
            _ => 0
        };
        
        let data = conn.read_exact(packet_len).unwrap();
        
        return Packet{
            packet_id: packet_id,
            packet_len: packet_len,
            data: data
        };
    }
    
    pub fn parse_player_ident(&self) -> PlayerIdent{
        let mut reader = MemReader::new(self.data.clone());
        return PlayerIdent{
            version: reader.read_u8().unwrap(),
            username: reader.read_mc_string(),
            verification_key: reader.read_mc_string(),
            unused: reader.read_u8().unwrap()
        };
    }
    
    pub fn parse_set_block(&self) -> SetBlock{
        let mut reader = MemReader::new(self.data.clone());
        return SetBlock{
            x: reader.read_be_i16().unwrap(),
            y: reader.read_be_i16().unwrap(),
            z: reader.read_be_i16().unwrap(),
            destroyed: match reader.read_u8().unwrap(){
                0x00 => true,
                0x01 => false,
                _ => false
            },
            block_id: reader.read_u8().unwrap()
        };
    }
    
    pub fn parse_position_and_orientation(&self) -> PositionAndOrientation{
        let mut reader = MemReader::new(self.data.clone());
        return PositionAndOrientation{
            player_id: reader.read_u8().unwrap(),
            x: reader.read_be_i16().unwrap(),
            y: reader.read_be_i16().unwrap(),
            z: reader.read_be_i16().unwrap(),
            yaw: reader.read_u8().unwrap(),
            pitch: reader.read_u8().unwrap()
        };
    }
    
    pub fn parse_message(&self) -> Message{
        let mut reader = MemReader::new(self.data.clone());
        return Message{
            unused: reader.read_u8().unwrap(),
            message: reader.read_mc_string()
        };
    }
}

#[deriving(Clone)]
struct PlayerIdent{
    pub version: u8,
    pub username: String,
    pub verification_key: String,
    unused: u8
}

#[deriving(Clone)]
struct SetBlock{
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub destroyed: bool,
    pub block_id: u8
}

#[deriving(Clone)]
struct PositionAndOrientation{
    pub player_id: u8,
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub yaw: u8,
    pub pitch: u8
}

#[deriving(Clone)]
struct Message{
    unused: u8,
    pub message: String
}



pub trait MCPackets{
    fn send_server_ident(&mut self, config: Configuration);
    fn send_ping(&mut self);
    fn send_level_init(&mut self);
    fn send_chunk_data(&mut self, length: i16, data: &[u8], percentage: u8);
    fn send_level_finalize(&mut self, x_size: i16, y_size: i16, z_size: i16);
    fn send_spawn_player(&mut self, x: i16, y: i16, z: i16, yaw: u8, pitch: u8);
    fn send_pos(&mut self, x: i16, y: i16, z: i16, yaw: u8, pitch: u8);
    fn send_chat_message(&mut self, player_id: i8, message: String);
}

impl MCPackets for TcpStream{
    fn send_server_ident(&mut self, config: Configuration){
        self.write_u8(0x00);
        self.write_u8(0x07);
        self.write_mc_string(config.server_name);
        self.write_mc_string(config.server_motd);
        self.write_u8(0x00);
    }
    
    fn send_ping(&mut self){
        self.write_u8(0x01);
    }
    
    fn send_level_init(&mut self){
        self.write_u8(0x02);
    }
    
    fn send_chunk_data(&mut self, length: i16, data: &[u8], percentage: u8){
        self.write_u8(0x03);
        self.write_be_i16(length);
        
        self.write(data);
        
        for i in range(0, 1024 - length){
            self.write_u8(0x00);
        }
        self.write_u8(percentage);
    }
    
    fn send_level_finalize(&mut self, x_size: i16, y_size: i16, z_size: i16){
        self.write_u8(0x04);
        self.write_be_i16(x_size);
        self.write_be_i16(y_size);
        self.write_be_i16(z_size);
    }
    
    fn send_spawn_player(&mut self, x: i16, y: i16, z: i16, yaw: u8, pitch: u8){
        self.write_u8(0x07);
        self.write_i8(-1);
        self.write_mc_string("gokberkdoga".to_string());
        self.write_be_i16(x);
        self.write_be_i16(y);
        self.write_be_i16(z);
        self.write_u8(yaw);
        self.write_u8(pitch);
    }
    
    fn send_pos(&mut self, x: i16, y: i16, z: i16, yaw: u8, pitch: u8){
        self.write_u8(0x08);
        self.write_i8(-1);
        self.write_be_i16(x);
        self.write_be_i16(y);
        self.write_be_i16(z);
        self.write_u8(yaw);
        self.write_u8(pitch);
    }
    
    fn send_chat_message(&mut self, player_id: i8, message: String){
        self.write_u8(0x0d);
        self.write_i8(player_id);
        self.write_mc_string(message);
    }
}
