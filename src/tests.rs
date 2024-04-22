#[cfg(test)]
mod convert_coordinates_to_addresses_and_back_0_to_n {

    use crate::Coordinate;
    
    const N: u32 = 1000;

    #[test]
    fn row_absolute_column_absolute() {
        for row in 0..N {
            for column in 0..N {
                let coordinate = Coordinate::new(row, column, false, false);
                let address = coordinate.to_address().to_owned();
                let parsed_coordinate = Coordinate::from_address(&address).unwrap();
                assert_eq!(coordinate, parsed_coordinate);
            }
        }
    }

    #[test]
    fn row_relative_column_relative() {
        for row in 0..N {
            for column in 0..N {
                let coordinate = Coordinate::new(row, column, false, false);
                let address = coordinate.to_address().to_owned();
                let parsed_coordinate = Coordinate::from_address(&address).unwrap();
                assert_eq!(coordinate, parsed_coordinate);
            }
        }
    }

    #[test]
    fn row_absolute_column_relative() {
        for row in 0..N {
            for column in 0..N {
                let coordinate = Coordinate::new(row, column, true, false);
                let address = coordinate.to_address().to_owned();
                let parsed_coordinate = Coordinate::from_address(&address).unwrap();
                assert_eq!(coordinate, parsed_coordinate);
            }
        }
    }

    #[test]
    fn row_relative_column_absolute() {
        for row in 0..N {
            for column in 0..N {
                let coordinate = Coordinate::new(row, column, false, true);
                let address = coordinate.to_address().to_owned();
                let parsed_coordinate = Coordinate::from_address(&address).unwrap();
                assert_eq!(coordinate, parsed_coordinate);
            }
        }
    }
}

