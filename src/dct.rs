use std::f32::consts::PI;
use nalgebra::{DMatrix, Dyn, OMatrix};

pub fn generate_cosine_table(size: usize) -> OMatrix<f32, Dyn, Dyn> {
    let mut data = Vec::with_capacity(size * size);
    for x in 0..size {
        for u in 0..size {
            data.push((PI * (x as f32 + 0.5) * u as f32 / size as f32).cos())
        }
    }
    // DMatrix::from_vec(size, size, data) // 这个是按列填充的，坑
    DMatrix::from_row_slice(size, size, &data)
}

pub fn generate_alpha_table(size: usize) -> OMatrix<f32, Dyn, Dyn> {
    let mut data = Vec::with_capacity(size);
    for u in 0..size {
        if u == 0 {
            data.push((1.0 / size as f32).sqrt());
        } else {
            data.push((2.0 / size as f32).sqrt());
        }
    }
    DMatrix::from_vec(1, size, data)
}


pub struct Dct {
    size: usize,
    cosine_table: OMatrix<f32, Dyn, Dyn>,
    alpha_table: OMatrix<f32, Dyn, Dyn>,
    cosine_table_transpose: OMatrix<f32, Dyn, Dyn>,
    alpha_table_transpose: OMatrix<f32, Dyn, Dyn>,
}

impl Dct {
    pub fn new(size: usize) -> Self {
        let cosine_table = generate_cosine_table(size);
        let alpha_table = generate_alpha_table(size);
        Self {
            size,
            cosine_table: cosine_table.clone(),
            alpha_table: alpha_table.clone(),
            cosine_table_transpose: cosine_table.transpose(),
            alpha_table_transpose: alpha_table.transpose(),
        }
    }

    pub fn dct_1d(&self, data: OMatrix<f32, Dyn, Dyn>) -> OMatrix<f32, Dyn, Dyn> {
        (data * &self.cosine_table).component_mul(&self.alpha_table)
    }

    pub fn idct_1d(&self, data: OMatrix<f32, Dyn, Dyn>) -> OMatrix<f32, Dyn, Dyn> {
        (data.component_mul(&self.alpha_table)) * &self.cosine_table_transpose
    }
}





pub struct Dct2D {
    row: usize,
    col: usize,
    cosine_table_row: OMatrix<f32, Dyn, Dyn>,
    alpha_table_row: OMatrix<f32, Dyn, Dyn>,
    cosine_table_transpose_row: OMatrix<f32, Dyn, Dyn>,
    alpha_table_transpose_row: OMatrix<f32, Dyn, Dyn>,

    cosine_table_col: OMatrix<f32, Dyn, Dyn>,
    alpha_table_col: OMatrix<f32, Dyn, Dyn>,
    cosine_table_transpose_col: OMatrix<f32, Dyn, Dyn>,
    alpha_table_transpose_col: OMatrix<f32, Dyn, Dyn>,
}
impl Dct2D {
    pub fn new(row: usize, col: usize) -> Self {
        let cosine_table_row = generate_cosine_table(row);
        let alpha_table_row = generate_alpha_table(row);
        let cosine_table_col = generate_cosine_table(col);
        let alpha_table_col = generate_alpha_table(col);

        // repeat
        let alpha_table_row = DMatrix::from_element(col, 1, 1.0) * alpha_table_row;
        let alpha_table_col = DMatrix::from_element(row, 1, 1.0) * alpha_table_col;

        Self {
            row,
            col,
            cosine_table_row: cosine_table_row.clone(),
            alpha_table_row: alpha_table_row.clone(),
            cosine_table_transpose_row: cosine_table_row.transpose(),
            alpha_table_transpose_row: alpha_table_row.transpose(),

            cosine_table_col: cosine_table_col.clone(),
            alpha_table_col: alpha_table_col.clone(),
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


    pub fn dct_2d2(&self, data: OMatrix<f32, Dyn, Dyn>) -> OMatrix<f32, Dyn, Dyn> {
        // 对每行做 dct
        let tmp = (data * &self.cosine_table_row).component_mul(&self.alpha_table_row);

        let tmp = tmp.transpose();

        let tmp2 = (tmp * &self.cosine_table_col).component_mul(&self.alpha_table_col);
        tmp2.transpose()
    }

    pub fn idct_2d(&self, data: OMatrix<f32, Dyn, Dyn>) -> OMatrix<f32, Dyn, Dyn> {
        println!("not yet!");

        //     对每一列做 idct
        let tmp1 = (data.transpose().component_mul(&self.alpha_table_transpose_col)) * &self.cosine_table_transpose_col;

        //     对每一行做 idct

        (tmp1.component_mul(&self.alpha_table_transpose_row)) * &self.cosine_table_col
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


struct Dct4x4 {
    cosine_table: [[f32; 4]; 4],
    alpha_table: [f32; 4],
}


struct Dct8x8 {
    cosine_table: [[f32; 8]; 8],
    alpha_table: [f32; 8],
}

