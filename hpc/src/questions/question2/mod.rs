use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests;

fn get_first_digit(number: f64) -> u64 {
    (number / 10f64.powi(number.log10().floor() as i32)) as u64
}

pub fn single_threaded_computation(to_compute: &[f64]) -> u64 {
    let mut sum = 0;
    for &x in to_compute {
        sum += get_first_digit(x.exp());
    }
    sum
}

pub fn multi_threaded_computation(to_compute: &[f64]) -> u64 {
    let sum = Arc::new(Mutex::new(0));
    assert!(to_compute.len() >= 8);
    assert!(to_compute.len() % 8 == 0);

    to_compute
        .chunks_exact(to_compute.len() / 8)
        .map(|chunk| {
            let sum_clone = Arc::clone(&sum);
            let chunk = chunk.to_vec();
            std::thread::spawn(move || {
                let chunk_sum: u64 = chunk
                    .into_iter()
                    .map(|x| get_first_digit(x.exp()))
                    .sum::<u64>();
                let mut sum = sum_clone.lock().unwrap();
                *sum += chunk_sum;
            })
        })
        .for_each(|handle| handle.join().unwrap());
    let sum: u64 = *sum.lock().unwrap();
    sum
}
