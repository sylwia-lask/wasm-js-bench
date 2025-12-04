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

#[wasm_bindgen]
pub fn blur_rgba(width: u32, height: u32, input: &[u8]) -> Vec<u8> {
    let width = width as i32;
    let height = height as i32;
    let mut output = vec![0u8; input.len()];

    let kernel_radius = 1;
    let kernel_size = 2 * kernel_radius + 1;
    let kernel_area = (kernel_size * kernel_size) as f32;

    for y in 0..height {
        for x in 0..width {
            let mut r: f32 = 0.0;
            let mut g: f32 = 0.0;
            let mut b: f32 = 0.0;
            let mut a: f32 = 0.0;

            for ky in -kernel_radius..=kernel_radius {
                let ny = (y + ky).clamp(0, height - 1);
                for kx in -kernel_radius..=kernel_radius {
                    let nx = (x + kx).clamp(0, width - 1);
                    let idx = ((ny * width + nx) * 4) as usize;
                    r += input[idx] as f32;
                    g += input[idx + 1] as f32;
                    b += input[idx + 2] as f32;
                    a += input[idx + 3] as f32;
                }
            }

            let out_idx = ((y * width + x) * 4) as usize;
            output[out_idx] = (r / kernel_area) as u8;
            output[out_idx + 1] = (g / kernel_area) as u8;
            output[out_idx + 2] = (b / kernel_area) as u8;
            output[out_idx + 3] = (a / kernel_area) as u8;
        }
    }

    output
}
