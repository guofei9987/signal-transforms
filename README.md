# signal-transforms
A comprehensive Rust library for discrete and wavelet transforms, including DCT, and more.


## How to use

dct 1 dimension

```rust
    use nalgebra::DMatrix;
use signal_transforms::dct::{Dct, Dct2D};
#[test]
fn example_dct_1d() {
    let dct = Dct::new(4);

    let vec1 = vec![52.0, 55.0, 61.0, 66.0];
    let vec1 = DMatrix::from_vec(1, 4, vec1);

    let dct_res = dct.dct_1d(vec1);

    println!("dct_res={}", dct_res);
    println!("dct_res={:?}", dct_res);

    let idct_res = dct.idct_1d(dct_res);

    println!("idct_res={}", idct_res);
}
```


DCT 2 Dimension

```rust
#[test]
fn example_dct_2d() {
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
```

