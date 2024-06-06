extern crate ndarray;
use csv::ReaderBuilder;
use ndarray::{Array1, Array2};
use std::error::Error;

#[no_mangle]
pub fn calc() -> Result<(), Box<dyn Error>> {
    // 定义状态转移矩阵
    let transition_matrix = read_csv_to_array2("data/transition_matrix.csv")?;
    // 转换为概率矩阵
    let transition_matrix = transition_matrix.mapv(|x| x / 100.0);
    // println!("Original matrix: {:?}", transition_matrix);

    // 定义状态名称
    let states = [
        "S_1", "S_2", "SF_1", "SF_2", "LF_1", "LF_2", "SB_1", "SB_2", "LB_1", "LB_2", "CB_1",
        "CB_2", "E/N_1", "E/N_2", "P_1", "P_2",
    ];

    // 示例应用，从对手的发球开始，例如 "S_2"
    let mut initial_vector: Array1<f64> = Array1::zeros(states.len());
    if let Some(index) = states.iter().position(|&s| s == "S_2") {
        initial_vector[index] = 1.0;
    }

    // 迭代计算
    iterative_calc(&states, &transition_matrix, &initial_vector, 4);

    Ok(())
}

fn read_csv_to_array2(filename: &str) -> Result<Array2<f64>, Box<dyn Error>> {
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

fn iterative_calc(
    states: &[&str; 16],
    transition_matrix: &Array2<f64>,
    initial_vector: &Array1<f64>,
    turn: usize,
) {
    // 传入参数：
    // 1. 定义状态名称
    // 2. 定义状态转移矩阵
    // 3. 初始状态向量
    let mut initial_vector = initial_vector.clone();

    // 迭代计算
    for _ in 0..turn {
        // 迭代次数
        initial_vector = initial_vector.dot(transition_matrix);
    }

    println!("收敛状态：\n{:?}\n{:?}", states, initial_vector);
}
