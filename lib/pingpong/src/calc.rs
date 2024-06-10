extern crate ndarray;
use crate::data::PingpongData;
use ndarray::{s, Array1, Array2};
use std::error::Error;

#[no_mangle]
pub fn suggest_best_action(
    data: &PingpongData,
    action: &str,
) -> Result<(String, f64, Vec<(String, f64)>), Box<dyn Error>> {
    let initial_state_index = data.states.iter().position(|x| x == action);
    match initial_state_index {
        Some(index_val) => {
            let mut action_scores = vec![];
            let mut first_step_probs = data.transition_matrix.row(index_val).to_owned();
            first_step_probs = first_step_probs.slice_mut(s![2..]).to_owned(); // skip first two rows

            for (i, _first_prob) in first_step_probs.iter().enumerate() {
                let second_step_probs = data.transition_matrix.row(i);
                let scenario_score = calculate_scenario_score(data, &second_step_probs);
                action_scores.push((data.states[i + 2].clone(), scenario_score));
            }

            let (best_action, best_score) = action_scores
                .iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            Ok((best_action.clone(), *best_score, action_scores))
        }

        None => Err("Action not found in states.".into()),
    }
}

fn calculate_scenario_score(
    data: &PingpongData,
    second_step_probs: &ndarray::ArrayView1<f64>,
) -> f64 {
    // Direct scoring probability calculation
    let direct_score_index = data.states.iter().position(|x| x == "P_1").unwrap() - 2;
    let direct_score = second_step_probs[direct_score_index];

    let states_to_check = vec!["LF_2", "SB_2", "LB_2", "SF_2", "E/N_2", "CB_2"];
    let mut combined_score = direct_score;

    for state in states_to_check {
        let index = data.states.iter().position(|x| x == state).unwrap() - 2;
        let state_score = second_step_probs[index] * data.transition_matrix[[index, 13]];
        combined_score += state_score;
    }

    combined_score
}

// 迭代计算
#[no_mangle]
pub fn evaluate_strategy(
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
