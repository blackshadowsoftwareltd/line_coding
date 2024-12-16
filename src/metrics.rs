pub fn calculate_average_power(signal: &Vec<i32>) -> f64 {
    signal.iter().map(|&x| (x * x) as f64).sum::<f64>() / signal.len() as f64
}

pub fn calculate_dc_component(signal: &Vec<i32>) -> f64 {
    signal.iter().sum::<i32>() as f64 / signal.len() as f64
}

pub fn calculate_transitions(signal: &Vec<i32>) -> usize {
    signal.windows(2).filter(|pair| pair[0] != pair[1]).count()
}
