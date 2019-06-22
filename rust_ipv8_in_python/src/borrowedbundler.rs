use pyo3::prelude::*;
use rust_ipv8::serialization::Packet;
use pyo3::exceptions::{ValueError, KeyError};
use pyo3::types::PyBytes;
use rust_ipv8::crypto::signature::KeyPair;
use rust_ipv8::util::as_fixed_size;

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
        seed: &PyBytes,
        key: &PyBytes,
        data: &PyBytes,
    ) -> PyResult<&'py PyBytes> {
        let p = Packet(data.as_bytes().to_vec());

        // Zerocopy that shiz
        let key_fixed: &[u8; 32] =
            as_fixed_size(key.as_bytes()).or(Err(KeyError::py_err("Key length was wrong")))?;
        let seed_fixed: &[u8; 32] =
            as_fixed_size(seed.as_bytes()).or(Err(KeyError::py_err("Seed length was wrong")))?;

        let ourkey = KeyPair::from_seed_checked(seed_fixed, key_fixed)
            .or(Err(KeyError::py_err("Invalid key given")))?;

        let signed = p.sign(&ourkey).or(Err(KeyError::py_err("Sign error")))?;

        Ok(PyBytes::new(py, &*signed.0))
    }

    Ok(())
}
