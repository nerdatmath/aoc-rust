mod data;
mod direction;
mod instructions;
mod nodes;
mod part1;
mod part2;
mod puzzle;
mod state;

fn main() {
    use data::INPUT;
    println!("Part 1: {}", part1::run(INPUT));
    println!("Part 2: {}", part2::run(INPUT));
}
