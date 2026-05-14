fn lcg_next(value: u64) -> u64 {
    const A: u64 = 1664525;
    const C: u64 = 1013904223;
    const M: u64 = 1u64 << 32;
    (A.wrapping_mul(value).wrapping_add(C)) % M
}

fn max_subarray_sum(n: usize, seed: u64, min_val: i64, max_val: i64) -> i64 {
    let range = (max_val - min_val + 1) as u64;
    let mut value = seed;
    let mut random_numbers = Vec::with_capacity(n);
    
    for _ in 0..n {
        value = lcg_next(value);
        let rand_num = (value % range) as i64 + min_val;
        random_numbers.push(rand_num);
    }
    
    let mut max_sum = i64::MIN;
    
    for i in 0..n {
        let mut current_sum: i64 = 0;
        for j in i..n {
            current_sum += random_numbers[j];
            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }
    }
    
    max_sum
}

fn total_max_subarray_sum(n: usize, initial_seed: u64, min_val: i64, max_val: i64) -> i64 {
    let mut total_sum: i64 = 0;
    let mut value = initial_seed;
    
    for _ in 0..20 {
        value = lcg_next(value);
        total_sum += max_subarray_sum(n, value, min_val, max_val);
    }
    
    total_sum
}

fn main() {
    let n = 10000;
    let initial_seed = 42;
    let min_val = -10;
    let max_val = 10;
    
    let start = std::time::Instant::now();
    let result = total_max_subarray_sum(n, initial_seed, min_val, max_val);
    let duration = start.elapsed();
    
    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", duration.as_secs_f64());
}