# convers

`convers` - is cool library for converting data from one format to another.
The Idea is to convert everything from one query, like from one lang, unit, etc. to another.
There are function that allow to do something like that.

## Example of usage `magic_convert` function:
```rust
use convers::convert::magic_convert;
 #[tokio::main]
 async fn main() {
 	 let query = "1m to cm".to_string();
 	 let result = magic_convert(&query).await.unwrap(); // Is String - "100 Centimeters"
 	 let translate_result = magic_convert(&String::from("en:de how are u?"))).await.unwrap(); // Is String - "wie geht es dir?" or smth like that
   let calculated = magic_convert(&String::from("15/3")).await.unwrap(); // Is String - "5" or "5.0"
 }
 ```

 ## Current available conversions:
 - Units:Length, Mass, Ampers, Watts,Size(like kb),Rem/Px/Em
 - Translate
 - Evaluate

 ## User Interfaces for **convers**:
 - [convers_prompt](https://github.com/veaquer/convers_prompt)(Works only in linux) - That's attempt to create something comfy like raycast.(Soon it will be better)

 # Soon will be added:
 - More units
 - Conversion of timezones, currencies
 - Conversion of temperature, pressure, speed, volume, area, etc.
 - CLI convers binary package to crates.io and aur
