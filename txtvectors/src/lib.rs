use std::collections::HashMap;

use pyo3::prelude::*;

#[pyfunction]
fn multiply(x: f64, y: f64) -> PyResult<f64> {
    Ok(x * y)
}

#[pyfunction]
fn frequency_encode(s: String) -> PyResult<HashMap<String, i32>> {
    // returns hash map { word: frequency }
    let mut result = HashMap::new();
    s.split(" ")
        .for_each(|w| *result.entry(w.to_string()).or_insert(0) += 1);
    Ok(result)
}

#[pyfunction]
fn batch_frequency_encode(batch: Vec<String>) -> PyResult<HashMap<String, i32>> {
    let mut result: HashMap<String, i32> = HashMap::new();
    for s in batch.iter() {
        s.split(" ")
            .for_each(|w| *result.entry(w.to_string()).or_insert(0) += 1);
    }
    return Ok(result);
}

/// A Python module implemented in Rust.
#[pymodule]
fn txtvectors(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(frequency_encode, m)?)?;
    m.add_function(wrap_pyfunction!(batch_frequency_encode, m)?)?;
    Ok(())
}
