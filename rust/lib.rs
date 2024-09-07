use pyo3::prelude::*;

#[pymodule]
mod _lib {
    use pyo3::prelude::*;
    use std::env;

    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

}