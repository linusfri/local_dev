use local_dev::cli::Cli;
use std::io;

fn main() -> io::Result<()> {
    while let Ok(val) = Cli::read_line() {
        println!("{}", val);
    }

    Ok(())
}
    
