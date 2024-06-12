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

    let data = data::PingpongData::init("data/transition_matrix_lite.csv").unwrap();

    let result = calc::suggest_best_action(&data, action).unwrap();
    // println!("{:?}", result);

    match calc::format_best_action(result.0, result.1, result.2) {
        Ok(json_str) => CString::new(json_str).unwrap().into_raw(),
        Err(e) => CString::new(format!("Error: {}", e)).unwrap().into_raw(),
    }
}

// This is present so it's easy to test that the code works natively in Rust via `cargo test`
#[cfg(test)]
pub mod test {

    use super::*;
    use ndarray::Array1;
    use std::{error::Error, ffi::CString};

    fn whisper_rust() {
        whisper(CString::new("(this is code from Rust)").unwrap().into_raw());
    }

    // This is meant to do the same stuff as the main function in the .go files
    #[test]
    fn simulated_main_function() {
        whisper_rust();
        println!("Best action:");
        whisper(get_best_action(CString::new("LB_1").unwrap().into_raw()));
    }

    #[test]
    pub fn calc_evaluate_strategy() -> Result<(), Box<dyn Error>> {
        whisper_rust();

        let data = data::PingpongData::init("data/transition_matrix.csv")?;

        // Example initial vector, as the initial state is S_2,
        // meaning the second player starts the game
        let mut initial_vector: Array1<f64> = Array1::zeros(data.states.len());
        if let Some(index) = data.states.iter().position(|s| s == "LB_1") {
            initial_vector[index] = 1.0;
        }

        // Evaluate the strategy
        calc::evaluate_strategy(&data.transition_matrix, &initial_vector, 4);

        Ok(())
    }
}
