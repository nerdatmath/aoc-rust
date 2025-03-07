use std::collections::HashSet;

use direction::{CardinalDirection, CardinalDirectionIter};
use disjoint_hash_set::DisjointHashSet;
use grid::Grid;
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct PlantType(char);

impl From<char> for PlantType {
    fn from(value: char) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
struct Region {
    _plant_type: PlantType,
    area: usize,
    perimeter: usize,
    sides: usize,
}

fn edges_iter(
    rows: (usize, usize, usize),
    cols: (usize, usize, usize),
) -> impl Iterator<Item = ((usize, usize), (usize, usize))> {
    (rows.0..rows.2)
        .zip(rows.1..rows.2)
        .cartesian_product((cols.0..cols.2).zip(cols.1..cols.2))
        .map(|((a, b), (c, d))| ((a, c), (b, d)))
}

#[derive(Debug)]
struct Map(Grid<PlantType>);

impl Map {
    fn plant_type(&self, plot: (usize, usize)) -> PlantType {
        self.0[plot]
    }

    fn same_plant_type(&self, a: (usize, usize), b: (usize, usize)) -> bool {
        self.plant_type(a) == self.plant_type(b)
    }

    fn connected_neighbor(
        &self,
        plot: (usize, usize),
        dir: CardinalDirection,
    ) -> Option<(usize, usize)> {
        let coord = dir.coord();
        let x = plot.1.checked_add_signed(coord.x.try_into().unwrap())?;
        let y = plot.0.checked_add_signed(coord.y.try_into().unwrap())?;
        _ = self.0.get(y, x)?;
        self.same_plant_type(plot, (y, x)).then_some((y, x))
    }

    fn vedges_iter(&self) -> impl Iterator<Item = ((usize, usize), (usize, usize))> + use<> {
        edges_iter((0, 1, self.0.rows()), (0, 0, self.0.cols()))
    }

    fn hedges_iter(&self) -> impl Iterator<Item = ((usize, usize), (usize, usize))> + use<> {
        edges_iter((0, 0, self.0.rows()), (0, 1, self.0.cols()))
    }

    fn connected_plots(&self) -> impl Iterator<Item = ((usize, usize), (usize, usize))> {
        self.vedges_iter()
            .chain(self.hedges_iter())
            .filter(|&(a, b)| self.same_plant_type(a, b))
    }

    fn make_region(&self, plots: HashSet<(usize, usize)>) -> Region {
        let plant_type = self.plant_type(*plots.iter().nth(0).unwrap());
        let disjoint_walls: DisjointHashSet<((usize, usize), CardinalDirection)> = plots
            .iter()
            .flat_map(|&plot| {
                CardinalDirectionIter::new()
                    .filter(move |&dir| self.connected_neighbor(plot, dir).is_none())
                    .map(
                        move |dir| match self.connected_neighbor(plot, dir.right90()) {
                            Some(other) if self.connected_neighbor(other, dir).is_none() => {
                                ((plot, dir), (other, dir))
                            }
                            _ => ((plot, dir), (plot, dir)),
                        },
                    )
            })
            .collect();
        let mut perimeter = 0;
        let mut sides = 0;
        for segment in disjoint_walls.sets() {
            sides += 1;
            perimeter += segment.len();
        }
        Region {
            _plant_type: plant_type,
            area: plots.len(),
            perimeter,
            sides,
        }
    }

    fn regions(&self) -> impl Iterator<Item = Region> {
        let mut disjoint_plots: DisjointHashSet<(usize, usize)> = self.connected_plots().collect();
        for plot in (0..self.0.rows()).cartesian_product(0..self.0.cols()) {
            disjoint_plots.insert(plot);
        }
        disjoint_plots.sets().map(|plots| self.make_region(plots))
    }
}

impl std::str::FromStr for Map {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .map(|line| {
                line.chars()
                    .map(PlantType::try_from)
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?
            .into()))
    }
}

fn run(input: &str, cost: impl Fn(Region) -> usize) -> usize {
    input
        .parse::<Map>()
        .expect("Parse failed.")
        .regions()
        .map(cost)
        .sum()
}

fn part1(input: &str) -> usize {
    run(input, |r| r.area * r.perimeter)
}

fn part2(input: &str) -> usize {
    run(input, |r| r.area * r.sides)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLES: &'static [&'static str] = &[
        include_str!("../data/example1/input"),
        include_str!("../data/example2/input"),
        include_str!("../data/example3/input"),
        include_str!("../data/example4/input"),
        include_str!("../data/example5/input"),
    ];

    #[test]
    fn test_part1() {
        let results = [140, 772, 1930];
        for (&input, want) in EXAMPLES.iter().zip(results) {
            assert_eq!(part1(input), want);
        }
    }

    #[test]
    fn test_part2() {
        let results = [80, 436, 1206, 236, 368];
        for (&input, want) in EXAMPLES.iter().zip(results) {
            assert_eq!(part2(input), want);
        }
    }
}

fn main() {
    let input = include_str!("../data/actual/input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
