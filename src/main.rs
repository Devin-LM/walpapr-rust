use dirs;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write, Result, Read};
use std::path::PathBuf;
use std::path::Path;

fn get_walpapr_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("walpapr-rust");
        path
    })
}
fn get_hyprland_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("hypr");
        path
    })
}
fn file_writer(file_path: &PathBuf, content: String) {
    let mut file = fs::File::create(file_path).expect("Couldn't create file");
    file.write(content.as_bytes())
        .expect("Data couldn't be written to {file}");
}
fn prepend_file<P: AsRef<Path> + ?Sized>(data: &[u8], path: &P) -> Result<()> {
    let mut f = File::open(path)?;
    let mut content = data.to_owned();
    f.read_to_end(&mut content)?;

    let mut f = File::create(path)?;
    f.write_all(content.as_slice())?;

    Ok(())
}

fn file_reader(file_path: String) {
    let input = fs::File::open(file_path).expect("Couldn't open file at: {file_path}");
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.expect("Error with BufReader object"));
    }
}

fn switch_profile() {
    let active_profile_dir = get_walpapr_path().expect("Unable to find wallpaper-rust in .config/");
    println!("What profile would you like to switch to: ");
    //list directories inside of get_walpapr_path()
    let path = get_walpapr_path() //TODO: CHECK FOR EMPTY AND THROW TO NEW PROFILE
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
    prepend_file("source = ~/.config/walpapr/active/colors.conf\n".as_bytes(), "/home/dawn/.config/hypr/hyprland.conf").expect("Error prepending to hyprland.conf file");
    let mut temp;
    for dir in paths {
        if dir.as_ref().unwrap().file_name().display().to_string() == input.trim() {
            let contents = fs::read_dir(dir.unwrap().path()).unwrap();
            for file in contents {
                //println!("{:?}", file);
                match file.as_ref().unwrap().file_name().to_str().expect("File doesn't exist") {
                    "wallpaper" => {
                        temp = active_profile_dir.to_owned();
                        temp.push("wallpaper");
                        fs::copy(file.unwrap().path(), temp)
                            .expect("Unable to copy wallpaper file to active profile directory");
                    }
                    "hyprpaper.conf" => {
                        temp = get_hyprland_path().expect("Unable to find hypr in .config/");
                        temp.push("hyprpaper.conf");
                        fs::copy(file.unwrap().path(),
                            temp).expect("Unable to copy hyprland.conf to hypr/");
                    }
                    "colors.conf" => {
                        temp = active_profile_dir.to_owned();
                        temp.push("colors.conf");
                        fs::copy(file.unwrap().path(), temp).expect("Unable to copy colors.conf to active profile directory");
                    }
                    _ => {println!("external file found in profile")}
                }
            }
            // COPY ALL FILES TO .config/walpapr-rust/active/
            // SINGLE CHANGE TO hyprpaper.conf CHECKED AT STARTUP OF SCRIPT
            // NEW hyprland.conf LINE : source = ~/.config/walpapr/active/colors.conf
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
    profile_dir.push(&profile_name);

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

    let mut colors_file = profile_dir.to_owned();
    colors_file.push("colors.conf"); // DOUBLE CHECK THIS IS CORRECT

    file_writer(&colors_file, colors_conf_data);

    println!("Input desired wallpaper directory");
    let mut wallpaper_old_dir = String::new();
    io::stdin().read_line(&mut wallpaper_old_dir).expect("Unable to read line");
    wallpaper_old_dir = wallpaper_old_dir.trim().to_string();
    //TODO: CHECK IF FILE ACTUALLY EXISTS
    let mut new_wallpaper_dir = profile_dir.to_owned();
    new_wallpaper_dir.push("wallpaper");
    fs::copy(wallpaper_old_dir, &new_wallpaper_dir).expect("Wallpaper could not be copied");

    println!("Generating hyprpaper.conf");
    let mut hyprpaper_conf_data = String::new();

    hyprpaper_conf_data.push_str("preload = ");
    hyprpaper_conf_data.push_str(&new_wallpaper_dir.to_str().expect("Could not convert new_wallpaper_dir to str"));
    hyprpaper_conf_data.push_str("\nwallpaper =, ");
    hyprpaper_conf_data.push_str(&new_wallpaper_dir.to_str().expect("Could not convert new_wallpaper_dir to str"));

    let mut hyprpaper_file = profile_dir.to_owned();
    hyprpaper_file.push("hyprpaper.conf");
    file_writer(&hyprpaper_file, hyprpaper_conf_data);
    println!("New profile \'{}\' added!", &profile_name);
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

    let mut hyprland = get_hyprland_path().expect("Couldn't get hyprland path");
    hyprland.push("hyprland.conf");
    match input.trim() {
        "switch" => switch_profile(),
        "new" => create_profile(),
        _ => println!("Invalid input"),
    }
}
