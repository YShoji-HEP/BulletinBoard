//! # Common code for Bulletin Board
use serde::{Deserialize, Serialize};

/// Command sent to the server.
#[derive(Serialize, Deserialize)]
pub enum Operation {
    Post,
    Read,
    Status,
    Log,
    ViewBoard,
    GetInfo,
    ClearRevision,
    Remove,
    Archive,
    Load,
    ListArchive,
    RenameArchive,
    DeleteArchive,
    Dump,
    Restore,
    Reset,
}

/// Response from the server.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Response {
    Ok,
    NotFound,
    NotUnique(Vec<String>),
}