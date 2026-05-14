fn lcg(mut seed: u32, a: u32, c: u32, m: u32) -> impl Iterator<Item = u32> {
    std::iter::repeat_with(move || {
        seed = (a * seed + c) % m;
        seed
    })
}

fn max_subarray_sum(random_numbers: &[u32]) -> i64 {
    let mut max_sum = std::i64::MIN;
    let n = random_numbers.len();
    for i in 0..n {
        let mut current_sum = 0;
        for j in i..n {
            current_sum += random_numbers[j] as i64;
            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }
    }
    max_sum
}

fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i64, max_val: i64) -> i64 {
    let a = 1664525;
    let c = 1013904223;
    let m = 2u32.pow(32);
    let mut total_sum = 0;
    let mut seed = initial_seed;
    
    for _ in 0..20 {
        let lcg_gen = lcg(seed, a, c, m);
        let random_numbers: Vec<u32> = lcg_gen.take(n).map(|num| ((num % (max_val - min_val + 1) as u32) + min_val as u32) as i64).clamp(-10, 10)).collect();
        total_sum += max_subarray_sum(&random_numbers);
        seed = lcg_gen.last().copied().unwrap_or(seed); // Ensure we get a new seed for the next iteration
    }
    total_sum
}

fn main() {
    let n = 10000;         // Number of random numbers
    let initial_seed = 42; // Initial seed for the LCG
    let min_val = -10;     // Minimum value of random numbers
    let max_val = 10;      // Maximum value of random numbers

    let start_time = std::time::Instant::now();
    let result = total_max_subarray_sum(n, initial_seed, min_val, max_val);
    let end_time = std::time::Instant::now();

    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", end_time.elapsed().as_secs_f64());
}