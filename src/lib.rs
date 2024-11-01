//! # convers
//!
//! `convers` is cool convert library.
//! The idea is to convert everything in one query, like text, units, just whatever.
//! Here are magic_convert function that can convert units and translate text.
//! ## Example of usage `magic_convert`:
//!
//! use convers::convert::magic_convert;
//! use tokio::runtime::Runtime;
//!
//! let query = "1m to cm".to_string();
//! let translate_query = "en:de how are u?".to_string();
//!
//! let rt = Runtime::new().unwrap();
//! let result = rt.block_on(async { magic_convert(&query).await }).unwrap();
//! let translate_result = rt.block_on(async { magic_convert(&translate_query).await }).unwrap();
//!
//! println!("{}", result); // prints "100.0 Centimeter"
//! println!("{}", translate_result); // prints translated text
//! ```
pub mod convert;
pub mod utils;
