//! # Common code for Bulletin Board
use serde::{Deserialize, Serialize};

/// Command sent to the server.
#[derive(Serialize, Deserialize)]
pub enum Operation {
    Post,
    Read,
    Relabel,
    Version,
    Status,
    Log,
    ViewBoard,
    GetInfo,
    ClearRevisions,
    Remove,
    Archive,
    Load,
    ListArchive,
    RenameArchive,
    DeleteArchive,
    Dump,
    Restore,
    ClearLog,
    Reset,
    Terminate,
}

/// Response from the server.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Response {
    Ok,
    NotFound,
    NotUnique(Vec<String>),
}

impl Default for Response {
    fn default() -> Self {
        Response::Ok
    }
}
