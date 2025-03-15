use game_grid::{Grid, GridPosition};
use itertools::iproduct;

fn parse(input: &str) -> Grid<char> {
    input.parse().unwrap()
}

// A 2D point struct deriving GridPosition.
#[derive(GridPosition, PartialEq, Eq, Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl std::ops::Add<(i32, i32)> for Pos {
    type Output = Self;

    fn add(self, (dx, dy): (i32, i32)) -> Self::Output {
        Self::Output {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

trait Part {
    fn xmas_count_at(grid: &Grid<char>, pos: Pos) -> usize;
    fn xmas_count(grid: &Grid<char>) -> usize {
        (0..grid.len())
            .map(|i| Self::xmas_count_at(grid, grid.position_for_index::<Pos>(i)))
            .sum()
    }
    fn run(input: &str) -> usize {
        let grid: Grid<char> = parse(input);
        Self::xmas_count(&grid)
    }
}

struct PartN<const N: u8>;
type Part1 = PartN<1>;
type Part2 = PartN<2>;

fn get_directional(grid: &Grid<char>, pos: Pos, dx: i32, dy: i32) -> impl Iterator<Item = char> {
    itertools::iterate(pos, move |&pos| pos + (dx, dy))
        .take_while(|&pos| grid.is_in_bounds(pos))
        .map(|pos| grid[pos])
}

fn match_string(grid: &Grid<char>, pos: Pos, dx: i32, dy: i32, s: &str) -> bool {
    get_directional(grid, pos, dx, dy)
        .take(s.len())
        .eq(s.chars())
}

impl Part for Part1 {
    fn xmas_count_at(grid: &Grid<char>, pos: Pos) -> usize {
        iproduct!(-1..=1, -1..=1)
            .filter(|&(dx, dy)| match_string(grid, pos, dx, dy, "XMAS"))
            .count()
    }
}

impl Part for Part2 {
    fn xmas_count_at(grid: &Grid<char>, pos: Pos) -> usize {
        [(1, 1), (1, -1)]
            .into_iter()
            .all(|(dx, dy)| {
                [(-dx, -dy), (dx, dy)]
                    .into_iter()
                    .any(|(dx, dy)| match_string(grid, pos + (-dx, -dy), dx, dy, "MAS"))
            })
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::{Part, Part1, Part2};

    const EXAMPLE: &'static str = include_str!("../data/example/input");

    #[test]
    fn test_part1() {
        assert_eq!(Part1::run(EXAMPLE), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Part2::run(EXAMPLE), 9);
    }
}

fn main() {
    let input = include_str!("../data/actual/input");
    println!("Part 1: {}", Part1::run(input));
    println!("Part 2: {}", Part2::run(input));
}
