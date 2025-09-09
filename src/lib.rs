// only enables the `doc_auto_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

mod decoder;
mod errors;
mod image;

pub use decoder::*;
pub use errors::*;
pub use image::*;

/// Returns a version of a `libde265` library as an array of version parts -
/// [major, minor, maintenance].
pub fn version() -> [u8; 3] {
    let major = unsafe { libde265_sys::de265_get_version_number_major() } as u8;
    let minor = unsafe { libde265_sys::de265_get_version_number_minor() } as u8;
    let maintenance = unsafe { libde265_sys::de265_get_version_number_maintenance() } as u8;
    [major, minor, maintenance]
}

pub fn disable_logging() {
    unsafe { libde265_sys::de265_disable_logging() };
}

pub fn set_verbosity(level: u8) {
    unsafe { libde265_sys::de265_set_verbosity(level as _) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = version();
        assert_eq!(version[0], 1);
        assert_eq!(version[1], 0);
    }
}
