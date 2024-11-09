use std::io::Cursor;
use std::net::TcpListener;

#[cfg(target_family = "unix")]
use std::os::unix::net::UnixListener;

use crate::board::BulletinBoard;
use crate::bulletin::Bulletin;
use crate::error::{ArchiveError, BulletinError};
use crate::logging;
use crate::{
    ACV_DIR, DEBUG, FILE_THRETHOLD, LISTEN_ADDR, LOG_FILE, LOG_LEVEL, TMP_DIR, TOT_MEM_LIMIT,
};
use bulletin_board_common::*;
use serde_bytes::ByteBuf;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::LazyLock;

pub struct ServerOptions {
    debug: bool,
    listen_addr: Option<String>,
    tmp_dir: Option<String>,
    acv_dir: Option<String>,
    tot_mem_limit: Option<String>,
    file_threshold: Option<String>,
    log_file: Option<String>,
    log_level: Option<u8>,
}

impl ServerOptions {
    pub fn new() -> Self {
        Self {
            debug: false,
            listen_addr: None,
            tmp_dir: None,
            acv_dir: None,
            tot_mem_limit: None,
            file_threshold: None,
            log_file: None,
            log_level: None,
        }
    }
    pub fn set_debug(&mut self) {
        self.debug = true;
    }
    pub fn set_listen_addr(&mut self, listen_addr: String) {
        self.listen_addr = Some(listen_addr);
    }
    pub fn set_tmp_dir(&mut self, tmp_dir: String) {
        self.tmp_dir = Some(tmp_dir);
    }
    pub fn set_acv_dir(&mut self, acv_dir: String) {
        self.acv_dir = Some(acv_dir);
    }
    pub fn set_tot_mem_limit(&mut self, tot_mem_limit: String) {
        self.tot_mem_limit = Some(tot_mem_limit);
    }
    pub fn set_file_threshold(&mut self, file_threshold: String) {
        self.file_threshold = Some(file_threshold);
    }
    pub fn set_log_file(&mut self, log_file: String) {
        self.log_file = Some(log_file);
    }
    pub fn set_log_level(&mut self, log_level: u8) {
        self.log_level = Some(log_level);
    }
    pub fn load_options(&self) {
        if self.debug {
            env::set_var("BB_DEBUG", "");
        }
        if let Some(listen_addr) = &self.listen_addr {
            env::set_var("BB_LISTEN_ADDR", listen_addr);
        }
        if let Some(tmp_dir) = &self.tmp_dir {
            env::set_var("BB_TMP_DIR", tmp_dir);
        }
        if let Some(acv_dir) = &self.acv_dir {
            env::set_var("BB_ACV_DIR", acv_dir);
        }
        if let Some(tot_mem_limit) = &self.tot_mem_limit {
            env::set_var("BB_TOT_MEM_LIMIT", tot_mem_limit);
        }
        if let Some(file_threshold) = &self.file_threshold {
            env::set_var("BB_FILE_THRETHOLD", file_threshold);
        }
        if let Some(log_file) = &self.log_file {
            env::set_var("BB_LOG_FILE", log_file);
        }
        if let Some(log_level) = &self.log_level {
            env::set_var("BB_LOG_LEVEL", log_level.to_string());
        }
        LazyLock::force(&DEBUG);
        LazyLock::force(&LISTEN_ADDR);
        LazyLock::force(&TMP_DIR);
        LazyLock::force(&ACV_DIR);
        LazyLock::force(&TOT_MEM_LIMIT);
        LazyLock::force(&FILE_THRETHOLD);
        LazyLock::force(&LOG_FILE);
        LazyLock::force(&LOG_LEVEL);
    }
}

pub struct BBServer {
    bulletinboard: BulletinBoard,
    archive_manipulations: Vec<(String, Option<String>)>,
}

impl BBServer {
    pub fn new() -> Result<Self, std::io::Error> {
        if *LOG_LEVEL == 5 {
            logging::warn("Server is running in verbose mode.".to_string());
        }
        Ok(Self {
            bulletinboard: BulletinBoard::new()?,
            archive_manipulations: vec![],
        })
    }
    pub fn listen(&mut self) -> Result<(), std::io::Error> {
        #[cfg(not(target_family = "unix"))]
        self.listen_tcp()?;
        #[cfg(target_family = "unix")]
        {
            let re = regex::Regex::new(":[0-9]+$").unwrap();
            if re.is_match(&*LISTEN_ADDR) {
                self.listen_tcp()?;
            } else {
                self.listen_unix()?;
            }
        }
        Ok(())
    }
    fn listen_tcp(&mut self) -> Result<(), std::io::Error> {
        {
            let version = env!("CARGO_PKG_VERSION");
            let message = format!("Bulletin Board Server v{version} started.");
            logging::notice(message);

            let message = format!("Listening on TCP socket: {}.", &*LISTEN_ADDR);
            logging::info(message);
        }
        let listener = TcpListener::bind(&*LISTEN_ADDR)?;
        for stream in listener.incoming() {
            let stream = stream?;
            match self.process(stream) {
                Ok(exit) => {
                    if exit {
                        break;
                    }
                }
                Err(err) => {
                    let err = Box::leak(err);
                    logging::error(err.to_string());
                }
            }
        }
        Ok(())
    }
    #[cfg(target_family = "unix")]
    fn listen_unix(&mut self) -> Result<(), std::io::Error> {
        if std::path::Path::new(&*LISTEN_ADDR).exists() {
            std::fs::remove_file(&*LISTEN_ADDR)?;
        }
        {
            let version = env!("CARGO_PKG_VERSION");
            let message = format!("Bulletin Board Server v{version} started.");
            logging::notice(message);

            let message = format!("Listening on Unix socket: {}.", &*LISTEN_ADDR);
            logging::info(message);
        }
        let listener = UnixListener::bind(&*LISTEN_ADDR)?;
        for stream in listener.incoming() {
            let stream = stream?;
            match self.process(stream) {
                Ok(exit) => {
                    if exit {
                        break;
                    }
                }
                Err(err) => {
                    let err = Box::leak(err);
                    logging::error(err.to_string());
                }
            }
        }
        Ok(())
    }
    fn process<S: std::io::Read + std::io::Write>(
        &mut self,
        mut stream: S,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        while let Ok(operation) = ciborium::from_reader(&mut stream) {
            match operation {
                Operation::Post => {
                    self.post(&mut stream)?;
                }
                Operation::Read => {
                    self.read(&mut stream)?;
                }
                Operation::Relabel => {
                    self.relabel(&mut stream)?;
                }
                Operation::Version => {
                    self.version(&mut stream)?;
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
                Operation::ClearRevisions => {
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
                    self.reset()?;
                    self.restore(&mut stream)?;
                }
                Operation::ClearLog => {
                    self.clear_log()?;
                }
                Operation::Reset => {
                    self.reset()?;
                }
                Operation::Terminate => {
                    self.reset()?;
                    return Ok(true);
                }
            };
        }
        Ok(false)
    }
    fn get_tag<S: std::io::Read + std::io::Write>(
        &self,
        operation: &str,
        title: &String,
        tag: Option<String>,
        stream: Option<&mut S>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match tag {
            Some(tag) => Ok(tag),
            None => {
                let tags = self.bulletinboard.find_tags(title);
                match tags.len() {
                    0 => {
                        if let Some(stream) = stream {
                            ciborium::into_writer(&Response::NotFound, stream)?;
                        }
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
                        if let Some(stream) = stream {
                            ciborium::into_writer(&Response::NotUnique(tags.clone()), stream)?;
                        }
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
    fn post<S: std::io::Read + std::io::Write>(
        &mut self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (title, tag, data): (String, String, ByteBuf) = ciborium::from_reader(&mut *stream)?;
        logging::debug(format!("(post) title: {title}, tag: {tag}."));
        let bulletin = Bulletin::from_data(data.to_vec());
        self.bulletinboard
            .post(title.clone(), tag.clone(), bulletin)
            .map_err(|err| BulletinError::new("post", err.to_string(), title, tag, None))?;
        Ok(())
    }
    fn read<S: std::io::Read + std::io::Write>(
        &mut self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (title, tag, revisions): (String, Option<String>, Vec<u64>) =
            ciborium::from_reader(&mut *stream)?;
        logging::debug(format!("(read) title: {title}, tag: {tag:?}."));
        let tag = self.get_tag("read", &title, tag, Some(&mut *stream))?;
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
    fn relabel<S: std::io::Read + std::io::Write>(
        &mut self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (title_from, tag_from, title_to, tag_to): (String, Option<String>, Option<String>, Option<String>) = ciborium::from_reader(&mut *stream)?;
        logging::debug(format!("(relabel) title_from: {title_from}, tag_from: {tag_from:?}, title_to: {title_to:?}, tag_to: {tag_to:?}."));
        let tag_from = self.get_tag("read", &title_from, tag_from, Some(&mut *stream))?;
        self.bulletinboard.relabel(title_from, tag_from, title_to, tag_to)?;
        Ok(())
    }
    fn version<S: std::io::Read + std::io::Write>(
        &self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        logging::debug(format!("(version)."));
        let version = env!("CARGO_PKG_VERSION").to_string();
        ciborium::into_writer(&version, stream)?;
        Ok(())
    }
    fn status<S: std::io::Read + std::io::Write>(
        &self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        logging::debug(format!("(status)."));
        let status = self.bulletinboard.status();
        ciborium::into_writer(&status, stream)?;
        Ok(())
    }
    fn log<S: std::io::Read + std::io::Write>(
        &self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        logging::debug(format!("(log)."));
        let log = if Path::new(&*LOG_FILE).exists() {
            std::fs::read_to_string(&*LOG_FILE)?
        } else {
            "No logs yet.\n".to_string()
        };
        ciborium::into_writer(&log, stream)?;
        Ok(())
    }
    fn view_board<S: std::io::Read + std::io::Write>(
        &self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        logging::debug(format!("(view_board)."));
        let board = self.bulletinboard.view();
        ciborium::into_writer(&board, stream)?;
        Ok(())
    }
    fn get_info<S: std::io::Read + std::io::Write>(
        &self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (title, tag): (String, Option<String>) = ciborium::from_reader(&mut *stream)?;
        logging::debug(format!("(get_info) title: {title}, tag: {tag:?}."));
        let tag = self.get_tag("get_info", &title, tag, Some(&mut *stream))?;
        match self.bulletinboard.get_info(title.clone(), tag.clone()) {
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
    fn clear_revisions<S: std::io::Read + std::io::Write>(
        &mut self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (title, tag, revisions): (String, Option<String>, Vec<u64>) =
            ciborium::from_reader(stream)?;
        logging::debug(format!(
            "(clear_revisions) title: {title}, tag: {tag:?}, revisions: {revisions:?}."
        ));
        let tag = self.get_tag("clear_revisions", &title, tag, None::<&mut S>)?;
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
    fn remove<S: std::io::Read + std::io::Write>(
        &mut self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (title, tag): (String, Option<String>) = ciborium::from_reader(stream)?;
        logging::debug(format!("(remove) title: {title}, tag: {tag:?}."));
        let tag = self.get_tag("remove", &title, tag, None::<&mut S>)?;
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
    fn archive<S: std::io::Read + std::io::Write>(
        &mut self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (acv_name, title, tag): (String, String, Option<String>) =
            ciborium::from_reader(stream)?;
        logging::debug(format!(
            "(arvhive) archive_name: {acv_name}, title: {title}, tag: {tag:?}."
        ));
        if acv_name.len() == 0 {
            return Err(Box::new(ArchiveError::new(
                "archive",
                "Wrong archive name.".to_string(),
                acv_name.clone(),
            )));
        }
        let tag = self.get_tag("archive", &title, tag, None::<&mut S>)?;
        self.bulletinboard
            .archive(acv_name, title.clone(), tag.clone())
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
    fn load<S: std::io::Read + std::io::Write>(
        &mut self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let acv_name: String = ciborium::from_reader(stream)?;
        logging::debug(format!("(load) archive_name: {acv_name}."));
        if acv_name.len() == 0 {
            return Err(Box::new(ArchiveError::new(
                "load",
                "Wrong archive name.".to_string(),
                acv_name.clone(),
            )));
        }
        self.bulletinboard
            .load(acv_name.clone())
            .map_err(|err| ArchiveError::new("load", err.to_string(), acv_name))?;
        Ok(())
    }
    fn list_archive<S: std::io::Read + std::io::Write>(
        &self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        logging::debug(format!("(list_archive)."));
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
    fn rename_archive<S: std::io::Read + std::io::Write>(
        &mut self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (acv_from, acv_to): (String, String) = ciborium::from_reader(stream)?;
        logging::debug(format!("(rename_archive) from: {acv_from}, to: {acv_to}."));
        if acv_from.len() == 0 {
            return Err(Box::new(ArchiveError::new(
                "rename_archive",
                "Wrong archive name.".to_string(),
                acv_from.clone(),
            )));
        }
        if acv_to.len() == 0 {
            return Err(Box::new(ArchiveError::new(
                "rename_archive",
                "Wrong archive name.".to_string(),
                acv_to.clone(),
            )));
        }
        self.archive_manipulations.push((acv_from, Some(acv_to)));
        Ok(())
    }
    fn delete_archive<S: std::io::Read + std::io::Write>(
        &mut self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let acv_name: String = ciborium::from_reader(stream)?;
        logging::debug(format!("(delete_archive) archive_name: {acv_name}."));
        if acv_name.len() == 0 {
            return Err(Box::new(ArchiveError::new(
                "delete_archive",
                "Wrong archive name.".to_string(),
                acv_name.clone(),
            )));
        }
        self.archive_manipulations.push((acv_name, None));
        Ok(())
    }
    fn dump<S: std::io::Read + std::io::Write>(
        &mut self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let acv_name: String = ciborium::from_reader(stream)?;
        logging::debug(format!("(dump) archive_name: {acv_name}."));
        if acv_name.len() == 0 {
            return Err(Box::new(ArchiveError::new(
                "dump",
                "Wrong archive name.".to_string(),
                acv_name.clone(),
            )));
        }
        self.bulletinboard.dump(acv_name)?;
        Ok(())
    }
    fn restore<S: std::io::Read + std::io::Write>(
        &mut self,
        stream: &mut S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let acv_name: String = ciborium::from_reader(stream)?;
        logging::debug(format!("(restore) archive_name: {acv_name}."));
        if acv_name.len() == 0 {
            return Err(Box::new(ArchiveError::new(
                "restore",
                "Wrong archive name.".to_string(),
                acv_name.clone(),
            )));
        }
        self.bulletinboard
            .restore(acv_name.clone())
            .map_err(|err| ArchiveError::new("restore", err.to_string(), acv_name))?;
        Ok(())
    }
    fn clear_log(&self) -> Result<(), Box<dyn std::error::Error>> {
        logging::debug(format!("(clear_log)."));
        if Path::new(&*LOG_FILE).exists() {
            fs::remove_file(&*LOG_FILE)?;
        }
        Ok(())
    }
    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        logging::debug(format!("(restore/reset/exit)."));
        self.bulletinboard.reset()?;
        for (name_from, name_to) in self.archive_manipulations.drain(..) {
            match name_to {
                Some(name_to) => {
                    self.bulletinboard
                        .rename_archive(name_from.clone(), name_to.clone())?;
                    logging::info(format!("Moved archive: {name_from} => {name_to}."));
                }
                None => {
                    self.bulletinboard.delete_archive(name_from.clone())?;
                    logging::info(format!("Deleted archive: {name_from}."));
                }
            }
        }
        logging::notice("Server restarted.".to_string());
        Ok(())
    }
}
