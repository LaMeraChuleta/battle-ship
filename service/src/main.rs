use std::{io::{ErrorKind, Read, Write}};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use serde::{Serialize, Deserialize};
use bincode::deserialize;

const LOCAL: &str = "192.168.100.7:3000";
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ServiceQuery {
    PlayersConnect
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PacketGameMessage {
    type_packet: ServiceQuery,
    attack_coordinate: Option<Vec<u8>>        
}
fn main() {
    let server = TcpListener::bind(LOCAL).expect("No se conecto a la Ip");
    server.set_nonblocking(true).expect("No se ejecuto set_nonblocking");
    let mut clients = vec![];
    let (sender, receiver) = mpsc::channel::<PacketGameMessage>();    
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Cliente: {} Conectado", addr);
            let sender = sender.clone();
            clients.push(socket.try_clone().expect("Fallo clonando el cliente"));
            thread::spawn(move || loop {
                let mut buff = vec![0; 32];
                match socket.read_exact(&mut buff) {                    
                    Ok(_) => {                                                
                        let entity_descer_byte: PacketGameMessage = deserialize(&buff[..]).unwrap();                                                
                        println!("{} dice: {:?}", addr, entity_descer_byte);                                           
                        sender.send(entity_descer_byte).unwrap();
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Cliente: {} Desconectado", addr);
                        break;
                    }
                }
                thread::sleep(::std::time::Duration::from_millis(100));
            });
        }
        if let Ok(packet) = receiver.try_recv() {                        
            let mut struct_byte = bincode::serialize(&packet).unwrap();    
            struct_byte.resize(32, 0);                                             
            clients[0].write_all(&struct_byte)
                .expect("escritura fallida en el socket");
        }
        thread::sleep(::std::time::Duration::from_millis(100));
    }
}
