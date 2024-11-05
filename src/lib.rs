//! # convers
//! `convers` - is a cool library for converting data from one format to another.
//! The idea is to convert everything from one query, like from one language, unit, etc., to another.
//! There are functions that allow you to do something like that.
//! ## Example of usage `magic_convert` function:
//! ```rust
//! use convers::convert::magic_convert;
//! #[tokio::main]
//! async fn main() {
//!     let query = "1m to cm".to_string();
//!     let result = magic_convert(&query).await.unwrap(); // Is String - "100 Centimeters"
//!     let translate_result = magic_convert(&String::from("en:de how are u?")).await.unwrap(); // Is String - "wie geht es dir?" or something like that
//!     let calculated = magic_convert(&String::from("15/3")).await.unwrap(); // Is String - "5" or "5.0"
//! }
//! ```
//! ## Also you can use provided things directly:
//! - _Translator_ *struct*
//! - _Measurement_ *struct*
//! - _Evaluate_ *fn*
//! ## Example of usage Measurement:
//! ```rust
//! use convers::utils::units::{Measurement, Unit};
//! let a = Measurement::new(50.0, Unit::Meter);
//! let b = Measurement::new(1.0, Unit::Kilometer);
//! let c = a + b; // anyhow::Result<Measurement>. If unwrap it equals 1.050 Kilometer
//! ```
//! ## Current available conversions:
//! - **Units**: Length, Mass, Amperes, Watts, Size (like kb), Rem/Px/Em
//! - **Translate**
//! - **Evaluate**
//! ## User Interfaces for **convers**:
//! - [convers_prompt](https://github.com/veaquer/convers_prompt) - An attempt to create something comfy like Raycast. (Soon it will be better)
//! # Soon will be added:
//! - **More units**
//! - **Conversion of timezones, currencies**
//! - **Conversion of temperature, pressure, speed, volume, area, etc.**
//! - **CLI convers binary package to crates.io and AUR**

pub mod convert;
pub mod utils;
