use serde::{Deserialize, Serialize};

///
/// {x} maniaco, comenta to' cristo pa mañana loko
/// brrrrrrrrrrrrrrrrrrrrrr
/// niga niga niga niga
/// 

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    ACCESS,
    ORDER,
    LOG,
    SHUTDOWN,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    request_type: RequestType,
    addr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBodyStruct {
    is_metaorder: bool, // if is true, the order will be run for the process(explicar mejor porfa)
    order_command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Body {
    OderBody(OrderBodyStruct),
    Key(String),
    LogInfo(String),
    ShutDownConf(bool),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    metadata: Metadata,
    body: Body,
}

impl Request {
    pub fn new(metadata: Metadata, body: Body) -> Request {
        Request { metadata, body }
    }
}

impl Metadata {
    pub fn new(request_type: RequestType, addr: String) -> Metadata {
        Metadata { request_type, addr }
    }
}

impl OrderBodyStruct {
    pub fn create(is_metaorder: bool, order_command: String) -> OrderBodyStruct {
        OrderBodyStruct {
            is_metaorder,
            order_command,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let request = Request::new(
            Metadata::new(RequestType::ORDER, "10.2.1.1".to_string()),
            Body::OderBody(OrderBodyStruct::create(true, "niga".to_string())),
        );

        let request_str = serde_json::to_string(&request).unwrap();
        println!("{request_str}");

        let request_order: Request = serde_json::from_str(&request_str).unwrap();
        
        println!("{:?}", request_order);
    }
}
