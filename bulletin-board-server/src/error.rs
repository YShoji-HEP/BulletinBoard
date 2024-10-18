use std::error::Error;
use std::fmt::{Debug, Display};

pub struct BulletinError {
    operation: String,
    message: String,
    title: String,
    tag: String,
    revision: Option<u64>,
}

impl BulletinError {
    pub fn new(
        operation: &str,
        message: String,
        title: String,
        tag: String,
        revision: Option<u64>,
    ) -> Self {
        Self {
            operation: operation.to_string(),
            message,
            title,
            tag,
            revision,
        }
    }
}

impl Display for BulletinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.revision {
            Some(rev) => write!(
                f,
                "message: \"{}\", operation: {}, title: {}, tag: {}, revision: {}.",
                self.message, self.operation, self.title, self.tag, rev,
            ),
            None => write!(
                f,
                "message: \"{}\", operation: {}, title: {}, tag: {}.",
                self.message, self.operation, self.title, self.tag,
            ),
        }
    }
}

impl Debug for BulletinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for BulletinError {}

pub struct ArchiveError {
    operation: String,
    message: String,
    acv_name: String,
}

impl ArchiveError {
    pub fn new(operation: &str, message: String, acv_name: String) -> Self {
        Self {
            operation: operation.to_string(),
            message,
            acv_name,
        }
    }
}

impl Display for ArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "message: \"{}\", operation: {}, archive: {}.",
            self.message, self.operation, self.acv_name
        )
    }
}

impl Debug for ArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ArchiveError {}
