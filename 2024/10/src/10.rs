aoc::parts!(1, 2);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn up(&self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y.checked_sub(1)?,
        })
    }
    fn dn(&self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y.checked_add(1)?,
        })
    }
    fn lt(&self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(1)?,
            y: self.y,
        })
    }
    fn rt(&self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_add(1)?,
            y: self.y,
        })
    }
    fn neighbors(&self) -> impl Iterator<Item = Self> {
        std::iter::empty()
            .chain(self.up().iter())
            .chain(self.dn().iter())
            .chain(self.lt().iter())
            .chain(self.rt().iter())
            .cloned()
            .collect::<Vec<Point>>()
            .into_iter()
    }
}

struct Map {
    data: std::collections::HashMap<Point, char>,
}

impl Map {
    fn trailheads_iter(&self) -> impl Iterator<Item = Point> + use<'_> {
        self.data
            .iter()
            .filter(|(_, &ch)| ch == '0')
            .map(|(p, _)| p)
            .cloned()
    }
}

impl std::str::FromStr for Map {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = std::collections::HashMap::<Point, char>::new();
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                data.insert(Point { x, y }, ch);
            }
        }
        Ok(Self { data })
    }
}

trait Attribute: std::iter::Sum + Clone + std::fmt::Debug {
    fn unit(p: Point) -> Self;
}

struct Attributes<'a, T: Attribute> {
    map: &'a Map,
    data: std::collections::HashMap<Point, T>,
}

impl<'a, T: Attribute> Attributes<'a, T> {
    fn new(map: &'a Map) -> Self {
        Self {
            map,
            data: std::collections::HashMap::<Point, T>::new(),
        }
    }

    fn calculate_attribute(&mut self, p: Point) -> T {
        let n = self.map.data[&p].to_digit(10).unwrap();
        if n == 9 {
            return T::unit(p);
        }
        p.neighbors()
            .filter_map(|other| {
                if self.map.data.get(&other)?.to_digit(10).unwrap() == n + 1 {
                    Some(self.get_attribute(other))
                } else {
                    None
                }
            })
            .sum()
    }

    fn get_attribute(&mut self, p: Point) -> T {
        if let Some(attribute) = self.data.get(&p) {
            return attribute.clone();
        }
        let score = self.calculate_attribute(p);
        self.data.insert(p, score.clone());
        score
    }
}

trait Part {
    type Output: ToString + std::iter::Sum;
    type Attribute: Attribute + Into<Self::Output>;

    fn run(input: aoc::Input) -> Self::Output {
        let map = input.raw().parse::<Map>().expect("Parse failed.");
        let mut attributes = Attributes::<Self::Attribute>::new(&map);
        map.trailheads_iter()
            .map(|p| attributes.get_attribute(p).into())
            .sum()
    }
}

mod part_1 {
    use super::*;

    pub enum Impl {}

    #[derive(Clone, Debug)]
    pub struct Score {
        targets: std::collections::HashSet<Point>,
    }

    impl std::iter::Sum for Score {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            Score {
                targets: iter.flat_map(|this| this.targets).collect(),
            }
        }
    }

    impl Attribute for Score {
        fn unit(p: Point) -> Self {
            Score {
                targets: std::iter::once(p).collect(),
            }
        }
    }

    impl Into<usize> for Score {
        fn into(self) -> usize {
            self.targets.len()
        }
    }

    impl super::Part for Impl {
        type Output = usize;
        type Attribute = Score;
    }
}

mod part_2 {
    use super::*;

    pub enum Impl {}

    #[derive(Clone, Debug)]
    pub struct Rating(usize);

    impl Attribute for Rating {
        fn unit(_p: Point) -> Self {
            Self(1)
        }
    }

    impl std::iter::Sum for Rating {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            Rating(iter.map(|Rating(x)| x).sum())
        }
    }

    impl Into<usize> for Rating {
        fn into(self) -> usize {
            self.0
        }
    }

    impl super::Part for Impl {
        type Output = usize;
        type Attribute = Rating;
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    part_1::Impl::run(input)
}

fn part_2(input: aoc::Input) -> impl ToString {
    part_2::Impl::run(input)
}
