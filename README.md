this doesn't work

```
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use failure::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };
    match point_to_json(point) {
        Ok(s) => println!("{}", s),
        Err(e) => panic!(e),
    }
}

fn point_to_json(point: Point) -> Result<String, Error> {
    serde_json::to_string(&point)?
}

```

and I don't know why :\ this is weird:
```
  --> src/main.rs:40:5
   |
40 |     serde_json::to_string(&point)?
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     expected enum `std::result::Result`, found struct `std::string::String`
   |     match arm with an incompatible type
   |
   = note: expected type `std::result::Result<std::string::String, failure::Error>`
              found type `std::string::String`
```

because it definitely [returns a Result<String, serde_json::Error>](https://docs.serde.rs/serde_json/fn.to_string.html)
if you change it so that you assign to a variable you get this:

```
  --> src/main.rs:41:5
   |
39 | fn point_to_json(point: Point) -> Result<String, Error> {
   |                                   --------------------- expected `std::result::Result<std::string::String, failure::Error>` because of return type
40 |     let s = serde_json::to_string(&point);
41 |     s
   |     ^ expected struct `failure::Error`, found struct `serde_json::Error`
   |
   = note: expected type `std::result::Result<_, failure::Error>`
              found type `std::result::Result<_, serde_json::Error>`
```

It definitely does work with `io::Error`s though :thinking_face:, I need halp :P
