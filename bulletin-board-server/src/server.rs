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
use crate::{LISTEN_ADDR, LOG_FILE};
use bulletin_board_common::*;
use chrono::Local;
use serde_bytes::ByteBuf;
use std::io;

pub struct BBServer {
    bulletinboard: BulletinBoard,
    archive_manipulations: Vec<(String, Option<String>)>,
}

impl BBServer {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Self {
            bulletinboard: BulletinBoard::new()?,
            archive_manipulations: vec![],
        })
    }
    pub fn listen(&mut self) -> Result<(), std::io::Error> {
        #[cfg(feature = "unix")]
        if std::path::Path::new(&*LISTEN_ADDR).exists() {
            std::fs::remove_file(&*LISTEN_ADDR)?;
        }
        {
            let datetime = Local::now().to_string();
            let version = env!("CARGO_PKG_VERSION");
            let message = format!("{datetime} Bulletin Board Server v{version} started.");
            std::fs::write(&*LOG_FILE, message)?;
        }
        let listener = TcpOrUnixListener::bind(&*LISTEN_ADDR)?;
        for stream in listener.incoming() {
            let stream = stream?;
            if let Err(err) = self.process(stream) {
                let err = Box::leak(err);
                let datetime = Local::now().to_string();
                let message = format!("{datetime} {err}");
                eprintln!("{message}");
                std::fs::write(&*LOG_FILE, message)?;
            };
        }
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
    fn post(&mut self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let (var_name, var_tag, data): (String, String, ByteBuf) =
            ciborium::from_reader(&mut *stream)?;
        let bulletin = Bulletin::from_data(data.to_vec());
        self.bulletinboard.post(var_name, var_tag, bulletin)?;
        Ok(())
    }
    fn get_tag(
        &self,
        var_name: &String,
        var_tag: Option<String>,
        stream: &mut TcpOrUnixStream,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match var_tag {
            Some(tag) => Ok(tag),
            None => {
                let tags = self.bulletinboard.find_tags(var_name);
                match tags.len() {
                    0 => {
                        ciborium::into_writer(&Response::NotFound, stream)?;
                        Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            "Not found.",
                        )))
                    }
                    1 => Ok(tags[0].clone()),
                    _ => {
                        ciborium::into_writer(&Response::NotUnique(tags.clone()), stream)?;
                        Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Found multiple entries having the same name.",
                        )))
                    }
                }
            }
        }
    }
    fn read(&mut self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let (var_name, var_tag, revisions): (String, Option<String>, Vec<u64>) =
            ciborium::from_reader(&mut *stream)?;
        let var_tag = self.get_tag(&var_name, var_tag, &mut *stream)?;
        let mut buf = Cursor::new(vec![]);

        if let Some(bulletins) = self.bulletinboard.take(var_name, var_tag) {
            if revisions.is_empty() {
                if let Some(bulletin) = bulletins.last_mut() {
                    ciborium::into_writer(&Response::Ok, &mut buf)?;
                    let data = bulletin.get()?;
                    ciborium::into_writer(&ByteBuf::from(data), &mut buf)?;
                    bulletin.close();
                } else {
                    ciborium::into_writer(&Response::NotFound, stream)?;
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Not found.",
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
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            "Not found.",
                        )));
                    }
                }
            }
        } else {
            ciborium::into_writer(&Response::NotFound, stream)?;
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Not found.",
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
        let (var_name, var_tag): (String, Option<String>) = ciborium::from_reader(&mut *stream)?;
        let var_tag = self.get_tag(&var_name, var_tag, &mut *stream)?;
        match self.bulletinboard.get_info(var_name, var_tag) {
            Some(info) => {
                let mut buf = Cursor::new(vec![]);
                ciborium::into_writer(&Response::Ok, &mut buf)?;
                ciborium::into_writer(&info, &mut buf)?;
                buf.set_position(0);
                io::copy(&mut buf, stream)?;
            }
            None => {
                ciborium::into_writer(&Response::NotFound, stream)?;
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Not found.",
                )));
            }
        }
        Ok(())
    }
    fn clear_revisions(
        &mut self,
        stream: &mut TcpOrUnixStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (var_name, var_tag, revisions): (String, String, Vec<u64>) =
            ciborium::from_reader(stream)?;
        self.bulletinboard
            .clear_revisions(var_name, var_tag, revisions)?;
        Ok(())
    }
    fn remove(&mut self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let (var_name, var_tag): (String, String) = ciborium::from_reader(stream)?;
        self.bulletinboard.remove(var_name, var_tag)?;
        Ok(())
    }
    fn archive(&mut self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let (var_name, var_tag, name): (String, String, String) = ciborium::from_reader(stream)?;
        self.bulletinboard.archive(var_name, var_tag, name)?;
        Ok(())
    }
    fn load(&mut self, stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let name: String = ciborium::from_reader(stream)?;
        self.bulletinboard.load(name)?;
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
        let name: String = ciborium::from_reader(stream)?;
        self.bulletinboard.restore(name)?;
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
