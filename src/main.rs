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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day23;
mod day24;
mod day25;

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
    println!("08-2: {}", day08::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day09.txt").unwrap();
    println!("09-1: {}", day09::part1(&input));
    println!("09-2: {}", day09::part2(&input));

    let input = std::fs::read_to_string("inputs/day10.txt").unwrap();
    println!("10-1: {}", day10::part1(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day11.txt").unwrap();
    println!("11-1: {}", day11::part1(&input));
    println!("11-2: {}", day11::part2(&input));

    let input = std::fs::read_to_string("inputs/day12.txt").unwrap();
    // println!("12-1: {}", day12::part1(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day13.txt").unwrap();
    println!("13-1: {}", day13::part1(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day14.txt").unwrap();
    println!("14-1: {}", day14::part1(&input).unwrap());
    println!("14-2: {}", day14::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day15.txt").unwrap();
    println!("15-1: {}", day15::part1(&input));
    println!("15-2: {}", day15::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day16.txt").unwrap();
    println!("16-1: {}", day16::part1(&input).unwrap());
    println!("16-2: {}", day16::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day17.txt").unwrap();
    println!("17-1: {}", day17::part1(&input).unwrap());
    println!("17-2: {}", day17::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day18.txt").unwrap();
    println!("18-1: {}", day18::part1(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day19.txt").unwrap();
    println!("19-1: {}", day19::part1(&input).unwrap());
    println!("19-2: {}", day19::part2(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day20.txt").unwrap();
    println!("20-1: {}", day20::part1(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day21.txt").unwrap();
    println!("21-1: {}", day21::part1(&input, 64).unwrap());
    // println!("21-2: {}", day21::part2(&input, 26501365).unwrap());

    let input = std::fs::read_to_string("inputs/day23.txt").unwrap();
    println!("23-1: {}", day23::part1(&input).unwrap());

    let input = std::fs::read_to_string("inputs/day24.txt").unwrap();
    println!("24-1: {}", day24::part1(&input, 2e14..=4e14).unwrap());

    let input = std::fs::read_to_string("inputs/day25.txt").unwrap();
    println!("25-1: {}", day25::part1(&input).unwrap());
}
