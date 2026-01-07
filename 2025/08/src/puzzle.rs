use parse_display::FromStr;
use parse_display_with::formats::delimiter;

#[derive(Debug, FromStr)]
#[display("{positions}")]
pub struct Puzzle {
    #[display(with=delimiter("\n"))]
    pub positions: Box<[Position]>,
}

#[derive(Clone, Copy, Debug, FromStr, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[display("{x},{y},{z}")]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Puzzle {
    pub fn distances_iter(&self) -> impl Iterator<Item = (Position, Position, usize)> {
        let positions = &self.positions;
        (0..positions.len()).flat_map(move |i| {
            (i + 1..positions.len()).map(move |j| {
                let (a, b) = (positions[i], positions[j]);
                let dist_sq =
                    a.x.abs_diff(b.x).pow(2) + a.y.abs_diff(b.y).pow(2) + a.z.abs_diff(b.z).pow(2);
                (a, b, dist_sq)
            })
        })
    }
}
