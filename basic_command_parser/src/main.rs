use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
enum Command {
    U,
    D,
    P(u8),
    N(u8),
    E(u8),
    S(u8),
    W(u8),
}

impl Command {
    fn from_string(s: &str) -> Option<Command> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts.get(0)? {
            &"U" => Some(Command::U),
            &"D" => Some(Command::D),
            &"P" => Some(Command::P(
                parts.get(1).expect("Invalid command argument").as_bytes()[0],
            )),
            &"N" => Some(Command::N(
                parts.get(1).expect("Invalid command argument").as_bytes()[0],
            )),
            &"E" => Some(Command::E(
                parts.get(1).expect("Invalid command argument").as_bytes()[0],
            )),
            &"S" => Some(Command::S(
                parts.get(1).expect("Invalid command argument").as_bytes()[0],
            )),
            &"W" => Some(Command::W(
                parts.get(1).expect("Invalid command argument").as_bytes()[0],
            )),
            _ => None,
        }
    }
}

fn main() {
    let lines = read_lines("./commands.txt".to_string());
    for line in lines {
        let line = line.unwrap();
        let command = Command::from_string(&line);
        execute_command(command);
    }
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).expect("Failed to open file");
    io::BufReader::new(file).lines()
}

fn execute_command(command: Option<Command>) -> () {
    match command {
        Some(Command::U) => pen_up(),
        Some(Command::D) => pen_down(),
        Some(Command::P(pixels_size)) => select_pen(pixels_size),
        Some(Command::N(pixels)) => go_north(pixels),
        Some(Command::S(pixels)) => go_south(pixels),
        Some(Command::E(pixels)) => go_east(pixels),
        Some(Command::W(pixels)) => go_west(pixels),
        _ => (),
    }
}

fn pen_up() -> () {
    println!("Pen lifted")
}
fn pen_down() -> () {
    println!("Pen ready to write")
}
fn select_pen(pixels_size: u8) -> () {
    println!("Selected pen size {} pixels", pixels_size)
}
fn go_north(pixels: u8) -> () {
    println!("Going north {} pixels", pixels)
}
fn go_south(pixels: u8) -> () {
    println!("Going south {} pixels", pixels)
}
fn go_east(pixels: u8) -> () {
    println!("Going east {} pixels", pixels)
}
fn go_west(pixels: u8) -> () {
    println!("Going west {} pixels", pixels)
}
