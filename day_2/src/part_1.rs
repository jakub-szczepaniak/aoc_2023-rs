use std::fmt::Error;
#[derive(Default)]
pub struct Draw {
    red: Option<usize>,
    green: Option<usize>,
    blue: Option<usize>,
}

impl Draw {
    pub fn new(red: Option<usize>, green: Option<usize>, blue: Option<usize>) -> Draw {
        Draw { red, green, blue }
    }

    pub fn red(&self) -> Option<usize> {
        self.red
    }

    pub fn blue(&self) -> Option<usize> {
        self.blue
    }

    pub fn green(&self) -> Option<usize> {
        self.green
    }

    pub fn possible(&self) -> bool {
        self.red <= Some(12) && self.green <= Some(13) && self.blue <= Some(14)
    }

    pub fn from_string(draw: &str) -> Draw {
        let mut result = Draw::default();
        for cube in draw.split(", ") {
            let (amount, color) = cube.split_once(' ').unwrap();
            let value: usize = amount.parse::<usize>().unwrap();
            match &color[0..1] {
                "r" => result.red = Some(value),
                "b" => result.blue = Some(value),
                "g" => result.green = Some(value),
                _ => panic!("Something is wrong"),
            }
        }
        result
    }
}

pub struct Game {
    id: usize,
    pub draws: Vec<Draw>,
}

impl Game {
    pub fn new(id: usize, draws: Vec<Draw>) -> Game {
        Game { id, draws }
    }
    pub fn valid(&self) -> bool {
        self.draws.iter().all(|draw| draw.possible())
    }

    pub fn from_string(line: &str) -> Game {
        let (id, rest) = Game::parse_line(line);
        let draws = Game::parse_draws(rest.unwrap());
        Game::new(id, draws)
    }

    pub fn id(&self) -> usize {
        self.id
    }

    fn parse_draws(draws: &str) -> Vec<Draw> {
        let mut result = Vec::new();
        for draw in draws.split("; ") {
            let parsed_draw = Draw::from_string(draw);
            result.push(parsed_draw);
        }
        result
    }

    fn parse_line(line: &str) -> (usize, Option<&str>) {
        let mut elements = line.split(": ");
        let id_part = elements.next().unwrap();
        let id = id_part.split_once(' ').unwrap().1.parse::<usize>().unwrap();
        let rest = elements.next();
        (id, rest)
    }
}

pub fn process(input: &str) -> Result<usize, Error> {
    let result: usize = input
        .lines()
        .map(Game::from_string)
        .filter(|game| game.valid())
        .map(|game| game.id())
        .sum();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_game_stats_one_draw() {
        let line: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from_string(line);

        assert!(game.valid())
    }

    #[test]
    fn test_game_with_one_draw_is_valid() {
        let draw = Draw::new(Some(1), None, None);
        let game = Game::new(1, vec![draw]);

        assert!(game.valid());
    }

    #[test]
    fn test_game_with_one_draw_is_invalid() {
        let draw = Draw::new(Some(17), None, None);
        let game = Game::new(1, vec![draw]);

        assert!(!game.valid());
    }
    #[test]
    fn test_game_with_many_draws_is_valid() {
        let draw1 = Draw::new(Some(12), Some(13), Some(14));
        let draw2 = Draw::new(Some(1), Some(12), Some(10));

        let game = Game::new(1, vec![draw1, draw2]);

        assert!(game.valid());
    }
    #[test]
    fn test_game_with_many_draws_is_invalid() {
        let draw1 = Draw::new(Some(12), Some(13), Some(14));
        let draw2 = Draw::new(Some(78), Some(12), Some(10));

        let game = Game::new(1, vec![draw1, draw2]);

        assert!(!game.valid());
    }
}
