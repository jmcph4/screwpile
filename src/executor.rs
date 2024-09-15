use std::{collections::HashMap, iter};

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};

/// Represents a Python execution environment
pub struct PythonExecutor {
    /// Import aliases
    pub imports: HashMap<String, String>,
}

impl PythonExecutor {
    pub fn new(imports: impl IntoIterator<Item = (String, String)>) -> Self {
        Self {
            imports: HashMap::from_iter(imports),
        }
    }

    /// Execute the given Python code
    pub fn run(&self, code: String) -> PyResult<Py<PyAny>> {
        Python::with_gil(|py| {
            Ok(py
                .eval_bound(code.as_ref(), None, self.locals(py).ok().as_ref())?
                .unbind())
        })
    }

    fn locals<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let aliases = self.imports.keys();
        let bounds = self
            .imports
            .values()
            .map(|lib| py.import_bound(lib.as_str()))
            .collect::<PyResult<Vec<Bound<'_, PyModule>>>>()?;

        Ok(iter::zip(aliases, bounds)
            .collect::<Vec<(&String, Bound<'_, PyModule>)>>()
            .into_py_dict_bound(py))
    }
}
