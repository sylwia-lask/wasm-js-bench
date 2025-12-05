use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

#[wasm_bindgen]
pub fn process_image_wasm(width: u32, height: u32, data: Clamped<Vec<u8>>) -> Clamped<Vec<u8>> {
    let w = width as usize;
    let h = height as usize;
    let mut img = data.0;

    if img.len() != w * h * 4 {
        return Clamped(img);
    }

    let mut tmp = img.clone();

    box_blur(&img, &mut tmp, w, h);
    gaussian_blur(&tmp, &mut img, w, h);
    grayscale_inplace(&mut img, w, h);

    let down_w = (w / 2).max(1);
    let down_h = (h / 2).max(1);
    let mut down = vec![0u8; down_w * down_h * 4];
    resize_nearest(&img, w, h, &mut down, down_w, down_h);

    let mut up = vec![0u8; w * h * 4];
    resize_nearest(&down, down_w, down_h, &mut up, w, h);

    wave_distortion(&up, &mut img, w, h);

    let base_after_wave = img.clone();

    for _ in 0..4 {
        sobel_edges(&img, &mut tmp, w, h);
        nonlinear_edges_inplace(&mut tmp, w, h);
        blend_edges(&mut img, &tmp, w, h);
    }

    intensity_hash_pass(&mut img, w, h);
    final_blend_with_base(&mut img, &base_after_wave, w, h);

    Clamped(img)
}

fn nonlinear_edges_inplace(edges: &mut [u8], w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) * 4;
            let v = edges[idx] as f32 / 255.0;

            let mut acc = v;
            for _ in 0..8 {
                let s = (acc * 12.0).sin().abs();
                acc = (acc * 0.85 + s).sqrt();
                if !acc.is_finite() {
                    acc = 0.0;
                    break;
                }
                if acc > 1.0 {
                    acc = 1.0;
                }
            }

            let out = (acc * 255.0).clamp(0.0, 255.0) as u8;
            edges[idx] = out;
            edges[idx + 1] = out;
            edges[idx + 2] = out;
        }
    }
}

fn pixel_hash_mix(seed: u32) -> u8 {
    const MOD: u64 = 1_000_000_007;
    const ROUNDS: u64 = 64;

    let mut acc: u64 = (seed as u64 + 1) % MOD;
    let s = seed as u64 % MOD;

    let mut i: u64 = 1;
    while i <= ROUNDS {
        let factor = (i + s) % MOD;
        acc = (acc * factor + i * 17) % MOD;
        i += 1;
    }

    (acc & 0xFF) as u8
}

fn intensity_hash_pass(img: &mut [u8], w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) * 4;

            let r = img[idx] as u32;
            let g = img[idx + 1] as u32;
            let b = img[idx + 2] as u32;

            let seed = (r + (g << 8) + (b << 16)) % 1000;
            let m = pixel_hash_mix(seed) as u32;

            let scale = 128 + m;

            let nr = ((r * scale) / 255).min(255) as u8;
            let ng = ((g * scale) / 255).min(255) as u8;
            let nb = ((b * scale) / 255).min(255) as u8;

            img[idx] = nr;
            img[idx + 1] = ng;
            img[idx + 2] = nb;
        }
    }
}

fn final_blend_with_base(img: &mut [u8], base: &[u8], w: usize, h: usize) {
    let alpha = 0.6f32;
    let beta = 1.0f32 - alpha;

    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) * 4;

            let br = base[idx] as f32;
            let bg = base[idx + 1] as f32;
            let bb = base[idx + 2] as f32;

            let pr = img[idx] as f32;
            let pg = img[idx + 1] as f32;
            let pb = img[idx + 2] as f32;

            img[idx] = (alpha * br + beta * pr).clamp(0.0, 255.0) as u8;
            img[idx + 1] = (alpha * bg + beta * pg).clamp(0.0, 255.0) as u8;
            img[idx + 2] = (alpha * bb + beta * pb).clamp(0.0, 255.0) as u8;
        }
    }
}

fn get_pixel(src: &[u8], w: usize, h: usize, x: isize, y: isize) -> [u8; 4] {
    let xx = x.clamp(0, (w - 1) as isize) as usize;
    let yy = y.clamp(0, (h - 1) as isize) as usize;
    let idx = (yy * w + xx) * 4;
    [src[idx], src[idx + 1], src[idx + 2], src[idx + 3]]
}

fn box_blur(src: &[u8], dst: &mut [u8], w: usize, h: usize) {
    let kernel = [1.0f32; 9];
    convolve_3x3(src, dst, w, h, &kernel, 1.0 / 9.0);
}

fn gaussian_blur(src: &[u8], dst: &mut [u8], w: usize, h: usize) {
    let kernel = [1.0, 2.0, 1.0, 2.0, 4.0, 2.0, 1.0, 2.0, 1.0];
    convolve_3x3(src, dst, w, h, &kernel, 1.0 / 16.0);
}

fn convolve_3x3(src: &[u8], dst: &mut [u8], w: usize, h: usize, kernel: &[f32; 9], norm: f32) {
    for y in 0..h {
        for x in 0..w {
            let mut r = 0.0f32;
            let mut g = 0.0f32;
            let mut b = 0.0f32;
            let mut a = 0.0f32;

            let mut k = 0;
            for ky in -1..=1 {
                for kx in -1..=1 {
                    let p = get_pixel(src, w, h, x as isize + kx, y as isize + ky);
                    let kv = kernel[k];
                    r += kv * p[0] as f32;
                    g += kv * p[1] as f32;
                    b += kv * p[2] as f32;
                    a += kv * p[3] as f32;
                    k += 1;
                }
            }

            let idx = (y * w + x) * 4;
            dst[idx] = (r * norm).clamp(0.0, 255.0) as u8;
            dst[idx + 1] = (g * norm).clamp(0.0, 255.0) as u8;
            dst[idx + 2] = (b * norm).clamp(0.0, 255.0) as u8;
            dst[idx + 3] = (a * norm).clamp(0.0, 255.0) as u8;
        }
    }
}

fn grayscale_inplace(img: &mut [u8], w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) * 4;
            let r = img[idx] as f32;
            let g = img[idx + 1] as f32;
            let b = img[idx + 2] as f32;
            let gray = (0.299 * r + 0.587 * g + 0.114 * b).clamp(0.0, 255.0) as u8;
            img[idx] = gray;
            img[idx + 1] = gray;
            img[idx + 2] = gray;
        }
    }
}

fn resize_nearest(
    src: &[u8],
    w_in: usize,
    h_in: usize,
    dst: &mut [u8],
    w_out: usize,
    h_out: usize,
) {
    for y in 0..h_out {
        let src_y = (y * h_in) / h_out;
        for x in 0..w_out {
            let src_x = (x * w_in) / w_out;
            let src_idx = (src_y * w_in + src_x) * 4;
            let dst_idx = (y * w_out + x) * 4;
            dst[dst_idx..dst_idx + 4].copy_from_slice(&src[src_idx..src_idx + 4]);
        }
    }
}

fn wave_distortion(src: &[u8], dst: &mut [u8], w: usize, h: usize) {
    let amplitude = 10.0f32;
    let frequency = 2.0f32 * std::f32::consts::PI / 80.0;

    for y in 0..h {
        for x in 0..w {
            let offset = (amplitude * (y as f32 * frequency).sin()) as isize;
            let sx = x as isize + offset;
            let sy = y as isize;

            let p = get_pixel(src, w, h, sx, sy);
            let idx = (y * w + x) * 4;
            dst[idx] = p[0];
            dst[idx + 1] = p[1];
            dst[idx + 2] = p[2];
            dst[idx + 3] = p[3];
        }
    }
}

fn sobel_edges(src: &[u8], dst: &mut [u8], w: usize, h: usize) {
    let gx: [i32; 9] = [-1, 0, 1, -2, 0, 2, -1, 0, 1];
    let gy: [i32; 9] = [-1, -2, -1, 0, 0, 0, 1, 2, 1];

    for y in 0..h {
        for x in 0..w {
            let mut sx = 0i32;
            let mut sy = 0i32;
            let mut k = 0usize;

            for ky in -1..=1 {
                for kx in -1..=1 {
                    let p = get_pixel(src, w, h, x as isize + kx, y as isize + ky);
                    let lum =
                        (0.299 * p[0] as f32 + 0.587 * p[1] as f32 + 0.114 * p[2] as f32) as i32;
                    sx += gx[k] * lum;
                    sy += gy[k] * lum;
                    k += 1;
                }
            }

            let mag = (((sx * sx + sy * sy) as f32).sqrt().clamp(0.0, 255.0)) as u8;
            let idx = (y * w + x) * 4;
            dst[idx] = mag;
            dst[idx + 1] = mag;
            dst[idx + 2] = mag;
            dst[idx + 3] = 255;
        }
    }
}

fn blend_edges(base: &mut [u8], edges: &[u8], w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) * 4;

            let br = base[idx] as f32;
            let bg = base[idx + 1] as f32;
            let bb = base[idx + 2] as f32;

            let er = edges[idx] as f32;
            let eg = edges[idx + 1] as f32;
            let eb = edges[idx + 2] as f32;

            let r = (0.7 * br + 0.8 * er).clamp(0.0, 255.0) as u8;
            let g = (0.7 * bg + 0.8 * eg).clamp(0.0, 255.0) as u8;
            let b = (0.7 * bb + 0.8 * eb).clamp(0.0, 255.0) as u8;

            base[idx] = r;
            base[idx + 1] = g;
            base[idx + 2] = b;
        }
    }
}
