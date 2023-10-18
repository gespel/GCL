use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

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
        let listener = TcpListener::bind("127.0.0.1:1234").unwrap();
        for stream in listener.incoming() {
            let mut s = stream.unwrap();
            let r: &mut String = &mut "".to_string();
            if let Some(handler) = self.fn_map.get("/") {
                handler(s);
            }
            else {
                s.write("Routing not set!".to_string().as_bytes()).expect("Error writing to socket!");
            }
        }
    }
}

