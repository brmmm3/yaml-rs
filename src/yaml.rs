use std::borrow::Cow;
use std::path::PathBuf;

use ordered_float::OrderedFloat;
use pyo3::exceptions::{PyIOError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString, PyTuple};
use saphyr::LoadableYamlNode;
use serde_yaml::Value;

use crate::decoder::encode;

fn yaml_to_py(yaml: &mut saphyr::Yaml, py: Python<'_>) -> PyResult<Py<PyAny>> {
    Ok(match yaml {
        saphyr::Yaml::Representation(..) => py.None(), // Should be resolved by now
        saphyr::Yaml::Value(scalar) => match scalar {
            saphyr::Scalar::String(s) => s.clone().into_pyobject(py)?.as_any().to_owned().unbind(),
            saphyr::Scalar::Integer(i) => (*i).into_pyobject(py)?.as_any().to_owned().unbind(),
            saphyr::Scalar::FloatingPoint(f) => {
                f.clone().into_pyobject(py)?.as_any().to_owned().unbind()
            }
            saphyr::Scalar::Boolean(b) => (*b).into_pyobject(py)?.as_any().to_owned().unbind(),
            saphyr::Scalar::Null => py.None(),
        },
        saphyr::Yaml::Sequence(seq) => {
            let list = PyList::empty(py);
            for item in seq.iter_mut() {
                list.append(yaml_to_py(item, py)?).unwrap();
            }
            list.into_pyobject(py)?.as_any().to_owned().unbind()
        }
        saphyr::Yaml::Mapping(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map.iter_mut() {
                let key_str = k.as_str(); // Cow<str> to String
                dict.set_item(key_str, yaml_to_py(v, py)?).unwrap();
            }
            dict.into_pyobject(py)?.as_any().to_owned().unbind()
        }
        saphyr::Yaml::Tagged(_, inner) => yaml_to_py(inner, py)?, // Recurse into tag
        saphyr::Yaml::Alias(_) | saphyr::Yaml::BadValue => py.None(),
    })
}

#[pyfunction]
#[pyo3(signature = (path, encoding=None, encoder_errors=None))]
pub fn load(
    path: PathBuf,
    encoding: Option<String>,
    encoder_errors: Option<String>,
    py: Python<'_>,
) -> PyResult<Py<PyAny>> {
    let data = py.detach(|| std::fs::read(path))?;
    from_bytes(data, encoding, encoder_errors, py)
}

#[pyfunction]
#[pyo3(signature = (data, encoding=None, encoder_errors=None))]
pub fn from_bytes(
    data: Vec<u8>,
    encoding: Option<String>,
    encoder_errors: Option<String>,
    py: Python<'_>,
) -> PyResult<Py<PyAny>> {
    let encoded_data =
        py.detach(|| encode(&data, encoding.as_deref(), encoder_errors.as_deref()))?;
    from_string(&encoded_data, py)
}

#[pyfunction]
pub fn from_string(data: &str, py: Python<'_>) -> PyResult<Py<PyAny>> {
    let mut docs = py.detach(|| {
        saphyr::Yaml::load_from_str(data)
            .map_err(|e| PyValueError::new_err(format!("YAML error: {e}")))
    })?;
    let list = PyList::empty(py);
    for doc in docs.iter_mut() {
        list.append(yaml_to_py(doc, py)?)?;
    }
    Ok(list.as_any().to_owned().unbind())
}

pub fn py_to_yaml(obj: Py<PyAny>, py: Python<'_>) -> PyResult<Value> {
    let obj = obj.bind(py);

    // Fast path for the most common primitive types
    if let Ok(s) = obj.cast::<PyString>() {
        // Prefer lossless UTF-8, fall back to lossy only if invalid UTF-8
        let s = s.to_str()?.to_owned();
        return Ok(Value::String(s));
    }
    if let Ok(i) = obj.cast::<PyInt>() {
        let i: i64 = i.extract()?;
        return Ok(Value::Number(i.into()));
    }
    if let Ok(f) = obj.cast::<PyFloat>() {
        let f: f64 = f.extract()?;
        return Ok(Value::Number(f.into()));
    }
    if let Ok(b) = obj.cast::<PyBool>() {
        let b: bool = b.extract()?;
        return Ok(Value::Bool(b));
    }
    if obj.is_none() {
        return Ok(Value::Null);
    }

    // ----- Sequences (list / tuple) -----
    if let Ok(list) = obj.cast::<PyList>() {
        let mut seq = Vec::with_capacity(list.len());
        for item in list.iter() {
            seq.push(py_to_yaml(item.into(), py)?);
        }
        return Ok(Value::Sequence(seq));
    }
    if let Ok(tuple) = obj.cast::<PyTuple>() {
        let mut seq = Vec::with_capacity(tuple.len());
        for item in tuple.iter() {
            seq.push(py_to_yaml(item.into(), py)?);
        }
        return Ok(Value::Sequence(seq));
    }

    // ----- Mapping (dict) -----
    if let Ok(dict) = obj.cast::<PyDict>() {
        let mut map = serde_yaml::Mapping::with_capacity(dict.len());
        for (k, v) in dict.iter() {
            let key = k
                .cast::<PyString>()
                .map_err(|_| PyErr::new::<PyTypeError, _>("Dict keys must be strings"))?
                .to_str()?
                .to_owned();
            let value = py_to_yaml(v.into(), py)?;
            map.insert(Value::String(key), value);
        }
        return Ok(Value::Mapping(map));
    }

    // ----- Fallback -----
    Err(PyErr::new::<PyTypeError, _>(format!(
        "Unsupported type: {}",
        obj.get_type().name()?
    )))
}

/// Return Python object -> YAML string
#[pyfunction]
pub fn dump(obj: Py<PyAny>, py: Python<'_>) -> PyResult<String> {
    let yaml = py_to_yaml(obj, py)?;
    py.detach(|| {
        serde_yaml::to_string(&yaml).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("YAML dump error: {}", e))
        })
    })
}

/// Write Python object -> YAML file
#[pyfunction]
pub fn save(path: &str, obj: Py<PyAny>, py: Python<'_>) -> PyResult<()> {
    let data = dump(obj, py)?;
    Ok(py.detach(|| {
        std::fs::write(path, data).map_err(|e| PyErr::new::<PyIOError, _>(e.to_string()))
    })?)
}

fn py_to_yaml_saphyr(obj: Py<PyAny>, py: Python<'_>) -> PyResult<saphyr::Yaml<'_>> {
    let obj = obj.bind(py);

    if let Ok(s) = obj.cast::<PyString>() {
        let s = s.to_str()?.to_owned();
        Ok(saphyr::Yaml::Value(saphyr::Scalar::String(Cow::Owned(s))))
    } else if let Ok(i) = obj.cast::<PyInt>() {
        let i: i64 = i.extract()?;
        Ok(saphyr::Yaml::Value(saphyr::Scalar::Integer(i)))
    } else if let Ok(f) = obj.cast::<PyFloat>() {
        let f: f64 = f.extract()?;
        Ok(saphyr::Yaml::Value(saphyr::Scalar::FloatingPoint(
            OrderedFloat(f),
        )))
    } else if let Ok(b) = obj.cast::<PyBool>() {
        let b: bool = b.extract()?;
        Ok(saphyr::Yaml::Value(saphyr::Scalar::Boolean(b)))
    } else if obj.is_none() {
        Ok(saphyr::Yaml::Value(saphyr::Scalar::Null))
    } else if let Ok(list) = obj.cast::<PyList>() {
        let mut seq = saphyr::Sequence::with_capacity(list.len());
        for item in list.iter() {
            seq.push(py_to_yaml_saphyr(item.into(), py)?);
        }
        Ok(saphyr::Yaml::Sequence(seq))
    } else if let Ok(tuple) = obj.cast::<PyTuple>() {
        let mut seq = saphyr::Sequence::with_capacity(tuple.len());
        for item in tuple.iter() {
            seq.push(py_to_yaml_saphyr(item.into(), py)?);
        }
        Ok(saphyr::Yaml::Sequence(seq))
    } else if let Ok(dict) = obj.cast::<PyDict>() {
        let mut map = saphyr::Mapping::with_capacity(dict.len());
        for (k, v) in dict.iter() {
            let key = k
                .cast::<PyString>()
                .map_err(|_| PyErr::new::<PyTypeError, _>("Dict keys must be strings"))?
                .to_str()?
                .to_owned();
            let value = py_to_yaml_saphyr(v.into(), py)?;
            map.insert(
                saphyr::Yaml::Value(saphyr::Scalar::String(Cow::Owned(key))),
                value,
            );
        }
        Ok(saphyr::Yaml::Mapping(map))
    } else {
        Err(PyErr::new::<PyTypeError, _>(format!(
            "Unsupported type: {}",
            obj.get_type().name()?
        )))
    }
}

// Helper: Yaml -> String using YamlEmitter
fn yaml_to_string(yaml: &saphyr::Yaml) -> PyResult<String> {
    let mut output = String::new();
    let mut emitter = saphyr::YamlEmitter::new(&mut output);
    emitter.dump(yaml).map_err(|e: saphyr::EmitError| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("YAML emit error: {e}"))
    })?;
    Ok(output)
}

/// Return Python object -> YAML string
#[pyfunction]
pub fn dump_saphyr(obj: Py<PyAny>, py: Python<'_>) -> PyResult<String> {
    let data = py_to_yaml_saphyr(obj, py)?;
    py.detach(|| {
        yaml_to_string(&data).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("YAML dump error: {}", e))
        })
    })
}

/// Write Python object -> YAML file
#[pyfunction]
pub fn save_saphyr(path: &str, obj: Py<PyAny>, py: Python<'_>) -> PyResult<()> {
    let data = dump_saphyr(obj, py)?;
    Ok(py.detach(|| {
        std::fs::write(path, data).map_err(|e| PyErr::new::<PyIOError, _>(e.to_string()))
    })?)
}
