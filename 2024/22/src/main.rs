mod part1;
mod part2;
mod puzzle;
mod secret;

fn main() {
    const INPUT: &'static str = include_str!("../data/input");
    println!("Part 1: {}", part1::run(INPUT));
    println!("Part 2: {}", part2::run(INPUT));
}
