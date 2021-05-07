use std::{io::{self, Read, Write}, net::TcpStream, sync::mpsc::{self, TryRecvError}, thread, time::Duration};
const LOCAL: &str = "192.168.100.7:3000";
fn main(){

    let mut client = TcpStream::connect(LOCAL).unwrap();
    client
    .set_nonblocking(true)
    .expect("No se ejecuto set_nonblocking");
    let (sender, reciver) = mpsc::channel::<String>();
    thread::spawn(move || loop{
        let mut buff = vec![0; 32];  
        match client.read_exact(&mut buff){
            Ok(_) => println!("{:?}", buff),
            _ => ()
        }

        match reciver.try_recv() {
            Ok(msg) => {
                let mut buff = msg
                    .clone()
                    .into_bytes();
                buff.resize(32, 0);
                client
                    .write_all(&buff)
                    .expect("escritura fallida en el socket");
                println!("Mensaje Enviado {:?}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }
        thread::sleep(Duration::from_millis(100))
    });

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
     


}