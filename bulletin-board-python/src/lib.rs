use std::time::Duration;
use bbclient::{adaptor::VecShape, DataType};
use num_complex::Complex64;
use pyo3::prelude::*;

/// Sets the server address.
#[pyfunction]
fn set_addr(addr: String) -> PyResult<()> {
    bbclient::set_addr(&addr);
    Ok(())
}

/// Sets timeout for TCP connections in msec. If the argument is None, timeout is disabled (default).
#[pyfunction]
#[pyo3(signature = (timeout=None))]
fn set_timeout(timeout: Option<u64>) -> PyResult<()> {
    bbclient::set_timeout(timeout.map(|t| Duration::from_millis(t)));
    Ok(())
}

#[pyfunction]
fn post_integer(title: String, tag: String, val: i128) -> PyResult<()> {
    let obj = val.try_into().unwrap();
    bbclient::post(&title, &tag, obj).unwrap();
    Ok(())
}

#[pyfunction]
fn post_real(title: String, tag: String, val: f64) -> PyResult<()> {
    let obj = val.try_into().unwrap();
    bbclient::post(&title, &tag, obj).unwrap();
    Ok(())
}

#[pyfunction]
fn post_complex(title: String, tag: String, val: Complex64) -> PyResult<()> {
    let obj = val.try_into().unwrap();
    bbclient::post(&title, &tag, obj).unwrap();
    Ok(())
}

#[pyfunction]
fn post_string(title: String, tag: String, val: String) -> PyResult<()> {
    let obj = val.try_into().unwrap();
    bbclient::post(&title, &tag, obj).unwrap();
    Ok(())
}

#[pyfunction]
fn post_integer_array(title: String, tag: String, val: Vec<i128>, shape: Vec<u64>) -> PyResult<()> {
    let obj = VecShape(val, shape).try_into().unwrap();
    bbclient::post(&title, &tag, obj).unwrap();
    Ok(())
}

#[pyfunction]
fn post_real_array(title: String, tag: String, val: Vec<f64>, shape: Vec<u64>) -> PyResult<()> {
    let obj = VecShape(val, shape).try_into().unwrap();
    bbclient::post(&title, &tag, obj).unwrap();
    Ok(())
}

#[pyfunction]
fn post_complex_array(
    title: String,
    tag: String,
    val: Vec<Complex64>,
    shape: Vec<u64>,
) -> PyResult<()> {
    let obj = VecShape(val, shape).try_into().unwrap();
    bbclient::post(&title, &tag, obj).unwrap();
    Ok(())
}

#[pyfunction]
fn post_string_array(
    title: String,
    tag: String,
    val: Vec<String>,
    shape: Vec<u64>,
) -> PyResult<()> {
    let obj = VecShape(val, shape).try_into().unwrap();
    bbclient::post(&title, &tag, obj).unwrap();
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (title, tag=None, revisions=None))]
fn read_raw(
    py: Python<'_>,
    title: String,
    tag: Option<String>,
    revisions: Option<Vec<u64>>,
) -> PyResult<PyObject> {
    let revisions = match revisions {
        Some(rev) => rev,
        None => vec![],
    };

    let list = bbclient::read(&title, tag.as_deref(), revisions).unwrap();
    let mut res = vec![];
    for elem in list {
        match elem.datatype() {
            DataType::UnsignedInteger => {
                if elem.dimension() == 0 {
                    let val: u128 = elem.try_into().unwrap();
                    res.push(val.to_object(py));
                } else {
                    let VecShape::<u128>(val, shape) = elem.try_into().unwrap();
                    let shape: Vec<usize> =
                        shape.into_iter().map(|x| x.try_into().unwrap()).collect();
                    res.push((val, shape).to_object(py));
                }
            }
            DataType::SignedInteger => {
                if elem.dimension() == 0 {
                    let val: i128 = elem.try_into().unwrap();
                    res.push(val.to_object(py));
                } else {
                    let VecShape::<i128>(val, shape) = elem.try_into().unwrap();
                    let shape: Vec<usize> =
                        shape.into_iter().map(|x| x.try_into().unwrap()).collect();
                    res.push((val, shape).to_object(py));
                }
            }
            DataType::Real => {
                if elem.dimension() == 0 {
                    let val: f64 = elem.try_into().unwrap();
                    res.push(val.to_object(py));
                } else {
                    let VecShape::<f64>(val, shape) = elem.try_into().unwrap();
                    let shape: Vec<usize> =
                        shape.into_iter().map(|x| x.try_into().unwrap()).collect();
                    res.push((val, shape).to_object(py));
                }
            }
            DataType::Complex => {
                if elem.dimension() == 0 {
                    let val: Complex64 = elem.try_into().unwrap();
                    res.push(val.to_object(py));
                } else {
                    let VecShape::<Complex64>(val, shape) = elem.try_into().unwrap();
                    res.push((val, shape).to_object(py));
                }
            }
            DataType::String => {
                if elem.dimension() == 0 {
                    let val: String = elem.try_into().unwrap();
                    res.push(val.to_object(py));
                } else {
                    let VecShape::<String>(val, shape) = elem.try_into().unwrap();
                    res.push((val, shape).to_object(py));
                }
            }
        }
    }
    Ok(res.to_object(py))
}

/// Relabels a bulletin.
#[pyfunction]
#[pyo3(signature = (title_from, tag_from=None, title_to=None, tag_to=None))]
fn relabel(
    title_from: String,
    tag_from: Option<String>,
    title_to: Option<String>,
    tag_to: Option<String>,
) -> PyResult<()> {
    bbclient::relabel(
        &title_from,
        tag_from.as_deref(),
        title_to.as_deref(),
        tag_to.as_deref(),
    )
    .unwrap();
    Ok(())
}

/// Returns the version of the client.
#[pyfunction]
fn client_version(py: Python<'_>) -> PyResult<PyObject> {
    let client_version = env!("CARGO_PKG_VERSION").to_string();
    Ok(client_version.to_object(py))
}

/// Returns the version of the server.
#[pyfunction]
fn server_version(py: Python<'_>) -> PyResult<PyObject> {
    let server_version = bbclient::server_version().unwrap();
    Ok(server_version.to_object(py))
}

/// Returns the status of the server.
#[pyfunction]
fn status_raw(py: Python<'_>) -> PyResult<PyObject> {
    Ok(bbclient::status().unwrap().to_object(py))
}

/// Returns the log of the server.
#[pyfunction]
fn log(py: Python<'_>) -> PyResult<PyObject> {
    Ok(bbclient::log().unwrap().to_object(py))
}

#[pyfunction]
fn view_board_raw(py: Python<'_>) -> PyResult<PyObject> {
    Ok(bbclient::view_board().unwrap().to_object(py))
}

#[pyfunction]
#[pyo3(signature = (title, tag=None))]
fn get_info_raw(py: Python<'_>, title: String, tag: Option<String>) -> PyResult<PyObject> {
    Ok(bbclient::get_info(&title, tag.as_deref())
        .unwrap()
        .to_object(py))
}

#[pyfunction]
#[pyo3(signature = (title, revisions, tag=None))]
fn clear_revisions_raw(title: String, revisions: Vec<u64>, tag: Option<String>) -> PyResult<()> {
    bbclient::clear_revisions(&title, tag.as_deref(), revisions).unwrap();
    Ok(())
}

/// Removes all the revisions and the database entry of a bulletin.
#[pyfunction]
#[pyo3(signature = (title, tag=None))]
fn remove(title: String, tag: Option<String>) -> PyResult<()> {
    bbclient::remove(&title, tag.as_deref()).unwrap();
    Ok(())
}

/// Moves a bulletin to a persistent archive.
#[pyfunction]
#[pyo3(signature = (acv_name, title, tag=None))]
fn archive(acv_name: String, title: String, tag: Option<String>) -> PyResult<()> {
    bbclient::archive(&acv_name, &title, tag.as_deref()).unwrap();
    Ok(())
}

/// Loads or reloads an archive. The data is directly read from the archive file and a suffix "acv_name:" is added to the tag.
#[pyfunction]
fn load(acv_name: String) -> PyResult<()> {
    bbclient::load(&acv_name).unwrap();
    Ok(())
}

/// Shows the list of archive.
#[pyfunction]
fn list_archive(py: Python<'_>) -> PyResult<PyObject> {
    Ok(bbclient::list_archive().unwrap().to_object(py))
}

/// Renames an archive. This will be applied after after calling reset_server.
#[pyfunction]
fn rename_archive(name_from: String, name_to: String) -> PyResult<()> {
    bbclient::rename_archive(&name_from, &name_to).unwrap();
    Ok(())
}

/// Deletes an archive. This will be applied after after calling reset_server.
#[pyfunction]
fn delete_archive(acv_name: String) -> PyResult<()> {
    bbclient::delete_archive(&acv_name).unwrap();
    Ok(())
}

/// Dumps all the unarchived data into an archive.
#[pyfunction]
fn dump(acv_name: String) -> PyResult<()> {
    bbclient::dump(&acv_name).unwrap();
    Ok(())
}

/// Delete all the temporary data and restores data from an archive. Each data is copied to memory or a separate file. No suffix is added to the tag.
#[pyfunction]
fn restore(acv_name: String) -> PyResult<()> {
    bbclient::restore(&acv_name).unwrap();
    Ok(())
}

/// Clears the log file of the server.
#[pyfunction]
fn clear_log() -> PyResult<()> {
    bbclient::clear_log().unwrap();
    Ok(())
}

/// Resets and clears the data. The archived data is not affected, but must be loaded before use.
#[pyfunction]
fn reset_server() -> PyResult<()> {
    bbclient::reset_server().unwrap();
    Ok(())
}

/// Terminates the server.
#[pyfunction]
fn terminate_server() -> PyResult<()> {
    bbclient::terminate_server().unwrap();
    Ok(())
}

/// BulletinBoard python client
#[pymodule]
fn bulletin_board_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_addr, m)?)?;
    m.add_function(wrap_pyfunction!(set_timeout, m)?)?;
    m.add_function(wrap_pyfunction!(post_integer, m)?)?;
    m.add_function(wrap_pyfunction!(post_real, m)?)?;
    m.add_function(wrap_pyfunction!(post_complex, m)?)?;
    m.add_function(wrap_pyfunction!(post_string, m)?)?;
    m.add_function(wrap_pyfunction!(post_integer_array, m)?)?;
    m.add_function(wrap_pyfunction!(post_real_array, m)?)?;
    m.add_function(wrap_pyfunction!(post_complex_array, m)?)?;
    m.add_function(wrap_pyfunction!(post_string_array, m)?)?;
    m.add_function(wrap_pyfunction!(read_raw, m)?)?;
    m.add_function(wrap_pyfunction!(relabel, m)?)?;
    m.add_function(wrap_pyfunction!(client_version, m)?)?;
    m.add_function(wrap_pyfunction!(server_version, m)?)?;
    m.add_function(wrap_pyfunction!(status_raw, m)?)?;
    m.add_function(wrap_pyfunction!(log, m)?)?;
    m.add_function(wrap_pyfunction!(view_board_raw, m)?)?;
    m.add_function(wrap_pyfunction!(get_info_raw, m)?)?;
    m.add_function(wrap_pyfunction!(clear_revisions_raw, m)?)?;
    m.add_function(wrap_pyfunction!(remove, m)?)?;
    m.add_function(wrap_pyfunction!(archive, m)?)?;
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(list_archive, m)?)?;
    m.add_function(wrap_pyfunction!(rename_archive, m)?)?;
    m.add_function(wrap_pyfunction!(delete_archive, m)?)?;
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    m.add_function(wrap_pyfunction!(restore, m)?)?;
    m.add_function(wrap_pyfunction!(clear_log, m)?)?;
    m.add_function(wrap_pyfunction!(reset_server, m)?)?;
    m.add_function(wrap_pyfunction!(terminate_server, m)?)?;
    Ok(())
}
