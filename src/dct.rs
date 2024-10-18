use std::f32::consts::PI;
use nalgebra::{DMatrix, Dyn, Matrix1x4, Matrix2, Matrix2x1, Matrix2x3, Matrix4, Matrix4x1, OMatrix, U4};

fn generate_cosine_table(size: usize) -> Vec<f32> {
    let mut data = Vec::with_capacity(size * size);
    for x in 0..size {
        for u in 0..size {
            data.push((PI * (x as f32 + 0.5) * u as f32 / size as f32).cos())
        }
    }
    data
    // DMatrix::from_vec(size, size, data) // 这个是按列填充的，坑
    // DMatrix::from_row_slice(size, size, &data)
}

fn generate_alpha_table(size: usize) -> Vec<f32> {
    let mut data = Vec::with_capacity(size);
    for u in 0..size {
        if u == 0 {
            data.push((1.0 / size as f32).sqrt());
        } else {
            data.push((2.0 / size as f32).sqrt());
        }
    }
    data
    // DMatrix::from_vec(1, size, data)
}


pub struct Dct {
    cosine_table: OMatrix<f32, Dyn, Dyn>,
    alpha_table: OMatrix<f32, Dyn, Dyn>,
    cosine_table_transpose: OMatrix<f32, Dyn, Dyn>,
}

impl Dct {
    pub fn new(size: usize) -> Self {
        let cosine_table = DMatrix::from_row_slice(size, size, &generate_cosine_table(size));
        let alpha_table = DMatrix::from_vec(1, size, generate_alpha_table(size));
        Self {
            cosine_table: cosine_table.clone(),
            alpha_table: alpha_table.clone(),
            cosine_table_transpose: cosine_table.transpose(),
        }
    }


    /// 一维离散余弦变换（DCT-II）
    ///
    /// 公式：
    /// ```latex
    /// F(u) = \alpha(u) \sum_{x=0}^{N-1} f(x) \cos \left( \frac{(2x + 1)u\pi}{2N} \right)
    ///
    /// \alpha(u) =
    /// \begin{cases}
    ///     \sqrt{\frac{1}{N}} & \text{if } u = 0 \
    ///     \sqrt{\frac{2}{N}} & \text{otherwise}
    /// \end{cases}
    /// ```

    pub fn dct_1d(&self, data: OMatrix<f32, Dyn, Dyn>) -> OMatrix<f32, Dyn, Dyn> {
        (data * &self.cosine_table).component_mul(&self.alpha_table)
    }


    /// 一维离散余弦逆变换（IDCT-III）
    ///
    /// 公式：
    /// ```latex
    /// f(x) = \sum_{u=0}^{N-1} \alpha(u) F(u) \cos \left( \frac{(2x + 1)u\pi}{2N} \right)
    ///
    /// \alpha(u) =
    /// \begin{cases}
    ///     \sqrt{\frac{1}{N}} & \text{if } u = 0 \
    ///     \sqrt{\frac{2}{N}} & \text{otherwise}
    /// \end{cases}
    /// ```

    pub fn idct_1d(&self, data: OMatrix<f32, Dyn, Dyn>) -> OMatrix<f32, Dyn, Dyn> {
        (data.component_mul(&self.alpha_table)) * &self.cosine_table_transpose
    }
}


pub struct Dct2D {
    cosine_table_row: OMatrix<f32, Dyn, Dyn>,
    alpha_table_row: OMatrix<f32, Dyn, Dyn>,
    cosine_table_transpose_row: OMatrix<f32, Dyn, Dyn>,

    cosine_table_col: OMatrix<f32, Dyn, Dyn>,
    cosine_table_transpose_col: OMatrix<f32, Dyn, Dyn>,
    alpha_table_transpose_col: OMatrix<f32, Dyn, Dyn>,
}
impl Dct2D {
    pub fn new(row: usize, col: usize) -> Self {
        let cosine_table_row = DMatrix::from_row_slice(col, col, &generate_cosine_table(col));
        let alpha_table_row = DMatrix::from_vec(1, col, generate_alpha_table(col));
        let cosine_table_col = DMatrix::from_row_slice(row, row, &generate_cosine_table(row));
        let alpha_table_col = DMatrix::from_vec(1, row, generate_alpha_table(row));

        // repeat
        let alpha_table_row = DMatrix::from_element(row, 1, 1.0) * alpha_table_row;
        let alpha_table_col = DMatrix::from_element(col, 1, 1.0) * alpha_table_col;

        Self {
            cosine_table_row: cosine_table_row.clone(),
            alpha_table_row: alpha_table_row.clone(),
            cosine_table_transpose_row: cosine_table_row.transpose(),

            cosine_table_col: cosine_table_col.clone(),
            cosine_table_transpose_col: cosine_table_col.transpose(),
            alpha_table_transpose_col: alpha_table_col.transpose(),

        }
    }

    pub fn dct_2d(&self, data: OMatrix<f32, Dyn, Dyn>) -> OMatrix<f32, Dyn, Dyn> {
        // 对每行做 dct
        let tmp = (data * &self.cosine_table_row).component_mul(&self.alpha_table_row);
        // 对每列做dct
        (&self.cosine_table_transpose_col * tmp).component_mul(&self.alpha_table_transpose_col)
    }


    pub fn idct_2d(&self, data: OMatrix<f32, Dyn, Dyn>) -> OMatrix<f32, Dyn, Dyn> {
        //     对每一列做 idct
        let tmp = &self.cosine_table_col * (data.component_mul(&self.alpha_table_transpose_col));

        //     对每一行做 idct
        (tmp.component_mul(&self.alpha_table_row)) * &self.cosine_table_transpose_row
    }
}


#[test]
fn tst3() {
    let matrix = vec![
        52.0, 55.0, 61.0, 66.0,
        70.0, 61.0, 64.0, 73.0,
        63.0, 59.0, 55.0, 90.0,
        67.0, 61.0, 68.0, 104.0,
    ];

    let matrix = DMatrix::from_row_slice(4, 4, &matrix);

    let dct = Dct2D::new(4, 4);

    let dct_res = dct.dct_2d(matrix);

    println!("dct_res={}", dct_res);

    let idct_res = dct.idct_2d(dct_res);

    println!("idct_res={}", idct_res);
}



#[test]
fn tst5() {
    let matrix = vec![
        52.0, 55.0, 61.0, 66.0,
        70.0, 61.0, 64.0, 73.0,
        63.0, 59.0, 55.0, 90.0,
    ];

    let matrix = DMatrix::from_row_slice(3, 4, &matrix);

    let dct = Dct2D::new(3, 4);

    let dct_res = dct.dct_2d(matrix);

    println!("dct_res={}", dct_res);

    let idct_res = dct.idct_2d(dct_res);

    println!("idct_res={}", idct_res);
}


struct Dct4x4 {
    cosine_table_row: OMatrix<f32, U4, U4>,
    alpha_table_row: OMatrix<f32, U4, U4>,
    cosine_table_transpose_row: OMatrix<f32, U4, U4>,

    cosine_table_col: OMatrix<f32, U4, U4>,
    cosine_table_transpose_col: OMatrix<f32, U4, U4>,
    alpha_table_transpose_col: OMatrix<f32, U4, U4>,
}

impl Dct4x4 {
    pub fn new() -> Self {
        let size = 4;
        let cosine_table_row = Matrix4::from_row_slice(&generate_cosine_table(size));
        let alpha_table_row = Matrix1x4::from_vec(generate_alpha_table(size));

        // repeat
        let alpha_table_row = Matrix4x1::from_element(1.0) * alpha_table_row;

        Self {
            cosine_table_row: cosine_table_row.clone(),
            alpha_table_row: alpha_table_row.clone(),
            cosine_table_transpose_row: cosine_table_row.transpose(),

            cosine_table_col: cosine_table_row.clone(),
            cosine_table_transpose_col: cosine_table_row.transpose(),
            alpha_table_transpose_col: alpha_table_row.transpose(),

        }
    }
}



#[test]
fn tst4() {
    let a = vec![1.0, 2.0, 3.0, 4.0, ];
    let a1 = Matrix2::from_row_slice(&a);

    let c1 = Matrix2::from_element(1.0) * a1;

    println!("{}", c1);
}


struct Dct8x8 {
    cosine_table: [[f32; 8]; 8],
    alpha_table: [f32; 8],
}




