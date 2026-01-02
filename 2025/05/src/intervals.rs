pub struct Intervals {
    pub starts: Box<[u64]>,
    pub ends: Box<[u64]>,
}

impl Intervals {
    pub fn new(ranges: &[crate::puzzle::Range]) -> Intervals {
        let (mut starts, mut ends): (Vec<u64>, Vec<u64>) =
            ranges.iter().map(|r| (r.start, r.end)).unzip();
        starts.sort();
        ends.sort();
        Intervals {
            starts: starts.into(),
            ends: ends.into(),
        }
    }

    pub fn contains(&self, item: &u64) -> bool {
        let start = self.starts.partition_point(|n| n <= item);
        let end = self.ends.partition_point(|n| n < item);
        return start > end;
    }
}
