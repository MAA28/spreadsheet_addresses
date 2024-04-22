//! # Spreadsheet addresses
//! [<img alt="github" src="https://img.shields.io/badge/github-Spreadsheet-Addresses?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/MAA28/spreadsheet_addresses)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/spreadsheet_addresses?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/spreadsheet_addresses)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-Spreadsheet-Addresses?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/crate/spreadsheet_addresses/latest)
//! 
//! > Convert spreadsheet addresses to coordinates and vice versa in a fully tested and documented way!
//! 
//! ![Excel training image](https://support.content.office.net/en-us/media/392b535f-990e-4f17-9d5d-be021cf1eacd.jpg)
//! 
//! Are you working on a spreadsheet application or something of the sort? What, you are not sure how to implement these address names? No worries - i have got you covered!
//! 
//! ## Examples
//! 
//! ### Convert from addresses to structs...
//! 
//! ```rust
//! use spreadsheet_addresses::{Coordinate, AddressParsingError};
//! 
//! let coordinate1 = Coordinate::from_address("$CV23");
//! let coordinate2 = Coordinate::from_address("Hello World");
//! 
//! assert_eq!(
//!     coordinate1,
//!     Ok(Coordinate {
//!         row: 22,
//!         column: 99,
//!         relative_row: true,
//!         relative_column: false,
//!     })
//! );
//! 
//! assert_eq!(
//!     coordinate2,
//!     Err(AddressParsingError {
//!         input: "Hello World".to_string()
//!     })
//! );
//! ```
//! 
//! ### ...or the other way around!
//! 
//! ```rust
//! use spreadsheet_addresses::Coordinate;
//! 
//! let coordinate = Coordinate::new(22, 99, true, false);
//! 
//! let address = coordinate.to_address();
//! 
//! assert_eq!(
//!     address,
//!     "$CV23".to_string()
//! );
//! ```


use std::str::from_utf8;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::digit1,
    combinator::opt,
    error::ErrorKind,
    IResult,
};

mod tests;

#[derive(Debug, PartialEq)]
/// This struct encodes an spreadsheet address.
pub struct Coordinate {
    pub row: u32,
    pub column: u32,
    pub relative_row: bool,
    pub relative_column: bool,
}

impl Coordinate {
    /// Constructs acoordinate struct
    ///
    /// # Example
    /// ```rust
    /// use spreadsheet_addresses::Coordinate;
    ///
    /// let coordinate = Coordinate::new(16, 2, true, false);
    ///
    /// assert_eq!(
    ///     coordinate,
    ///     Coordinate {
    ///         row: 16,
    ///         column: 2,
    ///         relative_row: true,
    ///         relative_column: false,
    ///     }
    /// );
    /// ```
    pub fn new(row: u32, column: u32, relative_row: bool, relative_column: bool) -> Coordinate {
        Coordinate {
            row,
            column,
            relative_row,
            relative_column,
        }
    }

    fn parse(input: &[u8]) -> IResult<&[u8], (bool, &[u8], bool, &[u8])> {
        let (input, absolute_character_1) = opt(tag("$".to_owned().as_bytes()))(input)?;
        let (input, capital_letters) = take_while1(|input| {
            "ABCDEFGHIJKLMNOPQRSTUWVXYZ".contains(from_utf8(&[input]).unwrap())
        })(input)?;
        let (input, absolute_character_2) = opt(tag("$".to_owned().as_bytes()))(input)?;
        let (input, numbers) = digit1(input)?;
        if input.len() > 0 {
            return Err(nom::Err::Error(nom::error::Error::<&[u8]> {
                code: ErrorKind::TooLarge,
                input,
            }));
        }
        return Ok((
            input,
            (
                absolute_character_1.is_some(),
                capital_letters,
                absolute_character_2.is_some(),
                numbers,
            ),
        ));
    }

    /// Construct a coordinate struct from an address
    ///
    /// # Examples
    /// ```rust
    /// use spreadsheet_addresses::{Coordinate, AddressParsingError};
    ///
    /// let coordinate1 = Coordinate::from_address("$CV23");
    /// let coordinate2 = Coordinate::from_address("Hello World");
    ///
    /// assert_eq!(
    ///     coordinate1,
    ///     Ok(Coordinate {
    ///         row: 22,
    ///         column: 99,
    ///         relative_row: true,
    ///         relative_column: false,
    ///     })
    /// );
    ///
    /// assert_eq!(
    ///     coordinate2,
    ///     Err(AddressParsingError {
    ///         input: "Hello World".to_string()
    ///     })
    /// );
    /// ```
    pub fn from_address(address: &str) -> Result<Coordinate, AddressParsingError> {
        let (_, (absolute_character_1, letgits, absolute_character_2, numbers)) =
            Coordinate::parse(address.as_bytes()).map_err(|_| AddressParsingError {
                input: address.to_owned(),
            })?;

        let column: u32 = letgits
            .iter()
            .enumerate()
            .map(|(i, letter)| {
                let power = letgits.len() as u32 - i as u32 - 1;
                let mantissa = letter.to_owned() as u32 - 64;
                return 26_u32.pow(power) * mantissa;
            })
            .sum::<u32>()
            - 1;

        let row: u32 = from_utf8(numbers)
            .map_err(|_| AddressParsingError {
                input: address.to_owned(),
            })?
            .parse::<u32>()
            .map_err(|_| AddressParsingError {
                input: address.to_owned(),
            })?
            - 1;

        return Ok(Coordinate::new(
            row as u32,
            column as u32,
            !absolute_character_2,
            !absolute_character_1,
        ));
    }

    /// Outputs an address from the coordinate
    ///
    /// # Examples
    /// ```rust
    /// use spreadsheet_addresses::Coordinate;
    ///
    /// let coordinate = Coordinate::new(22, 99, true, false);
    ///
    /// let address = coordinate.to_address();
    ///
    /// assert_eq!(
    ///     address,
    ///     "$CV23".to_string()
    /// );
    /// ```
    pub fn to_address(&self) -> String {
        let mut letgits = String::new();

        let mut rest = self.column as i32;

        while rest >= 0 {
            letgits.push(char::from_u32(rest as u32 % 26 + 65).unwrap());
            rest = rest.checked_div(26).unwrap() - 1;
        }
        letgits = letgits.chars().rev().collect::<String>();

        let digits = (self.row + 1).to_string();

        return format!(
            "{}{}{}{}",
            if self.relative_column { "" } else { "$" },
            letgits,
            if self.relative_row { "" } else { "$" },
            digits,
        );
    }
}

#[derive(Debug, PartialEq)]
/// The passed input is impossible to parse as an address.
pub struct AddressParsingError {
    /// This input can not be parsed as an address.
    pub input: String,
}
