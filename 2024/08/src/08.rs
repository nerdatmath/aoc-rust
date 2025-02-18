aoc::parts!(1, 2);

use std::{
    char,
    collections::{HashMap, HashSet},
};

fn bounds(s: &str) -> Bounds {
    Bounds {
        x: s.lines().map(|l| l.len()).max().expect("Parse failed.") - 1,
        y: s.lines().count() - 1,
    }
}

fn points_by_frequency(s: &str) -> HashMap<Frequency, HashSet<Point>> {
    let mut result = HashMap::<Frequency, HashSet<Point>>::new();
    for (y, line) in s.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                result
                    .entry(Frequency(ch))
                    .or_default()
                    .insert(Point { x, y });
            }
        }
    }
    result
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Frequency(char);

fn next_antinode_usize(a: usize, b: usize) -> Option<usize> {
    if a <= b {
        b.checked_add(b - a)
    } else {
        b.checked_sub(a - b)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn next_antinode(&self, &next: &Self) -> Option<Self> {
        Some(Point {
            x: next_antinode_usize(self.x, next.x)?,
            y: next_antinode_usize(self.y, next.y)?,
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Bounds {
    x: usize,
    y: usize,
}

impl Bounds {
    fn contains(&self, point: &Point) -> bool {
        point.x <= self.x && point.y <= self.y
    }
}

trait Part {
    type Antinodes: IntoIterator<Item = Point>;

    fn antinodes(a: Point, b: Point) -> Self::Antinodes;

    fn run(input: aoc::Input) -> usize {
        let bounds = bounds(input.raw());
        points_by_frequency(input.raw())
            .values()
            .flat_map(|points| {
                points
                    .iter()
                    .flat_map(|a| points.iter().map(|b| (*a, *b)))
                    .filter(|(a, b)| a != b)
                    .flat_map(|(a, b)| {
                        Self::antinodes(a, b)
                            .into_iter()
                            .take_while(|point| bounds.contains(point))
                    })
            })
            .collect::<HashSet<Point>>()
            .len()
    }
}

enum One {}

impl Part for One {
    type Antinodes = Option<Point>;

    fn antinodes(a: Point, b: Point) -> Self::Antinodes {
        Point::next_antinode(&a, &b)
    }
}

struct Part2Antinodes {
    curr: Option<Point>,
    next: Option<Point>,
}

impl Part2Antinodes {
    fn new(a: Point, b: Point) -> Part2Antinodes {
        Part2Antinodes {
            curr: Some(a),
            next: Some(b),
        }
    }
}

impl Iterator for Part2Antinodes {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        *self = Self {
            curr: self.next,
            next: (|| Point::next_antinode(&self.curr?, &self.next?))(),
        };
        self.curr
    }
}

enum Two {}

impl Part for Two {
    type Antinodes = Part2Antinodes;

    fn antinodes(a: Point, b: Point) -> Self::Antinodes {
        Part2Antinodes::new(a, b)
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    One::run(input)
}

fn part_2(input: aoc::Input) -> impl ToString {
    Two::run(input)
}
