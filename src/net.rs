use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use bstr::ByteSlice;

pub enum Message {
    //Success,
    Shutdown,
    //Error,
    Data(Vec<u8>),
}

pub fn spawn_tcp_thread(
    mut stream: TcpStream,
    _tx: Sender<Message>,
    rx: Receiver<Message>,
) -> thread::JoinHandle<()> {

    let child = thread::spawn(move || {
        let mut data: [u8; 1000] = [0; 1000];
        let mut running = true;
        while running {
            // data from server
            if let Ok(data_len) = stream.read(&mut data) {
                print!("{}", &data[..data_len].to_str_lossy());
            }
            // messages from client
            if let Ok(msg) = rx.try_recv() {
                match msg {
                    Message::Shutdown => {
                        running = false;
                    },
                    Message::Data(s) => {
                        stream.write(&s).unwrap();
                        stream.flush().unwrap();
                    },
                    _ => (),
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
    child
}
