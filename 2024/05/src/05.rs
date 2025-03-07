use std::{cmp::Ordering, collections::HashSet};

type Page = u32;

#[derive(Hash, PartialEq, Eq)]
struct Rule {
    before: Page,
    after: Page,
}

type Update = Vec<Page>;

fn middle(update: &Update) -> Page {
    update[update.len() / 2]
}

fn parse_rule(input: &str) -> Rule {
    let (before_str, after_str) = input
        .split_once("|")
        .expect("A rule must contain a '|' character.");
    Rule {
        before: before_str
            .parse()
            .expect("A rule must start with a page number."),
        after: after_str
            .parse()
            .expect("A rule must end with a page number."),
    }
}

fn parse_update(input: &str) -> Update {
    input
        .split(",")
        .map(|s| {
            s.parse()
                .expect("An update must contain page numbers separated by ','.")
        })
        .collect()
}

fn rules_comparer<'a>(rules: &'a HashSet<Rule>) -> impl Fn(&Page, &Page) -> Ordering + 'a{
    |&a, &b| {
        if rules.contains(&Rule {
            before: a,
            after: b,
        }) {
            Ordering::Less
        } else if rules.contains(&Rule {
            before: b,
            after: a,
        }) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

fn parse(input: &str) -> (HashSet<Rule>, Vec<Update>) {
    let (rules_str, updates_str) = input
        .split_once("\n\n")
        .expect("Input must contain rules and updates, separated by an empty line.");
    (
        rules_str.lines().map(parse_rule).collect(),
        updates_str.lines().map(parse_update).collect(),
    )
}

fn part1(input: &str) -> Page {
    let (rules, updates) = parse(input);
    let compare = rules_comparer(&rules);
    updates
        .iter()
        .filter(|pages| pages.is_sorted_by(|a, b| compare(a, b).is_lt()))
        .map(middle)
        .sum::<Page>()
}

fn part2(input: &str) -> Page {
    let (rules, mut updates) = parse(input);
    let compare = rules_comparer(&rules);
    updates
        .iter_mut()
        .filter(|pages| !pages.is_sorted_by(|a, b| compare(a, b).is_lt()))
        .map(|pages| {
            pages.sort_by(&compare);
            middle(&pages)
        })
        .sum::<Page>()
}
#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &'static str = include_str!("../data/example/input");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 123);
    }
}

fn main() {
    let input = include_str!("../data/actual/input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
