use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use log::{error, info, warn};

pub struct GWL {
    fn_map: HashMap<String, fn(TcpStream)>,
}

impl GWL {
    pub fn new(address: String) -> GWL {
        let fn_map: HashMap<String, fn(TcpStream)> = HashMap::new();
        return GWL {
            fn_map
        }
    }

    pub fn route(&mut self, dir: String, handler: fn(TcpStream)) {
        self.fn_map.insert(dir, handler);
    }

    pub fn serve(&self) {
        env_logger::init();
        let listener = TcpListener::bind("127.0.0.1:1234").unwrap();
        for stream in listener.incoming() {
            let mut s = stream.unwrap();
            let r: &mut String = &mut "".to_string();
            if let Some(handler) = self.fn_map.get("/") {
                info!("incoming connection: {}", s.peer_addr().unwrap());
                handler(s);
            }
        }
    }
}

