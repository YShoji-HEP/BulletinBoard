use crate::{DEBUG, LOG_FILE, LOG_LEVEL};
use chrono::Local;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};

fn write(message: String) {
    let datetime = Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let message = format!("{datetime} {message}\n");
    if *DEBUG {
        print!("{}", message);
    }
    if File::options()
        .write(true)
        .create(true)
        .truncate(false)
        .open(&*LOG_FILE)
        .and_then(|mut log_file| {
            log_file.seek(SeekFrom::End(0))?;
            log_file.write_all(message.as_bytes())?;
            Ok(())
        })
        .is_err()
    {
        println!("{datetime} [ERROR] Log file is not writable.");
    }
}

pub fn error(message: String) {
    if *LOG_LEVEL >= 1 {
        write(format!("[ERROR] {message}"));
    }
}

pub fn warn(message: String) {
    if *LOG_LEVEL >= 2 {
        write(format!("[WARN] {message}"));
    }
}

pub fn notice(message: String) {
    if *LOG_LEVEL >= 3 {
        write(format!("[NOTICE] {message}"));
    }
}

pub fn info(message: String) {
    if *LOG_LEVEL >= 4 {
        write(format!("[INFO] {message}"));
    }
}

pub fn debug(message: String) {
    if *LOG_LEVEL >= 5 {
        write(format!("[DEBUG] {message}"));
    }
}
