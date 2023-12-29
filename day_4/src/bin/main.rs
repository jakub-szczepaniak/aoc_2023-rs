use day_4::*;

fn main() {
    let input = include_str!("../../input.txt");

    let result = part_1(input);

    match result {
        Ok(result) => println!("Result part 1: {}", result),
        Err(error) => println!("Error part 1: {}", error),
    }

    let result2 = part_2(input);
    match result2 {
        Ok(result) => println!("Result part 2: {}", result),
        Err(error) => println!("Error part 2: {}", error),
    }
}
