use core::fmt;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct IP {
    addr: [u8; 4],
    port: u16,
}

impl IP {
    pub fn new(addr: [u8; 4], port: u16) -> Self {
        IP { addr, port }
    }
}

impl fmt::Display for IP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}:{}",
            self.addr[0], self.addr[1], self.addr[2], self.addr[3], self.port
        )
    }
}

pub struct Connection {
    friends: Vec<IP>,
    size: usize,
}

impl Connection {
    pub fn new() -> Self {
        Connection {
            friends: vec![],
            size: 0,
        }
    }

    pub fn add_friend(&mut self, friend: IP) {
        self.friends.push(friend);
        self.size += 1;

        // some function call for validation
    }

    fn __handle_stream(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_req: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        println!("{:?}", http_req);
    }

    pub fn serve(self, addr: IP) {
        let listener = TcpListener::bind(format!("{}", addr)).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap(); // need error handling
            Connection::__handle_stream(&self, stream);

            println!("Connected.");
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test1() {
        let myip: IP = IP::new([120, 12, 34, 14], 80);
        print!("{}", myip);
    }
}
