use std::{cmp::Ordering, fmt::Debug, hash::Hash, marker::PhantomData, str::FromStr};

use itertools::Itertools;

type Scalar = u16;

trait Modular: modular::Modular<Scalar = Scalar> {}

impl<T: modular::Modular<Scalar = Scalar>> Modular for T {}

mod modular;

modular!(Mod101, Scalar, 101);
modular!(Mod103, Scalar, 103);

use coord::Coord;

mod coord;

trait Orthant {
    type Output: Eq + Hash;
    fn orthant(&self) -> Self::Output;
}

fn center<Mod: Modular>() -> Scalar {
    (Mod::MODULUS - 1) / 2
}

impl<T> Orthant for T
where
    T: Modular + Into<Scalar> + Copy,
{
    type Output = Ordering;
    fn orthant(&self) -> Ordering {
        let s: Scalar = (*self).into();
        s.cmp(&center::<Self>())
    }
}

impl<Row, Col> Orthant for Coord<Row, Col>
where
    Row: Orthant,
    Col: Orthant,
{
    type Output = (<Row as Orthant>::Output, <Col as Orthant>::Output);
    fn orthant(&self) -> Self::Output {
        (self.row.orthant(), self.col.orthant())
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot<Coord> {
    p: Coord,
    v: Coord,
}

#[derive(Debug)]
struct ParseRobotError;

impl<Coord: FromStr> FromStr for Robot<Coord> {
    type Err = ParseRobotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(' ').ok_or(ParseRobotError)?;
        Ok(Self {
            p: p.strip_prefix("p=")
                .ok_or(ParseRobotError)?
                .parse()
                .map_err(|_| ParseRobotError)?,
            v: v.strip_prefix("v=")
                .ok_or(ParseRobotError)?
                .parse()
                .map_err(|_| ParseRobotError)?,
        })
    }
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

type Part1 = Part1Base<Coord<Mod103, Mod101>>;
type Part2 = Part2Base<Coord<Mod103, Mod101>>;

#[cfg(test)]
mod tests {
    use super::{Part, Part1Base, Scalar};
    use crate::modular;

    modular!(Mod7, Scalar, 7);
    modular!(Mod11, Scalar, 11);
    type Coord = super::Coord<Mod7, Mod11>;
    type Part1 = Part1Base<Coord>;

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
