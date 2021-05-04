
use std::net::TcpStream;
use std::option::Option;
pub struct Player<'a> {
    pub player_name: &'a str,
    tcp_player: TcpStream,
}
const LOCAL: &str = "192.168.100.7:3000";
impl<'a> Player<'a> {     
    pub fn new(player_name: &'a str) -> Option<Self> {
        let tcp_player = match TcpStream::connect(LOCAL){
            Ok(tcp) => {
                if tcp.set_nonblocking(true).is_ok() {
                    Some(tcp)                 
                } else {
                    None                    
                }                  
            },
            _ => None
        };       
        match tcp_player {
            Some(tcp) => Some(Self { player_name: player_name, tcp_player: tcp}),
            _ => None
        }                            
    }
}

