extern crate time;
extern crate curl;
extern crate flate2;

use curl::http;

use std::io::{Listener, Acceptor, IoResult, IoError, InvalidInput};
use std::io::net::tcp::{TcpListener, TcpStream};

use std::io::timer;
use std::time::Duration;

use std::io::MemReader;

use config::Configuration;
use mc_string::MCString;
use packets::MCPackets;
use heartbeat::Heartbeat;

mod mc_string;
mod packets;
mod config;
mod heartbeat;

struct Packet{
	packet_id: u8,
	packet_len: uint,
	data: Vec<u8>
}

fn send_heartbeat(config: Configuration) -> IoResult<()>{
	let start_time: f64 = time::precise_time_s();
    let response = http::handle().get(format!("https://minecraft.net/heartbeat.jsp?port={:u}&max={:u}&name={:s}&public={:s}&version=7&salt={:s}&users=0", config.port, config.max_players, config.server_name.as_slice(), config.is_public.as_slice(), config.salt.as_slice())).exec().unwrap();
    println!("Heartbeat done! Took {} seconds.", time::precise_time_s() - start_time);
    Ok(())
}

fn handle_connection(config: Configuration, mut conn: TcpStream) -> IoResult<()>{
	let ip = try!(conn.peer_name()).ip;
	println!("{} is connecting to us...", ip);
	loop{
		let packet = parse_packet(config.clone(), conn.clone());
        let mut packet_data = MemReader::new(packet.data);
		//println!("{}", packet.packet_id);
		
		if packet.packet_id == 0x00{
			conn.send_server_ident(config.clone());
			
			//Send debug level data
			conn.send_level_init();
			let mut data: Vec<u8> = Vec::new();
			for i in range(0u, 1000u){
				data.push(0x01);
			}
            for i in range(0u, 500u){
				data.push(0x03);
			}
            for i in range(0u, 100u){
				data.push(0x02);
			}
			conn.send_chunk_data(data);
			conn.send_level_finalize(10, 16, 10);
			
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
        salt: "DEMOSALT12341".to_string(),
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
