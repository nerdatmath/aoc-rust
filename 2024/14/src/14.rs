use orthant::Orthant;
use parse_display::FromStr;
use std::{cmp::Ordering, fmt::Debug, marker::PhantomData, str::FromStr};

use itertools::Itertools;

type Scalar = u8;

trait Modular: modular::Modular<Scalar = Scalar> {}

impl<T: modular::Modular<Scalar = Scalar>> Modular for T {}

mod modular;

use coord::Coord;

mod coord;

mod orthant;

#[derive(Debug, Clone, Copy, FromStr)]
#[display("{p} {v}")]
struct Robot<Coord> {
    #[display("p={}")]
    p: Coord,
    #[display("v={}")]
    v: Coord,
}

trait Runnable {
    fn run(&mut self, steps: Scalar);
    fn step(&mut self);
}

impl<Coord> Runnable for Robot<Coord>
where
    Coord: std::ops::Mul<Scalar, Output = Coord>,
    Coord: std::ops::AddAssign,
    Coord: Copy,
{
    fn run(&mut self, steps: Scalar) {
        self.p += self.v * steps;
    }
    fn step(&mut self) {
        self.p += self.v;
    }
}

trait Part {
    fn run(input: &str) -> usize;
}

struct Part1Base<Coord> {
    _marker: PhantomData<Coord>,
}

impl<Coord> Part for Part1Base<Coord>
where
    Coord: Orthant<Output = (Ordering, Ordering)> + Debug,
    Robot<Coord>: Runnable + FromStr + Clone,
    <Robot<Coord> as FromStr>::Err: Debug,
{
    fn run(input: &str) -> usize {
        input
            .lines()
            .map(|s| s.parse::<Robot<Coord>>().expect("Parse failed."))
            .map(|r| {
                let mut r = r.clone();
                r.run(100);
                r.p.orthant()
            })
            .counts()
            .into_iter()
            .filter_map(|((x, y), n)| (x.is_ne() && y.is_ne()).then_some(n))
            .product::<usize>()
    }
}

struct Part2Base<Coord> {
    _marker: PhantomData<Coord>,
}

fn map_robots<Row, Col>(robots: &Vec<Robot<Coord<Row, Col>>>) -> grid::Grid<bool>
where
    Row: Modular + Clone,
    Col: Modular + Clone,
    usize: From<Row>,
    usize: From<Col>,
{
    let mut grid = grid::Grid::new(
        <Row as modular::Modular>::MODULUS.into(),
        <Col as modular::Modular>::MODULUS.into(),
    );
    for r in robots {
        let Coord { row, col } = &r.p;
        grid[(row.clone().into(), col.clone().into())] = true;
    }
    grid
}

fn christmassy(grid: &grid::Grid<bool>) -> bool {
    grid.iter_rows().any(|row| {
        row.chunk_by(|&b| b)
            .into_iter()
            .any(|(&b, i)| b && i.count() > 15)
    })
}

impl<Row, Col> Part for Part2Base<Coord<Row, Col>>
where
    Row: Modular + Clone,
    Col: Modular + Clone,
    usize: From<Row> + From<Col>,
    Robot<Coord<Row, Col>>: Runnable + FromStr,
    <Robot<Coord<Row, Col>> as FromStr>::Err: Debug,
{
    fn run(input: &str) -> usize {
        let mut robots: Vec<Robot<Coord<Row, Col>>> = input
            .lines()
            .map(|s| s.parse().expect("Parse failed."))
            .collect();
        for i in 0..(usize::from(<Row as modular::Modular>::MODULUS)
            * usize::from(<Col as modular::Modular>::MODULUS))
        {
            for r in &mut robots {
                r.step();
            }
            if christmassy(&map_robots(&robots)) {
                return i + 1;
            }
        }
        0
    }
}

type Part1 = Part1Base<Coord<modular::Mod<103>, modular::Mod<101>>>;
type Part2 = Part2Base<Coord<modular::Mod<103>, modular::Mod<101>>>;

#[cfg(test)]
mod tests {
    use super::{Part, Part1Base};
    use crate::modular::Mod;

    type Part1 = Part1Base<super::Coord<Mod<7>, Mod<11>>>;

    const EXAMPLE: &'static str = include_str!("../data/example/input");

    #[test]
    fn test_part1() {
        assert_eq!(Part1::run(EXAMPLE), 12);
    }
}

fn main() {
    let input = include_str!("../data/actual/input");
    println!("Part 1: {}", Part1::run(input));
    println!("Part 2: {}", Part2::run(input));
}
