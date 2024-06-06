extern crate ndarray;
use csv::ReaderBuilder;
use ndarray::{Array1, Array2};
use std::error::Error;

#[no_mangle]
pub fn read_csv_to_array2(filename: &str) -> Result<Array2<f64>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(filename)?;
    let mut data = Vec::new();
    let mut cols = 0;
    for result in reader.records() {
        let record = result?;
        let row: Vec<f64> = record.iter().map(|x| x.parse().unwrap()).collect();
        cols = row.len();
        data.extend(row);
    }
    let rows = data.len() / cols;
    Ok(Array2::from_shape_vec((rows, cols), data)?)
}

#[no_mangle]
pub fn calc(states: &[&str; 16], transition_matrix: &Array2<f64>) {
    // 传入参数：
    // 1. 定义状态名称
    // 2. 定义状态转移矩阵

    // 示例应用，从对手的发球开始，例如 "S_2"
    let mut initial_vector: Array1<f64> = Array1::zeros(states.len());
    if let Some(index) = states.iter().position(|&s| s == "S_2") {
        initial_vector[index] = 1.0;
    }

    // 迭代计算
    for _ in 0..3 {
        // 迭代次数
        initial_vector = initial_vector.dot(transition_matrix);
    }

    println!("收敛状态： {:?}", initial_vector);
}
