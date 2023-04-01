use std::str::FromStr;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
enum Command {
    P,
    U,
    D,
    N,
    E,
    S,
    W,
}

fn main() {
    let lines = read_lines("./commands.txt".to_string());
    for line in lines {
        let line = line.unwrap();
        let command = get_command(&line);

        match Command::from_str(command).unwrap() {
            _ => (),
        }
    }
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn get_command(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    "P"
}
