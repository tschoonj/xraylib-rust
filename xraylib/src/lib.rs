#![allow(non_snake_case)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::{CStr, CString};
use std::ptr;

use ffi::xrl_error_free;

#[derive(Debug)]
pub struct Error {
    code: ffi::xrl_error_code,
    message: String,
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let code = match self.code {
            ffi::xrl_error_code_XRL_ERROR_MEMORY => "Memory error",
            ffi::xrl_error_code_XRL_ERROR_INVALID_ARGUMENT => "Invalid argument error",
            ffi::xrl_error_code_XRL_ERROR_IO => "I/O error",
            ffi::xrl_error_code_XRL_ERROR_TYPE => "Type error",
            ffi::xrl_error_code_XRL_ERROR_UNSUPPORTED => "Unsupported error",
            ffi::xrl_error_code_XRL_ERROR_RUNTIME => "Runtime error",
            _ => "Unknown error",
        };
        write!(f, "{}: {}", code, self.message)
    }
}

impl From<*mut ffi::xrl_error> for Error {
    fn from(error: *mut ffi::xrl_error) -> Self {
        if error.is_null() {
            panic!("Cannot create Error from null pointer!");
        }
        unsafe {
            let message: CString = CStr::from_ptr((*error).message).into();
            let message = message.to_str().unwrap().to_string();
            Error {
                code: (*error).code,
                message,
            }
        }
    }
}

pub fn AtomicWeight(Z: i32) -> Result<f64> {
    let mut xrl_error = ptr::null_mut();
    unsafe {
        let aw = ffi::AtomicWeight(Z, &mut xrl_error);
        if xrl_error.is_null() {
            Ok(aw)
        } else {
            let error: Error = xrl_error.into();
            xrl_error_free(xrl_error);
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use ffi::xrl_error_free;

    use super::*;
    use std::ptr;

    #[test]
    fn test_from_xrl_error() {
        let mut xrl_error = ptr::null_mut();
        unsafe {
            ffi::AtomicWeight(-3, &mut xrl_error);
            assert!(!xrl_error.is_null());

            let error: Error = xrl_error.into();
            xrl_error_free(xrl_error);
            assert_eq!(error.code, ffi::xrl_error_code_XRL_ERROR_INVALID_ARGUMENT);
            assert_eq!(error.message, "Z out of range");
            eprintln!("{}", error);
        }
    }
}
