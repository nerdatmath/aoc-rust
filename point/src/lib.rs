#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn up(&self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y.checked_sub(1)?,
        })
    }
    pub fn dn(&self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y.checked_add(1)?,
        })
    }
    pub fn lt(&self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(1)?,
            y: self.y,
        })
    }
    pub fn rt(&self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_add(1)?,
            y: self.y,
        })
    }
    pub fn neighbors(&self) -> impl Iterator<Item = Self> {
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
