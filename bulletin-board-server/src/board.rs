use crate::bulletin::{Bulletin, BulletinBackend};
use crate::{ACV_DIR, FILE_THRETHOLD, TMP_DIR, TOT_MEM_LIMIT};
use chrono::DateTime;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Cursor, Seek, SeekFrom};
use std::os::unix::fs::FileExt;
use std::path::Path;

pub struct BulletinBoard {
    datasize: u64,
    memory_used: u64,
    n_bulletins: u64,
    n_files: u64,
    n_archives: u64,
    bulletins: HashMap<(String, String), Vec<Bulletin>>,
}

impl BulletinBoard {
    pub fn new() -> Result<Self, std::io::Error> {
        if Path::new(&*TMP_DIR).exists() {
            fs::remove_dir_all(&*TMP_DIR)?;
        }
        Ok(Self {
            memory_used: 0,
            datasize: 0,
            n_bulletins: 0,
            n_files: 0,
            n_archives: 0,
            bulletins: HashMap::new(),
        })
    }
    pub fn post(
        &mut self,
        var_name: String,
        var_tag: String,
        mut bulletin: Bulletin,
    ) -> Result<(), std::io::Error> {
        let key = (var_name, var_tag);
        if bulletin.datasize < *FILE_THRETHOLD
            && self.memory_used + bulletin.datasize < *TOT_MEM_LIMIT
        {
            self.memory_used += bulletin.datasize;
        } else {
            bulletin.save_to_file()?;
            self.n_files += 1;
        }
        self.n_bulletins += 1;
        self.datasize += bulletin.datasize;
        let entry = self.bulletins.entry(key).or_default();
        entry.push(bulletin);
        Ok(())
    }
    pub fn take(&mut self, var_name: String, var_tag: String) -> Option<&mut Vec<Bulletin>> {
        self.bulletins.get_mut(&(var_name, var_tag))
    }
    pub fn find_tags(&self, var_name: &String) -> Vec<String> {
        self.bulletins
            .keys()
            .filter(|key| key.0 == *var_name)
            .map(|key| key.1.clone())
            .collect()
    }
    pub fn status(&self) -> (u64, u64, f64, u64, u64, u64) {
        (
            self.datasize,
            self.memory_used,
            self.memory_used as f64 / *TOT_MEM_LIMIT as f64,
            self.n_bulletins,
            self.n_files,
            self.n_archives,
        )
    }
    pub fn view(&self) -> Vec<(String, String, u64)> {
        self.bulletins
            .iter()
            .map(|((var_name, var_tag), v)| (var_name.clone(), var_tag.clone(), v.len() as u64))
            .collect()
    }
    pub fn get_info(
        &self,
        var_name: String,
        var_tag: String,
    ) -> Option<Vec<(u64, u64, String, String)>> {
        Some(
            self.bulletins
                .get(&(var_name, var_tag))?
                .iter()
                .enumerate()
                .map(|(i, val)| {
                    (
                        i as u64,
                        val.datasize,
                        val.timestamp.to_string(),
                        val.backend(),
                    )
                })
                .collect(),
        )
    }
    pub fn clear_revisions(
        &mut self,
        var_name: String,
        var_tag: String,
        revisions: Vec<u64>,
    ) -> Result<(), std::io::Error> {
        let list = self
            .bulletins
            .get_mut(&(var_name, var_tag))
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Not found.",
            ))?;
        for revision in revisions {
            let bulletin =
                list.get_mut::<usize>(revision.try_into().unwrap())
                    .ok_or(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Not found.",
                    ))?;
            let (datasize, mem_size, n_file) = bulletin.clear()?;
            self.datasize -= datasize;
            self.n_bulletins -= 1;
            self.n_files -= n_file;
            self.memory_used -= mem_size;
        }
        Ok(())
    }
    pub fn remove(&mut self, var_name: String, var_tag: String) -> Result<(), std::io::Error> {
        match self.bulletins.remove(&(var_name, var_tag)) {
            Some(mut bulletins) => {
                for bulletin in &mut bulletins {
                    let (datasize, mem_size, n_file) = bulletin.clear()?;
                    self.datasize -= datasize;
                    self.n_bulletins -= 1;
                    self.n_files -= n_file;
                    self.memory_used -= mem_size;
                }
                Ok(())
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Not found.",
            )),
        }
    }
    pub fn archive(
        &mut self,
        var_name: String,
        var_tag: String,
        acv_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.bulletins.remove(&(var_name.clone(), var_tag.clone())) {
            Some(mut rev_list) => {
                let dir = format!("{}/{}", *ACV_DIR, acv_name);
                if !Path::new(&dir).exists() {
                    fs::create_dir_all(&dir)?;
                    std::fs::write(dir.clone() + "/version.txt", env!("CARGO_PKG_VERSION"))?;
                }
                let filename_data = dir.clone() + "/data.bin";
                let filename_meta = dir + "/meta.bin";
                let mut file_data = File::options()
                    .write(true)
                    .create(true)
                    .truncate(false)
                    .open(&filename_data)?;
                let mut file_meta = File::options()
                    .write(true)
                    .create(true)
                    .truncate(false)
                    .open(&filename_meta)?;
                file_data.seek(SeekFrom::End(0))?;
                file_meta.seek(SeekFrom::End(0))?;
                let mut temp = vec![];
                let mut buffer = Cursor::new(vec![]);
                for bulletin in &mut rev_list {
                    match &mut bulletin.data {
                        BulletinBackend::Archive(_) => {}
                        BulletinBackend::Empty => {}
                        _ => {
                            let offset = file_data.stream_position()?;
                            io::copy(bulletin, &mut file_data)?;
                            ciborium::into_writer(
                                &(
                                    offset,
                                    bulletin.datasize,
                                    bulletin.timestamp.timestamp_nanos_opt().unwrap(),
                                ),
                                &mut buffer,
                            )?;
                            temp.push(Bulletin::from_archive(
                                &acv_name,
                                offset,
                                bulletin.datasize,
                                bulletin.timestamp,
                            ));
                            let (_, mem_size, n_file) = bulletin.clear()?;
                            self.n_files -= n_file;
                            self.memory_used -= mem_size;
                            self.n_archives += 1;
                        }
                    }
                }
                if !temp.is_empty() {
                    ciborium::into_writer(
                        &(var_name.clone(), var_tag.clone(), temp.len() as u64),
                        &mut file_meta,
                    )?;
                    buffer.set_position(0);
                    io::copy(&mut buffer, &mut file_meta)?;
                    let key = (var_name.clone(), format!("{acv_name}:{var_tag}"));
                    let entry = self.bulletins.entry(key).or_default();
                    for bulletin in temp {
                        entry.push(bulletin);
                    }
                }
                Ok(())
            }
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Not found.",
            ))),
        }
    }
    pub fn load(&mut self, acv_name: String) -> Result<(), std::io::Error> {
        let filename_meta = format!("{}/{}/meta.bin", *ACV_DIR, acv_name);
        let mut file_meta = File::open(&filename_meta)?;
        while let Ok((var_name, var_tag, revisions)) =
            ciborium::from_reader::<(_, String, u64), _>(&mut file_meta)
        {
            let key = (var_name, format!("{acv_name}:{var_tag}"));
            let entry = self.bulletins.entry(key).or_default();
            for _ in 0..revisions {
                if let Ok((offset, datasize, timestamp)) = ciborium::from_reader(&mut file_meta) {
                    let bulletin = Bulletin::from_archive(
                        &acv_name,
                        offset,
                        datasize,
                        DateTime::from_timestamp_nanos(timestamp).into(),
                    );
                    entry.push(bulletin);
                    self.datasize += datasize;
                    self.n_bulletins += 1;
                    self.n_archives += 1;
                } else {
                    panic!();
                }
            }
        }
        Ok(())
    }
    pub fn list_archive(&self) -> Result<Vec<String>, fs_extra::error::Error> {
        let dirs = fs_extra::dir::get_dir_content(&*ACV_DIR)?
            .files
            .iter()
            .filter(|x| x.contains("meta.bin"))
            .map(|x| {
                let mut x = x.clone();
                x.truncate(x.len() - 9);
                x.split_off(ACV_DIR.len() + 1)
            })
            .collect();
        Ok(dirs)
    }
    pub fn rename_archive(&self, name_from: String, name_to: String) -> Result<(), std::io::Error> {
        std::fs::rename(
            format!("{}/{}", *ACV_DIR, name_from),
            format!("{}/{}", *ACV_DIR, name_to),
        )?;
        Ok(())
    }
    pub fn delete_archive(&self, acv_name: String) -> Result<(), std::io::Error> {
        fs::remove_dir_all(format!("{}/{}", *ACV_DIR, acv_name))?;
        Ok(())
    }
    pub fn dump(&mut self, acv_name: String) -> Result<(), Box<dyn std::error::Error>> {
        let keys: Vec<_> = self.bulletins.keys().cloned().collect();
        for (var_name, var_tag) in keys {
            self.archive(var_name.clone(), var_tag.clone(), acv_name.clone())?;
        }
        Ok(())
    }
    pub fn restore(&mut self, acv_name: String) -> Result<(), Box<dyn std::error::Error>> {
        let filename_data = format!("{}/{}/data.bin", *ACV_DIR, acv_name);
        let filename_meta = format!("{}/{}/meta.bin", *ACV_DIR, acv_name);
        let file_data = File::open(&filename_data)?;
        let mut file_meta = File::open(&filename_meta)?;
        while let Ok((var_name, var_tag, revisions)) =
            ciborium::from_reader::<(String, String, u64), _>(&mut file_meta)
        {
            for _ in 0..revisions {
                if let Ok((offset, datasize, timestamp)) =
                    ciborium::from_reader::<(u64, u64, _), _>(&mut file_meta)
                {
                    let mut buf = vec![0u8; datasize.try_into().unwrap()];
                    file_data.read_exact_at(&mut buf, offset).unwrap();
                    let mut bulletin = Bulletin::from_data(buf);
                    bulletin.timestamp = DateTime::from_timestamp_nanos(timestamp).into();
                    self.post(var_name.clone(), var_tag.clone(), bulletin)?;
                } else {
                    panic!();
                }
            }
        }
        Ok(())
    }
    pub fn reset(&mut self) -> Result<(), std::io::Error> {
        *self = Self::new()?;
        Ok(())
    }
}
