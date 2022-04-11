extern crate libc;

use libc::{c_int, c_double, c_void};

extern "C" {
    pub fn AtomicWeight(Z: c_int, error: *mut c_void) -> c_double;
}

#[cfg(test)]
mod tests {
	use std::ptr;
	use super::*;

	#[test]
	fn test_version_works() {
		unsafe {
			println!("{}", AtomicWeight(26, ptr::null_mut()));
			assert_eq!(AtomicWeight(26, ptr::null_mut()), 55.85);
		}
	}
}