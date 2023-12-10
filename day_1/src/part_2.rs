use aho_corasick::AhoCorasick;
use std::fmt::Error;

pub fn process(input: &str) -> Result<usize, Error> {
    let aho = prepare_aho().unwrap();
    let result: usize = input
        .lines()
        .map(|line| extract_numbers(line, &aho).unwrap())
        .sum();
    Ok(result)
}

fn prepare_aho() -> Result<AhoCorasick, Error> {
    let keywords = vec![
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];

    Ok(AhoCorasick::new(keywords).unwrap())
}

fn extract_numbers(line: &str, pattern_matcher: &AhoCorasick) -> Result<usize, Error> {
    let mut numbers = pattern_matcher
        .find_overlapping_iter(line)
        .map(|m| m.pattern());
    let first = (numbers.next().unwrap().as_usize() / 2) + 1;
    // PaternID is the index of the keyword in the keywords vector.
    // for each digit we have 2 consecutive indexes, e.g - indexes 2, 3 -> 2
    if let Some(last_pattern) = numbers.last() {
        let last = last_pattern.as_usize() / 2 + 1;

        Ok(10 * first + last)
    } else {
        Ok(10 * first + first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    #[once]
    fn make_aho() -> AhoCorasick {
        prepare_aho().unwrap()
    }

    #[rstest]
    #[case::only_numbers("1twonine2", 12)]
    #[case::only_numbers("twonine", 29)]
    #[case::one_number("znfqfjcspf8md", 88)]
    fn test_extracting_numbers(#[case] input: &str, #[case] expected: usize) {
        let result = extract_numbers(input, &make_aho()).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn sample_input() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(process(input), Ok(281))
    }
}
