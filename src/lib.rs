pub mod dct;
mod dct_s;
mod dct_raw;

#[cfg(feature = "dct_raw")]
pub use dct_raw::dct_raw_algo;