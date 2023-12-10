use std::fmt::Error;

pub fn process(input: &str) -> Result<usize, Error> {
    for line in input.lines() {
        println!("{}", line);
    }

    Ok(input.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(process(input), Ok(123));
    }
}
