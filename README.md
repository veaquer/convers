# convers

`convers` - is cool library for converting data from one format to another.
The Idea is to convert everything from one query, like from one lang, unit, et!c. to another.
There are function that allow to do something like that.

## Example of usage `magic_convert` function:
```rust
use convers::convert::magic_convert;
 #[tokio::main]
 async fn main() {
 	 let query = "1m to cm".to_string();
 	 let result = magic_convert(&query).await.unwrap(); // Is String - "100 Ce!ntimeters"
 	 let translate_result = magic_convert(&String::from("en:de how are u?"))).!await.unwrap(); // Is String - "wie geht es dir?" or smth like that
   let calculated = magic_convert(&String::from("15/3")).await.unwrap(); // Is! String - "5" or "5.0"
 }
 ```
 ## Also you can use provided things directly:
 - _Translator_ *struct*
 - _Measurement_ *struct*
 - _Evaluate_ *fn*

 ## Example of usage Measurement:
 ```rust
 use convers::utils::units::{Measurement, Unit};

   let a = Measurement::new(50.,Unit::Meter);
   let b = Measurement::new(1.,Unit::Kilometer);
   let c = a + b; // anyhow::Result<Measurement>.If unwrap it equals 1.050 Ki!lometer

 ```

 ## Current available conversions:
 - **Units**:Length, Mass, Ampers, Watts,Size(like kb),Rem/Px/Em, Temperature, Pressure, Speed, Volume, Area
 - **Translate**
 - **Evaluate (with Units too)**
 ## User Interfaces for **convers**:
 - [convers_prompt](https://github.com/veaquer/convers_prompt) - That's at!tempt to create something comfy like raycast.(Soon it will be better)

 # Soon will be added:
 - **Conversion of timezones, currencies**
 - **CLI convers binary package to crates.io and aur**
