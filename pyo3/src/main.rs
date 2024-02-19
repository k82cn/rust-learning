use std::fs::File;
use std::io::prelude::*;

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let dill = py.import("dill")?;
        let load_fn = dill.getattr("loads")?;

        println!("Load task code");
        let mut f: File = File::open("service_code.pkl")?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        let task_code = PyBytes::new(py, &buffer);
        let task_code = PyTuple::new(py, &[task_code]);
        let task_fn: Py<PyAny> = load_fn.call(task_code, None)?.into();

        println!("Load task args");
        let mut f = File::open("service_args.pkl")?;
        let mut task_args = Vec::new();
        f.read_to_end(&mut task_args)?;

        let task_args = PyBytes::new(py, &task_args);
        let task_args = PyTuple::new(py, &[task_args]);
        let args = load_fn.call(task_args, None)?;
        let args: &PyTuple = args.extract()?;

        println!("Load task kwargs");
        let mut f = File::open("service_kwargs.pkl")?;
        let mut task_kwargs = Vec::new();
        f.read_to_end(&mut task_kwargs)?;

        let task_kwargs = PyBytes::new(py, &task_kwargs);
        let task_kwargs = PyTuple::new(py, &[task_kwargs]);
        let kwargs = load_fn.call(task_kwargs, None)?;
        let kwargs = kwargs.extract()?;

        println!("Execute task");
        let main = py.import("__main__")?;
        let globals = main.dict();

        let random = py.import("random")?;
        globals.set_item("random", random)?;

        let math = py.import("math")?;
        globals.set_item("math", math)?;

        let res = task_fn.call(py, args, Some(kwargs))?;

        println!("{}", res);

        let dump_fn = dill.getattr("dumps")?;
        let res = PyTuple::new(py, &[res]);
        let _: Py<PyAny> = dump_fn.call(res, None)?.into();

        Ok(())
    })
}
