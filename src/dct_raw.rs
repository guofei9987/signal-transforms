#[cfg(feature = "dct_raw")]
pub mod dct_raw_algo {
    use std::f64::consts::PI;

    fn alpha(u: usize, n: usize) -> f64 {
        if u == 0 {
            (1.0 / n as f64).sqrt()
        } else {
            (2.0 / n as f64).sqrt()
        }
    }

    pub fn dct_1d(vector: &[f64]) -> Vec<f64> {
        let n = vector.len();
        let mut result = vec![0.0; n];
        for u in 0..n {
            let mut sum = 0.0;
            for x in 0..n {
                sum += vector[x] * ((2 * x + 1) as f64 * u as f64 * PI / (2.0 * n as f64)).cos();
            }
            result[u] = alpha(u, n) * sum;
        }
        result
    }

    pub fn idct_1d(vector: &[f64]) -> Vec<f64> {
        let n = vector.len();
        let mut result = vec![0.0; n];
        for x in 0..n {
            let mut sum = 0.0;
            for u in 0..n {
                sum += alpha(u, n) * vector[u] * ((2 * x + 1) as f64 * u as f64 * PI / (2.0 * n as f64)).cos();
            }
            result[x] = sum;
        }
        result
    }

    /// 二维 DCT
    pub fn dct_2d(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = matrix.len();
        let mut intermediate = vec![vec![0.0; n]; n];
        let mut dct_matrix = vec![vec![0.0; n]; n];

        // 对每一行应用一维 DCT
        for i in 0..n {
            intermediate[i] = dct_1d(&matrix[i]);
        }

        // 对每一列应用一维 DCT
        for j in 0..n {
            let column: Vec<f64> = intermediate.iter().map(|row| row[j]).collect();
            let dct_col = dct_1d(&column);
            for i in 0..n {
                dct_matrix[i][j] = dct_col[i];
            }
        }

        dct_matrix
    }

    /// 二维 IDCT
    pub fn idct_2d(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = matrix.len();
        let mut intermediate = vec![vec![0.0; n]; n];
        let mut idct_matrix = vec![vec![0.0; n]; n];

        // 对每一列应用一维 IDCT
        for j in 0..n {
            let column: Vec<f64> = matrix.iter().map(|row| row[j]).collect();
            let idct_col = idct_1d(&column);
            for i in 0..n {
                intermediate[i][j] = idct_col[i];
            }
        }

        // 对每一行应用一维 IDCT
        for i in 0..n {
            idct_matrix[i] = idct_1d(&intermediate[i]);
        }

        idct_matrix
    }
}


