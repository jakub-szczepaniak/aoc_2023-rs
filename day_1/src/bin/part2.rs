use day_1::part_2::process;

fn main() {
    let input = include_str!("../../input.txt");

    let result = process(input);

    match result {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
