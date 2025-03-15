use derive_more::Deref;
use num::Integer;
use parse_display::FromStr;
use parse_display_with::formats::delimiter;

#[derive(FromStr)]
#[display("Button A: {button_a}\nButton B: {button_b}\nPrize: {prize}")]
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

#[derive(FromStr)]
#[display("{x}, {y}")]
struct Button {
    #[display("X+{}")]
    x: i64,
    #[display("Y+{}")]
    y: i64,
}

#[derive(FromStr)]
#[display("{x}, {y}")]
struct Prize {
    #[display("X={}")]
    x: i64,
    #[display("Y={}")]
    y: i64,
}

#[derive(FromStr, Deref)]
struct Input(#[display(with=delimiter("\n\n"))] Vec<Machine>);

impl Input {
    fn tokens(&self) -> i64 {
        self.iter()
            .map(|m: &Machine| m.min_tokens().unwrap_or(0))
            .sum()
    }
}

fn part1(input: &str) -> i64 {
    input.parse::<Input>().expect("Parse error.").tokens()
}

fn part2(input: &str) -> i64 {
    let mut machines = input.parse::<Input>().expect("Parse error.");
    for Machine { prize, .. } in machines.0.iter_mut() {
        prize.x += 10000000000000;
        prize.y += 10000000000000;
    }
    machines.tokens()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &'static str = include_str!("../data/example/input");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 875318608908);
    }
}

fn main() {
    let input = include_str!("../data/actual/input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
