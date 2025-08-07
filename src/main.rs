// TODO: FIX HYPRPAPER.conf GENERATION, SWAP TO NEVER COPY ONLY OVERWRITE wallpaper IN
// walpapr-rust/active
use dirs;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write, Result, Read};
use std::path::PathBuf;
use std::path::Path;
use std::process::{Command, Stdio};

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
fn compare_and_replace(file_path: &PathBuf, replace: &str) -> bool { //Returns true if file was
    //prepended, false if file was untouched
    let file = fs::File::open(&file_path).expect("Couldn't open file at: {file_path}");
    let buffered = BufReader::new(file);
    let mut flag: bool = false;

    for line in buffered.lines() {
        if &line.expect("Error with curr line: {line}").as_str() == &replace.trim() {
            println!("CHECK");
            flag = true;
        }
    }
    if !flag {
        prepend_file(replace.as_bytes(), &file_path.as_path()).expect("Error prepending file");
        return true;
    }
    return false;
}
fn replace_word_in_file(path: &PathBuf, from: &str, to: &str) {
    let mut file = File::open(&path).expect("Unable to open file.");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Couldn't read from file.");
    drop(file);

    let new_data = data.replace(from, to);

    let mut dst = File::create(&path).expect("File couldn't be created.");

    dst.write(new_data.as_bytes()).expect("Couldn't write to new file.");
}

fn switch_profile() {
    let mut active_profile_dir = get_walpapr_path().expect("Unable to find wallpaper-rust in .config/");
    active_profile_dir.push("active");
    if !active_profile_dir.exists() {
        fs::create_dir(&active_profile_dir).expect("Couldn't create active profile directory");
    }
    println!("What profile would you like to switch to: ");
    let path = get_walpapr_path() //TODO: CHECK FOR EMPTY AND THROW TO NEW PROFILE
        .expect("walpapr .config dir not found")
        .display()
        .to_string();
    let mut paths = fs::read_dir(&path).unwrap();
    print!("{{ ");
    for dir in paths {
        if !(dir.as_ref().unwrap().file_name() == "active") {
            print!("{} ", dir.unwrap().file_name().display());
        }
    }
    println!("}}");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input");
    paths = fs::read_dir(&path).unwrap();
    let mut temp;
    for dir in paths {
        if dir.as_ref().unwrap().file_name().display().to_string() == input.trim() {
            let contents = fs::read_dir(dir.unwrap().path()).unwrap();
            for file in contents {
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
                        Command::new("killall").arg("hyprpaper").output().unwrap(); // kill active
                        // hyprpaper sessions
                        Command::new("hyprpaper").stdout(Stdio::null()).spawn().unwrap(); // spawn
                        // hyprpaper in the background and pipe stdout to /dev/null
                    }
                    "colors.conf" => { //TODO: Add support for switching between RGB and RGBA
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
    let mut hyprland_conf_path = get_hyprland_path().to_owned().expect("Couldn't get hypr path in config dir");
    hyprland_conf_path.push("hyprland.conf");
    let prepended = compare_and_replace(&hyprland_conf_path, "source = ~/.config/walpapr-rust/active/colors.conf\n"); //setup flag
    if prepended {
        replace_word_in_file(&hyprland_conf_path, "col.active_border = ", "col.active_border = $ACTIVEONE $ACTIVETWO 45deg # ");
        replace_word_in_file(&hyprland_conf_path, "col.inactive_border = ", "col.inactive_border = $INACTIVE # ");
    }
    // col.inactive_border in hyprland.conf
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
    //Make sure everything is available/created ahead of time
    if let Some(config_path) = get_walpapr_path() {
        //check if dir exists
        if !config_path.exists() {
            //walpapr config dir does not exist
            fs::create_dir(config_path).expect("Error creating Walpapr .config directory.");

            let mut active_profile_dir = get_walpapr_path().expect("Could not find walpapr config directory");
            active_profile_dir.push("active");
            if !active_profile_dir.exists() {
                fs::create_dir(active_profile_dir).expect("Could not create active profile directory");
            }
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
