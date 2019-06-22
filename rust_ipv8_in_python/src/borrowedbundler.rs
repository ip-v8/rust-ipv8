use pyo3::prelude::*;
use rust_ipv8::serialization::Packet;
use pyo3::exceptions::{ValueError, KeyError};
use pyo3::types::PyBytes;

#[pymodule]
pub fn borrowed_bundler(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "borrowed_verify_signature")]
    fn borrowed_verify_signature<'py>(
        py: Python<'py>,
        _auth: &PyBytes,
        data: &PyBytes,
    ) -> PyResult<(bool, &'py PyBytes)> {
        // auth is not used as it is way faster to extract it again in rust
        let packet = Packet(data.as_bytes().to_vec());
        let mut de = packet
            .start_deserialize()
            .skip_header()
            .or(Err(ValueError::py_err("Could not ignore the header")))?;

        // release GIL during signature calculation
        let (result, de) = py.allow_threads(move || {
            let result = de.verify(); // For maximum efficiency this should be put on the threadpool
            (result, de)
        });

        Ok((result, PyBytes::new(py, &de.pntr.0[de.index..])))
    }

    #[pyfn(m, "borrowed_create_signature")]
    fn borrowed_create_signature<'py>(
        py: Python<'py>,
        key: &PyBytes,
        data: &PyBytes,
    ) -> PyResult<&'py PyBytes> {
        let p = Packet(data.as_bytes().to_vec());
        let ourkey = PrivateKey::from_vec(key.as_bytes().to_vec())
            .or(Err(KeyError::py_err("Invalid key given")))?;

        dbg!(key);
        let signed = p.sign(ourkey).or(Err(KeyError::py_err("Sign error")))?;

        Ok(PyBytes::new(py, &*signed.0))
    }

    Ok(())
}
