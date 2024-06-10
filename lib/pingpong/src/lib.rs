use std::ffi::CStr;
use std::ffi::CString;

use data::Init;
mod calc;
mod data;

#[no_mangle]
pub extern "C" fn whisper(message: *const libc::c_char) {
    let message_cstr = unsafe { CStr::from_ptr(message) };
    let message = message_cstr.to_str().unwrap();
    println!("{}", message);
}

#[no_mangle]
pub extern "C" fn get_best_action(action: *const libc::c_char) -> *const libc::c_char {
    let action_cstr = unsafe { CStr::from_ptr(action) };
    let action = action_cstr.to_str().unwrap();

    let data = data::PingpongData::init("data/transition_matrix.csv").unwrap();
    let result = calc::suggest_best_action(&data, action).unwrap();

    // CString::new(result).unwrap().into_raw()
    CString::new(format!("{:?}", result)).unwrap().into_raw()
}

// This is present so it's easy to test that the code works natively in Rust via `cargo test`
#[cfg(test)]
pub mod test {

    use super::*;
    use ndarray::Array1;
    use std::{error::Error, ffi::CString};

    // This is meant to do the same stuff as the main function in the .go files
    #[test]
    fn simulated_main_function() {
        whisper(get_best_action(CString::new("SF_2").unwrap().into_raw()));
        whisper(CString::new("(this is code from Rust)").unwrap().into_raw());
    }

    #[test]
    pub fn calc_evaluate_strategy() -> Result<(), Box<dyn Error>> {
        let data = data::PingpongData::init("data/transition_matrix.csv")?;

        // 示例应用，从对手的发球开始，例如 "S_2"
        let mut initial_vector: Array1<f64> = Array1::zeros(data.states.len());
        if let Some(index) = data.states.iter().position(|s| s == "S_2") {
            initial_vector[index] = 1.0;
        }

        // 迭代计算
        calc::evaluate_strategy(&data.transition_matrix, &initial_vector, 4);

        Ok(())
    }
}
