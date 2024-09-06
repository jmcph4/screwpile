use pyo3::Python;

use crate::executor::PythonExecutor;

pub fn tinygrad(py: Python<'_>) -> PythonExecutor {
    PythonExecutor::new(
        py,
        vec![
            ("np".to_string(), "numpy".to_string()),
            ("tinygrad".to_string(), "tinygrad".to_string()),
        ],
    )
}

pub fn data_science(py: Python<'_>) -> PythonExecutor {
    PythonExecutor::new(
        py,
        vec![
            ("np".to_string(), "numpy".to_string()),
            ("pd".to_string(), "pandas".to_string()),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tinygrad() {
        Python::with_gil(|py| {
            let exec = tinygrad(py);
            let res = exec.run(r#"(tinygrad.Tensor([1, 2, 3]) * tinygrad.Tensor([10, 20, 30])).realize()"#);
            assert!(res.is_ok());
        });
    }
}
