use std::{collections::HashMap, iter};

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};

/// Represents a Python execution environment
pub struct PythonExecutor<'a> {
    /// Import aliases
    pub imports: HashMap<String, String>,
    /// Main [`Python`]` handle
    pub py: Python<'a>,
}

impl<'a> PythonExecutor<'a> {
    pub fn new(
        py: Python<'a>,
        imports: impl IntoIterator<Item = (String, String)>,
    ) -> Self {
        Self {
            imports: HashMap::from_iter(imports),
            py,
        }
    }

    /// Execute the given Python code
    pub fn run<T>(&self, code: T) -> PyResult<Bound<'_, PyAny>>
    where
        T: AsRef<str>,
    {
        self.py
            .eval_bound(code.as_ref(), None, self.locals().ok().as_ref())
    }

    fn locals(&self) -> PyResult<Bound<'_, PyDict>> {
        let aliases = self.imports.keys();
        let bounds = self
            .imports
            .values()
            .map(|lib| self.py.import_bound(lib.as_str()))
            .collect::<PyResult<Vec<Bound<'_, PyModule>>>>()?;

        Ok(iter::zip(aliases, bounds)
            .collect::<Vec<(&String, Bound<'_, PyModule>)>>()
            .into_py_dict_bound(self.py))
    }
}
