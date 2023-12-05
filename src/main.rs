mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    println!("1-1: {}", day1::part1(&input));
    println!("1-2: {}", day1::part2(&input));

    let input = std::fs::read_to_string("inputs/day2.txt").unwrap();
    println!("2-1: {}", day2::part1(&input).unwrap());
    println!("2-2: {}", day2::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day3.txt").unwrap();
    println!("3-1: {}", day3::part1(&input));
    println!("3-2: {}", day3::part2(&input));

    let input = std::fs::read_to_string("inputs/day4.txt").unwrap();
    println!("4-1: {}", day4::part1(&input));
    println!("4-2: {}", day4::part2(&input));

    let input = std::fs::read_to_string("inputs/day5.txt").unwrap();
    println!("5-1: {}", day5::part1(&input).unwrap());
}
