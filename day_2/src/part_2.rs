use crate::part_1::*;
use std::cmp::max;
use std::fmt::Error;

impl Game {
    pub fn min_draw(&self) -> Draw {
        let mut result = Draw::default();
        for draw in self.draws.iter() {
            result = result.update_with(draw);
        }

        result
    }

    pub fn power_of_set(&self) -> usize {
        self.min_draw().power_of_draw()
    }
}

impl Draw {
    pub fn update_with(&self, other: &Draw) -> Draw {
        let red = max(self.red(), other.red());
        let blue = max(self.blue(), other.blue());
        let green = max(self.green(), other.green());

        Draw::new(red, green, blue)
    }
    pub fn power_of_draw(&self) -> usize {
        self.blue().unwrap() * self.green().unwrap() * self.red().unwrap()
    }
}

pub fn process(input: &str) -> Result<usize, Error> {
    let result: usize = input
        .lines()
        .map(Game::from_string)
        .map(|game| game.power_of_set())
        .sum();

    Ok(result)
}
