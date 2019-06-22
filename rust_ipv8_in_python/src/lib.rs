pub mod borrowedbundler;

use pyo3::prelude::*;
use pyo3::{wrap_pymodule};
use borrowedbundler::*;

//    fn get_available_formats(&self) -> PyResult<Vec<&str>> {
//        Ok(FORMATS.to_vec())
//    }
//
//    fn get_packer_for(&self, _name: &str) -> PyResult<()> {
//        unimplemented!("As this method is only called in a test. We do not implement it.")
//    }
//
//    fn add_packing_format(&self, _name: &str, _fmt: &PyAny) -> PyResult<()> {
//        unimplemented!("As this method is only called in a test. We do not implement it.")
//    }
//
//    fn pack<'py>(self, py: Python<'py>, format: &str, data: &PyAny) -> PyResult<&'py PyBytes> {
//        match format {
//            // boolean encoded as byte
//            "?" => {
//                if py.is_instance::<PyBool, _>(data)? {
//                    if
//                    Ok(PyBytes::new(py, &[]))
//                } else {
//                    Err(TypeError::py_err("yeet"))
//                }
//            }
//            _ => Err(ValueError::py_err("yeet")),
//        }
//    }
//}

#[pymodule]
pub fn rust_ipv8_in_python(_py: Python, m: &PyModule) -> PyResult<()> {
    //    m.add_class::<BorrowedBundler>()?;
    m.add_wrapped(wrap_pymodule!(borrowed_bundler))?;
    Ok(())
}
