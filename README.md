# signal-transforms

[![Crates.io](https://img.shields.io/crates/v/signal-transforms)](https://crates.io/crates/signal-transforms)
[![Build Status](https://github.com/guofei9987/signal-transforms/actions/workflows/rust.yml/badge.svg)](https://github.com/guofei9987/signal-transforms/actions)
[![Docs.rs](https://docs.rs/signal-transforms/badge.svg)](https://docs.rs/signal-transforms)
[![License](https://img.shields.io/crates/l/signal-transforms)](https://github.com/your-username/your-repo/blob/master/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/guofei9987/signal-transforms.svg?style=social&label=Star)](https://github.com/guofei9987/signal-transforms)
[![Forks](https://img.shields.io/github/forks/guofei9987/signal-transforms.svg?style=social&label=Fork)](https://github.com/guofei9987/signal-transforms/fork)
![Rust](https://img.shields.io/badge/Rust-1.60+-orange.svg)
[![Crates.io Downloads](https://img.shields.io/crates/d/signal-transforms)](https://crates.io/crates/signal-transforms)
[![GitHub Discussions](https://img.shields.io/github/discussions/guofei9987/signal-transforms)](https://github.com/guofei9987/signal-transforms/discussions)


**signal-transforms** is a Rust library dedicated to implementing various signal transformation algorithms, including:

- Discrete Cosine Transform (DCT)
- Inverse Discrete Cosine Transform (IDCT)
- Two-Dimensional Discrete Cosine Transform (DCT2)
- Inverse Two-Dimensional Discrete Cosine Transform (IDCT2)
- Future plans to support more signal processing algorithms

## Installation

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
signal-transforms = "0.1.3"
```

# How To Use

## Discrete Cosine Transform (DCT)

### One-Dimensional DCT

```rust
fn example_dct_1d() {
    use nalgebra::DMatrix;
    use signal_transforms::dct::Dct;
    
    // Create a new DCT instance with size 4
    let dct = Dct::new(4);

    // Define a vector of sample data
    let vec1 = vec![52.0, 55.0, 61.0, 66.0];
    let vec1 = DMatrix::from_vec(1, 4, vec1);

    // Perform the 1D DCT
    let dct_res = dct.dct_1d(&vec1);

    println!("DCT result = {}", dct_res);

    // Perform the inverse 1D DCT
    let idct_res = dct.idct_1d(&dct_res);

    println!("Inverse DCT result = {}", idct_res);
}
```

### Two-Dimensional DCT

```rust
fn example_dct_2d() {
    use nalgebra::DMatrix;
    use signal_transforms::dct::Dct2D;
    
    // Define a 4x4 matrix of sample data
    let matrix = vec![
        52.0, 55.0, 61.0, 66.0,
        70.0, 61.0, 64.0, 73.0,
        63.0, 59.0, 55.0, 90.0,
        67.0, 61.0, 68.0, 104.0,
    ];

    let matrix = DMatrix::from_row_slice(4, 4, &matrix);

    // Create a new 2D DCT instance with dimensions 4x4
    let dct = Dct2D::new(4, 4);

    // Perform the 2D DCT
    let dct_res = dct.dct_2d(&matrix);

    println!("DCT result = {}", dct_res);

    // Perform the inverse 2D DCT
    let idct_res = dct.idct_2d(&dct_res);
    println!("Inverse DCT result = {}", idct_res);
}
```


### `Dct4x4` is 20x faster than `Dct2D`

```rust
#[test]
fn example_dct_4x4() {
    use signal_transforms::dct::Dct4x4;
    let matrix = Matrix4::new(
        52.0, 55.0, 61.0, 66.0,
        70.0, 61.0, 64.0, 73.0,
        63.0, 59.0, 55.0, 90.0,
        67.0, 61.0, 68.0, 104.0,
    );

    let dct = Dct4x4::new();

    let dct_res = dct.dct_2d(&matrix);

    println!("dct result = {}", dct_res);

    let idct_res = dct.idct_2d(&dct_res);
    println!("idct result = {}", idct_res);
}
```



## Future Enhancements

- Support for additional signal processing algorithms.
- Optimization for performance and memory usage.
- Comprehensive documentation and examples for advanced use cases.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

For more information, visit the [official documentation](https://github.com/your-repo/signal-transforms).