use std::{io::{ErrorKind, Read, Write}, net::TcpStream};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "192.168.100.7:3000";
const MSG_SIZE: usize = 32;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}
fn main() {
    let server = TcpListener::bind(LOCAL).expect("No se conecto a la Ip");
    server.set_nonblocking(true).expect("No se ejecuto set_nonblocking");
    let mut clients = vec![];
    let (sender, receiver) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Cliente: {} Conectado", addr);
            let sender = sender.clone();
            clients.push(socket.try_clone().expect("Fallo clonando el cliente"));
            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg =  String::from_utf8(buff.into_iter()
                            .take_while(|&x| x != 0)
                            .collect::<Vec<u8>>())
                            .expect("Caracter UTF( invalido");                    
                        println!("{} dice: {:?}", addr, msg);
                        //Envia el mensaje
                        sender.send(msg).expect("Fallo al enviar");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Cliente: {} Desconectado", addr);
                        break;
                    }
                }
                sleep();
            });
        }
        if let Ok(msg) = receiver.try_recv() {
            println!("{}", msg);
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MSG_SIZE, 0);
                    client.write_all(&buff).map(|_| client).ok()
                })
                .collect::<Vec<TcpStream>>();
        }
        sleep();
    }
}
