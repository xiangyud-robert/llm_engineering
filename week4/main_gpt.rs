use std::time::Instant;

#[inline(always)]
fn lcg_step(x: u32) -> u32 {
    x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223)
}

#[inline(always)]
fn max_subarray_sum(n: usize, seed: u32, min_val: i64, max_val: i64) -> i128 {
    let width_i128 = (max_val as i128) - (min_val as i128) + 1;
    let use_u32_mod = width_i128 > 0 && width_i128 <= (u32::MAX as i128);

    let mut value = seed;
    let mut current_sum: i128 = 0;
    let mut max_sum: i128 = i128::MIN;

    if use_u32_mod {
        let width_u32 = width_i128 as u32;
        for _ in 0..n {
            value = lcg_step(value);
            let r = (value % width_u32) as i64 + min_val;
            let x = r as i128;
            current_sum += x;
            if current_sum > max_sum { max_sum = current_sum; }
            if current_sum < 0 { current_sum = 0; }
        }
    } else {
        // Fallback for very large ranges
        let width_u128 = width_i128 as u128;
        for _ in 0..n {
            value = lcg_step(value);
            let r = ((value as u128 % width_u128) as i128 + min_val as i128) as i128;
            current_sum += r;
            if current_sum > max_sum { max_sum = current_sum; }
            if current_sum < 0 { current_sum = 0; }
        }
    }

    max_sum
}

#[inline(always)]
fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i64, max_val: i64) -> i128 {
    let mut total_sum: i128 = 0;
    let mut seed = initial_seed;
    for _ in 0..20 {
        seed = lcg_step(seed);
        total_sum += max_subarray_sum(n, seed, min_val, max_val);
    }
    total_sum
}

fn main() {
    // Parameters
    let n: usize = 10000;
    let initial_seed: u32 = 42;
    let min_val: i64 = -10;
    let max_val: i64 = 10;

    // Timing the function
    let start_time = Instant::now();
    let result = total_max_subarray_sum(n, initial_seed, min_val, max_val);
    let duration = start_time.elapsed().as_secs_f64();

    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6f} seconds", duration);
}