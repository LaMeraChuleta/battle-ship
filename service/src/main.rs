use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "10.0.0.7:5000";
const MSG_SIZE: usize = 32;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}
fn main() {
    

    let server = TcpListener::bind(LOCAL).expect("no se pudo conectar a la Ip");

    server.set_nonblocking(true).expect("no se pudo realizar el set_non");

    let mut clients = vec![];
    let (sender, receiver) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Cliente: {} se conecto", addr);

            let sender = sender.clone();
            clients.push(socket.try_clone().expect("fallo clonando el cliente"));

            thread::spawn(move || loop {

                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Caracter UTF8 invalido");
                        println!("{}: {:?}", addr, msg);
                        sender.send(msg).expect("fallo al enviar");
                    }, 
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("El cliente: {} se desconecto", addr);
                        break;
                    }
                }

                sleep();
            });
        }

        if let Ok(msg) = receiver.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {

                println!("{:?}", msg);
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);

                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
            
        }

        sleep();
    }
}
