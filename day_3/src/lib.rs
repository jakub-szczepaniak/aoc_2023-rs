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

    if input.len() <= 0 {
        return Err(SchematicsError("Empty input".to_string()));
    }
    for (row, line) in input.lines().enumerate() {
        for (column, symbol) in line.chars().enumerate() {
            if symbol != '.' {
                symbol_schematics
                    .symbols
                    .insert((column as i32, row as i32));
            }
        }
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
}

impl Schematics {
    pub fn new() -> Schematics {
        Schematics {
            symbols: HashSet::new(),
        }
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
        expected.symbols.insert((0, 0));

        let result = parse("*");
        assert!(result.is_ok());
        let symbols_schematics = result.unwrap();
        assert_eq!(symbols_schematics, expected)
    }
    #[rstest]
    fn test_parse_two_lines_single_symbol(symbols: Schematics) {
        let mut expected = symbols;
        expected.symbols.insert((0, 1));

        let result = parse(".\n@");
        assert!(result.is_ok());
        let symbol_schematics = result.unwrap();
        assert_eq!(symbol_schematics, expected);
    }
}
