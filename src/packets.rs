extern crate flate2;

use flate2::writer::GzEncoder;
use std::io::MemWriter;

use std::io::net::tcp::TcpStream;
use std::io::{IoResult, IoError};

use std::io::stdout;

use config::Configuration;

use mc_string::MCString;

pub trait MCPackets{
	fn send_server_ident(&mut self, config: Configuration);
	fn send_ping(&mut self);
	fn send_level_init(&mut self);
	fn send_chunk_data(&mut self, length: i16, data: &[u8], percentage: u8);
	fn send_level_finalize(&mut self, x_size: i16, y_size: i16, z_size: i16);
	fn send_spawn_player(&mut self, x: i16, y: i16, z: i16, yaw: u8, pitch: u8);
	fn send_pos(&mut self, x: i16, y: i16, z: i16, yaw: u8, pitch: u8);
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
}
