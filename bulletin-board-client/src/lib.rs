//! # Bulletin Board Client
//! A rust client for the bulletin board.
//!
//! # Examples
//! To post and read the bulletins,
//! ```
//! use bulletin_board_client as bbclient;
//! use bbclient::*;
//!
//! let data: ArrayObject = vec![1f32, 2., -3., 5.].try_into().unwrap();
//! bbclient::post("x", "tag", data.clone()).unwrap();
//!
//! let recv = bbclient::read("x", Some("tag"), vec![]).unwrap().pop().unwrap();
//! let restored = recv.try_into().unwrap();
//! assert_eq!(data, restored);
//! ```
//!
//! Make the data persistent.
//! ```
//! use bulletin_board_client as bbclient;
//!
//! bbclient::archive("acv", "x", Some("tag")).unwrap();
//! bbclient::reset_server().unwrap(); // Delete all temporary data.
//!
//! bbclient::load("acv").unwrap();
//! dbg!(bbclient::view_board().unwrap());
//! ```
//!

/// Low-level functions that isolate the opening and closing functions of a socket. These can be used to speed up commucation with the server when you do many operations at the same time.
pub mod low_level;

pub use array_object::{adaptor, ArrayObject, DataType, Pack, Unpack};

use low_level::*;
use std::sync::{LazyLock, Mutex};

static ADDR: LazyLock<Mutex<String>> = LazyLock::new(|| {
    let addr = std::env::var("BB_ADDR").unwrap_or("127.0.0.1:7578".to_string());
    Mutex::new(addr)
});

/// Sets the server address.
pub fn set_addr(new_addr: &str) {
    let mut addr = ADDR.lock().unwrap();
    *addr = new_addr.to_string();
}

/// Posts an ArrayObject.
pub fn post(title: &str, tag: &str, obj: ArrayObject) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.post_raw(title, tag, obj.pack())?;
    Ok(())
}

/// Posts an ArrayObject without compression.
pub fn post_as_it_is(
    title: &str,
    tag: &str,
    obj: ArrayObject,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.post_raw(title, tag, obj.pack_as_it_is())?;
    Ok(())
}

/// Reads ArrayObjects.
///
/// Tag can be None if there is only one tag exists for the title.
/// When revisions is empty, the latest revision is returned.
pub fn read(
    title: &str,
    tag: Option<&str>,
    revisions: Vec<u64>,
) -> Result<Vec<ArrayObject>, Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    let list = stream.read_raw(title, tag, revisions)?;
    let mut objs = vec![];
    for data in list {
        objs.push(ArrayObject::unpack(data)?);
    }
    Ok(objs)
}

/// Relabels a bulletin.
pub fn relabel(
    title_from: &str,
    tag_from: Option<&str>,
    title_to: Option<&str>,
    tag_to: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.relabel(title_from, tag_from, title_to, tag_to)?;
    Ok(())
}

/// Returns the version of the server.
pub fn version() -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    let version = stream.version()?;
    Ok(version)
}

/// Returns the status of the server.
///
/// The return values are (total datasize (bytes), memory used (bytes), memory used (%), the number of objects, the number of objects backed by files, the number of archived objects)
///
/// The total datasize does not include the size of metadata such as timestamp.
pub fn status() -> Result<(u64, u64, f64, u64, u64, u64), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    let status = stream.status()?;
    Ok(status)
}

/// Returns the log of the server.
pub fn log() -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    let log = stream.log()?;
    Ok(log)
}

/// Returns the list of the bulletins.
pub fn view_board() -> Result<Vec<(String, String, u64)>, Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    let list = stream.view_board()?;
    Ok(list)
}

/// Returns the details of a bulletin. The return values are a vector of (revision number, datasize (bytes), timestamp, backend).
pub fn get_info(
    title: &str,
    tag: Option<&str>,
) -> Result<Vec<(u64, u64, String, String)>, Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    let list = stream.get_info(title, tag)?;
    Ok(list)
}

/// Deletes specific revisions from a bulletin.
pub fn clear_revisions(
    title: &str,
    tag: Option<&str>,
    revisions: Vec<u64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.clear_revisions(title, tag, revisions)?;
    Ok(())
}

/// Removes all the revisions and the database entry of a bulletin.
pub fn remove(title: &str, tag: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.remove(title, tag)?;
    Ok(())
}

/// Moves a bulletin to a persistent archive.
pub fn archive(
    acv_name: &str,
    title: &str,
    tag: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.archive(acv_name, title, tag)?;
    Ok(())
}

/// Loads or reloads an archive. The data is directly read from the archive file and a suffix "acv_name:" is added to the tag.
pub fn load(acv_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.load(acv_name)?;
    Ok(())
}

/// Shows the list of archive.
pub fn list_archive() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    let list = stream.list_archive()?;
    Ok(list)
}

/// Renames an archive. Applied after reset.
pub fn rename_archive(name_from: &str, name_to: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.rename_archive(name_from, name_to)?;
    Ok(())
}

/// Deletes an archive. Applied after reset.
pub fn delete_archive(acv_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.delete_archive(acv_name)?;
    Ok(())
}

/// Dumps all the unarchived data into an archive.
pub fn dump(acv_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.dump(acv_name)?;
    Ok(())
}

/// Delete all the temporary data and restores data from an archive. Each data is copied to memory or a separate file. No suffix is added to the tag.
pub fn restore(acv_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.restore(acv_name)?;
    Ok(())
}

/// Clears the log file of the server.
pub fn clear_log() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.clear_log()?;
    Ok(())
}

/// Resets and clears the data. The archived data is not affected, but must be loaded before use.
pub fn reset_server() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.reset_server()?;
    Ok(())
}

/// Terminates the server.
pub fn terminate_server() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpOrUnixStream::connect()?;
    stream.terminate_server()?;
    Ok(())
}
