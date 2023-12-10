use std::fmt::Error;

pub fn process(input: &str) -> Result<usize, Error> {
    let result = input
        .lines()
        .map(|line| extract_numbers(line).unwrap())
        .sum();

    Ok(result)
}

fn extract_numbers(line: &str) -> Result<usize, Error> {
    let numbers = line
        .chars()
        .filter(|ch| ch.is_ascii_digit())
        .map(|ch| ch as u8 - b'0')
        .collect::<Vec<_>>();
    let first = *numbers.first().unwrap();
    let last = *numbers.last().unwrap();
    Ok((10 * (first as u16) + (last as u16)).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::only_numbers("12", 12)]
    #[case::only_two_numbers("1abc2", 12)]
    #[case::three_numbers_no_text("132", 12)]
    #[case::three_numbers_with_text("13ab23nain2", 12)]
    fn test_extracting_numbers(#[case] input: &str, #[case] expected: usize) {
        let result = extract_numbers(input);
        assert!(result.is_ok());
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_sample_input() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(process(input), Ok(142));
    }
}
