use std::collections::HashSet;
use std::fmt;

pub fn part_1(input: &str) -> Result<usize, SchematicsError> {
    Ok(parse(input).unwrap().symbols.len())
}

pub fn part_2(input: &str) -> Result<usize, SchematicsError> {
    Ok(parse(input).unwrap().symbols.len())
}

fn parse(input: &str) -> Result<Schematics, SchematicsError> {
    let mut symbol_schematics: Schematics = Schematics::new();

    if input.is_empty() {
        return Err(SchematicsError("Empty input".to_string()));
    }

    let mut current_number: Option<PartNumber> = None;

    for (row, line) in input.lines().enumerate() {
        for (column, symbol) in line.chars().enumerate() {
            if symbol.is_ascii_digit() {
                if let Some(num) = current_number.as_mut() {
                    num.add_digit(symbol)
                } else {
                    current_number = Some(PartNumber::new(symbol, column as i32, row as i32));
                }
            } else {
                if let Some(num) = current_number.take() {
                    symbol_schematics.add_part_number(num);
                }
                if symbol != '.' {
                    symbol_schematics.add_symbol((column as i32, row as i32));
                }
            }
        }
    }
    if let Some(num) = current_number.take() {
        symbol_schematics.add_part_number(num);
    }

    Ok(symbol_schematics)
}

#[derive(Debug)]
pub struct SchematicsError(String);

impl fmt::Display for SchematicsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for SchematicsError {}

#[derive(PartialEq, Debug)]
struct Schematics {
    symbols: HashSet<(i32, i32)>,
    part_numbers: Vec<PartNumber>,
}

impl Schematics {
    pub fn new() -> Schematics {
        Schematics {
            symbols: HashSet::new(),
            part_numbers: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, coordinates: (i32, i32)) {
        self.symbols.insert(coordinates);
    }

    pub fn add_part_number(&mut self, part_number: PartNumber) {
        self.part_numbers.push(part_number);
    }
}

#[derive(PartialEq, Debug)]
struct PartNumber {
    value: i32,
    coordinates: HashSet<(i32, i32)>,
}

impl PartNumber {
    pub fn new(value: char, x: i32, y: i32) -> PartNumber {
        PartNumber {
            value: (value as u8 - b'0') as i32,
            coordinates: PartNumber::adjacent_coordinates(x, y),
        }
    }

    fn adjacent_coordinates(x: i32, y: i32) -> HashSet<(i32, i32)> {
        let mut result = HashSet::new();
        result.insert((x - 1, y));
        result.insert((x - 1, y + 1));
        result.insert((x - 1, y - 1));
        result.insert((x, y + 1));
        result.insert((x, y - 1));
        result.insert((x + 1, y));
        result.insert((x + 1, y + 1));
        result.insert((x + 1, y - 1));
        result
    }
    pub fn add_digit(&mut self, symbol: char) {
        let new_digit = (symbol as u8 - b'0') as i32;
        self.value = self.value * 10 + new_digit;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;

    #[fixture]
    fn symbols() -> Schematics {
        let result = Schematics::new();
        return result;
    }

    #[rstest]
    fn test_parse_single_line_no_symbols(symbols: Schematics) {
        let result = parse(".");
        assert!(result.is_ok());
        let symbol_schematic = result.unwrap();
        assert_eq!(symbol_schematic, symbols);
    }
    #[rstest]
    fn test_error_when_empty_input() {
        let result = parse("");
        assert!(result.is_err());
    }
    #[rstest]
    fn test_parse_single_line_single_symbol(symbols: Schematics) {
        let mut expected = symbols;
        expected.add_symbol((0, 0));

        let result = parse("*");
        assert!(result.is_ok());
        let symbols_schematics = result.unwrap();
        assert_eq!(symbols_schematics, expected)
    }
    #[rstest]
    fn test_parse_two_lines_single_symbol(symbols: Schematics) {
        let mut expected = symbols;
        expected.add_symbol((0, 1));

        let result = parse(".\n@");
        assert!(result.is_ok());
        let symbol_schematics = result.unwrap();
        assert_eq!(symbol_schematics, expected);
    }

    #[rstest]
    fn test_parse_one_line_single_number(symbols: Schematics) {
        let mut expected = symbols;
        expected.add_part_number(PartNumber {
            value: 1,
            coordinates: adjacent_coordinates(0, 0),
        });

        let result = parse("1");
        assert!(result.is_ok());
        let result_schematics = result.unwrap();
        assert_eq!(result_schematics, expected);
    }
    #[rstest]
    fn test_parse_one_line_two_digits(symbols: Schematics) {
        let mut expected = symbols;

        expected.add_part_number(PartNumber {
            value: 12,
            coordinates: adjacent_coordinates(0, 0),
        });

        let result = parse("12");

        assert!(result.is_ok());

        let result_schematics = result.unwrap();
        assert_eq!(result_schematics, expected);
    }

    #[rstest]
    fn test_parse_two_lines_two_numbers(symbols: Schematics) {
        let mut expected = symbols;
        expected.add_part_number(PartNumber {
            value: 13,
            coordinates: adjacent_coordinates(2, 0),
        });
        expected.add_part_number(PartNumber {
            value: 56,
            coordinates: adjacent_coordinates(6, 0),
        });
        expected.add_symbol((0, 0));

        let result = parse("*.13..56..");

        assert!(result.is_ok());
        let result_schematics = result.unwrap();

        assert_eq!(result_schematics, expected)
    }

    fn adjacent_coordinates(x: i32, y: i32) -> HashSet<(i32, i32)> {
        return PartNumber::adjacent_coordinates(x, y);
    }

    #[rstest]
    fn test_parse_one_line_part_number_with_coordinates(symbols: Schematics) {
        let mut expected = symbols;
        let mut part_number = PartNumber {
            value: 12,
            coordinates: adjacent_coordinates(0, 0),
        };
        part_number.coordinates.extend([(1, 0), (1, 1), (1, -1)]);
        expected.add_part_number(part_number);

        let result = parse("12");
        assert!(result.is_ok());

        let result_schematics = result.unwrap();

        assert_eq!(result_schematics, expected);
    }
}
