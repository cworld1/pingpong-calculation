use std::ffi::CStr;
use std::ffi::CString;
mod calc;
mod data;

#[no_mangle]
pub extern "C" fn pingpong(name: *const libc::c_char) {
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name = name_cstr.to_str().unwrap();
    println!("Hello {}!", name);
}

#[no_mangle]
pub extern "C" fn return_str(message: *const libc::c_char) -> *const libc::c_char {
    let message_cstr = unsafe { CStr::from_ptr(message) };
    let message = message_cstr.to_str().unwrap();

    CString::new(message).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn whisper(message: *const libc::c_char) {
    let message_cstr = unsafe { CStr::from_ptr(message) };
    let message = message_cstr.to_str().unwrap();
    println!("({})", message);
}

// This is present so it's easy to test that the code works natively in Rust via `cargo test`
#[cfg(test)]
pub mod test {

    use super::*;
    use std::{error::Error, ffi::CString};

    // This is meant to do the same stuff as the main function in the .go files
    #[test]
    fn simulated_main_function() {
        pingpong(CString::new("world").unwrap().into_raw());
        whisper(CString::new("this is code from Rust").unwrap().into_raw());
        whisper(return_str(CString::new("result").unwrap().into_raw()));
    }

    #[test]
    fn calc_main_function() -> Result<(), Box<dyn Error>> {
        calc::calc()
    }
}
