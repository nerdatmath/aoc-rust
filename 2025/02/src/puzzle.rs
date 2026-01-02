use lazy_regex::regex_captures;

type Range = std::ops::RangeInclusive<u64>;

#[derive(Debug)]
pub struct Puzzle {
    pub ranges: Vec<Range>,
}

#[derive(Debug)]
pub struct ParseError();

impl From<std::num::ParseIntError> for ParseError {
    fn from(_value: std::num::ParseIntError) -> Self {
        Self()
    }
}

fn parse_range(s: &str) -> Result<Range, ParseError> {
    let (_, start, end) = regex_captures!(r#"(\d+)-(\d+)"#, s).ok_or(ParseError())?;
    Ok((start.parse()?)..=(end.parse()?))
}

impl std::str::FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            ranges: s.split(',').map(parse_range).collect::<Result<_, _>>()?,
        })
    }
}
