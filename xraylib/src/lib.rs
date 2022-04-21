#![allow(non_snake_case)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(non_camel_case_types)]
#![feature(trace_macros)]

trace_macros!(true);

use std::ffi::{CStr, CString};
use std::os::raw;
use std::ptr;
use std::slice;

// re-export all symbols from xraylib-sys,
// to gain access to shell, line, etc constants
pub use ffi::*;

#[derive(Debug)]
pub struct Error {
    code: ffi::xrl_error_code,
    message: String,
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub struct compoundData {
    pub nElements: i32,
    pub nAtomsAll: f64,
    pub Elements: Vec<i32>,
    pub massFractions: Vec<f64>,
    pub nAtoms: Vec<f64>,
    pub molarMass: f64,
}

impl From<*mut ffi::compoundData> for compoundData {
    fn from(cd: *mut ffi::compoundData) -> Self {
        if cd.is_null() {
            panic!("Cannot create compoundData from null pointer!");
        }
        unsafe {
            compoundData {
                nElements: (*cd).nElements,
                nAtomsAll: (*cd).nAtomsAll,
                Elements: slice::from_raw_parts((*cd).Elements, (*cd).nElements as usize).to_vec(),
                massFractions: slice::from_raw_parts((*cd).massFractions, (*cd).nElements as usize)
                    .to_vec(),
                nAtoms: slice::from_raw_parts((*cd).nAtoms, (*cd).nElements as usize).to_vec(),
                molarMass: (*cd).molarMass,
            }
        }
    }
}

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

// pub fn AtomicWeight(Z: i32) -> Result<f64> {
//     let mut xrl_error = ptr::null_mut();
//     unsafe {
//         let aw = ffi::AtomicWeight(Z, &mut xrl_error);
//         if xrl_error.is_null() {
//             Ok(aw)
//         } else {
//             let error: Error = xrl_error.into();
//             xrl_error_free(xrl_error);
//             Err(error)
//         }
//     }
// }

macro_rules! wrap_xraylib_function {
    ($result_var:ident, $result_type:ty, $function:ident, $($args:ident)+, $($types:ty)+, $process_input1:stmt, $process_input2:stmt, $process_output1:stmt, $process_output2:stmt) => {
        pub fn $function($($args : $types,)*) -> Result<$result_type> {
            let mut xrl_error = ptr::null_mut();
            unsafe {
                $process_input1
                $process_input2
                let $result_var = ffi::$function($($args,)* &mut xrl_error);
                if xrl_error.is_null() {
                    $process_output1
                    $process_output2
                    Ok($result_var)
                } else {
                    let error: Error = xrl_error.into();
                    xrl_error_free(xrl_error);
                    Err(error)
                }
            }
        }
    };
}

fn process_output_c_string(ptr: *mut raw::c_char) -> String {
    unsafe {
        let rv = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        ffi::xrlFree(ptr as *mut raw::c_void);
        rv
    }
}

fn process_output_compound_data(ptr: *mut ffi::compoundData) -> compoundData {
    unsafe {
        let rv = ptr.into();
        ffi::FreeCompoundData(ptr);
        rv
    }
}

wrap_xraylib_function!(rv, f64, AtomicWeight, Z, i32, {}, {}, {}, {});
wrap_xraylib_function!(rv, f64, ComptonProfile, Z pz, i32 f64, {}, {}, {}, {});
wrap_xraylib_function!(rv, f64, ComptonProfile_Partial, Z shell pz, i32 i32 f64, {}, {}, {}, {});
wrap_xraylib_function!(rv, i32, SymbolToAtomicNumber, symbol, &str, let c_str = CString::new(symbol).unwrap(), let symbol = c_str.as_ptr() as *const raw::c_char, {}, {});
wrap_xraylib_function!(rv, f64, CS_Total_CP, compound E, &str f64, let c_str = CString::new(compound).unwrap(), let compound = c_str.as_ptr() as *const raw::c_char, {}, {});
wrap_xraylib_function!(rv, String, AtomicNumberToSymbol, Z, i32, {}, {}, let rv = process_output_c_string(rv), {});
wrap_xraylib_function!(rv, compoundData, CompoundParser, compound, &str, let c_str = CString::new(compound).unwrap(), let compound = c_str.as_ptr() as *const raw::c_char, let rv = process_output_compound_data(rv), {});

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
