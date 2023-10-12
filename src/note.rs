use std::fs::File;
use std::io::Write;

pub struct GCLNote {
    name: String,
    filename: String,
    note: String
}

impl GCLNote {
    pub fn new(name: String, note: String) -> GCLNote {
        let filename = name.clone() + ".gnote";
        GCLNote {
            name,
            filename,
            note,
        }
    }

    pub fn write_to_file(&self) {
        let mut file = File::create(&self.filename).unwrap();
        file.write_all(&self.note.as_bytes()).expect("Could not write to file!");
        println!("Data has been written to {}!", &self.filename);
    }
}