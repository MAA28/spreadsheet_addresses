# Spreadsheet addresses
[<img alt="github" src="https://img.shields.io/badge/github--MAA28-Signed--Distance--Fields?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/MAA28/spreadsheet_addresses)
[<img alt="crates.io" src="https://img.shields.io/crates/v/signed_distance_field?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/spreadsheet_addresses)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-Signed--Distance--Field-b84a6e1?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/crate/spreadsheet_addresses/latest)

> Convert spreadsheet addresses to coordinates and vice versa in a fully tested and documented way!

![Excel training image](https://support.content.office.net/en-us/media/392b535f-990e-4f17-9d5d-be021cf1eacd.jpg)

Are you working on a spreadsheet application or something of the sort? What, you are not sure how to implement these address names? No worries - i have got you covered!

## Examples

### Convert from addresses to structs...

```rust
use spreadsheet_addresses::{Coordinate, AddressParsingError};

let coordinate1 = Coordinate::from_address("$CV23");
let coordinate2 = Coordinate::from_address("Hello World");

assert_eq!(
    coordinate1,
    Ok(Coordinate {
        row: 22,
        column: 99,
        relative_row: true,
        relative_column: false,
    })
);

assert_eq!(
    coordinate2,
    Err(AddressParsingError {
        input: "Hello World".to_string()
    })
);
```

### ...or the other way around!

```rust
use spreadsheet_addresses::Coordinate;

let coordinate = Coordinate::new(22, 99, true, false);

let address = coordinate.to_address();

assert_eq!(
    address,
    "$CV23".to_string()
);
```


