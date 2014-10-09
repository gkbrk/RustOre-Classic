extern crate flate2;
extern crate curl;

use curl::http;

use std::io::{Listener, Acceptor, IoResult, IoError, InvalidInput};
use std::io::net::tcp::{TcpListener, TcpStream};

use std::io::timer;
use std::time::Duration;

use std::rand::task_rng;

use config::Configuration;
use mc_string::MCString;
use packets::MCPackets;
use heartbeat::Heartbeat;
use world::World;

mod mc_string;
mod packets;
mod config;
mod heartbeat;
mod world;

struct Packet{
	packet_id: u8,
	packet_len: uint,
	data: Vec<u8>
}

fn handle_connection(config: Configuration, mut conn: TcpStream) -> IoResult<()>{
	let ip = try!(conn.peer_name()).ip;
	println!("{} is connecting to us...", ip);
	loop{
		let packet = parse_packet(config.clone(), conn.clone());
		//println!("{}", packet.packet_id);
		
		if packet.packet_id == 0x00{
			conn.send_server_ident(config.clone());
			
			//Send debug level data
			let mut level = World::new(10, 10, 10);
            for i in range(0u, 10){
                for i1 in range(0u, 10){
                    for i2 in range(0u, 10){
                        level.set_block(i, i1, i2, 0x01);
                    }
                }
            }
            level.set_block(0, 0, 0, 0x01);
            level.send_world(conn.clone());
			
			//conn.send_spawn_player(5*32, 15*32, 5*32, 5, 5);
			conn.send_pos(5*32, 25*32, 5*32, 5, 5);
		}else if packet.packet_id == 0x08{
            //println!("Player moved");
        }
	}
	Ok(())
}

fn parse_packet(config: Configuration, mut conn: TcpStream) -> Packet{
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
	}
}

fn main(){
    let config = Configuration{
        address: "0.0.0.0".to_string(),
        port: 25565,
        max_players: 20,
        server_name: "RustServerBetaDontJoin".to_string(),
        server_motd: "A Minecraft classic server written in Rust!".to_string(),
        is_public: "True".to_string(),
        salt: task_rng().gen_ascii_str(16).to_string(),
        heartbeat_interval: 45
    };
    
    let heartbeat_sender = Heartbeat::new(config.clone());
    heartbeat_sender.spawn_task();
    
    let mut acceptor = TcpListener::bind(config.address.as_slice(), config.port).listen().unwrap();
    println!("Rustymine is listening on {}:{}", config.address, config.port);
    for connection in acceptor.incoming(){
		let config_clone = config.clone();
		spawn(proc() {
			handle_connection(config_clone, connection.unwrap());
		});
	}
}
