//! Various utility functions to be used in conjunction with ipv8

use std::error::Error;

create_error!(ConversionError, "Converting to a fixed size array failed.");

/// Helper function types which have the [FromBytes](zerocopy::FromBytes) trait to be converted to some fixed size variant.
/// Doesn't copy.
pub fn as_fixed_size<T>(data: &[u8]) -> Result<&T, Box<dyn Error>>
where
    T: zerocopy::FromBytes,
{
    Ok(
        (zerocopy::LayoutVerified::<_, T>::new(data).ok_or_else(|| Box::new(ConversionError))?)
            .into_ref(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_fixed_size_valid() {
        let data = &[0u8, 1u8, 2u8];
        let fixed: [u8; 3] = *as_fixed_size(data).unwrap();
        assert_eq!(fixed, [0u8, 1u8, 2u8]);
    }

    #[test]
    fn test_as_fixed_size_invalid_too_large() {
        let data = &[0u8, 1u8, 2u8];
        let fixed: Result<&[u8; 4], Box<dyn Error>> = as_fixed_size(data);

        assert!(fixed.is_err());
    }

    #[test]
    fn test_as_fixed_size_invalid_too_small() {
        let data = &[0u8, 1u8, 2u8];
        let fixed: Result<&[u8; 2], Box<dyn Error>> = as_fixed_size(data);

        assert!(fixed.is_err());
    }

}
