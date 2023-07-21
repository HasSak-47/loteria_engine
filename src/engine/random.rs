
#[cfg(target_arch="wasm32")]
use super::wasm::random as wasm_random;

#[cfg(not(target_arch="wasm32"))]
use rand::random as rand_random;

#[cfg(target_arch="wasm32")]
pub fn random() -> usize {
    (wasm_random() * (1 << 23) as f32) as usize
}

#[cfg(not(target_arch="wasm32"))]
pub fn random() -> usize {
    rand_random()
}
