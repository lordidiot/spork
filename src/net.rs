use std::error::Error;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::{Receiver, Sender};

use std::time::Duration;

pub enum Message {
    Success,
    Error(Box<dyn Error>),
    Shutdown,
    Data(String),
}

pub fn spawn_tcp_thread(
    stream: &mut TcpStream,
    tx: Sender<Message>,
    rx: Receiver<Message>,
) -> thread::JoinHandle<()> {

    //stream.set_nonblocking(true).unwrap();

    let child = thread::spawn(move || {
        loop {
            //stream.read()
            if let Ok(msg) = rx.try_recv() {
                match msg {
                    Message::Shutdown => {
                        return ();
                    },
                    Message::Data(s) => {
                        println!("Echo: {}", s);
                    },
                    _ => (),
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
    child
}
