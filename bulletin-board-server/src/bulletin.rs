use crate::{ACV_DIR, TMP_DIR};
use chrono::{DateTime, Local};
use std::fs::{self, File};
use std::io::{self, Cursor, Read, Seek, Write};
use std::path::Path;
use uuid::Uuid;

pub struct Bulletin {
    pub data: BulletinBackend,
    pub datasize: u64,
    pub timestamp: DateTime<Local>,
    file_opened: Option<File>,
}

pub enum BulletinBackend {
    File(String),
    Memory(Cursor<Vec<u8>>),
    Archive((String, u64)),
    Empty,
}

impl Write for Bulletin {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match &mut self.data {
            BulletinBackend::Memory(data) => {
                let size = data.write(buf)?;
                self.datasize += size as u64;
                Ok(size)
            }
            BulletinBackend::File(filename) => {
                let file = self.file_opened.get_or_insert(File::open(filename)?);
                let size = file.write(buf)?;
                self.datasize += size as u64;
                Ok(size)
            }
            BulletinBackend::Archive(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Archive is read only.",
            )),
            BulletinBackend::Empty => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No data backend.",
            )),
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        match &mut self.data {
            BulletinBackend::Memory(data) => data.flush(),
            BulletinBackend::File(_) => match &mut self.file_opened {
                Some(file) => file.flush(),
                None => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "File is not opened.",
                )),
            },
            BulletinBackend::Archive(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Archive is read only.",
            )),
            BulletinBackend::Empty => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No data backend.",
            )),
        }
    }
}

impl Read for Bulletin {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.data {
            BulletinBackend::Memory(data) => data.read(buf),
            BulletinBackend::File(filename) => {
                let file = self.file_opened.get_or_insert(File::open(filename)?);
                file.read(buf)
            }
            BulletinBackend::Archive((name, offset)) => {
                let filename = format!("{}/{}/data.bin", *ACV_DIR, name);
                let file = self.file_opened.get_or_insert({
                    let mut file = File::open(filename)?;
                    file.seek_relative(*offset as i64)?;
                    file
                });
                file.read(buf)
            }
            BulletinBackend::Empty => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No data backend.",
            )),
        }
    }
}

impl Bulletin {
    // pub fn new() -> Self {
    //     Self {
    //         data: BulletinBackend::Empty,
    //         datasize: 0,
    //         timestamp: Local::now(),
    //         file_opened: None,
    //     }
    // }
    pub fn from_archive(
        name: &str,
        offset: u64,
        datasize: u64,
        timestamp: DateTime<Local>,
    ) -> Self {
        Self {
            data: BulletinBackend::Archive((name.to_owned(), offset)),
            datasize,
            timestamp,
            file_opened: None,
        }
    }
    pub fn from_data(data: Vec<u8>) -> Self {
        let datasize = data.len() as u64;
        Self {
            data: BulletinBackend::Memory(Cursor::new(data)),
            datasize,
            timestamp: Local::now(),
            file_opened: None,
        }
    }
    pub fn get(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = vec![0; self.datasize.try_into().unwrap()];
        self.read_exact(&mut buf)?;
        self.close();
        Ok(buf)
    }
    pub fn clear(&mut self) -> Result<(u64, u64, u64), std::io::Error> {
        match &mut self.data {
            BulletinBackend::Memory(_) => {
                self.data = BulletinBackend::Empty;
                Ok((self.datasize, self.datasize, 0))
            }
            BulletinBackend::File(filename) => {
                fs::remove_file(filename)?;
                self.data = BulletinBackend::Empty;
                Ok((self.datasize, 0, 1))
            }
            BulletinBackend::Archive(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Archived entry cannot be deleted.",
            )),
            BulletinBackend::Empty => Ok((0, 0, 0)),
        }
    }
    pub fn save_to_file(&mut self) -> Result<(), std::io::Error> {
        match &mut self.data {
            BulletinBackend::Memory(data) => {
                let mut uuid = Uuid::new_v4().to_string();
                let first: String = uuid.drain(..2).collect();
                let second: String = uuid.drain(..2).collect();
                let dir = format!("{}/{}/{}", *TMP_DIR, first, second);
                if !Path::new(&dir).exists() {
                    fs::create_dir_all(&dir)?;
                }
                let filename = [dir, uuid].join("/");
                let mut file = File::create(&filename)?;
                io::copy(data, &mut file)?;
                self.data = BulletinBackend::File(filename);
                Ok(())
            }
            BulletinBackend::Empty => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No data backend.",
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Already moved out of memory.",
            )),
        }
    }
    pub fn close(&mut self) {
        match &mut self.data {
            BulletinBackend::Memory(data) => {
                data.set_position(0);
            }
            BulletinBackend::Empty => {}
            _ => {
                self.file_opened = None;
            }
        }
    }
    pub fn backend(&self) -> String {
        match &self.data {
            BulletinBackend::Memory(_) => "memory".to_string(),
            BulletinBackend::File(filename) => format!("file:{filename}"),
            BulletinBackend::Archive((name, offset)) => format!("archive:{name}:{offset}"),
            BulletinBackend::Empty => "deleted".to_string(),
        }
    }
}
