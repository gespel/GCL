mod note;

use std::{env, io};
use std::io::Write;
use crate::note::GCLNote;

struct CommandParser;

impl CommandParser {
    fn parse_commands(&self, g: &GCL, input: Vec<String>) {
        let basecmd: &String = &input[0];
        if basecmd == "note" {
            GCLNote::new(String::from(&input[1]), String::from(&input[2])).write_to_file();
        }
        else if basecmd == "console" {
            loop {
                print!("gcl> ");
                io::stdout().flush().expect("Fehler beim Schreiben auf stdout");
                let mut input= String::new();
                io::stdin().read_line(&mut input).expect("Error on stdin");
                let mut input_split = input.split_whitespace().map(|s| s.to_string()).collect();
                self.parse_commands(g, input_split);
            }
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
