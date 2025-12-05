mod numeric;
mod particles;
mod image_pipeline;

pub use numeric::{matmul_sum, factorial_mod};
pub use particles::simulate_particles;
pub use image_pipeline::process_image_wasm;
