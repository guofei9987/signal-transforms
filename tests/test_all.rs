#[cfg(test)]
mod tests_all {
    use signal_transforms::dct::{Dct, Dct2D};
    use nalgebra::{DMatrix, Matrix4};
    use rand::Rng;

    /// 辅助函数：生成随机一维数据
    fn generate_random_1d(size: usize) -> Vec<f32> {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| rng.gen_range(0.0..=255.0)).collect()
    }

    /// 辅助函数：生成随机二维数据
    fn generate_random_2d(rows: usize, cols: usize) -> Vec<f32> {
        let mut rng = rand::thread_rng();
        (0..rows * cols).map(|_| rng.gen_range(0.0..=255.0)).collect()
    }

    fn assert_matrices_close(a: &DMatrix<f32>, b: &DMatrix<f32>, epsilon: f32) {
        let max_diff = (a - b).abs().max();
        assert!(max_diff < epsilon, "max difference = {}", max_diff);
    }


    const EPSILON: f32 = 1e-2;

    fn test_dct_1d(size: usize) {
        let dct = Dct::new(size);
        // 生成随机输入数据
        let input_data = generate_random_1d(size);
        let input = DMatrix::from_row_slice(1, size, &input_data);

        // 进行 DCT 变换
        let dct_result = dct.dct_1d(input.clone());

        // 进行 IDCT 逆变换
        let idct_result = dct.idct_1d(dct_result);

        // 比较原始输入和逆变换后的结果
        assert_matrices_close(&input, &idct_result, EPSILON);
    }

    fn test_dct_2d(rows: usize, cols: usize) {
        let dct2d = Dct2D::new(rows, cols);

        // 生成随机输入数据
        let input_data = generate_random_2d(rows, cols);
        let input = DMatrix::from_row_slice(rows, cols, &input_data);

        // 进行 2D DCT 变换
        let dct_result = dct2d.dct_2d(input.clone());

        // 进行 2D IDCT 逆变换
        let idct_result = dct2d.idct_2d(dct_result);

        // 比较原始输入和逆变换后的结果
        assert_matrices_close(&input, &idct_result, EPSILON);
    }


    #[test]
    fn test_dct_idct_1d_random() {
        let mut rng = rand::thread_rng();
        // 进行多次随机测试
        for _ in 0..100 {
            let size = rng.gen_range(1..=32);
            test_dct_1d(size);
        }
    }

    #[test]
    fn test_dct_2d_random() {
        let mut rng = rand::thread_rng();
        // 进行多次随机测试
        for _ in 0..100 {
            let rows = rng.gen_range(1..=32);
            let cols = rng.gen_range(1..=32);
            test_dct_2d(rows, cols);
        }
    }

    use signal_transforms::dct::{Dct4x4};

    #[test]
    fn test_dct_4x4_equal_to_dct() {
        let dct = Dct2D::new(4, 4);
        let dct_4x4 = Dct4x4::new();

        let input_data = generate_random_2d(4, 4);

        let matrix1 = DMatrix::from_row_slice(4, 4, &input_data);

        let matrix2 = Matrix4::from_row_slice(&input_data);

        let dct_res1 = dct.dct_2d(matrix1);
        let dct_res2 = dct_4x4.dct_2d(matrix2);

        let dct_res2 = DMatrix::from_row_slice(4, 4, dct_res2.as_slice()).transpose();

        assert_matrices_close(&dct_res1, &dct_res2, EPSILON);
    }


    #[test]
    fn test_dct_4x4() {
        let mut rng = rand::thread_rng();
        // 进行多次随机测试
        for _ in 0..100 {
            let dct2d = Dct4x4::new();

            // 生成随机输入数据
            let input_data = generate_random_2d(4, 4);
            let input = Matrix4::from_row_slice(&input_data);

            // 进行 2D DCT 变换
            let dct_result = dct2d.dct_2d(input.clone());

            // 进行 2D IDCT 逆变换
            let idct_result = dct2d.idct_2d(dct_result);


            let input_dyn = DMatrix::from_row_slice(4, 4, input.as_slice()).transpose();
            let idct_result_dyn = DMatrix::from_row_slice(4, 4, idct_result.as_slice()).transpose();

            // 比较原始输入和逆变换后的结果
            assert_matrices_close(&input_dyn, &idct_result_dyn, EPSILON);
        }
    }
}