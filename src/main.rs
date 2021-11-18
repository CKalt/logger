use std::fs::OpenOptions;
//use std::io::{Read, Write, Seek, SeekFrom};
use std::io::{Read, Seek, SeekFrom};
use gag::Redirect;
//use dirs::data_local_dir;

use std::env;
use std::io;
use std::fs;
use std::path::PathBuf;

use log::{debug, error, log_enabled, info, Level};

fn log_file_path() -> io::Result<PathBuf> {
    let exe = env::current_exe()?;
    let dir = exe.parent().expect("Executable must be in some directory");
    let mut dir = dir.join("");
    dir.pop();
    dir.pop();
    dir.push("logs");
    if !dir.as_path().is_dir() {
        if dir.as_path().exists() {
            panic!(
                r#"The file path used for logs {} is not a directory. Please resolve conflict."#,
                dir.display());
        } else {
            fs::create_dir(&dir)?;
        }
    }

    dir.push("rshot.log");

    Ok(dir)
}

fn config_file_path() -> io::Result<PathBuf> {
    let exe = env::current_exe()?;
    let dir = exe.parent().expect("Executable must be in some directory");
    let mut dir = dir.join("");
    dir.pop();
    dir.pop();
    dir.push("config.toml");
    Ok(dir)
}

fn main() {
    let pathbuf = config_file_path().expect("Couldn't");
    println!("config file = {}", pathbuf.display());

    let pathbuf = log_file_path().expect("Couldn't");
    println!("log file = {}", pathbuf.display());
    println!("This is stdout which is always displayed.");

    // Open a log
    let log = OpenOptions::new()
        .truncate(true)
        .read(true)
        .create(true)
        .write(true)
        .open(pathbuf)
        .unwrap();

    // redirect stderr to log file
    let print_redirect = Redirect::stderr(log).unwrap();

    // env_logger test
    env_logger::init();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }

    // Extract redirect
    let mut log = print_redirect.into_inner();

    let mut buf = String::new();
    log.seek(SeekFrom::Start(0)).unwrap();
    log.read_to_string(&mut buf).unwrap();
    println!("buf is = {}", &buf[..]);
}
