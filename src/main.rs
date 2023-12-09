mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    println!("5-2: {}", day5::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
    println!("6-1: {}", day6::part1(&input).unwrap());
    println!("6-2: {}", day6::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day7.txt").unwrap();
    println!("7-1: {}", day7::part1(&input).unwrap());
    println!("7-2: {}", day7::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day8.txt").unwrap();
    println!("8-1: {}", day8::part1(&input).unwrap());
    // println!("8-2: {}", day8::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day9.txt").unwrap();
    println!("9-1: {}", day9::part1(&input));
    println!("9-2: {}", day9::part2(&input));
}
