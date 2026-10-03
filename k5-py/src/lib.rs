use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn k5_512(data: &[u8]) -> String {
    hex::encode(k5_core::k5_512(data))
}

#[pyfunction]
fn k5_1024(data: &[u8]) -> String {
    hex::encode(k5_core::k5_1024(data))
}

#[pyfunction]
fn k2048(data: &[u8]) -> String {
    hex::encode(k5_core::k2048(data))
}

#[pyfunction]
fn kof(data: &[u8], bits: usize) -> String {
    hex::encode(k5_core::kof(data, bits))
}

#[pyfunction]
fn k5_h(data: &[u8], mem_kib: usize, time_cost: u32) -> String {
    hex::encode(k5_core::k5_h(data, mem_kib, time_cost))
}

#[pymodule]
fn k5_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(k5_512, m)?)?;
    m.add_function(wrap_pyfunction!(k5_1024, m)?)?;
    m.add_function(wrap_pyfunction!(k2048, m)?)?;
    m.add_function(wrap_pyfunction!(kof, m)?)?;
    m.add_function(wrap_pyfunction!(k5_h, m)?)?;
    Ok(())
}
