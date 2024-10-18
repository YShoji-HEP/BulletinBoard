use std::io::Cursor;
#[cfg(not(feature = "unix"))]
use std::net::TcpListener as TcpOrUnixListener;
#[cfg(not(feature = "unix"))]
use std::net::TcpStream as TcpOrUnixStream;
#[cfg(feature = "unix")]
use std::os::unix::net::UnixListener as TcpOrUnixListener;
#[cfg(feature = "unix")]
use std::os::unix::net::UnixStream as TcpOrUnixStream;

use crate::board::BulletinBoard;
use crate::bulletin::Bulletin;
use crate::error::{ArchiveError, BulletinError};
use crate::{LISTEN_ADDR, LOG_FILE};
use bulletin_board_common::*;
use chrono::Local;
use serde_bytes::ByteBuf;
use std::fs::File;
use std::io::{self, Seek, SeekFrom, Write};

pub struct BBServer {
    bulletinboard: BulletinBoard,
    archive_manipulations: Vec<(String, Option<String>)>,
    debug: bool,
}

impl BBServer {
    pub fn new(debug: bool) -> Result<Self, std::io::Error> {
        Ok(Self {
            bulletinboard: BulletinBoard::new()?,
            archive_manipulations: vec![],
            debug,
        })
    }
    pub fn listen(&mut self) -> Result<(), std::io::Error> {
        #[cfg(feature = "unix")]
        if std::path::Path::new(&*LISTEN_ADDR).exists() {
            std::fs::remove_file(&*LISTEN_ADDR)?;
        }
        {
            let version = env!("CARGO_PKG_VERSION");
            let message = format!("Bulletin Board Server v{version} started.");
            self.write_log(message)?;
        }
        let listener = TcpOrUnixListener::bind(&*LISTEN_ADDR)?;
        for stream in listener.incoming() {
            let stream = stream?;
            if let Err(err) = self.process(stream) {
                let err = Box::leak(err);
                self.write_log(err.to_string())?;
            };
        }
        Ok(())
    }
    fn write_log(&self, message: String) -> Result<(), std::io::Error> {
        let datetime = Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let message = format!("{datetime} {message}\n");
        if self.debug {
            print!("{}", message);
        }
        let mut log_file = File::options()
            .write(true)
            .create(true)
            .truncate(false)
            .open(&*LOG_FILE)?;
        log_file.seek(SeekFrom::End(0))?;
        log_file.write_all(message.as_bytes())?;
        Ok(())
    }
    fn process(&mut self, mut stream: TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        while let Ok(operation) = ciborium::from_reader(&mut stream) {
            match operation {
                Operation::Post => {
                    self.post(&mut stream)?;
                }
                Operation::Read => {
                    self.read(&mut stream)?;
                }
                Operation::Status => {
                    self.status(&mut stream)?;
                }
                Operation::Log => {
                    self.log(&mut stream)?;
                }
                Operation::ViewBoard => {
                    self.view_board(&mut stream)?;
                }
                Operation::GetInfo => {
                    self.get_info(&mut stream)?;
                }
                Operation::ClearRevision => {
                    self.clear_revisions(&mut stream)?;
                }
                Operation::Remove => {
                    self.remove(&mut stream)?;
                }
                Operation::Archive => {
                    self.archive(&mut stream)?;
                }
                Operation::Load => {
                    self.load(&mut stream)?;
                }
                Operation::ListArchive => {
                    self.list_archive(&mut stream)?;
                }
                Operation::RenameArchive => {
                    self.rename_archive(&mut stream)?;
                }
                Operation::DeleteArchive => {
                    self.delete_archive(&mut stream)?;
                }
                Operation::Dump => {
                    self.dump(&mut stream)?;
                }
                Operation::Restore => {
                    self.restore(&mut stream)?;
                }
                Operation::Reset => {
                    self.reset()?;
                }
            };
        }
        Ok(())
    }
    fn get_tag(
        &self,
        operation: &str,
        title: &String,
        tag: Option<String>,
        stream: &mut TcpOrUnixStream,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match tag {
            Some(tag) => Ok(tag),
            None => {
                let tags = self.bulletinboard.find_tags(title);
                match tags.len() {
                    0 => {
                        ciborium::into_writer(&Response::NotFound, stream)?;
                        Err(Box::new(BulletinError::new(
                            operation,
                            "Not found.".to_string(),
                            title.clone(),
                            "NA".to_string(),
                            None,
                        )))
                    }
                    1 => Ok(tags[0].clone()),
                    _ => {
                        ciborium::into_writer(&Response::NotUnique(tags.clone()), stream)?;
                        Err(Box::new(BulletinError::new(
                            operation,
                            "Found multiple entries having the same name.".to_string(),
                            title.clone(),
                            "NA".to_string(),
                            None,
                        )))
                    }
                }
            }
        }
    }
    fn post(&mut self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let (title, tag, data): (String, String, ByteBuf) =
            ciborium::from_reader(&mut *stream)?;
        let bulletin = Bulletin::from_data(data.to_vec());
        self.bulletinboard
            .post(title.clone(), tag.clone(), bulletin)
            .map_err(|err| BulletinError::new("post", err.to_string(), title, tag, None))?;
        Ok(())
    }
    fn read(&mut self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let (title, tag, revisions): (String, Option<String>, Vec<u64>) =
            ciborium::from_reader(&mut *stream)?;
        let tag = self.get_tag("read", &title, tag, &mut *stream)?;
        let mut buf = Cursor::new(vec![]);

        if let Some(bulletins) = self.bulletinboard.take(title.clone(), tag.clone()) {
            if revisions.is_empty() {
                if let Some(bulletin) = bulletins.last_mut() {
                    ciborium::into_writer(&Response::Ok, &mut buf)?;
                    let data = bulletin.get()?;
                    ciborium::into_writer(&ByteBuf::from(data), &mut buf)?;
                    bulletin.close();
                } else {
                    ciborium::into_writer(&Response::NotFound, stream)?;
                    return Err(Box::new(BulletinError::new(
                        "read",
                        "Not found.".to_string(),
                        title,
                        tag,
                        None,
                    )));
                }
            } else {
                for revision in revisions {
                    if let Some(bulletin) = bulletins.get_mut::<usize>(revision.try_into().unwrap())
                    {
                        ciborium::into_writer(&Response::Ok, &mut buf)?;
                        let data = bulletin.get()?;
                        ciborium::into_writer(&ByteBuf::from(data), &mut buf)?;
                        bulletin.close();
                    } else {
                        ciborium::into_writer(&Response::NotFound, stream)?;
                        return Err(Box::new(BulletinError::new(
                            "read",
                            "Not found.".to_string(),
                            title,
                            tag,
                            None,
                        )));
                    }
                }
            }
        } else {
            ciborium::into_writer(&Response::NotFound, stream)?;
            return Err(Box::new(BulletinError::new(
                "read",
                "Not found.".to_string(),
                title,
                tag,
                None,
            )));
        };
        buf.set_position(0);
        io::copy(&mut buf, stream)?;

        Ok(())
    }
    fn status(&self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let status = self.bulletinboard.status();
        ciborium::into_writer(&status, stream)?;
        Ok(())
    }
    fn log(&self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let log = std::fs::read_to_string(&*LOG_FILE)?;
        ciborium::into_writer(&log, stream)?;
        Ok(())
    }
    fn view_board(&self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let board = self.bulletinboard.view();
        ciborium::into_writer(&board, stream)?;
        Ok(())
    }
    fn get_info(&self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let (title, tag): (String, Option<String>) = ciborium::from_reader(&mut *stream)?;
        let tag = self.get_tag("get_info", &title, tag, &mut *stream)?;
        match self
            .bulletinboard
            .get_info(title.clone(), tag.clone())
        {
            Some(info) => {
                let mut buf = Cursor::new(vec![]);
                ciborium::into_writer(&Response::Ok, &mut buf)?;
                ciborium::into_writer(&info, &mut buf)?;
                buf.set_position(0);
                io::copy(&mut buf, stream)?;
            }
            None => {
                ciborium::into_writer(&Response::NotFound, stream)?;
                return Err(Box::new(BulletinError::new(
                    "get_info",
                    "Not found.".to_string(),
                    title,
                    tag,
                    None,
                )));
            }
        }
        Ok(())
    }
    fn clear_revisions(
        &mut self,
        stream: &mut TcpOrUnixStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (title, tag, revisions): (String, String, Vec<u64>) =
            ciborium::from_reader(stream)?;
        self.bulletinboard
            .clear_revisions(title.clone(), tag.clone(), revisions)
            .map_err(|err| {
                Box::new(BulletinError::new(
                    "clear_revisions",
                    err.to_string(),
                    title,
                    tag,
                    None,
                ))
            })?;
        Ok(())
    }
    fn remove(&mut self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let (title, tag): (String, String) = ciborium::from_reader(stream)?;
        self.bulletinboard
            .remove(title.clone(), tag.clone())
            .map_err(|err| {
                Box::new(BulletinError::new(
                    "remove",
                    err.to_string(),
                    title,
                    tag,
                    None,
                ))
            })?;
        Ok(())
    }
    fn archive(&mut self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let (title, tag, acv_name): (String, String, String) =
            ciborium::from_reader(stream)?;
        self.bulletinboard
            .archive(title.clone(), tag.clone(), acv_name)
            .map_err(|err| {
                Box::new(BulletinError::new(
                    "archive",
                    err.to_string(),
                    title,
                    tag,
                    None,
                ))
            })?;
        Ok(())
    }
    fn load(&mut self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let acv_name: String = ciborium::from_reader(stream)?;
        self.bulletinboard
            .load(acv_name.clone())
            .map_err(|err| ArchiveError::new("load", err.to_string(), acv_name))?;
        Ok(())
    }
    fn list_archive(&self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        match self.bulletinboard.list_archive() {
            Ok(list) => {
                ciborium::into_writer(&list, stream)?;
            }
            Err(_) => {
                let empty: Vec<String> = vec![];
                ciborium::into_writer(&empty, stream)?;
            }
        }
        Ok(())
    }
    fn rename_archive(
        &mut self,
        stream: &mut TcpOrUnixStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (name_from, name_to): (String, String) = ciborium::from_reader(stream)?;
        self.archive_manipulations.push((name_from, Some(name_to)));
        Ok(())
    }
    fn delete_archive(
        &mut self,
        stream: &mut TcpOrUnixStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let name: String = ciborium::from_reader(stream)?;
        self.archive_manipulations.push((name, None));
        Ok(())
    }
    fn dump(&mut self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let name: String = ciborium::from_reader(stream)?;
        self.bulletinboard.dump(name)?;
        Ok(())
    }
    fn restore(&mut self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let acv_name: String = ciborium::from_reader(stream)?;
        self.bulletinboard
            .restore(acv_name.clone())
            .map_err(|err| ArchiveError::new("restore", err.to_string(), acv_name))?;
        Ok(())
    }
    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.bulletinboard.reset()?;
        for (name_from, name_to) in self.archive_manipulations.drain(..) {
            match name_to {
                Some(name_to) => {
                    self.bulletinboard.rename_archive(name_from, name_to)?;
                }
                None => {
                    self.bulletinboard.delete_archive(name_from)?;
                }
            }
        }
        Ok(())
    }
}
