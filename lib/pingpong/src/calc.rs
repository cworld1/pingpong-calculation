extern crate ndarray;
use crate::data::{Init, PingpongData};
use ndarray::{Array1, Array2};
use std::error::Error;

#[no_mangle]
pub fn calc() -> Result<(), Box<dyn Error>> {
    let data = PingpongData::init("data/transition_matrix.csv")?;

    // 示例应用，从对手的发球开始，例如 "S_2"
    let mut initial_vector: Array1<f64> = Array1::zeros(data.states.len());
    if let Some(index) = data.states.iter().position(|s| s == "S_2") {
        initial_vector[index] = 1.0;
    }

    // 迭代计算
    evaluate_strategy(&data.transition_matrix, &initial_vector, 4);

    Ok(())
}

// 迭代计算
fn evaluate_strategy(
    transition_matrix: &Array2<f64>,
    initial_vector: &Array1<f64>,
    num_steps: usize,
) {
    // 传入参数：
    // 1. 定义状态转移矩阵
    // 2. 初始状态向量
    let mut initial_vector = initial_vector.clone();

    if num_steps == 1 {
        initial_vector = initial_vector.dot(transition_matrix);
    } else {
        for _ in 0..num_steps {
            initial_vector = initial_vector.dot(transition_matrix);
        }
    }
    println!("Convergent state: \n{:?}", initial_vector);
}
