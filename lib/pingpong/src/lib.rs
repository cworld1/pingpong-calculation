use std::ffi::CStr;
use std::ffi::CString;
mod calc;

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
        // 定义状态转移矩阵
        let transition_matrix = calc::read_csv_to_array2("data/transition_matrix.csv")?;
        // 转换为概率矩阵
        let transition_matrix = transition_matrix.mapv(|x| x / 100.0);
        // println!("Original matrix: {:?}", transition_matrix);

        // 定义状态名称
        let states = [
            "S_1", "S_2", "SF_1", "SF_2", "LF_1", "LF_2", "SB_1", "SB_2", "LB_1", "LB_2", "CB_1",
            "CB_2", "E/N_1", "E/N_2", "P_1", "P_2",
        ];

        calc::calc(&states, &transition_matrix); // 调用函数
        Ok(())
    }
}
