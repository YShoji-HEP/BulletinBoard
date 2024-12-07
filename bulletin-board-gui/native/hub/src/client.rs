use crate::messages::*;
use bulletin_board_client as bbclient;
use bulletin_board_server::*;
// use rinf::debug_print;
use std::sync::Mutex;
use std::thread;

pub static SERVER: Mutex<Option<thread::JoinHandle<()>>> = Mutex::new(None);

pub async fn set_addr() {
    let receiver = ReqSetAddr::get_dart_signal_receiver();
    while let Some(req) = receiver.recv().await {
        bbclient::set_addr(&req.message.address);
    }
}

pub async fn built_in_server_status() {
    let receiver = ReqBuiltInServerStatus::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        let handle = SERVER.lock().unwrap();
        ResBuiltInServerStatus {
            started: !handle.is_none(),
        }
        .send_signal_to_dart();
    }
}

pub async fn start_server() {
    let receiver = ReqStartServer::get_dart_signal_receiver();
    while let Some(req) = receiver.recv().await {
        let mut handle = SERVER.lock().unwrap();
        if handle.is_none() {
            let addr = req.message.address;
            let dir = req.message.directory;

            *handle = Some(thread::spawn(move || {
                let mut opt = ServerOptions::new();
                opt.set_listen_addr(addr);
                opt.set_tmp_dir(format!("{dir}/tmp"));
                opt.set_acv_dir(format!("{dir}/acv"));
                opt.set_log_file(format!("{dir}/bulletin-board.log"));
                opt.load_options();
                let mut server = BBServer::new().unwrap();
                server.listen().unwrap();
            }));
        }
    }
}

pub async fn stop_server() {
    let receiver = ReqStopServer::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        let mut handle = SERVER.lock().unwrap();
        if handle.is_some() {
            let _ = bbclient::terminate_server().is_ok();
            *handle = None;
        }
    }
}

pub async fn relabel() {
    let receiver = ReqRelabel::get_dart_signal_receiver();
    while let Some(req) = receiver.recv().await {
        let _ = bbclient::relabel(
            &req.message.title_from,
            Some(&req.message.tag_from),
            Some(&req.message.title_to),
            Some(&req.message.tag_to),
        )
        .is_ok();
    }
}

pub async fn status() {
    let receiver = ReqStatus::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        if let Ok((
            total_datasize,
            memory_used,
            memory_used_percentage,
            bulletins,
            files,
            archives,
        )) = bbclient::status()
        {
            ResStatus {
                total_datasize,
                memory_used,
                memory_used_percentage,
                bulletins,
                files,
                archives,
            }
            .send_signal_to_dart();
        }
    }
}

pub async fn log() {
    let receiver = ReqLog::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        if let Ok(log) = bbclient::log() {
            ResLog { log }.send_signal_to_dart();
        }
    }
}

pub async fn view_board() {
    let receiver = ReqViewBoard::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        if let Ok(list) = bbclient::view_board() {
            let mut require_tag = vec![];
            for elem in &list {
                require_tag.push(list.iter().any(|x| x.0 == elem.0 && x.1 != elem.1))
            }
            let bulletins = list
                .into_iter()
                .enumerate()
                .map(|(i, (title, tag, revisions))| ResBulletinItem {
                    title,
                    tag,
                    revisions,
                    require_tag: require_tag[i],
                })
                .collect();
            ResViewBoard { bulletins }.send_signal_to_dart();
        }
    }
}

pub async fn get_info() {
    let receiver = ReqGetInfo::get_dart_signal_receiver();
    while let Some(req) = receiver.recv().await {
        if let Ok(list) = bbclient::get_info(&req.message.title, Some(&req.message.tag)) {
            let info = list
                .into_iter()
                .map(|(revision, datasize, timestamp, backend)| ResBulletinInfo {
                    revision,
                    datasize,
                    timestamp,
                    backend,
                })
                .collect();
            ResGetInfo { info }.send_signal_to_dart();
        }
    }
}

pub async fn remove() {
    let receiver = ReqRemove::get_dart_signal_receiver();
    while let Some(req) = receiver.recv().await {
        let _ = bbclient::remove(&req.message.title, Some(&req.message.tag)).is_ok();
    }
}

pub async fn archive() {
    let receiver = ReqArchive::get_dart_signal_receiver();
    while let Some(req) = receiver.recv().await {
        let _ = bbclient::archive(
            &req.message.acv_name,
            &req.message.title,
            Some(&req.message.tag),
        )
        .is_ok();
    }
}

pub async fn load() {
    let receiver = ReqLoad::get_dart_signal_receiver();
    while let Some(req) = receiver.recv().await {
        let _ = bbclient::load(&req.message.acv_name).is_ok();
    }
}

pub async fn list_archive() {
    let receiver = ReqListArchive::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        if let Ok(archives) = bbclient::list_archive() {
            ResListArchive { archives }.send_signal_to_dart();
        }
    }
}

pub async fn rename_archive() {
    let receiver = ReqRenameArchive::get_dart_signal_receiver();
    while let Some(req) = receiver.recv().await {
        let _ = bbclient::rename_archive(&req.message.acv_from, &req.message.acv_to).is_ok();
    }
}

pub async fn delete_archive() {
    let receiver = ReqDeleteArchive::get_dart_signal_receiver();
    while let Some(req) = receiver.recv().await {
        let _ = bbclient::delete_archive(&req.message.acv_name).is_ok();
    }
}

pub async fn dump() {
    let receiver = ReqDump::get_dart_signal_receiver();
    while let Some(req) = receiver.recv().await {
        let _ = bbclient::dump(&req.message.acv_name).is_ok();
    }
}

pub async fn restore() {
    let receiver = ReqRestore::get_dart_signal_receiver();
    while let Some(req) = receiver.recv().await {
        let _ = bbclient::restore(&req.message.acv_name).is_ok();
    }
}

pub async fn clear_log() {
    let receiver = ReqClearLog::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        let _ = bbclient::clear_log().is_ok();
    }
}

pub async fn reset_server() {
    let receiver = ReqReset::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        let _ = bbclient::reset_server().is_ok();
    }
}

pub async fn terminate_server() {
    let receiver = ReqExit::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        let _ = bbclient::terminate_server().is_ok();
    }
}

// pub async fn key_input() {
//     use enigo::*;
//     let receiver = ReqKeyInput::get_dart_signal_receiver();
//     while let Some(req) = receiver.recv().await {
//         let mut enigo = Enigo::new(&enigo::Settings::default()).unwrap();
//         #[cfg(target_family = "unix")]
//         {
//             enigo.key(Key::Meta, Direction::Press).unwrap();
//             enigo.key(Key::Tab, Direction::Click).unwrap();
//             enigo.key(Key::Meta, Direction::Release).unwrap();
//         }
//         #[cfg(target_family = "windows")]
//         {
//             enigo.key(Key::Alt, Direction::Press).unwrap();
//             enigo.key(Key::Shift, Direction::Press).unwrap();
//             enigo.key(Key::Tab, Direction::Click).unwrap();
//             enigo.key(Key::Shift, Direction::Release).unwrap();
//             enigo.key(Key::Alt, Direction::Release).unwrap();
//         }
//         std::thread::sleep(std::time::Duration::from_millis(10));
//         enigo.text(&req.message.text).unwrap();
//     }
// }
