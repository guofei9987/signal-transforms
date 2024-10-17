use std::f64::consts::PI;

fn alpha(u: usize, n: usize) -> f64 {
    if u == 0 {
        (1.0 / n as f64).sqrt()
    } else {
        (2.0 / n as f64).sqrt()
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
fn dct_1d(vector: &[f64]) -> Vec<f64> {
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
fn idct_1d(vector: &[f64]) -> Vec<f64> {
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
fn dct_2d(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
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
fn idct_2d(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
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



#[test]
fn tst_dct_1d() {
    let vec1 = vec![52.0, 55.0, 61.0, 66.0];


    let vec2 = dct_1d(&vec1);
    println!("{:?}", vec2);
}
#[test]
fn tst_dct_2d() {
    // 示例 4x4 矩阵（例如灰度图像块）
    let matrix = vec![
        vec![52.0, 55.0, 61.0, 66.0],
        vec![70.0, 61.0, 64.0, 73.0],
        vec![63.0, 59.0, 55.0, 90.0],
        vec![67.0, 61.0, 68.0, 104.0],
    ];

    println!("原始矩阵:");
    for row in &matrix {
        println!("{:?}", row);
    }

    // 计算二维 DCT
    let dct = dct_2d(&matrix);
    println!("\nDCT 矩阵:");
    for row in &dct {
        println!("{:?}", row);
    }

    // 计算二维 IDCT
    let idct = idct_2d(&dct);
    println!("\nIDCT 矩阵:");
    for row in &idct {
        println!("{:?}", row);
    }
}
