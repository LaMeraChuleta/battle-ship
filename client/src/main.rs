mod game;
use game::GameBattleShip;


fn main() {
    
    match GameBattleShip::new() {
        Some(mut game) => game.init_game(),
        None => ()
    }
    // let mut client = TcpStream::connect(LOCAL).expect("Tcpstream no creado");
    // client.set_nonblocking(true).expect("No se ejecuto set_nonblocking");
    // let (sender, reciver) = mpsc::channel::<String>();
    // thread::spawn(move || loop {
    //     let mut buff = vec![0; MSG_SIZE];
    //     //Estara leyendo reciviendo los mensajes del service para confimar que el mensaje llego
    //     match client.read_exact(&mut buff) {
    //         Ok(_) => {
    //             let msg = buff
    //             .into_iter()
    //             .take_while(|&x| x != 0)
    //             .collect::<Vec<u8>>();
    //             println!("Mensaje Recibido en el Servidor {:?}",msg)
    //         },
    //         Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
    //         Err(_) => {
    //             println!("El servidor se a desconectado");
    //             break
    //         }
    //     }
    //     //Revisara los mensajes que estamos enviando
    //     match reciver.try_recv() {
    //         Ok(msg) => {
    //             let mut buff = msg
    //                 .clone()
    //                 .into_bytes();
    //             buff.resize(MSG_SIZE, 0);
    //             client
    //                 .write_all(&buff)
    //                 .expect("escritura fallida en el socket");
    //             println!("Mensaje Enviado {:?}", msg);
    //         }
    //         Err(TryRecvError::Empty) => (),
    //         Err(TryRecvError::Disconnected) => break,
    //     }
    //     thread::sleep(Duration::from_millis(100));
    // });    
    // let mut screenPlayer = Screen::new();
    // loop {
    //     let mut screenView: u8 = 0;
    //     match screenView { 
    //         0 => {      
    //             screenPlayer.screen_new_game()    
    //         },
    //         1 => {
    //             let mut buff = String::new();
    //             io::stdin()
    //                 .read_line(&mut buff)
    //                 .expect("fallo al leer el mensaje");
    //             let msg = buff.trim().to_string();
    //         }
    //         _ => println!("Mal")
    //     }  
    // }    
}