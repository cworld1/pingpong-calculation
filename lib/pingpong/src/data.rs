use csv::ReaderBuilder;
use ndarray::Array2;
use std::error::Error;

pub struct PingpongData {
    pub transition_matrix: Array2<f64>,
    pub states: Vec<String>,
}

// Impletions for init function
pub trait Init<T> {
    fn init(input: T) -> Result<PingpongData, Box<dyn Error>>;
}
impl Init<&str> for PingpongData {
    fn init(file: &str) -> Result<PingpongData, Box<dyn Error>> {
        let transition_matrix = Self::read_csv_to_array2(file)?;
        // 转换为概率矩阵
        let transition_matrix = transition_matrix.mapv(|x| x / 100.0);
        // println!("Original matrix: {:?}", transition_matrix);

        // 定义状态名称
        let states = vec![
            "S_1", "S_2", "SF_1", "SF_2", "LF_1", "LF_2", "SB_1", "SB_2", "LB_1", "LB_2", "CB_1",
            "CB_2", "E/N_1", "E/N_2", "P_1", "P_2",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        Ok(PingpongData {
            transition_matrix,
            states,
        })
    }
}
impl Init<()> for PingpongData {
    fn init(_: ()) -> Result<PingpongData, Box<dyn Error>> {
        Self::init("data/transition_matrix.csv")
    }
}

impl PingpongData {
    // 读取 CSV 文件到二维数组
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
}
