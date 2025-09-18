use std::{
    error::Error,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

const GITHUB_URL: &str = "https://github.com/archlinux/aur.git";
const GIT_COMMAND: [&str; 2] = ["git clone --branch", "--single-branch"];
const CACHE_PATH: &str = "~/.cache/aura";

fn expand_cache_path(cache_path: &str) -> PathBuf {
    let home_directory = std::env::home_dir().expect("Failed to fetch home directory!");
    let mut result: PathBuf = PathBuf::new();
    if cache_path.contains("~/") {
        result.push(home_directory);
        result.push(cache_path.replace("~/", ""));
    }
    result
}

fn create_tmp_directory() -> Result<PathBuf, Box<dyn Error>> {
    let aura_path = expand_cache_path(CACHE_PATH);
    fs::create_dir_all(aura_path.clone())?;
    Ok(aura_path)
}

fn build_package(mut aura_path: PathBuf, package_name: &str) -> Result<(), Box<dyn Error>> {
    aura_path.push(package_name);

    if !Path::new(&aura_path).exists() {
        std::process::exit(0);
    }

    std::process::Command::new("makepkg")
        .arg("-si")
        .current_dir(&aura_path)
        .spawn()?
        .wait()?;

    Ok(())
}

fn execute_command(package_name: &str) -> Result<(), Box<dyn Error>> {
    // git clone --branch <package_name> --single-branch https://github.com/archlinux/aur.git <package_name>
    let aura_path = create_tmp_directory()?;
    let command: String = format!(
        "{} {package_name} {} {GITHUB_URL} {package_name}",
        GIT_COMMAND[0], GIT_COMMAND[1]
    );

    println!("{command}");

    match std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(&aura_path)
        .spawn()
    {
        Ok(ref mut child) => {
            child.wait()?;
        }
        Err(e) => eprintln!("Failed to fetch package from Github's AUR, {e}"),
    }

    build_package(aura_path, package_name)?;

    Ok(())
}

fn clean_cache_path(cache_path: &str) -> Result<(), Box<dyn Error>> {
    let home_directory = std::env::home_dir().expect("Failed to fetch home directory!");
    let mut result: PathBuf = PathBuf::new();
    if cache_path.contains("~/") {
        result.push(home_directory);
        result.push(cache_path.replace("~/", ""));
    }

    println!("{:?}", result);

    if Path::new(&result).exists() {
        std::process::Command::new("du")
            .arg("-sh")
            .arg(&result)
            .spawn()?
            .wait()?;
    } else {
        println!("Cache folder doesn't exist :D");
        return Ok(());
    }
    let mut choice: String = String::new();
    print!("clear Cache folder for aura? (Y/n): ");
    std::io::stdout().flush()?;
    io::stdin().read_line(&mut choice)?;
    choice = choice.trim().to_lowercase();
    println!("{choice}");
    if choice != "n" {
        std::process::Command::new("rm")
            .arg("-rf")
            .arg(&result)
            .spawn()?
            .wait()?;
        println!("Removed {:?}", result);
    }

    Ok(())
}

fn help() {
    println!(
        "\naura\nAUR helper for Github mirror repo: https://github.com/archlinux/aur\n\n[USAGE]\n\taura -S visual-studio-code-bin\tTo install packages\n\taura -Sc\t\t\tTo clear cache folder",
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let cmd_args: Vec<String> = std::env::args().collect();

    if cmd_args.len() == 3 {
        if cmd_args[1] == "-S" {
            let package_name: &str = cmd_args[2].trim();
            execute_command(package_name)?;
        } else {
            println!("Invalid args.");
        }
    } else if cmd_args.len() == 2 {
        if cmd_args[1] == "-S" {
            println!("Please provide the Package name");
            std::process::exit(0);
        } else if cmd_args[1] == "-Scc" || cmd_args[1] == "-Sc" {
            clean_cache_path(CACHE_PATH)?;
        } else if cmd_args[1] == "--help" || cmd_args[1] == "-h" {
            help();
        } else {
            println!("Invalid args..");
        }
    } else {
        println!("Invalid args...");
        help();
    }
    Ok(())
}
