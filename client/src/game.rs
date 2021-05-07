use client::Player;
use client::Screen;
use client::ScreenType;
use std::{io::{Read, ErrorKind, Write}, net::{TcpStream}, sync::mpsc::{self, TryRecvError}, vec};
use std::time::Duration;
use std::thread;
use std::option::Option;
use serde::{Serialize, Deserialize};

const LOCAL: &str = "192.168.100.7:3000";
pub struct GameBattleShip {
    player_ship: Player,
    screen_game: Screen,
    type_view: ScreenType,
    conection: TcpStream   
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Entity {
    x: String,
    y: i32,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ServiceQuery {
    PlayersConnect
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PacketGameMessage {
    type_packet: ServiceQuery,
    attack_coordinate: Option<Vec<u8>>        
}

impl GameBattleShip {
    pub fn new() -> Option<Self>{
        let tcp_player = TcpStream::connect(LOCAL)
            .expect("no se creo el tcp");      
        tcp_player.set_nonblocking(true).unwrap();    
        let player = Self{            
                player_ship: Player::default(),
                screen_game: Screen::default(),
                type_view: ScreenType::ScreenInitGame,
                conection: tcp_player
        }; 
        Some(player)   
    }
    pub fn init_game(&mut self) {               
        let (sender, reciver) = mpsc::channel::<ServiceQuery>();   
        let (sender_packet, reciver_packet) = mpsc::channel::<PacketGameMessage>();     
        let mut copy_connection = self.conection.try_clone().unwrap();
        copy_connection.set_nonblocking(true).expect("No se ejecuto set_nonblocking");
        thread::spawn(move || loop {               
            match reciver.try_recv() {                
                Ok(query_servie) => {                                      
                    match query_servie {
                        ServiceQuery::PlayersConnect => {
                            let packet = PacketGameMessage { type_packet: ServiceQuery::PlayersConnect, attack_coordinate: None};
                            let mut struct_byte = bincode::serialize(&packet).unwrap();                                                                                                                                                               
                            struct_byte.resize(32, 0);                            
                            copy_connection
                                .write_all(&struct_byte)
                                .expect("escritura fallida en el socket");                                                                   
                        }                                                                                                         
                    }                          
                }
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => break,
            }
            thread::sleep(Duration::from_millis(100));            
        });
        let mut copy_connection_other = self.conection.try_clone().unwrap();
        copy_connection_other.set_nonblocking(true).expect("No se ejecuto set_nonblocking");

        thread::spawn(move || loop {
            let mut buff = vec![0,32];     
            match copy_connection_other.read(&mut buff) {
                Ok(_) => {                                                                  
                    buff.resize(32, 0);                               
                    let package_message: PacketGameMessage = bincode::deserialize(&buff[..]).unwrap();                    
                    sender_packet.send(package_message).unwrap();      
                    thread::sleep(Duration::from_secs(10));         
                },
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                  break
                }
            };
            thread::sleep(Duration::from_millis(100));  
        });

        loop {
            match self.type_view {
                ScreenType::ScreenInitGame => {
                    self.type_view = self.screen_game.screen_new_game()
                },                                                                            
                ScreenType::ScreenSearchPlayer => {                      
                    sender.send(ServiceQuery::PlayersConnect).unwrap();                   
                    self.type_view = ScreenType::ScreenWait;                                                                                                      
                },
                ScreenType::ScreenWait => {
                    loop {                        
                        match reciver_packet.try_recv(){
                            Ok(packet) => {                                
                                println!("{:?}", &packet);
                                self.type_view = ScreenType::ScreenInitGame;                                
                                break
                            },
                            Err(TryRecvError::Empty) => (),    
                            Err(TryRecvError::Disconnected) => break,                                                        
                        }
                    }
                }
                ScreenType::ScreenOut => break
            }            
        }        
    }
}