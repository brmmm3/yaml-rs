use pyo3::prelude::*;

mod decoder;
mod yaml;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// yaml_rs is the Python layer for the saphyr YAML parser/generator.
#[pymodule]
#[pyo3(name = "yaml_rs")]
fn init(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add("__doc__", env!("CARGO_PKG_DESCRIPTION"))?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(yaml::load, m)?)?;
    m.add_function(wrap_pyfunction!(yaml::from_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(yaml::from_string, m)?)?;
    m.add_function(wrap_pyfunction!(yaml::dump, m)?)?;
    m.add_function(wrap_pyfunction!(yaml::save, m)?)?;
    m.add_function(wrap_pyfunction!(yaml::dump_saphyr, m)?)?;
    m.add_function(wrap_pyfunction!(yaml::save_saphyr, m)?)?;
    Ok(())
}
