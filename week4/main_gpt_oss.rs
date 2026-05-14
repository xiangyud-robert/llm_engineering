use std::time::Instant;

const A: u32 = 1664525;
const C: u32 = 1013904223;

#[derive(Clone, Copy)]
struct LCG {
    value: u32,
}

impl LCG {
    fn new(seed: u32) -> Self {
        Self { value: seed }
    }

    #[inline(always)]
    fn next(&mut self) -> u32 {
        // Linear congruential generator (LCG)
        self.value = self.value.wrapping_mul(A).wrapping_add(C);
        self.value
    }
}

/// Generate an array of `n` integers in the range [min_val, max_val] inclusively.
/// The array is generated deterministically using an LCG seeded with `seed`.
fn generate_random_numbers(seed: u32, n: usize, min_val: i64, max_val: i64) -> Vec<i64> {
    let mut rng = LCG::new(seed);
    let range_size = (max_val - min_val + 1) as u32; // positive value

    let mut numbers = Vec::with_capacity(n);
    for _ in 0..n {
        let val = rng.next() % range_size;
        numbers.push(val as i64 + min_val);
    }
    numbers
}

/// Compute the maximum sub‑array sum of `nums` using Kadane’s algorithm.
fn max_subarray_sum(nums: &[i64]) -> i64 {
    let mut current = 0_i64;
    let mut best = std::i64::MIN;

    for &v in nums {
        current += v;
        if current > best {
            best = current;
        }
        if current < 0 {
            current = 0;
        }
    }
    best
}

/// Perform 20 random runs and sum the best sub‑array sum from each run.
fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i64, max_val: i64) -> i64 {
    let mut lcg = LCG::new(initial_seed);
    let mut total = 0_i64;

    for _ in 0..20 {
        let seed = lcg.next();
        let numbers = generate_random_numbers(seed, n, min_val, max_val);
        let best = max_subarray_sum(&numbers);
        total += best;
    }
    total
}

fn main() {
    // Parameters identical to the original Python example
    const N: usize = 1000;
    const INITIAL_SEED: u32 = 12345;   // Starting seed for the LCG
    const MIN_VAL: i64 = -10;          // Minimum random value
    const MAX_VAL: i64 = 10;           // Maximum random value

    let start = Instant::now();
    let result = total_max_subarray_sum(N, INITIAL_SEED, MIN_VAL, MAX_VAL);
    let elapsed = start.elapsed();

    // Convert elapsed time to seconds with fractional part
    let elapsed_seconds =
        elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000.0;

    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", elapsed_seconds);
}