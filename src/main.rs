use dirs;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write, Result, Read};
use std::path::{PathBuf, Path};
use std::process::{Command, Stdio};
use walkdir::WalkDir;

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

fn file_writer(file_path: &PathBuf, content: String) -> Result<()> {
    let mut file = fs::File::create(file_path)?;
    file.write(content.as_bytes())?;
    Ok(())
}

fn prepend_file<P: AsRef<Path> + ?Sized>(data: &[u8], path: &P) -> Result<()> {
    let mut f = File::open(path)?;
    let mut content = data.to_owned();
    f.read_to_end(&mut content)?;

    let mut f = File::create(path)?;
    f.write_all(content.as_slice())?;

    Ok(())
}

//Returns true if file was prepended, false if file was untouched
fn compare_and_replace(file_path: &PathBuf, replace: &str) -> bool {
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

    let path = get_walpapr_path()
        .expect("walpapr .config dir not found")
        .display()
        .to_string();

    if WalkDir::new(&path).into_iter().count() <= 4 {
        println!("No profiles found, create a new one!");
        create_profile().expect("Couldn't create new profile");
    }

    println!("What profile would you like to switch to: ");

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
                        // TODO: LOOK INTO AWAIT FUNCTIONALITY
                        Command::new("killall").arg("hyprpaper").output().unwrap(); // kill active
                        // hyprpaper sessions
                        Command::new("hyprpaper").stdout(Stdio::null()).spawn().unwrap(); // spawn
                        // hyprpaper in the background and pipe stdout to /dev/null
                    }
                    "colors.conf" => {
                        temp = active_profile_dir.to_owned();
                        temp.push("colors.conf");
                        fs::copy(file.unwrap().path(), temp).expect("Unable to copy colors.conf to active profile directory");
                    }
                    _ => {println!("external file found in profile")}
                }
            }
        }
    }

    let mut hyprland_conf_path = get_hyprland_path().to_owned().expect("Couldn't get hypr path in config dir");
    hyprland_conf_path.push("hyprland.conf");
    let prepended = compare_and_replace(&hyprland_conf_path, "source = ~/.config/walpapr-rust/active/colors.conf\n");
    if prepended {
        replace_word_in_file(&hyprland_conf_path, "col.active_border = ", "col.active_border = $ACTIVEONE $ACTIVETWO 45deg # ");
        replace_word_in_file(&hyprland_conf_path, "col.inactive_border = ", "col.inactive_border = $INACTIVE # ");
    }
}

fn generate_profile(profile_dir: PathBuf, colors_data: String, wallpaper_dir: &Path) {
    // This is where we will handle file creation instead of streamlining it

    //Generate colors.conf from provided values
    let mut colors_file = profile_dir.to_owned();
    colors_file.push("colors.conf");

    //Make copy of desired wallpaper picture
    let mut new_wallpaper_dir = profile_dir.to_owned();
    new_wallpaper_dir.push("wallpaper");


    let wallpaper_str = &new_wallpaper_dir.to_str().expect("Couldn't convert new_wallpaper_dir to str");

    //Generate hyprpaper.conf file
    let mut hyprpaper_data = String::new();
    hyprpaper_data.push_str("preload = ");
    hyprpaper_data.push_str(wallpaper_str);

    //TODO: Add support for unique backgrounds per monitor
    hyprpaper_data.push_str("\nwallpaper =, ");
    hyprpaper_data.push_str(wallpaper_str);

    //Touch hyprpaper.conf file
    let mut hyprpaper_file = profile_dir.to_owned();
    hyprpaper_file.push("hyprpaper.conf");

    let files = || -> Result<()> {
        fs::create_dir(&profile_dir)?;
        file_writer(&colors_file, colors_data)?;
        fs::copy(wallpaper_dir, &new_wallpaper_dir)?;
        file_writer(&hyprpaper_file, hyprpaper_data)?;
        Ok(())
    };

    //Profile generated
    if let Err(_err) = files() {
        println!("Profile could not be generated! {:?}", _err);
    }
}

// TODO: Dont create profile if something goes wrong with creation
fn create_profile() -> Result<()> {
    println!("Input name of new profile");
    let mut profile_name = String::new();
    io::stdin().read_line(&mut profile_name).expect("Unable to read input");
    profile_name = profile_name.trim().to_string();

    let mut profile_dir = get_walpapr_path().unwrap();
    profile_dir.push(&profile_name);
    let _prof_check = match profile_dir.exists() {
        true => {panic!("This profile already exists!")},
        false => {},
    };

    println!("Create colors.conf");
    let mut rgba = false;
    println!("Include alpha? (yes/y/no/N):");
    let mut rgb_switch = String::new();
    io::stdin().read_line(&mut rgb_switch).expect("Couldn't read line");
    match rgb_switch.to_lowercase().as_str().trim() {
        "yes" => {rgba = true},
        "y" => {rgba = true},
        "no" => {rgba = false},
        "n" => {rgba = false},
        _ => {println!("Not expected");}
    }

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

    let colors_data;
    if !rgba {
        colors_data = format!("$ACTIVEONE = rgb({})\n$ACTIVETWO = rgb({})\nINACTIVE = rgb({})", &active_one, &active_two, &inactive);
    } else {
        colors_data = format!("$ACTIVEONE = rgba({})\n$ACTIVETWO = rgba({})\nINACTIVE = rgba({})", &active_one, &active_two, &inactive);
    }

    let mut colors_file = profile_dir.to_owned();
    colors_file.push("colors.conf");

    println!("Input desired wallpaper directory");
    let mut wallpaper_str = String::new();
    io::stdin().read_line(&mut wallpaper_str).expect("Unable to read line");
    wallpaper_str = wallpaper_str.trim().to_string();
    let wallpaper_dir = Path::new(&wallpaper_str);

    let _wall_check = match wallpaper_dir.exists() {
        true => {},
        false => {panic!("This wallpaper doesn't exist!")},
    };
    generate_profile(profile_dir, colors_data, wallpaper_dir);
    println!("New profile \'{}\' added!", &profile_name);

    Ok(())
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
        "new" => match create_profile() {
            Ok(_c) => {},
            Err(e) => {panic!("Couldn't create new profile: {e}")},
        },
        _ => println!("Invalid input"),
    }
}
