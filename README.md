# convers

`convers` - is cool library for converting data from one format to another.
The Idea is to convert everything from one query, like from one lang, unit, etc. to another.
There are function that allow to do something like that.

## Example of usage `magic_convert` function:
```rust
 #[tokio::main]
 async fn main() {
 	 let query = "1m to cm".to_string();
 	 let result = magic_convert(&query).await.unwrap();
 	 let translate_result = magic_convert(&"en:de how are u?".to_string()).await.unwrap();
 	 println!("{}", result); // prints translated text
 	 println!("{}", result); // prints "100.0 Centimeter"
 }
 ```
