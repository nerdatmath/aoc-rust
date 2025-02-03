aoc::parts!(1, 2);

use std::iter::{repeat, zip};

use array2d::{self, Array2D};

fn parse(input: aoc::Input) -> Array2D<char> {
    let rows: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    Array2D::from_rows(&rows).unwrap()
}

fn is_xmas(iter: &mut dyn Iterator<Item = (usize, usize)>, array: &Array2D<char>) -> bool {
    iter.take(4)
        .map(|(row, column)| array.get(row, column).unwrap().clone())
        .collect::<String>()
        == "XMAS"
}

fn part_1(input: aoc::Input) -> impl ToString {
    let array = parse(input);
    let mut count = 0usize;
    for row in 0..array.num_rows() {
        for column in 0..array.num_columns() {
            let iters: [&mut dyn Iterator<Item = (usize, usize)>; 8] = [
                &mut zip(repeat(row), column..array.num_columns()),
                &mut zip((0..=row).rev(), column..array.num_columns()),
                &mut zip((0..=row).rev(), repeat(column)),
                &mut zip((0..=row).rev(), (0..=column).rev()),
                &mut zip(repeat(row), (0..=column).rev()),
                &mut zip(row..array.num_rows(), (0..=column).rev()),
                &mut zip(row..array.num_rows(), repeat(column)),
                &mut zip(row..array.num_rows(), column..array.num_columns()),
            ];
            for iter in iters {
                if is_xmas(iter, &array) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn is_x_mas(row: usize, column: usize, array: &Array2D<char>) -> bool {
    row != 0
        && column != 0
        && match array.get(row, column) {
            Some('A') => true,
            _ => false,
        }
        && match (
            array.get(row - 1, column - 1),
            array.get(row + 1, column + 1),
        ) {
            (Some('M'), Some('S')) => true,
            (Some('S'), Some('M')) => true,
            _ => return false,
        }
        && match (
            array.get(row + 1, column - 1),
            array.get(row - 1, column + 1),
        ) {
            (Some('M'), Some('S')) => true,
            (Some('S'), Some('M')) => true,
            _ => return false,
        }
}

fn part_2(input: aoc::Input) -> impl ToString {
    let array = parse(input);
    let mut count = 0usize;
    for row in 0..array.num_rows() {
        for column in 0..array.num_columns() {
            if is_x_mas(row, column, &array) {
                count += 1;
            }
        }
    }
    count
}
