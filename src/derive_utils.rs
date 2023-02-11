// Copyright (c) 2017-present PyO3 Project and Contributors
//
// based on Daniel Grunwald's https://github.com/dgrunwald/rust-cpython

//! Functionality for the code generated by the derive backend

use crate::{types::PyModule, Python};

/// Enum to abstract over the arguments of Python function wrappers.
pub enum PyFunctionArguments<'a> {
    Python(Python<'a>),
    PyModule(&'a PyModule),
}

impl<'a> PyFunctionArguments<'a> {
    pub fn into_py_and_maybe_module(self) -> (Python<'a>, Option<&'a PyModule>) {
        match self {
            PyFunctionArguments::Python(py) => (py, None),
            PyFunctionArguments::PyModule(module) => {
                let py = module.py();
                (py, Some(module))
            }
        }
    }
}

impl<'a> From<Python<'a>> for PyFunctionArguments<'a> {
    fn from(py: Python<'a>) -> PyFunctionArguments<'a> {
        PyFunctionArguments::Python(py)
    }
}

impl<'a> From<&'a PyModule> for PyFunctionArguments<'a> {
    fn from(module: &'a PyModule) -> PyFunctionArguments<'a> {
        PyFunctionArguments::PyModule(module)
    }
}
