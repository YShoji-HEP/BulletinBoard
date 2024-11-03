use crate::ADDR;
use bulletin_board_common::*;
use serde_bytes::ByteBuf;
use std::io::{self, Cursor};

#[cfg(not(feature = "unix"))]
use std::net::TcpStream as TcpOrUnixStream;
#[cfg(feature = "unix")]
use std::os::unix::net::UnixStream as TcpOrUnixStream;

/// Opens a TCP/UNIX socket. Notice that this blocks the server until the socket is closed.
pub fn connect() -> Result<TcpOrUnixStream, Box<dyn std::error::Error>> {
    let addr = ADDR.lock().unwrap();
    Ok(TcpOrUnixStream::connect(&*addr)?)
}

/// Posts binary of ArrayObject.
pub fn post_ll(
    stream: &mut TcpOrUnixStream,
    title: &str,
    tag: &str,
    binary: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let val = serde_bytes::ByteBuf::from(binary);
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::Post, &mut buffer)?;
    ciborium::into_writer(&(title.to_string(), tag.to_string(), val), &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, stream)?;
    Ok(())
}

/// Reads ArrayObject as binary.
///
/// When revisions is empty, the latest revision is returned.
pub fn read_ll(
    stream: &mut TcpOrUnixStream,
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
    buffer.set_position(0);
    io::copy(&mut buffer, &mut *stream)?;

    for _ in 0..revisions.len().max(1) {
        let res = ciborium::from_reader(&mut *stream)?;
        let binary = match res {
            Response::Ok => {
                let val: ByteBuf = ciborium::from_reader(&mut *stream)?;
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

/// Returns the version of the server.
pub fn version_ll(stream: &mut TcpOrUnixStream) -> Result<String, Box<dyn std::error::Error>> {
    ciborium::into_writer(&Operation::Version, &mut *stream)?;
    let version: String = ciborium::from_reader(stream)?;
    Ok(version)
}

/// Returns the status of the server.
///
/// The return values are (total datasize (bytes), memory used (bytes), memory used (%), the number of objects, the number of objects backed by files, the number of archived objects)
///
/// The total datasize does not include the size of metadata such as timestamp.
pub fn status_ll(
    stream: &mut TcpOrUnixStream,
) -> Result<(u64, u64, f64, u64, u64, u64), Box<dyn std::error::Error>> {
    ciborium::into_writer(&Operation::Status, &mut *stream)?;
    let status: (u64, u64, f64, u64, u64, u64) = ciborium::from_reader(stream)?;
    Ok(status)
}

/// Returns the log of the server.
pub fn log_ll(stream: &mut TcpOrUnixStream) -> Result<String, Box<dyn std::error::Error>> {
    ciborium::into_writer(&Operation::Log, &mut *stream)?;
    let log: String = ciborium::from_reader(stream)?;
    Ok(log)
}

/// Returns the list of the bulletins.
pub fn view_board_ll(
    stream: &mut TcpOrUnixStream,
) -> Result<Vec<(String, String, u64)>, Box<dyn std::error::Error>> {
    ciborium::into_writer(&Operation::ViewBoard, &mut *stream)?;
    let list: Vec<(String, String, u64)> = ciborium::from_reader(stream)?;
    Ok(list)
}

/// Returns the details of a bulletin. The return values are a vector of (revision number, datasize (bytes), timestamp, backend).
pub fn get_info_ll(
    stream: &mut TcpOrUnixStream,
    title: &str,
    tag: Option<&str>,
) -> Result<Vec<(u64, u64, String, String)>, Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::GetInfo, &mut buffer)?;
    ciborium::into_writer(
        &(title.to_string(), tag.map(|x| x.to_string())),
        &mut buffer,
    )?;
    buffer.set_position(0);
    io::copy(&mut buffer, &mut *stream)?;
    let res = ciborium::from_reader(&mut *stream)?;
    match res {
        Response::Ok => {
            let list: Vec<(u64, u64, String, String)> = ciborium::from_reader(stream)?;
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
pub fn clear_revisions_ll(
    stream: &mut TcpOrUnixStream,
    title: &str,
    tag: Option<&str>,
    revisions: Vec<u64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::ClearRevision, &mut buffer)?;
    ciborium::into_writer(
        &(title.to_string(), tag.map(|x| x.to_string()), revisions),
        &mut buffer,
    )?;
    buffer.set_position(0);
    io::copy(&mut buffer, stream)?;
    Ok(())
}

/// Removes all the revisions and the database entry of a bulletin.
pub fn remove_ll(
    stream: &mut TcpOrUnixStream,
    title: &str,
    tag: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::Remove, &mut buffer)?;
    ciborium::into_writer(
        &(title.to_string(), tag.map(|x| x.to_string())),
        &mut buffer,
    )?;
    buffer.set_position(0);
    io::copy(&mut buffer, stream)?;
    Ok(())
}

/// Moves a bulletin to a persistent archive.
pub fn archive_ll(
    stream: &mut TcpOrUnixStream,
    title: &str,
    tag: Option<&str>,
    acv_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::Archive, &mut buffer)?;
    ciborium::into_writer(
        &(
            title.to_string(),
            tag.map(|x| x.to_string()),
            acv_name.to_string(),
        ),
        &mut buffer,
    )?;
    buffer.set_position(0);
    io::copy(&mut buffer, stream)?;
    Ok(())
}

/// Loads an archive. The data is directly read from the archive file and a suffix "acv_name:" is added to the tag. Multiple loads will result in multiple entries of the same data.
pub fn load_ll(
    stream: &mut TcpOrUnixStream,
    acv_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::Load, &mut buffer)?;
    ciborium::into_writer(&acv_name.to_string(), &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, stream)?;
    Ok(())
}

/// Shows the list of archive.
pub fn list_archive_ll(
    stream: &mut TcpOrUnixStream,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    ciborium::into_writer(&Operation::ListArchive, &mut *stream)?;
    let list: Vec<String> = ciborium::from_reader(stream)?;
    Ok(list)
}

/// Renames an archive. Applied after reset.
pub fn rename_archive_ll(
    stream: &mut TcpOrUnixStream,
    name_from: &str,
    name_to: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::RenameArchive, &mut buffer)?;
    ciborium::into_writer(&(name_from.to_string(), name_to.to_string()), &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, stream)?;
    Ok(())
}

/// Deletes an archive. Applied after reset.
pub fn delete_archive_ll(
    stream: &mut TcpOrUnixStream,
    acv_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::DeleteArchive, &mut buffer)?;
    ciborium::into_writer(&acv_name.to_string(), &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, stream)?;
    Ok(())
}

/// Dumps all the unarchived data into an archive.
pub fn dump_ll(
    stream: &mut TcpOrUnixStream,
    acv_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::Dump, &mut buffer)?;
    ciborium::into_writer(&acv_name.to_string(), &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, stream)?;
    Ok(())
}

/// Restores the data from an archive. Each data is copied to memory or a separate file. No suffix is added to the tag. If the same name and tag exists, the entries are added as new revisions.
pub fn restore_ll(
    stream: &mut TcpOrUnixStream,
    acv_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(vec![]);
    ciborium::into_writer(&Operation::Restore, &mut buffer)?;
    ciborium::into_writer(&acv_name.to_string(), &mut buffer)?;
    buffer.set_position(0);
    io::copy(&mut buffer, stream)?;
    Ok(())
}

/// Clears the log file of the server.
pub fn clear_log_ll(stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
    ciborium::into_writer(&Operation::ClearLog, stream)?;
    Ok(())
}

/// Resets and clears the data. The archived data is not affected, but must be loaded before use.
pub fn reset_server_ll(stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
    ciborium::into_writer(&Operation::Reset, stream)?;
    Ok(())
}

/// Terminates the server.
pub fn terminate_server_ll(stream: &mut TcpOrUnixStream) -> Result<(), Box<dyn std::error::Error>> {
    ciborium::into_writer(&Operation::Terminate, stream)?;
    Ok(())
}
