use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Serialize, Deserialize)]
pub struct Request {
    addr: (SocketAddr, u16),
    client_type: String,
    body: String,
}

impl Request {
    pub fn new(addr: (SocketAddr, u16), client_type: String, body: String) -> Self {
        Request { addr , client_type, body }
    }

    pub async fn send(socket: &mut TcpStream, request: &Request) {
        match socket
            .write_all(serde_json::to_string(request).unwrap().as_bytes())
            .await
        {
            Ok(()) => {
                println!("[INFO] ==> the data was sended");
            }
            Err(e) => {
                eprintln!("[ERROR] ==> the data wasn't sended \n{}", e);
            }
        }
    }

    pub async fn recive(socket: &mut TcpStream) -> Request {
        let mut buffer = [0u8; 1024];

        let n = match socket.read(&mut buffer).await {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{}", e);
                0
            }
        };

        let request_string = String::from_utf8_lossy(&buffer[..n]);

        serde_json::from_str(&request_string).unwrap()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
