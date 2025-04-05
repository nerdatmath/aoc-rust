mod instruction;
mod machine;
mod puzzle;
mod part1;
mod part2;

fn main() {
    const INPUT: &'static str = include_str!("../data/input");
    println!("Part 1: {}", part1::run(INPUT));
    println!("Part 2: {}", part2::run(INPUT));
}
