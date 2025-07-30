use std::fs;
use std::io::{BufRead, BufReader, Write};

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
fn main() {
    println!("Welcome to Walpapr-Rust!");
}
