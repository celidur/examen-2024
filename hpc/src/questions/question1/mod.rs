#[cfg(test)]
mod tests;

pub fn single_threaded_computation(to_compute: &[u8]) -> Vec<f64> {
    let mut result = vec![];
    for &x in to_compute {
        let mut factorial = 1.0;
        for n in 2..=x {
            factorial *= n as f64;
        }
        result.push(factorial);
    }
    result
}

pub fn multi_threaded_computation(to_compute: &[u8]) -> Vec<f64> {
    to_compute
        .into_iter()
        .map(|&x| (2..=x).map(|n| n as f64).product::<f64>())
        .collect()
}
