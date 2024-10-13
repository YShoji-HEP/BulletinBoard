//! # Bulletin Board Client
//! A rust client for the bulletin board.
//! 
//! # Examples
//! To post and read the bulletins, 
//! ```
//! use bulletin_board_client as bbclient;
//! use array_object::*;
//!
//! fn main() {
//!     let data: ArrayObject = vec![1f32, 2., -3., 5.].into();
//!     bbclient::post("x".to_string(), "tag".to_string(), data.clone());
//!     let rcvd = bbclient::read("x".to_string());
//!     let restored = rcvd.unpack().unwrap();
//!     assert_eq!(data, restored);
//! }
//! ```
//!
//! Make the data persistent.
//! ```
//! use bulletin_board_client as bbclient;
//!
//! fn main() {
//!     bbclient::archive("x".to_string(),"tag".to_string(),"acv".to_string());
//!     bbclient::reset();
//!     bbclient::load("acv".to_string());
//!     dbg!(bbclient::view_board());
//! }
//! ```
//! 

#[cfg(not(feature = "unix"))]
use std::net::TcpStream as TcpOrUnixStream;
#[cfg(feature = "unix")]
use std::os::unix::net::UnixStream as TcpOrUnixStream;

pub use array_object::adaptor::*;
pub use array_object::{ArrayObject, DataType, Pack, Unpack};
use bulletin_board_common::*;
use serde_bytes::ByteBuf;
use std::io::{self, Cursor};
use std::sync::LazyLock;

static ADDR: LazyLock<String> = LazyLock::new(|| {
    #[cfg(not(feature = "unix"))]
    let addr = std::env::var("BB_ADDR").unwrap_or("127.0.0.1:7578".to_string());
    #[cfg(feature = "unix")]
    let addr = std::env::var("BB_ADDR").unwrap_or("/tmp/bb.sock".to_string());
    addr
});

/// Post an ArrayObject.
pub fn post(
    var_name: String,
    var_tag: String,
    obj: ArrayObject,
) -> Result<(), Box<dyn std::error::Error>> {
    let val = serde_bytes::ByteBuf::from(obj.pack());
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::Post, &mut buffer)?;
    ciborium::into_writer(&(var_name, var_tag, val), &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, &mut stream)?;
    Ok(())
}

/// Read ArrayObjects. When revisions is empty, the latest revision is returned.
pub fn read(
    var_name: String,
    var_tag: Option<String>,
    revisions: Vec<u64>,
) -> Result<Vec<ArrayObject>, Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    let mut list = vec![];
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::Read, &mut buffer)?;
    ciborium::into_writer(
        &(var_name.clone(), var_tag.clone(), revisions.clone()),
        &mut buffer,
    )?;
    buffer.set_position(0);
    io::copy(&mut buffer, &mut stream)?;
    for _ in 0..revisions.len().max(1) {
        let res = ciborium::from_reader(&mut stream)?;
        let bc = match res {
            Response::Ok => {
                let val: ByteBuf = ciborium::from_reader(&mut stream)?;
                ArrayObject::unpack(val.to_vec())?
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
        list.push(bc);
    }
    Ok(list)
}

/// See the status of the server. Returns (total datasize (bytes), memory used (bytes), memory used (%), the number of bulletins, the number of bulletins backed by files, the number of archived bulletins)
pub fn status() -> Result<(u64, u64, f64, u64, u64, u64), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    ciborium::into_writer(&Operation::Status, &mut stream)?;
    let status: (u64, u64, f64, u64, u64, u64) = ciborium::from_reader(&mut stream)?;
    Ok(status)
}

/// Returns the log of the server.
pub fn log() -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    ciborium::into_writer(&Operation::Log, &mut stream)?;
    let log: String = ciborium::from_reader(&mut stream)?;
    Ok(log)
}

/// See the list of the bulletins.
pub fn view_board() -> Result<Vec<(String, String, u64)>, Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    ciborium::into_writer(&Operation::ViewBoard, &mut stream)?;
    let list: Vec<(String, String, u64)> = ciborium::from_reader(&mut stream)?;
    Ok(list)
}

/// See the details of a bulletin. Returns a vector of (revision number, datasize (bytes), timestamp, backend).
pub fn get_info(
    var_name: String,
    var_tag: Option<String>,
) -> Result<Vec<(u64, u64, String, String)>, Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::GetInfo, &mut buffer)?;
    ciborium::into_writer(&(var_name, var_tag), &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, &mut stream)?;
    let res = ciborium::from_reader(&mut stream)?;
    match res {
        Response::Ok => {
            let list: Vec<(u64, u64, String, String)> = ciborium::from_reader(&mut stream)?;
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

/// Delete specific revisions from a bulletin.
pub fn clear_revisions(
    var_name: String,
    var_tag: String,
    revisions: Vec<u64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::ClearRevision, &mut buffer)?;
    ciborium::into_writer(&(var_name, var_tag, revisions), &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, &mut stream)?;
    Ok(())
}

/// Remove all the revisions and the database entry of a bulletin.
pub fn remove(var_name: String, var_tag: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::Remove, &mut buffer)?;
    ciborium::into_writer(&(var_name, var_tag), &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, &mut stream)?;
    Ok(())
}

/// Move the bulletins to a persistent archive. A suffix "acv_name:" is added to the tag.
pub fn archive(
    var_name: String,
    var_tag: String,
    acv_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::Archive, &mut buffer)?;
    ciborium::into_writer(&(var_name, var_tag, acv_name), &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, &mut stream)?;
    Ok(())
}

/// Load an archive. The data is directly read from the archive file and a suffix "acv_name:" is added to the tag. Multiple loads will result in multiple entries of the same data.
pub fn load(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::Load, &mut buffer)?;
    ciborium::into_writer(&name, &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, &mut stream)?;
    Ok(())
}

/// Show the list of archive.
pub fn list_archive() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    ciborium::into_writer(&Operation::ListArchive, &mut stream)?;
    let list: Vec<String> = ciborium::from_reader(&mut stream)?;
    Ok(list)
}

/// Rename an archive. Applied after reset.
pub fn rename_archive(
    name_from: String,
    name_to: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::RenameArchive, &mut buffer)?;
    ciborium::into_writer(&(name_from, name_to), &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, &mut stream)?;
    Ok(())
}

/// Delete an archive. Applied after reset.
pub fn delete_archive(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::DeleteArchive, &mut buffer)?;
    ciborium::into_writer(&name, &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, &mut stream)?;
    Ok(())
}

/// Dump all the unarchived data into an archive.
pub fn dump(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::Dump, &mut buffer)?;
    ciborium::into_writer(&name, &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, &mut stream)?;
    Ok(())
}

/// Restore the data from an archive. Each data is copied to memory or a separate file. No suffix is added to the tag. If the same name and tag exists, the entries are added as new revisions.
pub fn restore(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::Restore, &mut buffer)?;
    ciborium::into_writer(&name, &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, &mut stream)?;
    Ok(())
}

/// Reset and clear the data. The archived data is not affected, but must be loaded before use.
pub fn reset() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect(&*ADDR)?;
    ciborium::into_writer(&Operation::Reset, &mut stream)?;
    Ok(())
}
