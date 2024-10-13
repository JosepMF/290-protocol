use request::Request;
use tokio::{self, io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use serde_json;

///
/// manito se que no quieres documentar ahora pero mañana es lo que te toca hacer
/// 

pub async fn send(stream: &mut TcpStream, data: Request) {
    let data_json = match serde_json::to_string(&data)  {
        Ok(json) => json,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        },
    };

    match stream.write_all(&data_json.as_bytes()).await {
        Ok(_) =>  {
            return;
        },
        Err(e) => {
            eprintln!("[Error] ===> {:?}", e);
        },
    }
}

// hacer mejor, manejar posibles errores etc
pub async fn recive(stream: &mut TcpStream) -> Request {
    let mut buffer = [0u8; 1024];

    let n: usize = stream.read(&mut buffer).await.unwrap();

    let request_str = String::from_utf8_lossy(&buffer[..n]);

    let request_data: Request = serde_json::from_str(&request_str).unwrap();

    request_data
}

#[cfg(test)]
mod tests {
    use super::*;
}
