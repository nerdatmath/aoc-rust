use lazy_regex::regex_captures;

pub type Range = std::ops::RangeInclusive<u64>;

#[derive(Debug)]
pub struct ParseError();

impl From<std::num::ParseIntError> for ParseError {
    fn from(_value: std::num::ParseIntError) -> Self {
        Self()
    }
}

pub struct RangeFormat;

impl parse_display::FromStrFormat<Range> for RangeFormat {
    type Err = ParseError;

    fn parse(&self, s: &str) -> Result<Range, Self::Err> {
        let (_, start, end) = regex_captures!(r#"(\d+)-(\d+)"#, s).ok_or(ParseError())?;
        Ok((start.parse()?)..=(end.parse()?))
    }

    fn regex(&self) -> Option<String> {
        Some(r#"(\d+)-(\d+)"#.into())
    }
}

impl parse_display::DisplayFormat<Range> for RangeFormat {
    fn write(&self, f: &mut std::fmt::Formatter, value: &Range) -> std::fmt::Result {
        write!(f, "{}-{}", value.start(), value.end())
    }
}
