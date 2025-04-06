mod find_path;
mod part1;
mod part2;
mod puzzle;

fn main() {
    const INPUT: &'static str = include_str!("../data/input");
    println!("Part 1: {}", part1::run(INPUT, 70, 1024));
    println!("Part 2: {}", part2::run(INPUT, 70));
}
