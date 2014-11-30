extern crate flate2;

use flate2::writer::GzEncoder;
use std::io::MemWriter;

use std::io::net::tcp::TcpStream;

use packets::MCPackets;

#[deriving(Clone)]
pub struct World{
    pub x_size: uint,
    pub y_size: uint,
    pub z_size: uint,
    pub blocks: Vec<u8>    
}

impl World{
    pub fn new(x_size: uint, y_size: uint, z_size: uint) -> World{
        let mut block_vec: Vec<u8> = Vec::new();
        for i in range(0u, x_size * y_size * z_size){
            block_vec.push(0x00);
        }
        return World{
            x_size: x_size,
            y_size: y_size,
            z_size: z_size,
            blocks: block_vec
        };
    }
    
    pub fn calculate_block_from_coord(&mut self, x: uint, y: uint, z: uint) -> uint{
        return (y * self.z_size + z) * self.x_size + x;
    }
    
    pub fn set_block(&mut self, x: uint, y: uint, z: uint, block_id: u8){
        let block = self.calculate_block_from_coord(x, y, z);
        *self.blocks.get_mut(block).unwrap() = block_id;
    }
    
    pub fn get_block(&mut self, x: uint, y: uint, z: uint) -> u8{
        let block = self.calculate_block_from_coord(x, y, z);
        return *self.blocks.get_mut(block).unwrap();
    }
    
    pub fn gzip_world(&mut self) -> Vec<u8>{
        let mut gzipper = GzEncoder::new(MemWriter::new(), flate2::Default);
        gzipper.write_be_i32((self.x_size * self.y_size * self.z_size) as i32);
        for block in self.blocks.iter(){
            gzipper.write_u8(*block);
        }
        return gzipper.finish().unwrap().unwrap();
    }
    
    pub fn send_world(&mut self, mut conn: TcpStream){
        conn.send_level_init();
        
        let gb = self.gzip_world();
        let bytes = gb.as_slice();
        let total_bytes = bytes.len();
        let mut cur_byte: uint = 0;
        loop{
            if total_bytes - cur_byte > 1024{
                let bytes_vec = bytes.to_vec();
                let partial_bytes = bytes_vec.slice(cur_byte, cur_byte + 1024);
                conn.send_chunk_data(1024, partial_bytes, ((cur_byte / total_bytes * 100) as u8));
                cur_byte += 1024;
            }else if total_bytes - cur_byte > 0{
                let bytes_vec = bytes.to_vec();
                let partial_bytes = bytes_vec.slice(cur_byte, total_bytes);
                conn.send_chunk_data((total_bytes - cur_byte) as i16, partial_bytes, ((cur_byte / total_bytes * 100) as u8));
                cur_byte += total_bytes - cur_byte;
            }else{
                break;
            }
        }
        
        conn.send_level_finalize(self.x_size as i16, self.y_size as i16, self.z_size as i16);
    }
}
