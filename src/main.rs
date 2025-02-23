use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <log_file> [keyword]", args[0]);
        return;
    }

    let log_file = &args[1];
    let keyword = if args.len() > 2 { Some(&args[2]) } else { None };

    if let Err(e) = read_logs(log_file, keyword) {
        eprintln!("Error reading log file: {}", e);
    }
}

fn read_logs(filename: &str, keyword: Option<&String>) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if let Some(key) = keyword {
            if line.contains(key) {
                println!("{}", line);
            }
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}

