use nalgebra::{DMatrix};

pub fn assert_matrices_close(a: &DMatrix<f32>, b: &DMatrix<f32>, epsilon: f32) {
    let max_diff = (a - b).abs().max();
    assert!(max_diff < epsilon, "max difference = {}", max_diff);
}