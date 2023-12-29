use std::{fmt, num::ParseIntError};

#[derive(Debug)]
pub struct ScratchingError(String);

impl fmt::Display for ScratchingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ScratchingError {}

pub fn part_1(input: &str) -> Result<i32, ScratchingError> {
    let result = parse(input);
    match result {
        Err(ScratchingError(x)) => Err(ScratchingError(x)),
        Ok(scratch_cards) => Ok(scratch_cards.iter().map(|card| card.score()).sum()),
    }
}

pub fn part_2(input: &str) -> Result<i32, ScratchingError> {
    let scratch_cards = parse(input).unwrap();

    let mut piles = vec![1usize; scratch_cards.len()];
    for (index, scratch_card) in scratch_cards.iter().enumerate() {
        let winners = scratch_card.winners() as usize;
        for i in index + 1..index + 1 + winners {
            piles[i] += piles[index]
        }
    }

    let result = piles.iter().sum::<usize>() as i32;
    Ok(result)
}

fn parse(input: &str) -> Result<Vec<ScratchCard>, ScratchingError> {
    if input.is_empty() {
        return Err(ScratchingError("Error: Empty input".to_string()));
    }

    let mut result: Vec<ScratchCard> = Vec::new();
    for line in input.lines() {
        let scratch_card = ScratchCard::from_string(line).unwrap();
        result.push(scratch_card);
    }
    Ok(result)
}
#[derive(PartialEq, Debug)]
struct ScratchCard {
    id: i32,
    winning_numbers: Vec<i32>,
    chosen_numbers: Vec<i32>,
}

impl ScratchCard {
    pub fn from_string(line: &str) -> Result<ScratchCard, ScratchingError> {
        let (card_id, numbers) = line.split_once(": ").unwrap();
        let (winning, chosen) = numbers.split_once(" | ").unwrap();
        let id = ScratchCard::parse_id(card_id).unwrap();

        let winning_numbers = ScratchCard::parse_numbers(winning).unwrap();
        let chosen_numbers = ScratchCard::parse_numbers(chosen).unwrap();
        Ok(ScratchCard {
            id,
            winning_numbers,
            chosen_numbers,
        })
    }
    fn parse_id(card_id: &str) -> Result<i32, ParseIntError> {
        let (_, raw_id) = card_id.split_once(' ').unwrap();
        raw_id.trim().to_string().parse::<i32>()
    }

    fn parse_numbers(input: &str) -> Result<Vec<i32>, ParseIntError> {
        let mut result = Vec::new();

        for raw_number in input.split_whitespace() {
            result.push(raw_number.trim().parse::<i32>().unwrap());
        }

        Ok(result)
    }

    pub fn score(&self) -> i32 {
        let count = self
            .chosen_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count() as i32;
        match count {
            0 => count,
            x => 1 << (x - 1),
        }
    }

    pub fn winners(&self) -> i32 {
        self.chosen_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    #[rstest]
    fn test_matches_count_can_non_zero() {
        let scratch_card = ScratchCard::from_string("Card   1: 23 | 23 9").unwrap();

        assert_eq!(scratch_card.winners(), 1);
    }

    #[rstest]
    fn test_matches_count_can_be_zero() {
        let scratch_card = ScratchCard::from_string("Card   1: 23 | 63 9").unwrap();

        assert_eq!(scratch_card.winners(), 0);
    }

    #[rstest]
    fn test_score_can_be_zero() {
        let scratch_card = ScratchCard::from_string("Card   1: 23 | 63 9").unwrap();

        assert_eq!(scratch_card.score(), 0);
    }
    #[rstest]
    fn test_can_be_one() {
        let scratch_card = ScratchCard::from_string("Card   1: 23 | 23 9").unwrap();

        assert_eq!(scratch_card.score(), 1);
    }

    #[rstest]
    fn test_score_is_doubled_for_more_than_two_matches() {
        let scratch_card = ScratchCard::from_string("Card   1: 23 9 17 | 23 9 17 66").unwrap();

        assert_eq!(scratch_card.score(), 4);
    }

    #[rstest]
    fn test_scratchcard_is_parsed() {
        let result = parse("vCard   1: 23 | 63 9");
        let expected = &ScratchCard {
            id: 1,
            winning_numbers: vec![23],
            chosen_numbers: vec![63, 9],
        };

        assert!(result.is_ok());
        let result_scratch_card = &result.unwrap()[0];

        assert_eq!(result_scratch_card, expected)
    }

    #[rstest]
    fn test_create_scratchcard_from_string() {
        let result = parse("Card   1: 23 | 63 9");

        assert!(result.is_ok());
        let expected = result.unwrap();
        assert_eq!(expected.len(), 1);
    }

    #[rstest]
    fn test_parse_error_when_empty_input() {
        let result = parse("");

        assert!(result.is_err());
    }
}
