use day_2::part_1::process;
fn main() {
    let input = include_str!("../../input.txt");

    let result = process(input);

    match result {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
