use bulletin_board_client::{ArrayObject, DataType, Pair, VecShape, VecVecShape};
use wolfram_library_link::{self as wll, generate_loader, wstp};

generate_loader!(load_dbgbb);

#[wll::export(wstp)]
fn post_integer(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 3);
    let var_name = link.get_string().unwrap();
    let var_tag = link.get_string().unwrap();
    let val = link.get_i64().unwrap();
    let bc = val.try_into().unwrap();
    bulletin_board_client::post(var_name, var_tag, bc).unwrap();
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn post_real(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 3);
    let var_name = link.get_string().unwrap();
    let var_tag = link.get_string().unwrap();
    let val = link.get_f64().unwrap();
    let bc = val.try_into().unwrap();
    bulletin_board_client::post(var_name, var_tag, bc).unwrap();
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn post_complex(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 4);
    let var_name = link.get_string().unwrap();
    let var_tag = link.get_string().unwrap();
    let re = link.get_f64().unwrap();
    let im = link.get_f64().unwrap();
    let bc = Pair(re, im).try_into().unwrap();
    bulletin_board_client::post(var_name, var_tag, bc).unwrap();
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn post_string(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 3);
    let var_name = link.get_string().unwrap();
    let var_tag = link.get_string().unwrap();
    let val = link.get_string().unwrap();
    let bc = val.try_into().unwrap();
    bulletin_board_client::post(var_name, var_tag, bc).unwrap();
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn post_integer_array(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 3);
    {
        let var_name = link.get_string().unwrap();
        let var_tag = link.get_string().unwrap();
        let arr = link.get_i64_array().unwrap();
        let shape = arr.dimensions().into_iter().map(|&x| x as u64).collect();
        let data = arr.data().into_iter().copied().collect();
        let bc = VecShape(data, shape).try_into().unwrap();
        bulletin_board_client::post(var_name, var_tag, bc).unwrap();
    }
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn post_real_array(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 3);
    {
        let var_name = link.get_string().unwrap();
        let var_tag = link.get_string().unwrap();
        let arr = link.get_f64_array().unwrap();
        let shape = arr.dimensions().into_iter().map(|&x| x as u64).collect();
        let data = arr.data().into_iter().copied().collect();
        let bc = VecShape(data, shape).try_into().unwrap();
        bulletin_board_client::post(var_name, var_tag, bc).unwrap();
    }
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn post_complex_array(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 4);
    {
        let var_name = link.get_string().unwrap();
        let var_tag = link.get_string().unwrap();
        let (re, shape) = {
            let re_arr = link.get_f64_array().unwrap();
            let re = re_arr.data().into_iter().copied().collect();
            let shape = re_arr.dimensions().into_iter().map(|&x| x as u64).collect();
            (re, shape)
        };
        let im = {
            let im_arr = link.get_f64_array().unwrap();
            im_arr.data().into_iter().copied().collect()
        };
        let bc = VecVecShape(re, im, shape).try_into().unwrap();
        bulletin_board_client::post(var_name, var_tag, bc).unwrap();
    }
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn post_string_array(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 4);
    {
        let var_name = link.get_string().unwrap();
        let var_tag = link.get_string().unwrap();
        let len = link.test_head("System`List").unwrap();
        let mut data = vec![];
        for _ in 0..len {
            let text = link.get_string().unwrap();
            data.push(text);
        }
        let shape = link
            .get_i64_array()
            .unwrap()
            .data()
            .into_iter()
            .map(|&x| x.try_into().unwrap())
            .collect();
        let bc = VecShape(data, shape).try_into().unwrap();
        bulletin_board_client::post(var_name, var_tag, bc).unwrap();
    }
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn read(link: &mut wstp::Link) {
    let argc = link.test_head("System`List").unwrap();
    let (var_name, var_tag, revisions) = match argc {
        1 => (link.get_string().unwrap(), None, vec![]),
        2 => {
            let var_name = link.get_string().unwrap();
            if link.get_type().unwrap() == wstp::TokenType::String {
                (var_name, Some(link.get_string().unwrap()), vec![])
            } else {
                if link.get_type().unwrap() == wstp::TokenType::Integer {
                    (
                        var_name,
                        None,
                        vec![link.get_i64().unwrap().try_into().unwrap()],
                    )
                } else {
                    (
                        var_name,
                        None,
                        link.get_i64_array()
                            .unwrap()
                            .data()
                            .into_iter()
                            .map(|&x| x.try_into().unwrap())
                            .collect(),
                    )
                }
            }
        }
        3 => {
            let var_name = link.get_string().unwrap();
            let var_tag = Some(link.get_string().unwrap());
            if link.get_type().unwrap() == wstp::TokenType::Integer {
                (
                    var_name,
                    var_tag,
                    vec![link.get_i64().unwrap().try_into().unwrap()],
                )
            } else {
                (
                    var_name,
                    var_tag,
                    link.get_i64_array()
                        .unwrap()
                        .data()
                        .into_iter()
                        .map(|&x| x.try_into().unwrap())
                        .collect(),
                )
            }
        }
        _ => panic!(),
    };
    let list = bulletin_board_client::read(var_name, var_tag, revisions).unwrap();
    if list.len() > 1 {
        link.put_function("System`List", list.len()).unwrap();
    }
    for data in list {
        put_data(link, data);
    }
}

fn put_data(link: &mut wstp::Link, data: ArrayObject) {
    match data.datatype() {
        DataType::UnsignedInteger | DataType::SignedInteger => {
            if data.dimension() == 0 {
                let val = data.try_into().unwrap();
                link.put_i64(val).unwrap();
            } else {
                let VecShape(val, shape) = data.try_into().unwrap();
                let shape: Vec<usize> = shape.into_iter().map(|x| x.try_into().unwrap()).collect();
                link.put_i64_array(&val, &shape).unwrap();
            }
        }
        DataType::Real => {
            if data.dimension() == 0 {
                let val = data.try_into().unwrap();
                link.put_f64(val).unwrap();
            } else {
                let VecShape(val, shape) = data.try_into().unwrap();
                let shape: Vec<usize> = shape.into_iter().map(|x| x.try_into().unwrap()).collect();
                link.put_f64_array(&val, &shape).unwrap();
            }
        }
        DataType::Complex => {
            if data.dimension() == 0 {
                let Pair(re, im) = data.try_into().unwrap();
                link.put_function("System`Complex", 2).unwrap();
                link.put_f64(re).unwrap();
                link.put_f64(im).unwrap();
            } else {
                let VecVecShape(re, im, shape) = data.try_into().unwrap();
                link.put_function("System`ArrayReshape", 2).unwrap();
                link.put_function("System`List", re.len()).unwrap();
                for (re, im) in re.into_iter().zip(im.into_iter()) {
                    link.put_function("System`Complex", 2).unwrap();
                    link.put_f64(re).unwrap();
                    link.put_f64(im).unwrap();
                }
                link.put_function("System`List", shape.len()).unwrap();
                for d in shape {
                    link.put_i64(d.try_into().unwrap()).unwrap();
                }
            }
        }
        DataType::String => {
            if data.dimension() == 0 {
                let val: String = data.try_into().unwrap();
                link.put_str(&val).unwrap();
            } else {
                let VecShape::<String>(val, shape) = data.try_into().unwrap();
                link.put_function("System`ArrayReshape", 2).unwrap();
                link.put_function("System`List", val.len()).unwrap();
                for s in val {
                    link.put_str(&s).unwrap();
                }
                link.put_function("System`List", shape.len()).unwrap();
                for d in shape {
                    link.put_i64(d.try_into().unwrap()).unwrap();
                }
            }
        }
    }
}

#[wll::export(wstp)]
fn status(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 0);
    let (datasize, memory_used, memory_used_percent, n_bulletins, n_files, n_archives) =
        bulletin_board_client::status().unwrap();
    link.put_function("System`List", 6).unwrap();
    link.put_i64(datasize.try_into().unwrap()).unwrap();
    link.put_i64(memory_used.try_into().unwrap()).unwrap();
    link.put_f64(memory_used_percent).unwrap();
    link.put_i64(n_bulletins.try_into().unwrap()).unwrap();
    link.put_i64(n_files.try_into().unwrap()).unwrap();
    link.put_i64(n_archives.try_into().unwrap()).unwrap();
}

#[wll::export(wstp)]
fn log(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 0);
    let log = bulletin_board_client::log().unwrap();
    link.put_str(&log).unwrap();
}

#[wll::export(wstp)]
fn view_board(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 0);
    let board = bulletin_board_client::view_board().unwrap();
    link.put_function("System`List", board.len()).unwrap();
    for (var_name, var_tag, revisions) in board {
        link.put_function("System`List", 3).unwrap();
        link.put_str(&var_name).unwrap();
        link.put_str(&var_tag).unwrap();
        link.put_i64(revisions.try_into().unwrap()).unwrap();
    }
}

#[wll::export(wstp)]
fn get_info(link: &mut wstp::Link) {
    let argc = link.test_head("System`List").unwrap();
    let (var_name, var_tag) = match argc {
        1 => (link.get_string().unwrap(), None),
        2 => (link.get_string().unwrap(), Some(link.get_string().unwrap())),
        _ => panic!(),
    };
    let info = bulletin_board_client::get_info(var_name, var_tag).unwrap();
    link.put_function("System`List", info.len()).unwrap();
    for (revision, datasize, timestamp, backend) in info {
        link.put_function("System`List", 4).unwrap();
        link.put_i64(revision.try_into().unwrap()).unwrap();
        link.put_i64(datasize.try_into().unwrap()).unwrap();
        link.put_str(&timestamp).unwrap();
        link.put_str(&backend).unwrap();
    }
}

#[wll::export(wstp)]
fn clear_revisions(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 3);
    let var_name = link.get_string().unwrap();
    let var_tag = link.get_string().unwrap();
    if link.get_type().unwrap() == wstp::TokenType::Integer {
        let revision = link.get_i64().unwrap().try_into().unwrap();
        bulletin_board_client::clear_revisions(var_name, var_tag, vec![revision]).unwrap();
    } else {
        let revisions = link
            .get_i64_array()
            .unwrap()
            .data()
            .into_iter()
            .map(|&x| x.try_into().unwrap())
            .collect();
        bulletin_board_client::clear_revisions(var_name, var_tag, revisions).unwrap();
    }
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn remove(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 2);
    let var_name = link.get_string().unwrap();
    let var_tag = link.get_string().unwrap();
    bulletin_board_client::remove(var_name, var_tag).unwrap();
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn archive(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 2);
    let var_name = link.get_string().unwrap();
    let var_tag = link.get_string().unwrap();
    let name = link.get_string().unwrap();
    bulletin_board_client::archive(var_name, var_tag, name).unwrap();
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn load(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 1);
    let name = link.get_string().unwrap();
    bulletin_board_client::load(name).unwrap();
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn list_archive(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 0);
    let list = bulletin_board_client::list_archive().unwrap();
    link.put_function("System`List", list.len()).unwrap();
    for name in list {
        link.put_str(&name).unwrap();
    }
}

#[wll::export(wstp)]
fn rename_archive(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 2);
    let name_from = link.get_string().unwrap();
    let name_to = link.get_string().unwrap();
    bulletin_board_client::rename_archive(name_from, name_to).unwrap();
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn delete_archive(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 1);
    let name = link.get_string().unwrap();
    bulletin_board_client::delete_archive(name).unwrap();
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn dump(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 1);
    let name = link.get_string().unwrap();
    bulletin_board_client::dump(name).unwrap();
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn restore(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 1);
    let name = link.get_string().unwrap();
    bulletin_board_client::restore(name).unwrap();
    link.put_str("Sent").unwrap();
}

#[wll::export(wstp)]
fn reset(link: &mut wstp::Link) {
    assert_eq!(link.test_head("System`List").unwrap(), 0);
    bulletin_board_client::reset().unwrap();
    link.put_str("Sent").unwrap();
}