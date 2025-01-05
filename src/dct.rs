use std::f32::consts::PI;
use nalgebra::{DMatrix, Dyn, Matrix1x4, Matrix4, Matrix4x1, OMatrix, U4};

fn gen_d_matrix(size: usize) -> OMatrix<f32, Dyn, Dyn> {
    let mut data = Vec::with_capacity(size * size);
    for x in 0..size {
        for u in 0..size {
            let alpha = if u == 0 { (1.0 / size as f32).sqrt() } else { (2.0 / size as f32).sqrt() };
            data.push(alpha * (PI * (x as f32 + 0.5) * u as f32 / size as f32).cos())
        }
    }
    DMatrix::from_row_slice(size, size, &data)
}


pub struct Dct {
    d: OMatrix<f32, Dyn, Dyn>,
    d_t: OMatrix<f32, Dyn, Dyn>,
}

impl Dct {
    pub fn new(size: usize) -> Self {
        let d = gen_d_matrix(size);
        Self {
            d: d.clone(),
            d_t: d.transpose(),
        }
    }


    /// One-dimensional Discrete Cosine Transform（DCT-II）
    ///
    /// Formula:
    /// ```latex
    /// F(u) = \alpha(u) \sum_{x=0}^{N-1} f(x) \cos \left( \frac{(2x + 1)u\pi}{2N} \right)
    ///
    /// \alpha(u) =
    /// \begin{cases}
    ///     \sqrt{\frac{1}{N}} & \text{if } u = 0 \
    ///     \sqrt{\frac{2}{N}} & \text{otherwise}
    /// \end{cases}
    /// ```

    pub fn dct_1d(&self, data: &OMatrix<f32, Dyn, Dyn>) -> OMatrix<f32, Dyn, Dyn> {
        data * &self.d
    }


    /// One-dimensional Inverse Discrete Cosine Transform（IDCT-III）
    ///
    /// Formula:
    /// ```latex
    /// f(x) = \sum_{u=0}^{N-1} \alpha(u) F(u) \cos \left( \frac{(2x + 1)u\pi}{2N} \right)
    ///
    /// \alpha(u) =
    /// \begin{cases}
    ///     \sqrt{\frac{1}{N}} & \text{if } u = 0 \
    ///     \sqrt{\frac{2}{N}} & \text{otherwise}
    /// \end{cases}
    /// ```
    pub fn idct_1d(&self, data: &OMatrix<f32, Dyn, Dyn>) -> OMatrix<f32, Dyn, Dyn> {
        data * &self.d_t
    }
}


pub struct Dct2D {
    d1: OMatrix<f32, Dyn, Dyn>,
    d1_t: OMatrix<f32, Dyn, Dyn>,
    d2: OMatrix<f32, Dyn, Dyn>,
    d2_t: OMatrix<f32, Dyn, Dyn>,
}
impl Dct2D {
    pub fn new(row: usize, col: usize) -> Self {
        //  d1 用于行dct，d2用于列dct
        let d1 = gen_d_matrix(col);
        let d2 = gen_d_matrix(row);

        Self {
            d1: d1.clone(),
            d1_t: d1.transpose(),
            d2: d2.clone(),
            d2_t: d2.transpose(),
        }
    }

    /// Two-dimensional Discrete Cosine Transform（DCT-II）
    pub fn dct_2d(&self, data: &OMatrix<f32, Dyn, Dyn>) -> OMatrix<f32, Dyn, Dyn> {
        &self.d2_t * data * &self.d1
    }


    /// Two-dimensional Inverse Discrete Cosine Transform（IDCT-III）
    pub fn idct_2d(&self, data: &OMatrix<f32, Dyn, Dyn>) -> OMatrix<f32, Dyn, Dyn> {
        &self.d2 * data * &self.d1_t
    }
}


fn gen_d_matrix_4x4() -> OMatrix<f32, U4, U4> {
    let size = 4;
    let mut data = Vec::with_capacity(size * size);
    for x in 0..size {
        for u in 0..size {
            let alpha = if u == 0 { (1.0 / size as f32).sqrt() } else { (2.0 / size as f32).sqrt() };
            data.push(alpha * (PI * (x as f32 + 0.5) * u as f32 / size as f32).cos())
        }
    }
    Matrix4::from_row_slice(&data)
}

/// `Dct4x4` is 20x faster than `Dct2D`
pub struct Dct4x4 {
    d: OMatrix<f32, U4, U4>,
    d_t: OMatrix<f32, U4, U4>,
}

impl Dct4x4 {
    pub fn new() -> Self {
        let d1 = gen_d_matrix_4x4();

        Self {
            d: d1.clone(),
            d_t: d1.transpose(),
        }
    }


    pub fn dct_2d(&self, data: &OMatrix<f32, U4, U4>) -> OMatrix<f32, U4, U4> {
        &self.d_t * data * &self.d
    }


    pub fn idct_2d(&self, data: &OMatrix<f32, U4, U4>) -> OMatrix<f32, U4, U4> {
        &self.d * data * &self.d_t
    }
}




