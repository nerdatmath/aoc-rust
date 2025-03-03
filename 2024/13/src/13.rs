aoc::parts!(1, 2);

use std::str::FromStr;

use num::Integer;

struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

fn solve(da: i64, db: i64, target: i64) -> Option<(i64, i64)> {
    let num::integer::ExtendedGcd { gcd, x: a, y: b } = i64::extended_gcd(&da, &db);
    let n = num::Integer::div_floor(&b, &da);
    let (scale, rem) = target.div_mod_floor(&gcd);
    (a + n * db > 0 && rem == 0).then_some(((a + n * db) * scale, (b - n * da) * scale))
}

impl Machine {
    fn min_tokens(&self) -> Option<i64> {
        let &Machine {
            button_a: Button { x: ax, y: ay, .. },
            button_b: Button { x: bx, y: by, .. },
            prize: Prize { x: px, y: py, .. },
        } = self;
        let (a, b) = match (ax * by - ay * bx, px * by - py * bx, ax * py - ay * px) {
            (0, 0, _) => None,
            (0, _, _) => {
                if ax >= 3 * bx {
                    solve(ax, bx, px)
                } else {
                    solve(bx, ax, px).map(|(b, a)| (a, b))
                }
            }
            (det, adet, bdet) if adet.is_multiple_of(&det) && bdet.is_multiple_of(&det) => {
                let (a, b) = (adet / det, bdet / det);
                (a >= 0 && b >= 0).then_some((a, b))
            }
            _ => None,
        }?;
        assert!(px == a * ax + b * bx);
        assert!(py == a * ay + b * by);
        Some(a * 3 + b)
    }
}

#[derive(Debug)]
enum ParseMachineError {
    Incomplete,
    ExtraLines,
    Button(ParseButtonError),
    Prize(ParsePrizeError),
}

impl FromStr for Machine {
    type Err = ParseMachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("\n");
        let button_a: Button = lines
            .next()
            .ok_or(ParseMachineError::Incomplete)?
            .parse()
            .map_err(ParseMachineError::Button)?;
        let button_b: Button = lines
            .next()
            .ok_or(ParseMachineError::Incomplete)?
            .parse()
            .map_err(ParseMachineError::Button)?;
        let prize: Prize = lines
            .next()
            .ok_or(ParseMachineError::Incomplete)?
            .parse()
            .map_err(ParseMachineError::Prize)?;
        if lines.next().is_some() {
            return Err(ParseMachineError::ExtraLines);
        };
        Ok(Machine {
            button_a,
            button_b,
            prize,
        })
    }
}

struct Button {
    _name: String,
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct ParseButtonError;

impl FromStr for Button {
    type Err = ParseButtonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, s) = s
            .strip_prefix("Button ")
            .and_then(|s| s.split_once(':'))
            .ok_or(ParseButtonError)?;
        let name = name.to_string();
        let (x, y) = s.split_once(',').ok_or(ParseButtonError)?;
        let x = x
            .strip_prefix(" X+")
            .ok_or(ParseButtonError)?
            .parse()
            .map_err(|_| ParseButtonError)?;
        let y = y
            .strip_prefix(" Y+")
            .ok_or(ParseButtonError)?
            .parse()
            .map_err(|_| ParseButtonError)?;
        Ok(Button { _name: name, x, y })
    }
}

struct Prize {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct ParsePrizeError;

impl FromStr for Prize {
    type Err = ParsePrizeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix("Prize:")
            .and_then(|s| s.split_once(','))
            .ok_or(ParsePrizeError)?;
        let x = x
            .strip_prefix(" X=")
            .ok_or(ParsePrizeError)?
            .parse()
            .map_err(|_| ParsePrizeError)?;
        let y = y
            .strip_prefix(" Y=")
            .ok_or(ParsePrizeError)?
            .parse()
            .map_err(|_| ParsePrizeError)?;
        Ok(Prize { x, y })
    }
}

struct Input(Vec<Machine>);

impl FromStr for Input {
    type Err = ParseMachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input(
            s.split("\n\n")
                .map(|s| s.parse::<Machine>())
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Input {
    fn tokens(&self) -> i64 {
        self.0
            .iter()
            .map(|m: &Machine| m.min_tokens().unwrap_or(0))
            .sum()
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    input.raw().parse::<Input>().expect("Parse error.").tokens()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut machines = input.raw().parse::<Input>().expect("Parse error.");
    for Machine { prize, .. } in machines.0.iter_mut() {
        prize.x += 10000000000000;
        prize.y += 10000000000000;
    }
    machines.tokens()
}
