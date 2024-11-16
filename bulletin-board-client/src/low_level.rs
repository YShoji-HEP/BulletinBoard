use crate::ADDR;
use bulletin_board_common::*;
use serde::de::DeserializeOwned;
use serde_bytes::ByteBuf;
use std::io::{self, Cursor};
use std::net::TcpStream;

#[cfg(target_family = "unix")]
use std::os::unix::net::UnixStream;

#[cfg(not(target_family = "unix"))]
pub enum TcpOrUnixStream {
    TCP(TcpStream),
}

/// Abstraction of stream and a set of control functions.
#[cfg(target_family = "unix")]
pub enum TcpOrUnixStream {
    TCP(TcpStream),
    Unix(UnixStream),
}

impl TcpOrUnixStream {
    /// Open a TCP/UNIX socket. This blocks the server until the instance is dropped.
    pub fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        let addr = ADDR.lock().unwrap().clone();
        #[cfg(target_family = "unix")]
        let stream = {
            let re = regex::Regex::new(":[0-9]+$").unwrap();
            if re.is_match(&addr) {
                TcpOrUnixStream::TCP(TcpStream::connect(&addr)?)
            } else {
                TcpOrUnixStream::Unix(UnixStream::connect(&addr)?)
            }
        };
        #[cfg(not(target_family = "unix"))]
        let stream = TcpOrUnixStream::TCP(TcpStream::connect(&addr)?);
        Ok(stream)
    }

    fn send(&mut self, mut buffer: Cursor<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
        buffer.set_position(0);
        #[cfg(target_family = "unix")]
        match self {
            TcpOrUnixStream::TCP(stream) => {
                io::copy(&mut buffer, stream)?;
            }
            TcpOrUnixStream::Unix(stream) => {
                io::copy(&mut buffer, stream)?;
            }
        }
        #[cfg(not(target_family = "unix"))]
        match self {
            TcpOrUnixStream::TCP(stream) => {
                io::copy(&mut buffer, stream)?;
            }
        }
        Ok(())
    }

    fn receive<T: DeserializeOwned>(&mut self) -> Result<T, Box<dyn std::error::Error>> {
        #[cfg(target_family = "unix")]
        match self {
            TcpOrUnixStream::TCP(stream) => Ok(ciborium::from_reader(stream)?),
            TcpOrUnixStream::Unix(stream) => Ok(ciborium::from_reader(stream)?),
        }
        #[cfg(not(target_family = "unix"))]
        match self {
            TcpOrUnixStream::TCP(stream) => Ok(ciborium::from_reader(stream)?),
        }
    }

    /// Posts binary of ArrayObject.
    pub fn post_raw(
        &mut self,
        title: &str,
        tag: &str,
        binary: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val = serde_bytes::ByteBuf::from(binary);
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::Post, &mut buffer)?;
        ciborium::into_writer(&(title.to_string(), tag.to_string(), val), &mut buffer)?;
        self.send(buffer)?;
        Ok(())
    }

    /// Reads ArrayObject as binary.
    ///
    /// Tag can be None if there is only one tag exists for the title.
    /// When revisions is empty, the latest revision is returned.
    pub fn read_raw(
        &mut self,
        title: &str,
        tag: Option<&str>,
        revisions: Vec<u64>,
    ) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        let mut list = vec![];
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::Read, &mut buffer)?;
        ciborium::into_writer(
            &(
                title.to_string(),
                tag.map(|x| x.to_string()),
                revisions.clone(),
            ),
            &mut buffer,
        )?;
        self.send(buffer)?;

        for _ in 0..revisions.len().max(1) {
            let res = self.receive()?;
            let binary = match res {
                Response::Ok => {
                    let val: ByteBuf = self.receive()?;
                    val.to_vec()
                }
                Response::NotFound => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Not found.",
                    )));
                }
                Response::NotUnique(list) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Multiple data found: {}", list.join(", ")),
                    )));
                }
            };
            list.push(binary);
        }
        Ok(list)
    }

    /// Relabels a bulletin.
    pub fn relabel(
        &mut self,
        title_from: &str,
        tag_from: Option<&str>,
        title_to: Option<&str>,
        tag_to: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::Relabel, &mut buffer)?;
        ciborium::into_writer(
            &(
                title_from.to_string(),
                tag_from.map(|x| x.to_string()),
                title_to.map(|x| x.to_string()),
                tag_to.map(|x| x.to_string()),
            ),
            &mut buffer,
        )?;
        self.send(buffer)?;
        Ok(())
    }

    /// Returns the version of the server.
    pub fn server_version(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::Version, &mut buffer)?;
        self.send(buffer)?;
        let version: String = self.receive()?;
        Ok(version)
    }

    /// Returns the status of the server.
    ///
    /// The return values are (total datasize (bytes), memory used (bytes), memory used (%), the number of objects, the number of objects backed by files, the number of archived objects)
    ///
    /// The total datasize does not include the size of metadata such as timestamp.
    pub fn status(&mut self) -> Result<(u64, u64, f64, u64, u64, u64), Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::Status, &mut buffer)?;
        self.send(buffer)?;
        let status: (u64, u64, f64, u64, u64, u64) = self.receive()?;
        Ok(status)
    }

    /// Returns the log of the server.
    pub fn log(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::Log, &mut buffer)?;
        self.send(buffer)?;
        let log: String = self.receive()?;
        Ok(log)
    }

    /// Returns the list of the bulletins.
    pub fn view_board(&mut self) -> Result<Vec<(String, String, u64)>, Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::ViewBoard, &mut buffer)?;
        self.send(buffer)?;
        let list: Vec<(String, String, u64)> = self.receive()?;
        Ok(list)
    }

    /// Returns the details of a bulletin. The return values are a vector of (revision number, datasize (bytes), timestamp, backend).
    pub fn get_info(
        &mut self,
        title: &str,
        tag: Option<&str>,
    ) -> Result<Vec<(u64, u64, String, String)>, Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::GetInfo, &mut buffer)?;
        ciborium::into_writer(
            &(title.to_string(), tag.map(|x| x.to_string())),
            &mut buffer,
        )?;
        self.send(buffer)?;
        let res = self.receive()?;
        match res {
            Response::Ok => {
                let list: Vec<(u64, u64, String, String)> = self.receive()?;
                Ok(list)
            }
            Response::NotFound => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Not found.",
            ))),
            Response::NotUnique(list) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Multiple data found: {}", list.join(", ")),
            ))),
        }
    }

    /// Deletes specific revisions from a bulletin.
    pub fn clear_revisions(
        &mut self,
        title: &str,
        tag: Option<&str>,
        revisions: Vec<u64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::ClearRevisions, &mut buffer)?;
        ciborium::into_writer(
            &(title.to_string(), tag.map(|x| x.to_string()), revisions),
            &mut buffer,
        )?;
        self.send(buffer)?;
        Ok(())
    }

    /// Removes all the revisions and the database entry of a bulletin.
    pub fn remove(
        &mut self,
        title: &str,
        tag: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::Remove, &mut buffer)?;
        ciborium::into_writer(
            &(title.to_string(), tag.map(|x| x.to_string())),
            &mut buffer,
        )?;
        self.send(buffer)?;
        Ok(())
    }

    /// Moves a bulletin to a persistent archive.
    pub fn archive(
        &mut self,
        acv_name: &str,
        title: &str,
        tag: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::Archive, &mut buffer)?;
        ciborium::into_writer(
            &(
                acv_name.to_string(),
                title.to_string(),
                tag.map(|x| x.to_string()),
            ),
            &mut buffer,
        )?;
        self.send(buffer)?;
        Ok(())
    }

    /// Loads or reloads an archive. The data is directly read from the archive file and a suffix "acv_name:" is added to the tag.
    pub fn load(&mut self, acv_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::Load, &mut buffer)?;
        ciborium::into_writer(&acv_name.to_string(), &mut buffer)?;
        self.send(buffer)?;
        Ok(())
    }

    /// Shows the list of archive.
    pub fn list_archive(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::ListArchive, &mut buffer)?;
        self.send(buffer)?;
        let list: Vec<String> = self.receive()?;
        Ok(list)
    }

    /// Renames an archive. This will be applied after calling reset_server.
    pub fn rename_archive(
        &mut self,
        name_from: &str,
        name_to: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::RenameArchive, &mut buffer)?;
        ciborium::into_writer(&(name_from.to_string(), name_to.to_string()), &mut buffer)?;
        self.send(buffer)?;
        Ok(())
    }

    /// Deletes an archive. This will be applied after after calling reset_server.
    pub fn delete_archive(&mut self, acv_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::DeleteArchive, &mut buffer)?;
        ciborium::into_writer(&acv_name.to_string(), &mut buffer)?;
        self.send(buffer)?;
        Ok(())
    }

    /// Dumps all the unarchived data into an archive.
    pub fn dump(&mut self, acv_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::Dump, &mut buffer)?;
        ciborium::into_writer(&acv_name.to_string(), &mut buffer)?;
        self.send(buffer)?;
        Ok(())
    }

    /// Delete all the temporary data and restores data from an archive. Each data is copied to memory or a separate file. No suffix is added to the tag.
    pub fn restore(&mut self, acv_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::Restore, &mut buffer)?;
        ciborium::into_writer(&acv_name.to_string(), &mut buffer)?;
        self.send(buffer)?;
        Ok(())
    }

    /// Clears the log file of the server.
    pub fn clear_log(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::ClearLog, &mut buffer)?;
        self.send(buffer)?;
        Ok(())
    }

    /// Resets and clears the data. The archived data is not affected, but must be loaded before use.
    pub fn reset_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::Reset, &mut buffer)?;
        self.send(buffer)?;
        Ok(())
    }

    /// Terminates the server.
    pub fn terminate_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Cursor::new(vec![]);
        ciborium::into_writer(&Operation::Terminate, &mut buffer)?;
        self.send(buffer)?;
        Ok(())
    }
}
