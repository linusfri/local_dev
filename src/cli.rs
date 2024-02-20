use std::io;

pub enum Steps {
    Init,
    FileGen
} 

pub struct Cli {
    step: Steps
}

impl Cli {
    pub fn read_line() -> Result<String, io::Error> {
        let stdin = io::stdin();
        let mut buffer = String::new();

        match stdin.read_line(&mut buffer) {
            Ok(_) => Ok(buffer.clone()),
            Err(error) => Err(error) 
        }
    }
}