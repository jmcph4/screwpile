use crate::executor::PythonExecutor;

pub fn data_science() -> PythonExecutor {
    PythonExecutor::new(vec![
        ("np".to_string(), "numpy".to_string()),
        ("pd".to_string(), "pandas".to_string()),
    ])
}
