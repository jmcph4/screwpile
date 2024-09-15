use crate::executor::PythonExecutor;

pub fn tinygrad() -> PythonExecutor {
    PythonExecutor::new(vec![
        ("np".to_string(), "numpy".to_string()),
        ("tinygrad".to_string(), "tinygrad".to_string()),
    ])
}

macro_rules! tinygrad {
    ($s:expr) => {
        crate::presets::tinygrad().run($s)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tinygrad_run() {
        let res = tinygrad().run(r#"(tinygrad.Tensor([1, 2, 3]) * tinygrad.Tensor([10, 20, 30])).realize()"#.to_string());
        dbg!(&res);
        assert!(res.is_ok());
    }

    #[test]
    fn test_tinygrad_macro() {
        let res = tinygrad!(r#"(tinygrad.Tensor([1, 2, 3]) * tinygrad.Tensor([10, 20, 30])).realize()"#.to_string());
        assert!(res.is_ok());
    }
}
