#![feature(struct_variant)]

extern crate flate2;
extern crate curl;

extern crate sync;

use std::io::{Listener, Acceptor};
use std::io::net::tcp::{TcpListener, TcpStream};

use std::rand::{task_rng, Rng};

use sync::{Mutex, Arc};

use config::Configuration;
use packets::{Packet, MCPackets};
use heartbeat::Heartbeat;
use authentication_verifier::is_authenticated;
use world::World;

mod mc_string;
mod packets;
mod config;
mod heartbeat;
mod authentication_verifier;
mod world;

fn handle_connection(config: Configuration, mut conn: TcpStream, mutex_world: Arc<Mutex<World>>){
    let ip = match conn.peer_name(){
        Ok(x) => x.ip,
        Err(x) => {return;}
    };
    println!("{} is connecting to us...", ip);
    loop{
        let packet = Packet::receive(conn.clone());
        //println!("{}", packet.packet_id);
        
        if packet.packet_id == 0x00{
            let parsed = packet.parse_player_ident();
            if config.online_mode & !is_authenticated(config.clone().salt, parsed.clone().username, parsed.clone().verification_key){
                println!("Player tried to join without auth!");
                conn.close_read();
                return;
            }
            println!("{}", parsed.username);
            
            conn.send_server_ident(config.clone());
            
            
            //Send debug level data
            let mut level = mutex_world.lock();
            level.send_world(conn.clone());
            
            //conn.send_spawn_player(5*32, 15*32, 5*32, 5, 5);
            conn.send_pos(5*32, 25*32, 5*32, 5, 5);
        }else if packet.packet_id == 0x08{
            //println!("Player moved");
        }else if packet.packet_id == 0x05{
            let parsed = packet.parse_set_block();
            let mut level = mutex_world.lock();
            if parsed.destroyed{
                level.set_block(parsed.x as uint, parsed.y as uint, parsed.z as uint, 0x00);
            }else{
                level.set_block(parsed.x as uint, parsed.y as uint, parsed.z as uint, parsed.block_id);
            }
        }else if packet.packet_id == 0x0d{
            let parsed = packet.parse_message();
            println!("{}", parsed.message);
        }
    }
}

fn main(){
    let config = Configuration{
        address: "0.0.0.0".to_string(),
        port: 25565,
        max_players: 20,
        server_name: "RustOreBeta".to_string(),
        server_motd: "A Minecraft classic server written in Rust!".to_string(),
        is_public: "True".to_string(),
        online_mode: false,
        salt: task_rng().gen_ascii_chars().take(16).collect(),
        heartbeat_interval: 45
    };
    
    let mut mc_world = World::new(10, 10, 10);
    for i in range(0u, 10){
        for i1 in range(0u, 10){
            for i2 in range(0u, 10){
                mc_world.set_block(i, i1, i2, 0x01);
            }
        }
    }
    let mutex_world = Arc::new(Mutex::new(mc_world));
    
    let heartbeat_sender = Heartbeat::new(config.clone());
    heartbeat_sender.spawn_task();
    
    let mut acceptor = TcpListener::bind(config.address.as_slice(), config.port).listen().unwrap();
    println!("RustOre is listening on {}:{}", config.address, config.port);
    for connection in acceptor.incoming(){
        let config_clone = config.clone();
        let mutex_world_clone = mutex_world.clone();
        spawn(proc() {
            handle_connection(config_clone, connection.unwrap(), mutex_world_clone);
        });
    }
}
