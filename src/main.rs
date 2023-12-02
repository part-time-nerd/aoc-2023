mod day1;
mod day2;

fn main() {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    println!("1-1: {}", day1::part1(&input));
    println!("1-2: {}", day1::part2(&input));

    let input = std::fs::read_to_string("inputs/day2.txt").unwrap();
    println!("2-1: {}", day2::part1(&input).unwrap());
    println!("2-2: {}", day2::part2(&input).unwrap());
}
