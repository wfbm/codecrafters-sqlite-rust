use anyhow::{bail, Result};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let mut file = File::open(&args[1])?;
            let mut header = [0; 100];
            file.read_exact(&mut header)?;

            let page_size = u16::from_be_bytes([header[16], header[17]]);
            let mut first_page = vec![0; page_size as usize];
            file.read_exact(&mut first_page)?;

            let table_count = u16::from_be_bytes([first_page[3], first_page[4]]);

            println!("database page size: {}", page_size);
            println!("number of tables: {}", table_count);
        }
        ".table" => {
            let mut file = File::open(&args[1])?;
            let mut header = [0; 100];
            file.read_exact(&mut header)?;

            let page_size = u16::from_be_bytes([header[16], header[17]]);

            let mut first_page = vec![0; page_size as usize];
            file.read_exact(&mut first_page)?;

            let pointer = u16::from_be_bytes([first_page[5], first_page[6]]);
            let pointer = pointer - 100;

            let content = &first_page[pointer as usize..page_size as usize];
            let commands = String::from_utf8_lossy(&content);

            let re = regex::Regex::new(r"CREATE TABLE (?P<table_name>[a-zA-Z_-]+)").unwrap();

            for table_name in re.captures_iter(&commands) {
                println!("{:?}", &table_name["table_name"]);
            }
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
