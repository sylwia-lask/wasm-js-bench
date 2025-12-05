use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn simulate_particles(
    steps: u32,
    dt: f32,
    width: f32,
    height: f32,
    positions: &[f32],
    velocities: &[f32],
) -> Vec<f32> {
    let n = positions.len() / 2;
    let mut pos = positions.to_vec();
    let mut vel = velocities.to_vec();

    let cx = width * 0.5;
    let cy = height * 0.5;
    let k: f32 = 0.0005;

    for _ in 0..steps {
        for i in 0..n {
            let idx = i * 2;

            let x = pos[idx];
            let y = pos[idx + 1];

            let dx = cx - x;
            let dy = cy - y;

            let mut vx = vel[idx] + dx * k * dt;
            let mut vy = vel[idx + 1] + dy * k * dt;

            let mut new_x = x + vx * dt;
            let mut new_y = y + vy * dt;

            if new_x < 0.0 || new_x > width {
                vx = -vx * 0.8;
                if new_x < 0.0 {
                    new_x = 0.0;
                }
                if new_x > width {
                    new_x = width;
                }
            }

            if new_y < 0.0 || new_y > height {
                vy = -vy * 0.8;
                if new_y < 0.0 {
                    new_y = 0.0;
                }
                if new_y > height {
                    new_y = height;
                }
            }

            pos[idx] = new_x;
            pos[idx + 1] = new_y;
            vel[idx] = vx;
            vel[idx + 1] = vy;
        }
    }

    let mut out = Vec::with_capacity(n * 4);
    out.extend_from_slice(&pos);
    out.extend_from_slice(&vel);
    out
}
