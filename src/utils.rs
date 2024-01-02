use rand::{distributions::Standard, Rng};
use std::ops::Range;

/// Generates a random floating point number
pub fn random() -> f32 {
    rand::thread_rng().sample::<f32, Standard>(Standard)
}

/// Generates a random number in the given range
pub fn random_range(r: Range<f32>) -> f32 {
    rand::thread_rng().gen_range(r)
}
