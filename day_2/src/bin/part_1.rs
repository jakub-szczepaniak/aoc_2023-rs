use day_2::part_1::process;
use day_2::part_2::process as part2;
fn main() {
    let input = include_str!("../../input.txt");

    let result = process(input);

    match result {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    let result2 = part2(input);
    match result2 {
        Ok(result) => println!("Result part 2: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
