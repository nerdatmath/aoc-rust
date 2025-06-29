use parse_display::{Display, FromStr};
use parse_display_with::formats::delimiter;

#[derive(Clone, Debug, FromStr)]
#[display("seeds: {seeds}\n\n{maps}")]
pub struct Puzzle {
    #[display(with=delimiter(" "))]
    pub seeds: Vec<u64>,
    #[display(with=delimiter("\n\n"))]
    pub maps: Vec<Map>,
}

impl Puzzle {
    pub fn lookup_min(&self, intervals: impl IntoIterator<Item = [u64; 2]>) -> u64 {
        intervals
            .into_iter()
            .map(|[source, len]| Map::lookup_min(&self.maps, source, len))
            .min()
            .unwrap()
    }
}

#[derive(Clone, Debug, Display, FromStr)]
#[display("{label} map:\n{entries}")]
#[from_str(new=Map::new(label, entries))]
pub struct Map {
    #[allow(unused)]
    pub label: Label,
    #[display(with=delimiter("\n"))]
    pub entries: Vec<MapEntry>,
}

impl Map {
    fn new(label: Label, mut entries: Vec<MapEntry>) -> Self {
        entries.sort_by_key(|entry| entry.source);
        Self { label, entries }
    }

    fn lookup_interval(&self, source: u64, len: u64) -> (u64, u64) {
        let idx = self
            .entries
            .partition_point(|entry| entry.source + entry.len <= source);
        self.entries.get(idx).map_or_else(
            || (source, len),
            |entry| {
                if entry.source <= source {
                    let offset = source - entry.source;
                    (entry.dest + offset, len.min(entry.len - offset))
                } else {
                    (source, len.min(entry.source - source))
                }
            },
        )
    }

    fn lookup_min(maps: &[Self], mut source: u64, mut len: u64) -> u64 {
        match maps {
            [] => source,
            [map, rest @ ..] => std::iter::from_fn(move || {
                if len > 0 {
                    let interval = map.lookup_interval(source, len);
                    source += interval.1;
                    len -= interval.1;
                    Some(Self::lookup_min(rest, interval.0, interval.1))
                } else {
                    None
                }
            })
            .min()
            .unwrap(),
        }
    }
}

#[derive(Clone, Debug, Display, FromStr, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display("{source}-to-{dest}")]
pub struct Label {
    pub source: String,
    pub dest: String,
}

#[derive(Clone, Copy, Debug, Display, FromStr)]
#[display("{dest} {source} {len}")]
pub struct MapEntry {
    pub dest: u64,
    pub source: u64,
    pub len: u64,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::EXAMPLE1;

    #[test]
    fn test1() -> Result<(), parse_display::ParseError> {
        EXAMPLE1.parse::<Puzzle>()?;
        Ok(())
    }
}
