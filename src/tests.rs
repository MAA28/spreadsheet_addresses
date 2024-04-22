#[cfg(test)]
mod development_tests {

    use crate::Coordinate;
    
    const N: u32 = 1000;

    #[test]
    fn convert_absolute_coordinates_to_addresses_and_back_0_to_n() {
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
    fn convert_relative_coordinates_to_addresses_and_back_0_to_n() {
        for row in 0..N {
            for column in 0..N {
                let coordinate = Coordinate::new(row, column, true, true);
                let address = coordinate.to_address().to_owned();
                let parsed_coordinate = Coordinate::from_address(&address).unwrap();
                assert_eq!(coordinate, parsed_coordinate);
            }
        }
    }
}

#[cfg(test)]
mod complete_tests {

    use crate::Coordinate;

    #[test]
    fn convert_absolute_coordinates_to_addresses_and_back_0_to_max() {
        for row in 0..u32::MAX {
            for column in 0..u32::MAX {
                let coordinate = Coordinate::new(row, column, false, false);
                let address = coordinate.to_address().to_owned();
                let parsed_coordinate = Coordinate::from_address(&address).unwrap();
                assert_eq!(coordinate, parsed_coordinate);
            }
        }
    }

    #[test]
    fn convert_relative_coordinates_to_addresses_and_back_0_to_max() {
        for row in 0..u32::MAX {
            for column in 0..u32::MAX {
                let coordinate = Coordinate::new(row, column, true, true);
                let address = coordinate.to_address().to_owned();
                let parsed_coordinate = Coordinate::from_address(&address).unwrap();
                assert_eq!(coordinate, parsed_coordinate);
            }
        }
    }
}
