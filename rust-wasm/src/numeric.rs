use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn matmul_sum(n: u32) -> f64 {
    let n = n as u32;
    const MOD: f64 = 1_000_000_007.0;
    let mut sum: f64 = 0.0;

    let mut i: u32 = 0;
    while i < n {
        let mut k: u32 = 0;
        while k < n {
            let a = ((i + k) % 10) as f64;
            let mut j: u32 = 0;
            while j < n {
                let b = ((k + j) % 10) as f64;
                sum += a * b;
                if sum >= MOD {
                    sum = sum % MOD;
                }
                j += 1;
            }
            k += 1;
        }
        i += 1;
    }

    sum % MOD
}

#[wasm_bindgen]
pub fn factorial_mod(n: u32) -> u32 {
    const MOD: u64 = 1_000_000_007;
    let mut res: u64 = 1;
    let mut i: u64 = 1;
    let limit = n as u64;

    while i <= limit {
        res = (res * i) % MOD;
        i += 1;
    }

    res as u32
}
