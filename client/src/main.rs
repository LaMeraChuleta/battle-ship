use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "10.0.0.7:5000";
const MSG_SIZE: usize = 32;

fn main() {

    let mut client = TcpStream::connect(LOCAL).expect("TCPstream no creado");
    client
        .set_nonblocking(true)
        .expect("No se ejecuto set_nonblocking");

    let (sender, reciver) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];

        //Estara leyendo reciviendo los mensajes del service para confimar que el mensaje llego
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                
                println!(
                    "Mensaje Recibido en el Servidor {:?}",
                    msg
                );
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }

        //Rebisara los mensajes que estamos enviando
        match reciver.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("escritura fallida en el socket");
                println!("Mensaje Enviado {:?}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Escriba el mensaje:");
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("fallo al leer el mensaje");
        let msg = buff.trim().to_string();
        if msg == ":quit" || sender.send(msg).is_err() {
            break;
        }
    }
    println!("bye bye!");
}
