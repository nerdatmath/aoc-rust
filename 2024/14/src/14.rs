use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, Mul},
    str::FromStr,
};

type Scalar = i32;

trait Modular {
    const MODULUS: Scalar;

    fn normalize(n: Scalar) -> Self;
}

macro_rules! modular {
    ( $name:ident, $modulus:literal ) => {
        #[derive(Clone)]
        struct $name(Scalar);

        impl Modular for $name {
            const MODULUS: Scalar = $modulus;

            fn normalize(n: Scalar) -> $name {
                $name(n.rem_euclid(Self::MODULUS))
            }
        }

        impl FromStr for $name {
            type Err = <Scalar as FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self::normalize(s.parse::<Scalar>()?))
            }
        }

        impl From<$name> for Scalar {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self::normalize(self.0 + rhs.0)
            }
        }

        impl Mul<Scalar> for $name {
            type Output = Self;

            fn mul(self, rhs: Scalar) -> Self {
                Self::normalize(self.0 * rhs)
            }
        }
    };
}

modular!(Mod7, 7);
modular!(Mod11, 11);
modular!(Mod101, 101);
modular!(Mod103, 103);

#[derive(Clone, Copy, Debug)]
struct Coord<Row, Col> {
    row: Row,
    col: Col,
}

struct ParseCoordError;

impl<Row: FromStr, Col: FromStr> FromStr for Coord<Row, Col> {
    type Err = ParseCoordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (col, row) = s.split_once(',').ok_or(ParseCoordError)?;
        Ok(Self {
            row: row.parse().map_err(|_| ParseCoordError)?,
            col: col.parse().map_err(|_| ParseCoordError)?,
        })
    }
}

impl<Row: Add, Col: Add> Add for Coord<Row, Col> {
    type Output = Coord<<Row as Add>::Output, <Col as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl<Row: Mul<Scalar>, Col: Mul<Scalar>> Mul<Scalar> for Coord<Row, Col> {
    type Output = Coord<<Row as Mul<Scalar>>::Output, <Col as Mul<Scalar>>::Output>;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Self::Output {
            row: self.row * rhs,
            col: self.col * rhs,
        }
    }
}

type Quadrant = (Ordering, Ordering);

trait HasQuadrant {
    fn quadrant(self) -> Quadrant;
}

impl<Row, Col> HasQuadrant for Coord<Row, Col>
where
    Row: Modular + Into<Scalar> + Clone,
    Col: Modular + Into<Scalar> + Clone,
{
    fn quadrant(self) -> Quadrant {
        let Coord { row, col } = self;
        (
            row.into().cmp(&((<Row as Modular>::MODULUS - 1) / 2)),
            col.into().cmp(&((<Col as Modular>::MODULUS - 1) / 2)),
        )
    }
}

#[derive(Debug)]
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
    type Output;

    fn run(&self, steps: Scalar) -> Self::Output;
}

impl<Coord> Runnable for Robot<Coord>
where
    Coord: Mul<Scalar, Output = Coord>,
    Coord: Add<Output = Coord>,
    Coord: Clone,
{
    type Output = Coord;

    fn run(&self, steps: Scalar) -> Self::Output {
        self.p.clone() + (self.v.clone() * steps)
    }
}

fn part1<Coord>(input: &str) -> usize
where
    Robot<Coord>: Runnable + FromStr,
    <Robot<Coord> as Runnable>::Output: HasQuadrant,
    <Robot<Coord> as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|s| s.parse::<Robot<Coord>>().expect("Parse failed."))
        .map(|r| r.run(100))
        .map(|c| c.quadrant())
        .collect::<bag::Bag<Quadrant>>()
        .into_iter()
        .filter(|(q, _)| !q.0.is_eq() && !q.1.is_eq())
        .map(|(_, n)| n)
        .product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::{part1 /*part2*/, Coord, Mod11, Mod7};

    const EXAMPLE: &'static str = include_str!("../data/example/input");

    #[test]
    fn test_part1() {
        assert_eq!(part1::<Coord<Mod7, Mod11>>(EXAMPLE), 12);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(EXAMPLE), xx);
    // }
}

fn main() {
    let input = include_str!("../data/actual/input");
    println!("Part 1: {}", part1::<Coord<Mod103, Mod101>>(input));
    // println!("Part 2: {}", part2(input));
}
