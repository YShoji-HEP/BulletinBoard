use bbclient::{DataType, VecShape};
use num_complex::Complex64;
use pyo3::prelude::*;

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

#[pyfunction]
fn status_raw(py: Python<'_>) -> PyResult<PyObject> {
    Ok(bbclient::status().unwrap().to_object(py))
}

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
fn clear_revisions(title: String, tag: String, revisions: Vec<u64>) -> PyResult<()> {
    bbclient::clear_revisions(&title, &tag, revisions).unwrap();
    Ok(())
}

#[pyfunction]
fn remove(title: String, tag: String) -> PyResult<()> {
    bbclient::remove(&title, &tag).unwrap();
    Ok(())
}

#[pyfunction]
fn archive(title: String, tag: String, acv_name: String) -> PyResult<()> {
    bbclient::archive(&title, &tag, &acv_name).unwrap();
    Ok(())
}

#[pyfunction]
fn load(acv_name: String) -> PyResult<()> {
    bbclient::load(&acv_name).unwrap();
    Ok(())
}

#[pyfunction]
fn list_archive(py: Python<'_>) -> PyResult<PyObject> {
    Ok(bbclient::list_archive().unwrap().to_object(py))
}

#[pyfunction]
fn rename_archive(name_from: String, name_to: String) -> PyResult<()> {
    bbclient::rename_archive(&name_from, &name_to).unwrap();
    Ok(())
}

#[pyfunction]
fn delete_archive(acv_name: String) -> PyResult<()> {
    bbclient::delete_archive(&acv_name).unwrap();
    Ok(())
}

#[pyfunction]
fn dump(acv_name: String) -> PyResult<()> {
    bbclient::dump(&acv_name).unwrap();
    Ok(())
}

#[pyfunction]
fn restore(acv_name: String) -> PyResult<()> {
    bbclient::restore(&acv_name).unwrap();
    Ok(())
}

#[pyfunction]
fn reset() -> PyResult<()> {
    bbclient::reset().unwrap();
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn bulletin_board_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(post_integer, m)?)?;
    m.add_function(wrap_pyfunction!(post_real, m)?)?;
    m.add_function(wrap_pyfunction!(post_complex, m)?)?;
    m.add_function(wrap_pyfunction!(post_string, m)?)?;
    m.add_function(wrap_pyfunction!(post_integer_array, m)?)?;
    m.add_function(wrap_pyfunction!(post_real_array, m)?)?;
    m.add_function(wrap_pyfunction!(post_complex_array, m)?)?;
    m.add_function(wrap_pyfunction!(post_string_array, m)?)?;
    m.add_function(wrap_pyfunction!(read_raw, m)?)?;
    m.add_function(wrap_pyfunction!(status_raw, m)?)?;
    m.add_function(wrap_pyfunction!(log, m)?)?;
    m.add_function(wrap_pyfunction!(view_board_raw, m)?)?;
    m.add_function(wrap_pyfunction!(get_info_raw, m)?)?;
    m.add_function(wrap_pyfunction!(clear_revisions, m)?)?;
    m.add_function(wrap_pyfunction!(remove, m)?)?;
    m.add_function(wrap_pyfunction!(archive, m)?)?;
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(list_archive, m)?)?;
    m.add_function(wrap_pyfunction!(rename_archive, m)?)?;
    m.add_function(wrap_pyfunction!(delete_archive, m)?)?;
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    m.add_function(wrap_pyfunction!(restore, m)?)?;
    m.add_function(wrap_pyfunction!(reset, m)?)?;
    Ok(())
}
