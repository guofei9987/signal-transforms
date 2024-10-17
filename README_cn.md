# signal-transforms

**signal-transforms** 是一个全面的 Rust 库，专注于实现各种信号变换算法，包括离散余弦变换（DCT）、逆离散余弦变换（IDCT）、二维离散余弦变换（DCT2）、逆二维离散余弦变换（IDCT2）、离散小波变换（DWT2）、逆离散小波变换（IDWT2）等。未来计划支持更多信号处理算法。

## 特性

- **离散余弦变换（DCT）**
    - 支持多种 DCT 类型（I-IV）
    - 高效的实现，适用于图像压缩和信号处理

- **逆离散余弦变换（IDCT）**
    - 与 DCT 完全兼容的逆变换

- **二维离散余弦变换（DCT2）**
    - 针对二维信号（如图像）的高效实现

- **逆二维离散余弦变换（IDCT2）**
    - 与 DCT2 完全兼容的逆变换


- **模块化设计**
    - 使用 feature flags 允许用户按需启用特定功能，减少不必要的依赖

## 安装

在您的 `Cargo.toml` 文件中添加以下依赖项：

```toml
[dependencies]
signal-transforms = "0.1.0"
```

## 选择性启用功能

```toml
[dependencies.signal-transforms]
version = "0.1.0"
features = ["dct", "dwt2"]
```

# 使用示例

## 离散余弦变换（DCT）

```rust
use signal_transforms::dct::dct_1d;

fn main() {
let input = vec![1.0, 2.0, 3.0, 4.0];
let output = dct_1d(&input);
println!("DCT Output: {:?}", output);
}
```

## 离散小波变换（DWT2）

