mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    let input = std::fs::read_to_string("inputs/day01.txt").unwrap();
    println!("01-1: {}", day01::part1(&input));
    println!("01-2: {}", day01::part2(&input));

    let input = std::fs::read_to_string("inputs/day02.txt").unwrap();
    println!("02-1: {}", day02::part1(&input).unwrap());
    println!("02-2: {}", day02::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day03.txt").unwrap();
    println!("03-1: {}", day03::part1(&input));
    println!("03-2: {}", day03::part2(&input));

    let input = std::fs::read_to_string("inputs/day04.txt").unwrap();
    println!("04-1: {}", day04::part1(&input));
    println!("04-2: {}", day04::part2(&input));

    let input = std::fs::read_to_string("inputs/day05.txt").unwrap();
    println!("05-1: {}", day05::part1(&input).unwrap());
    println!("05-2: {}", day05::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day06.txt").unwrap();
    println!("06-1: {}", day06::part1(&input).unwrap());
    println!("06-2: {}", day06::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day07.txt").unwrap();
    println!("07-1: {}", day07::part1(&input).unwrap());
    println!("07-2: {}", day07::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day08.txt").unwrap();
    println!("08-1: {}", day08::part1(&input).unwrap());
    // println!("08-2: {}", day08::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day09.txt").unwrap();
    println!("09-1: {}", day09::part1(&input));
    println!("09-2: {}", day09::part2(&input));

    let input = std::fs::read_to_string("inputs/day10.txt").unwrap();
    println!("10-1: {}", day10::part1(&input).unwrap());
}
