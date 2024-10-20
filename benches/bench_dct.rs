use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nalgebra::{DMatrix, Matrix4};
use signal_transforms::dct::{Dct2D, Dct4x4};

pub fn bench_dct(c: &mut Criterion) {
    let data = vec![
        52.0, 55.0, 61.0, 66.0,
        70.0, 61.0, 64.0, 73.0,
        63.0, 59.0, 55.0, 90.0,
        67.0, 61.0, 68.0, 104.0,
    ];

    let matrix = DMatrix::from_row_slice(4, 4, &data);

    let dct = Dct2D::new(4, 4);

    c.bench_function("dct_2d", |b| b.iter(|| dct.dct_2d(black_box(&matrix))));
}
pub fn bench_idct(c: &mut Criterion) {
    let data = vec![
        52.0, 55.0, 61.0, 66.0,
        70.0, 61.0, 64.0, 73.0,
        63.0, 59.0, 55.0, 90.0,
        67.0, 61.0, 68.0, 104.0,
    ];

    let matrix = DMatrix::from_row_slice(4, 4, &data);

    let dct = Dct2D::new(4, 4);

    c.bench_function("idct_2d", |b| b.iter(|| dct.idct_2d(black_box(&matrix))));
}

pub fn bench_dct_4x4(c: &mut Criterion) {
    let matrix = Matrix4::new(
        52.0, 55.0, 61.0, 66.0,
        70.0, 61.0, 64.0, 73.0,
        63.0, 59.0, 55.0, 90.0,
        67.0, 61.0, 68.0, 104.0, );

    let dct = Dct4x4::new();

    c.bench_function("dct_4x4", |b| b.iter(|| dct.dct_2d(black_box(&matrix))));
}
pub fn bench_idct_4x4(c: &mut Criterion) {
    let matrix = Matrix4::new(
        52.0, 55.0, 61.0, 66.0,
        70.0, 61.0, 64.0, 73.0,
        63.0, 59.0, 55.0, 90.0,
        67.0, 61.0, 68.0, 104.0, );

    let dct = Dct4x4::new();

    c.bench_function("dct_4x4", |b| b.iter(|| dct.idct_2d(black_box(&matrix))));
}

criterion_group!(benches, bench_dct, bench_idct, bench_dct_4x4, bench_idct_4x4);

// 生成基准测试主函数
criterion_main!(benches);
