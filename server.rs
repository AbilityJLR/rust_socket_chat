use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::thread;
use std::sync::{Arc, Mutex};

fn handle_connection(mut stream: TcpStream, connections: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buf = [0;512];
    loop {
        match stream.read(&mut buf) {
            Ok(size) if size > 0 => {
                let message = String::from_utf8_lossy(&buf[..size]);
                println!("Receive {}", message);

                if message == "bye" {
                    stream.write_all(message.trim().as_bytes());
                    break;
                }
                let connections = connections.lock().unwrap();
                for mut conn in connections.iter() {
                    if let Err(e) = conn.write_all(message.as_bytes()) {
                        println!("Failed to send message to client: {}", e);
                    }
                }
            }
            Ok(_) => break,
            Err(e) => {
                println!("Failed to read from client: {}", e);
                break;
            }

        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Couldn't receive connection!!!");
    println!("Waiting for connection...");

    let connections = Arc::new(Mutex::new(Vec::new()));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let connections = Arc::clone(&connections);
                thread::spawn(move || {
                    connections.lock().unwrap().push(stream.try_clone().unwrap());
                    println!("New connection!");
                    handle_connection(stream, connections);
                });
            }
            Err(e) => {
                println!("Connection failed!");
                break
            }
        };
    }
}
