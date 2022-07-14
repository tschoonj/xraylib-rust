#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// silence cargo about dereferencing null pointers in automatically generated tests
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_call_without_excepion_handling() {
        unsafe {
            println!("{}", AtomicWeight(26, ptr::null_mut()));
            assert_eq!(AtomicWeight(26, ptr::null_mut()), 55.85);
        }
    }

    #[test]
    fn test_calls_with_excepion_handling_no_error() {
        let mut error = ptr::null_mut();
        unsafe {
            let aw = AtomicWeight(26, &mut error);
            assert!(error.is_null());

            println!("{}", aw);
            assert_eq!(aw, 55.85);
        }
    }

    #[test]
    fn test_calls_with_excepion_handling_with_error() {
        let mut error = ptr::null_mut();
        unsafe {
            let aw = AtomicWeight(-3, &mut error);
            assert!(!error.is_null());
            assert_eq!((*error).code, xrl_error_code_XRL_ERROR_INVALID_ARGUMENT);

            println!("{}", aw);
            assert_eq!(aw, 0.0);
            xrl_clear_error(&mut error);
            assert!(error.is_null());
        }
    }
}
