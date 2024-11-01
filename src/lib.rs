//! # convers
//!
//! `convers` is cool convert library.
//! The idea is to convert everything in one query, like text, units, just whatever.
//! Here are magic_convert function that can convert units and translate text.
//! ## Example of usage `magic_convert`:
//! ```
//! use convers::convert::magic_convert;
//!
//! #[tokio::main]
//! async fn main() {
//! 	 let query = "1m to cm".to_string();
//! 	 let result = magic_convert(&query).await.unwrap();
//! 	 let translate_result = magic_convert(&"en:de how are u?".to_string()).await.unwrap();
//! 	 println!("{}", result); // prints translated text
//! 	 println!("{}", result); // prints "100.0 Centimeter"
//! }

pub mod convert;
pub mod utils;
