use dirs;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

fn get_walpapr_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("walpapr");
        path
    })
}
fn file_writer(file_name: String, content: String) {
    let mut file = fs::File::create(file_name).expect("Couldn't create file");
    file.write(content.as_bytes())
        .expect("Data couldn't be written to {file}");
}

fn file_reader(file_path: String) {
    let input = fs::File::open(file_path).expect("Couldn't open file at: {file_path}");
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.expect("Error with BufReader object"));
    }
}

fn tests() {
    let file = String::from("/home/dawn/testFile.txt");
    let data = String::from("testing data");
    file_writer(file, data);
    file_reader(String::from("/home/dawn/testFile.txt"));
}

fn switch_profile(input: String) {
    println!("We be switching it up.");
}
fn create_profile(input: String) {
    println!("We be creating in here.");
}
fn init() {
    if let Some(config_path) = get_walpapr_path() {
        //check if dir exists
        if !config_path.exists() {
            //walpapr config dir does not exist
            fs::create_dir(config_path).expect("Error creating Walpapr .config directory.");
        }
    } else {
        eprintln!("Could not determine configuration directory.");
    }
}

fn main() {
    init();
    println!("Welcome to Walpapr-Rust!");
    println!("Switch profiles or Create new profile? (switch/new)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input.");

    match input.trim() {
        "switch" => switch_profile(input),
        "new" => create_profile(input),
        _ => println!("Invalid input"),
    }
}
