#[cfg(test)]
#[cfg(feature = "dct_raw")]
mod tests_dct_raw {
    use signal_transforms::dct_raw_algo::{dct_1d, dct_2d, idct_1d, idct_2d};


    #[test]
    fn tst_dct_1d() {
        let vec1 = vec![52.0, 55.0, 61.0, 66.0];
        let dct = dct_1d(&vec1);
        println!("dct result: {:?}", dct);
        let idct = idct_1d(&dct);
        println!("idct result: {:?}", idct);
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
}

