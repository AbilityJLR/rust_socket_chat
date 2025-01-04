use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::thread;

fn handle_connection(mut stream: TcpStream) {
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
            }
            Ok(_) => break,
            Err(e) => {
                println!("Failed to read from client: {}", e);
                break;
            }

        }
    }
    println!("Shutting down...");
    stream.shutdown(Shutdown::Both).expect("Shutdown call failed...");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Couldn't receive connection!!!");
    println!("Waiting for connection...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    println!("New connection!");
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Connection failed!");
                break
            }
        };
    }
}
