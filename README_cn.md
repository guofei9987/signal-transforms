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


**signal-transforms** 是一个 Rust 库，专注于实现各种信号变换算法。包括：
- 离散余弦变换（DCT）
- 逆离散余弦变换（IDCT）
- 二维离散余弦变换（DCT2）
- 逆二维离散余弦变换（IDCT2）
- 未来计划支持更多信号处理算法


## 安装

在您的 `Cargo.toml` 文件中添加以下依赖项：

```toml
[dependencies]
signal-transforms = "0.1.3"
```


# How To Use

## 离散余弦变换（DCT）

### 一维 DCT

```rust
fn example_dct_1d() {
    use nalgebra::DMatrix;
    use signal_transforms::dct::Dct;
    let dct = Dct::new(4);

    let vec1 = vec![52.0, 55.0, 61.0, 66.0];
    let vec1 = DMatrix::from_vec(1, 4, vec1);

    let dct_res = dct.dct_1d(&vec1);

    println!("dct result = {}", dct_res);

    let idct_res = dct.idct_1d(&dct_res);

    println!("idct result = {}", idct_res);
}
```

### 二维 DCT
```rust
fn example_dct_2d() {
    use nalgebra::DMatrix;
    use signal_transforms::dct::Dct2D;
    let matrix = vec![
        52.0, 55.0, 61.0, 66.0,
        70.0, 61.0, 64.0, 73.0,
        63.0, 59.0, 55.0, 90.0,
        67.0, 61.0, 68.0, 104.0,
    ];

    let matrix = DMatrix::from_row_slice(4, 4, &matrix);

    let dct = Dct2D::new(4, 4);

    let dct_res = dct.dct_2d(&matrix);

    println!("dct result = {}", dct_res);

    let idct_res = dct.idct_2d(&dct_res);
    println!("idct result = {}", idct_res);
}
```

### `Dct4x4` 比 `Dct2D` 快 20 倍

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



