#![deny(unused_extern_crates)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![cfg_attr(feature = "cargo-clippy", warn(cast_ptr_alignment))]

#[cfg(feature = "from_image")]
extern crate image as image_crate;
#[macro_use]
extern crate log;

pub use ffi::{
    zbar_color_e as ZBarColor,
    zbar_config_e as ZBarConfig,
    zbar_error_e as ZBarError,
    zbar_symbol_type_e as ZBarSymbolType
};
#[cfg(feature = "zbar_fork")]
pub use ffi::{
    zbar_modifier_e as ZBarModifier,
    zbar_orientation_e as ZBarOrientation
};
use std::{
    error::Error,
    ffi::{
        CStr,
        OsString,
    },
    fmt,
    mem,
    os::raw::{
        c_char,
        c_void
    },
};

pub mod decoder;
#[allow(dead_code)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy))]
mod ffi;
pub mod format;
pub mod image;
pub mod symbol;
pub mod symbol_set;
pub mod image_scanner;
pub mod processor;
pub mod prelude;

pub type ZBarResult<T> = Result<T, ZBarErrorType>;

#[derive(Debug)]
pub enum ZBarErrorType {
    Simple(i32),
    Complex(ZBarError)
}
impl Error for ZBarErrorType {}
impl fmt::Display for ZBarErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ZBarError::*;

        match *self {
            ZBarErrorType::Simple(e)  => write!(f, "ZBar simple error {}", e),
            ZBarErrorType::Complex(e) => match e {
                ZBAR_ERR_NOMEM => write!(f, "out of memory"),
                ZBAR_ERR_INTERNAL => write!(f, "internal library error"),
                ZBAR_ERR_UNSUPPORTED => write!(f, "unsupported request"),
                ZBAR_ERR_INVALID => write!(f, "invalid request"),
                ZBAR_ERR_LOCKING => write!(f, "system error"),
                ZBAR_ERR_SYSTEM => write!(f, "locking error"),
                ZBAR_ERR_BUSY => write!(f, "all resources busy "),
                ZBAR_ERR_XDISPLAY => write!(f, "X11 display error"),
                ZBAR_ERR_XPROTO => write!(f, "X11 protocol error"),
                ZBAR_ERR_CLOSED => write!(f, "output window is closed"),
                ZBAR_ERR_WINAPI => write!(f, "windows system error"),
                ZBAR_ERR_NUM => write!(f, "number of error codes"),
                ZBAR_OK => write!(f, "success")
             }
        }
    }
}

impl From<i32> for ZBarErrorType {
    fn from(error: i32) -> Self { ZBarErrorType::Complex(unsafe { mem::transmute(error) } ) }
}

pub fn version() -> (u32, u32, u32) {
    unsafe {
        let mut version = (0, 0, 0);
        ffi::zbar_version(
            &mut version.0 as *mut u32, &mut version.1 as *mut u32, &mut version.2 as *mut u32);
        version
    }
}

pub fn set_verbosity(verbosity: i32) {
    unsafe { ffi::zbar_set_verbosity(verbosity) }
}

pub fn increase_verbosity() {
    unsafe { ffi::zbar_increase_verbosity() }
}

pub fn symbol_name(symbol_type: ZBarSymbolType) -> &'static str {
    unsafe { from_cstr(ffi::zbar_get_symbol_name(symbol_type)) }
}

#[cfg(feature = "zbar_fork")]
pub fn config_name(config: ZBarConfig) -> &'static str {
    unsafe { from_cstr(ffi::zbar_get_config_name(config)) }
}

#[cfg(feature = "zbar_fork")]
pub fn modifier_name(modifier: ZBarModifier) -> &'static str {
    unsafe { from_cstr(ffi::zbar_get_modifier_name(modifier)) }
}

#[cfg(feature = "zbar_fork")]
pub fn orientation_name(orientation: ZBarOrientation) -> &'static str {
    unsafe { from_cstr(ffi::zbar_get_orientation_name(orientation)) }
}

pub fn parse_config(config_string: impl AsRef<str>) -> ZBarResult<(ZBarSymbolType, ZBarConfig, i32)> {
    let mut symbol_type = ZBarSymbolType::ZBAR_NONE;
    let mut config = ZBarConfig::ZBAR_CFG_ENABLE;
    let mut value = 0;
    unsafe {
        match ffi::zbar_parse_config(
            as_char_ptr(config_string),
            &mut symbol_type as *mut ZBarSymbolType,
            &mut config as *mut ZBarConfig,
            &mut value as *mut i32,
        ) {
            0 => Ok((symbol_type, config, value)),
            e => Err(ZBarErrorType::Simple(e))
        }
    }
}

pub(crate) unsafe fn error_code(object: *const c_void) -> ZBarError {
    ffi::_zbar_get_error_code(object)
}

//pub fn addon_name()

unsafe fn as_char_ptr(value: impl AsRef<str>) -> *const i8 {
    OsString::from(value.as_ref()).to_str().unwrap().as_ptr() as *const i8
}

unsafe fn from_cstr(ptr: *const c_char) -> &'static str { CStr::from_ptr(ptr).to_str().unwrap() }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_version() {
        let (major, minor) = version();
        assert_ne!(major + minor, 0);
    }

    #[test]
    fn test_symbol_name() {
        assert_eq!(symbol_name(ZBarSymbolType::ZBAR_QRCODE), "QR-Code");
        assert_eq!(symbol_name(ZBarSymbolType::ZBAR_CODE128), "CODE-128");
    }

    #[test]
    fn test_parse_config() {
        assert_eq!(
            parse_config("qrcode.enable=1").unwrap(),
            (ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
        );
    }

    #[test]
    fn test_parse_config_err() {
        assert!(parse_config("Not valid").is_err());
    }
}
