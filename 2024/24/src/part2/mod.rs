use crate::puzzle::{Puzzle, Wire};
use gates::FullAdderNotFoundError;

mod gates;

pub fn run(input: &str) -> impl std::fmt::Display {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let gates: gates::Gates = puzzle.gates.iter().collect();

    let bits = u8::try_from(puzzle.fixed_wires.len() / 2).unwrap(); // input bits
    assert_eq!(puzzle.gates.len(), (bits * 5 - 3).into());

    let mut swaps = Vec::<Wire>::new();
    let half_adder_0 = gates
        .find_half_adder_with_inputs((Wire::x(0), Wire::y(0)).into())
        .unwrap();
    assert_eq!(half_adder_0.output.sum, Wire::z(0));
    let mut carry = half_adder_0.output.carry;
    for bit in 1..bits {
        let full_adder = gates.find_full_adder_for_bit_and_input_carry(bit, carry);
        match full_adder {
            Ok(full_adder) => {
                carry = full_adder.output_carry;
            }
            Err(FullAdderNotFoundError::OkWithSwap(full_adder, a, b)) => {
                swaps.push(a);
                swaps.push(b);
                carry = full_adder.output_carry;
            }
            Err(err) => {
                println!("Finding full adder for bit {bit}: {err}");
                return "ERROR".into();
            }
        }
    }
    if carry != Wire::z(bits) {
        println!("Final carry was {carry:?}, want {}", Wire::z(bits));
    }
    swaps.sort();
    swaps
        .into_iter()
        .map(|wire| wire.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
