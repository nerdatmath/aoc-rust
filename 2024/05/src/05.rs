use std::{cmp::Ordering, collections::HashSet};

aoc::parts!(1, 2);

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

fn parse(input: aoc::Input) -> (HashSet<Rule>, Vec<Update>) {
    let (rules_str, updates_str) = input
        .raw()
        .split_once("\n\n")
        .expect("Input must contain rules and updates, separated by an empty line.");
    (
        rules_str.lines().map(parse_rule).collect(),
        updates_str.lines().map(parse_update).collect(),
    )
}

fn part_1(input: aoc::Input) -> impl ToString {
    let (rules, updates) = parse(input);
    let compare = rules_comparer(&rules);
    updates
        .iter()
        .filter(|pages| pages.is_sorted_by(|a, b| compare(a, b).is_lt()))
        .map(middle)
        .sum::<Page>()
}

fn part_2(input: aoc::Input) -> impl ToString {
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
