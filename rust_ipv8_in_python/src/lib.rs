use ipv8;
use ipv8::serialization::Packet;
use pyo3::exceptions::{TypeError, ValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyBool, PyBytes};

//pub struct ListenerProxy{
//    obj: PyObject
//}
//
//impl Receiver for ListenerProxy{
//    fn on_receive(&self, packet: Packet, address: Address){
//        let gil = Python::acquire_gil(); // Acquire the GIL so that we can call some python methods
//        let py = gil.python(); // python is a 0 size marker struct with a lifetime of the python interpreter
//        self.obj.call_method1(py, "on_packet", (packet,address); // `call_method1` is with only args
//    }
//}

static FORMATS: [&str; 31] = [
    "?",
    "B",
    "BBH",
    "BH",
    "c",
    "f",
    "d",
    "H",
    "HH",
    "I",
    "l",
    "LL",
    "Q",
    "QH",
    "QL",
    "QQHHBH",
    "ccB",
    "4SH",
    "20s",
    "32s",
    "64s",
    "74s",
    "c20s",
    "bits",
    "raw",
    "varlenBx2",
    "varlenH",
    "varlenHx20",
    "varlenI",
    "doublevarlenH",
    "payload",
];

#[pyclass]
#[derive(Copy, Clone)]
pub struct Serializer {}

#[pymethods]
impl Serializer {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(Serializer {})
    }

    fn get_available_formats(&self) -> PyResult<Vec<&str>> {
        Ok(FORMATS.to_vec())
    }

    fn get_packer_for<'py>(&self, _name: &str) -> PyResult<()> {
        unimplemented!("As this method is only called in a test. We do not implement it.");
        Ok(())
    }

    fn add_packing_format(&self, _name: &str, _fmt: &PyAny) -> PyResult<()> {
        unimplemented!("As this method is only called in a test. We do not implement it.");
        Ok(())
    }

    fn pack<'py>(self, py: Python<'py>, format: &str, data: &PyAny) -> PyResult<&'py PyBytes> {
        match format {
            // boolean encoded as byte
            "?" => {
                if py.is_instance::<PyBool, _>(data)? {
                    Ok(PyBytes::new(py, &[]))
                } else {
                    Err(TypeError::py_err("yeet"))
                }
            }
            _ => Err(ValueError::py_err("yeet")),
        }
    }
}

#[pymodule]
pub fn rust_ipv8_in_python(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Serializer>()?;
    Ok(())
}

//    #[pyclass]
//    pub struct Endpoint{
//        listeners: HashSet<ListenerProxy>
//    }
//
//    #[pymethods]
//    impl Endpoint {
//        #[new]
//        fn new(obj: &PyRawObject) {
//            obj.init(
//                Endpoint {
//                    listeners: HashSet::new()
//                }
//            )
//        }
//
//        fn add_listener(&mut self, listener: PyObject) -> PyResult<()> {
//            let proxy = ListenerProxy {
//                obj: (listener),
//
//            };
//
//            self.listeners.insert(proxy);
//        }
//
//        fn remove_listener(&self, listener: PyObject) -> PyResult<()> {
//
//        }
//    }
