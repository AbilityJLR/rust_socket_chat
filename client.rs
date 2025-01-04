use std::net::{TcpStream, Shutdown};
use std::io::{self, Write, Read};
use std::thread;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

fn handle_incoming(mut stream: TcpStream, running: Arc<AtomicBool>) {
    let mut buf = [0; 512];
    loop {
        match stream.read(&mut buf) {
            Ok(size) if size > 0 => {
                let message = String::from_utf8_lossy(&buf[..size]);
                println!("Receive {}", message);

                if message.trim() == "bye" {
                    running.store(false, Ordering::Relaxed); 
                    break;
                }
            }
            Ok(_) => break,
            Err(e) => {
                println!("Failed to read from server: {}", e);
                break;
            }
        }
    }
    println!("Shutting down...");
    stream.shutdown(Shutdown::Both).expect("Shutdown call failed...");
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Couldn't connect to server!!!");
    println!("Connected to the server!");

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    let stream_clone = stream.try_clone().expect("Clone failed!");
    thread::spawn(move || {
        handle_incoming(stream_clone, running_clone);
    });

    while running.load(Ordering::Relaxed) {
        let mut message = String::new();
        io::stdin().read_line(&mut message).unwrap();

        if message.trim() == "bye" {
            running.store(false, Ordering::Relaxed); 
            break;
        }

        if let Err(e) = stream.write_all(message.trim().as_bytes()) {
            println!("Failed to send message: {}", e);
            break;
        }
    }

    println!("Exiting program...");
}
