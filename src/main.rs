// in standard crate
use std::fs;
use std::fs::File;
use std::env;
use std::process;
use std::process::Command;
use std::io::{copy, Write};
use std::io::prelude::*;
use std::path::Path;
// additional dependencies
use reqwest::blocking;
use colored::Colorize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = "/etc/maia/";
    let install = "/usr/bin/";
    fs::create_dir_all(install)?;
    fs::create_dir_all(config)?;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}: No operation specified", "Error".yellow().bold());
        process::exit(3);
    }
    if args[1] != "install" && args[1] != "override" && args[1] != "remove" && args[1] != "setup" && args[1] != "help" && args[1] != "test" {
        println!("{}: {} is not a valid subcommand", "Error".yellow().bold(), args[1]);
        process::exit(3);
    }
    if args[1] == "install" {
        if args.len() < 3 {
            println!("{}: No application specified", "Error".yellow().bold());
            process::exit(3);
        }
        let repo: Vec<&str> = args[2].split("/").collect();
        if repo.len() < 2 {
            println!("{}: Please specify a repository (e.g. 'sudo maia install {}/REPONAME')", "Error".yellow().bold(), args[2]);
            process::exit(3);
        }
        print!("Preparing to install {} ... ", args[2]);
        std::io::stdout().flush().unwrap();
        let install = args[2].to_lowercase();
        let url = format!("https://api.github.com/repos/{}/releases/latest", &install);
        fs::create_dir_all(format!("{}{}", config, repo[0].to_lowercase()))?;
        // get data
        let raw = Command::new("curl")
            .arg(url)
            .output()
            .expect(&format!("{}", "Faliure".red().bold()));
        let data = String::from_utf8(raw.stdout).expect(&format!("{}", "Faliure".red().bold()));
        let parts: Vec<&str> = data.split("\n").collect();
        // check for matching files
        for part in &parts {
            if part.contains("\"name\": ") {
                let split: Vec<&str> = part.split(": ").collect();
                let mut fname = split[1].to_string();
                fname.retain(|c| c != '"');
                fname.retain(|c| c != ',');
                let splitname: Vec<&str> = fname.split(".").collect();
                let truename = splitname[0].to_lowercase();
                let pathstr = format!("/usr/bin/{}", truename);
                let path = Path::new(&pathstr);
                if path.exists() {
                    println!("{}: {} is already installed (did you mean 'sudo maia override {}')", "Faliure".red().bold(), args[2], args[2]);
                    process::exit(3);
                }
            }
            // check for error messages
            if part.contains("message") {
                if part.contains("Not Found") {
                    println!("{}: 404 not found", "Faliure".red().bold());
                    process::exit(2);
                }
                if part.contains("Moved Permanently") {
                    println!("{}: Repository {} has moved", "Faliure".red().bold(), args[2]);
                    process::exit(2);
                }
                if part.contains("API rate limit exceeded") {
                    println!("{}: API rate limit exceeded", "Faliure".red().bold());
                    process::exit(2);
                }
            }
        }
        // download files
        let mut say = true;
        let mut file = File::create(format!("{}{}", config, install))?;
        for part in parts {
            if part.contains("browser_download_url") {
                if say {
                    println!("{}", "Success".green().bold());
                    say = false;
                }
                let download: Vec<&str> = part.split(": ").collect();
                let mut link: String = download[1].to_string();
                link.retain(|c| c != '"');
                let fname: Vec<&str> = link.split("/").collect();
                print!("Downloading file {} ... ", fname[fname.len() - 1]);
                std::io::stdout().flush().unwrap();
                let splitname: Vec<&str> = fname[fname.len() - 1].split(".").collect();
                let truename = splitname[0].to_lowercase();
                let response = blocking::get(&link).unwrap();
                let mut dest = File::create(format!("/usr/bin/{}", truename)).unwrap();
                let content = response.bytes()?;
                copy(&mut content.as_ref(), &mut dest).unwrap();
                println!("{}", "Success".green().bold());
                print!("Updating configuration ... ");
                std::io::stdout().flush().unwrap();
                file.write_all(format!("{}\n", truename).as_bytes())?;
                Command::new("chmod")
                    .arg("a+x")
                    .arg(&format!("/usr/bin/{}", truename))
                    .spawn()
                    .expect(&"Faliure".red().bold());
                println!("{}", "Success".green().bold());
            }
        }
    }
    if args[1] == "remove" {
        if args.len() < 3 {
            println!("{}: No application specified", "Error".yellow().bold());
            process::exit(3);
        }
        print!("Getting {}'s file list ... ", args[2]);
        std::io::stdout().flush().unwrap();
        let remove = args[2].to_lowercase();
        let pathstr = format!("{}{}", config, remove);
        let path = Path::new(&pathstr);
        if path.exists() {
            let mut file = File::open(pathstr)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            println!("{}", "Success".green().bold());
            let split: Vec<&str> = contents.split("\n").collect();
            for i in split {
                if i != "" {
                    print!("Removing file {} ... ", i);
                    std::io::stdout().flush().unwrap();
                    fs::remove_file(format!("{}{}", install, i))?;
                    println!("{}", "Success".green().bold());
                }
            }
            print!("Removing {}'s file list ... ", remove);
            std::io::stdout().flush().unwrap();
            fs::remove_file(format!("{}{}", config, remove))?;
            println!("{}", "Success".green().bold());
        } else {
            println!("{}: {} is not installed", "Faliure".red().bold(), args[2]);
        }
    }
    if args[1] == "override" {
        if args.len() < 3 {
            println!("{}: No application specified", "Error".yellow().bold());
            process::exit(3);
        }
        let repo: Vec<&str> = args[2].split("/").collect();
        if repo.len() < 2 {
            println!("{}: Please specify a repository (e.g. 'sudo maia install {}/REPONAME')", "Error".yellow().bold(), args[2]);
            process::exit(3);
        }
        print!("Preparing to install {} ... ", args[2]);
        std::io::stdout().flush().unwrap();
        let over = args[2].to_lowercase();
        let url = format!("https://api.github.com/repos/{}/releases/latest", &over);
        fs::create_dir_all(format!("{}{}", config, repo[0]))?;
        // get data
        let raw = Command::new("curl")
            .arg(url)
            .output()
            .expect(&format!("{}", "Faliure".red().bold()));
        let data = String::from_utf8(raw.stdout).expect(&format!("{}", "Faliure".red().bold()));
        let parts: Vec<&str> = data.split("\n").collect();
        // download files
        let mut say = true;
        let mut file = File::create(format!("{}{}", config, over))?;
        for part in parts {
            if part.contains("browser_download_url") {
                if say {
                    println!("{}", "Success".green().bold());
                    say = false;
                }
                let download: Vec<&str> = part.split(": ").collect();
                let mut link: String = download[1].to_string();
                link.retain(|c| c != '"');
                let fname: Vec<&str> = link.split("/").collect();
                print!("Downloading file {} ... ", fname[fname.len() - 1]);
                std::io::stdout().flush().unwrap();
                let splitname: Vec<&str> = fname[fname.len() - 1].split(".").collect();
                let truename = splitname[0].to_lowercase();
                let response = blocking::get(&link).unwrap();
                let mut dest = File::create(format!("/usr/bin/{}", truename)).unwrap();
                let content = response.bytes()?;
                copy(&mut content.as_ref(), &mut dest).unwrap();
                println!("{}", "Success".green().bold());
                print!("Updating configuration ... ");
                std::io::stdout().flush().unwrap();
                file.write_all(format!("{}\n", truename).as_bytes())?;
                Command::new("chmod")
                    .arg("a+x")
                    .arg(&format!("/usr/bin/{}", truename))
                    .spawn()
                    .expect(&"Faliure".red().bold());
                println!("{}", "Success".green().bold());
            }
            if part.contains("message") {
                if part.contains("Not Found") {
                    println!("{}: 404 not found", "Faliure".red().bold());
                    fs::remove_file(format!("{}{}", config, args[2]))?;
                    process::exit(2);
                }
                if part.contains("Moved Permanently") {
                    println!("{}: Repository {} has moved", "Faliure".red().bold(), args[2]);
                    fs::remove_file(format!("{}{}", config, args[2]))?;
                    process::exit(2);
                }
                if part.contains("API rate limit exceeded") {
                    println!("{}: API rate limit exceeded", "Faliure".red().bold());
                    fs::remove_file(format!("{}{}", config, args[2]))?;
                    process::exit(2);
                }
            }
        }
    }
    if args[1] == "setup" {
        if args.len() > 2 {
            println!("{}: Subcommand setup needs no additional arguments", "Error".yellow().bold());
            process::exit(3);
        }
        print!("Moving executable to /usr/bin ... ");
        std::io::stdout().flush().unwrap();
        let currentpath = env::current_exe()?.display().to_string();
        fs::rename(currentpath, "/usr/bin/maia")?;
        println!("{}", "Success".green().bold());
        print!("Creating confiuration ... ");
        std::io::stdout().flush().unwrap();
        fs::create_dir_all("/etc/maia/milorad-co/")?;
        let file = File::create("/etc/maia/milorad-co/maia");
        file?.write_all(b"maia")?;
        println!("{}", "Success".green().bold());
        println!("The Milorad Automated Installation Application is now installed on your device! Run 'maia help' for a list of subcommands");
    }
    if args[1] == "help" {
        if args.len() < 3 {
            println!("Milorad Automated Installation Application v0.0.0
Usage: maia <subcommand> <arguments>

MAIA is a commandline installation application. It is designed to automate
the installation of files from the extremely large open-source application
sharing website known as GitHub.

Subcommands:
install     install an application
remove      remove an application
override    same as install, but does not throw an error if it has to
            overwrite files, can be used to update applications
help        show this help, or show more detailed information about a
            specific subcommand

Run 'maia help <subcommand>' for more information about a specific subcommand.");
        } else {
            if args[2] == "install" || args[2] == "override" {
                println!("MAIA install and override subcommands
Usage: sudo maia <install or override> <account name>/<repository name>

The install subcommand is used to install applications. To specify the
application to install, you must enter the name of the account which
owns the repository, followed by a forward slash (/), followed by the
name of the repository. For example, you could run
'sudo maia install milorad-co/mica' to install MICA, our image editor.
Please note that to install an application, the targeted repository
must have at least one release and its latest release must have at least
one asset that is not source code. The install subcommand will fail if
it tries to overwrite files.

The override subcommand is similar to install, except it will not fail if
forced to overwrite files. It can be used to update applications, as it
will install the latest release and overwrite outdated files.");
            }
            if args[2] == "remove" {
                println!("MAIA remove subcommand
Usage: sudo maia remove <account name>/<repository name>

Remove removes applications and any configuration data that MAIA created
for them. Note that some data may remain, such as extremely out of date
files whose references were removed during an override.");
            }
        }
    }
    Ok(())
}
