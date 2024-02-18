use std::fs::File;
use std::io::prelude::*;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let mut f = File::open("service_code.pkl")?;
        let mut buffer = Vec::new();

        // read the whole file
        f.read_to_end(&mut buffer)?;

        let dill = py.import("dill")?;
        let load_fn = dill.getattr("loads")?;
        let args = PyTuple::new(py, &[buffer]);

        let _task_fn = load_fn.call(args, None)?;

        // let mut f = File::open("service_input.pkl")?;
        // let mut input = Vec::new();

        // // read the whole file
        // f.read_to_end(&mut input)?;

        // task_fn.call(args, None);

        Ok(())
    })
}
