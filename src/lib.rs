use text_io::read;
use std::error::Error;
use std::net::TcpStream;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use std::time::Duration;

mod net;
use net::{Message,spawn_tcp_thread};

#[derive(PartialEq)]
pub struct Config {
    host: String,
    port: u16,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments.\nUsage: spork [destination] [port]");
        }

        let host = args[1].clone();
        let port = match args[2].parse::<u16>() {
            Ok(num) => num,
            Err(_err) => return Err("Port is not a valid 16-bit number."),
        };

        Ok(Config { host, port })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let addr = format!("{}:{}", config.host, config.port);
    let mut stream = TcpStream::connect(&addr)?;

    // Channel for Client to Server
    let (tx_c2s, rx_c2s): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    // Channel for Server to Client
    let (tx_s2c, rx_s2c): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    let tcp_thread = spawn_tcp_thread(&mut stream, tx_s2c, rx_c2s);

    loop {
        let line: String = read!("{}\n");
        match line.as_str() {
            "exit" => break,
            "quit" => break,
            _ => {
                let msg = Message::Data(line);
                tx_c2s.send(msg).unwrap();
                println!("[+] Sent to thread");
            },
        };
    }

    tcp_thread.join().unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_config() {
        let host = "localhost";
        let port = "1234";
        let test_args = vec!["./spork".to_string(), host.to_string(), port.to_string()];
        let config = Config::new(&test_args).unwrap();
        assert_eq!(config.host, host);
        assert_eq!(config.port, 1234);
    }
}
