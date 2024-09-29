use std::net::SocketAddr;

use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // ip: the server ip
    #[arg(short, long)]
    ip: String,

    // port: the port where own server will be running
    #[arg(short, long)]
    port: u16,
}

fn data_parser(data: String) -> Vec<String> {
    let mut request: Vec<String> = Vec::new();

    for key in data.split("|") {
        request.push(key.to_string());
    }

    request
}

async fn get_data(socket: &mut TcpStream) -> Vec<String> {
    let mut buffer = [0u8; 1024];
    let n = socket.read(&mut buffer).await.unwrap();
    let request = String::from_utf8_lossy(&buffer[..n]);

    let data = data_parser(request.to_string());

    data
}

fn is_controller_client(data: &Vec<String>) -> bool {
    match data.get(0) {
        Some(client_type) if *client_type == "Controller".to_string() => {
            println!("I am the controller");
            true
        }
        _ => {
            eprintln!("error");
            false
        }
    }
}

#[tokio::main]
async fn main() {
    // parsing the agruments
    let args = Args::parse();

    // parse the direcction addrs
    let addr = format!("{}:{}", args.ip, args.port);

    // init the server
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => {
            println!("the server is running...");
            listener
        }
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };

    let mut connections: Vec<(TcpStream, SocketAddr)> = Vec::new();
    loop {
        let (mut socket, addr) = match listener.accept().await {
            Ok((socket, addr)) => {
                println!("[{addr}] ==> connected");
                (socket, addr)
            }
            Err(e) => {
                eprintln!("{:?}", e);
                return;
            }
        };

        let data = get_data(&mut socket).await;


        if is_controller_client(&data) {
            println!("{:?}", data);
            println!("{:?}", connections);

            let mut response_container = String::new();

            for (stream, addr) in connections.iter_mut() {
                if let Some(command) = data.get(1) {
                    stream.write_all(command.as_bytes()).await.unwrap();
                }

                let mut buffer = [0u8;1024];
                let n = match stream.read(&mut buffer).await {
                    Ok(n) if n == 0 => continue,
                    Ok(n) => {
                        n
                    }
                    Err(e) => {
                        eprintln!("{:?}",e);
                        continue;
                    },
                };
                let data = String::from_utf8_lossy(&buffer[..n]);
                let response = format!("\n[{:?}]\n{data}", addr);
                response_container.push_str(&response);
            }

            socket.write_all(response_container.as_bytes()).await.unwrap();
        } else {
            connections.push((socket, addr));
        }
    }
}

#[test]
fn parse_request() {
    let a = data_parser("Controller|elpepe|fulanito".to_string());

    assert_eq!(
        a,
        vec![
            "Controller".to_string(),
            "elpepe".to_string(),
            "fulanito".to_string()
        ]
    )
}
