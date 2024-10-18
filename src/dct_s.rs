use nalgebra::Matrix3;

#[test]
fn tst1(){


    let m2 = Matrix3::new(
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    );

}




struct Dct8x8 {
    cosine_table: [[f32; 8]; 8],
    alpha_table: [f32; 8],
}

