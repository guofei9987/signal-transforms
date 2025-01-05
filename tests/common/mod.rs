use std::f32::consts::PI;
use nalgebra::{DMatrix, Dyn, Matrix1x4, Matrix4, Matrix4x1, OMatrix, U4};

pub fn assert_matrices_close(a: &DMatrix<f32>, b: &DMatrix<f32>, epsilon: f32) {
    let max_diff = (a - b).abs().max();
    assert!(max_diff < epsilon, "max difference = {}", max_diff);
}