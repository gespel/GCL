mod note;

use std::{env, io};
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use crate::note::GCLNote;

struct CommandParser;

impl CommandParser {
    fn parse_commands(&self, g: &GCL, input: Vec<String>) {
        let basecmd: &String = &input[0];
        if basecmd == "note" {
            g.create_note(String::from(&input[1]), String::from(&input[2]));
            //GCLNote::new(String::from(&input[1]), String::from(&input[2])).write_to_file();
        }
        else if basecmd == "console" {
            loop {
                print!("gcl> ");
                io::stdout().flush().expect("Fehler beim Schreiben auf stdout");
                let mut input= String::new();
                io::stdin().read_line(&mut input).expect("Error on stdin");
                let input_split = input.split_whitespace().map(|s| s.to_string()).collect();
                self.parse_commands(g, input_split);
            }
        }
        else if basecmd == "serve" {
            let listener = TcpListener::bind("127.0.0.1:1234").unwrap();
            for stream in listener.incoming() {
                let s = stream.unwrap();
                g.handle_connection(s);
            }
        }
        else if basecmd == "exit" {
            println!("Thank you for using GCL and have a good one!");
            exit(0);
        }
        else {
            println!("Unknown command!");
        }
    }
}


struct GCL;

impl GCL {
    fn create_note(&self, name: String, note: String) {
        GCLNote::new(name, note).write_to_file();
    }
    fn handle_connection(&self, mut stream: TcpStream) {
        let r = "\
        HTTP/1.1 200 OK\r\n\
        Content-Type: application/json\r\n\r\n\
        {\"name\": \"Sten\"}
        ";
        stream.write_all(r.as_bytes());
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.len() > 0 {
        let g = GCL {};
        let cp = CommandParser{};
        cp.parse_commands(&g, args);
    }
    else {
        println!("Not enough arguments!");
    }
}
