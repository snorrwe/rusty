#![feature(proc_macro)]

extern crate pyo3;

use pyo3::prelude::{Python, PyModule, PyResult, PyTuple, PyDict};
use pyo3::py::modinit;

/// Module documentation string
/// Cont....
#[modinit(rusty)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "run", args = "*", kwargs = "**")]
    fn run_fn(_py: Python, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<()> {
        run(args, kwargs)
    }

    #[pyfn(m, "val")]
    fn val(_py: Python) -> PyResult<i32> {
        Ok(42)
    }

    Ok(())
}

fn run(args: &PyTuple, kwargs: Option<&PyDict>)->PyResult<()> {
    println!("Rust says hello! \\o");
    for arg in args.iter() {
        println!("Got arg: {}", arg);
    }

    if let Some(kwargs) = kwargs {

        for (key,val) in kwargs.iter() {
            println!("Got karg: {}: {}", key, val);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
