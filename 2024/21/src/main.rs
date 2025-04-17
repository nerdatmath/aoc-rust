mod costs;
mod data;
mod direction;
mod directional_keypad;
mod keypad;
mod numeric_keypad;
mod part1;
mod part2;
mod puzzle;

fn main() {
    use data::INPUT;
    println!("Part 1: {}", part1::run(INPUT));
    println!("Part 2: {}", part2::run(INPUT));
}
