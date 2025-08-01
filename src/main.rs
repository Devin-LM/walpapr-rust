use dirs;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

fn get_walpapr_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("walpapr-rust");
        path
    })
}
fn file_writer(file_path: &PathBuf, content: String) {
    let mut file = fs::File::create(file_path).expect("Couldn't create file");
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

fn switch_profile() {
    println!("What profile would you like to switch to: ");
    //list directories inside of get_walpapr_path()
    let path = get_walpapr_path() //CHECK FOR EMPTY AND THROW TO NEW PROFILE
        .expect("walpapr .config dir not found")
        .display()
        .to_string();
    let mut paths = fs::read_dir(&path).unwrap();
    print!("{{ ");
    for dir in paths {
        print!("{} ", dir.unwrap().file_name().display());
    }
    println!("}}");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input");
    paths = fs::read_dir(&path).unwrap();
    for dir in paths {
        if dir.unwrap().file_name().display().to_string() == input.trim() {
            println!("MATCH");
        }
    }
}

fn create_profile() {
    println!("Input name of new profile");
    let mut profile_name = String::new();
    io::stdin().read_line(&mut profile_name).expect("Unable to read input");
    profile_name = profile_name.trim().to_string();
    //TODO: CHECK THIS NAME DOESN'T EXIST

    let mut profile_dir = get_walpapr_path().unwrap();
    profile_dir.push(profile_name);

    fs::create_dir(&profile_dir).expect("Error creating new profile");

    println!("Create colors.conf");

    println!("\nActiveOne: ");
    let mut active_one = String::new();
    io::stdin().read_line(&mut active_one).expect("Unable to read line");
    active_one = active_one.trim().to_string();

    println!("ActiveTwo: ");
    let mut active_two = String::new();
    io::stdin().read_line(&mut active_two).expect("Unable to read line");
    active_two = active_two.trim().to_string();

    println!("Inactive: ");
    let mut inactive = String::new();
    io::stdin().read_line(&mut inactive).expect("Unable to read line");
    inactive = inactive.trim().to_string();

    let mut colors_conf_data =  String::new();

    colors_conf_data.push_str("$ACTIVEONE = rgb(");
    colors_conf_data.push_str(&active_one);
    colors_conf_data.push_str(")\n$ACTIVETWO = rgb(");
    colors_conf_data.push_str(&active_two);
    colors_conf_data.push_str(")\n$INACTIVE = rgb(");
    colors_conf_data.push_str(&inactive);
    colors_conf_data.push_str(")");

    let mut colors_file = profile_dir;
    colors_file.push("colors.conf");

    file_writer(&colors_file, colors_conf_data);

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
        "switch" => switch_profile(),
        "new" => create_profile(),
        _ => println!("Invalid input"),
    }
}
