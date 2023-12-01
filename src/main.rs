mod day1;

fn main() {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    println!("1-1: {}", day1::part1(&input));
    println!("1-2: {}", day1::part2(&input));
}
