use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::{thread, vec};

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn sleep(){
    thread::sleep(::std::time::Duration::from_millis(100));
}


fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener faield to bind"); // bind the server and tcpListener to that local str, if it faisl to bidn retur nmsg
    server.set_nonblocking(true).expect("Faield to initialize non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop{
        if let Ok((mut socket, addr)) = server.accept() {
            println!("clients {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failedto clone clients"));

            thread::spawn(move || loop{
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff){
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("invalit utf8 message");

                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("failed to send msg to rx");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connectino with: {}", addr);
                        break;
                    }
                }
                sleep();
            });
        }
        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();

                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }
        sleep();
    }
}
