use std::{io::{ErrorKind, Read, Write}};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use serde::{Serialize, Deserialize};
use bincode::deserialize;
use serde_json;

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
                const MESSAGE_SIZE: usize = 500;
                let mut buff: Vec<u8> = vec![];
                let mut rx_bytes = [0u8; MESSAGE_SIZE];
                match socket.read(&mut rx_bytes) {                    
                    Ok(bytes_size) => {  

                        buff.extend_from_slice(&rx_bytes[..bytes_size]);                                             
                        println!("{:?}",&buff);
                        let strin_json = String::from_utf8(buff).unwrap();  
                        print!("{:?}", strin_json); 
                        let mut entity_descer_byte: PacketGameMessage = serde_json::from_str(&strin_json).unwrap();
                        let vec_dir = vec![2, 6, 7];
                        entity_descer_byte.attack_coordinate = Some(vec_dir);
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
            let mut struct_byte = serde_json::to_string(&packet).unwrap();  
            println!("{:?}",struct_byte);  
            //struct_byte.resize(32, 0);                                             
            clients[0].write_all(&struct_byte.as_bytes())
                .expect("escritura fallida en el socket");
            clients[0].flush().unwrap()
        }
        thread::sleep(::std::time::Duration::from_millis(100));
    }
}
