use crate::puzzle::{FixedWire, Gate, Puzzle, Wire, WireState};
use ascii::{AsAsciiStr, AsciiChar};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Fixed(WireState),
    Undetermined(Gate),
}

struct Wires(HashMap<Wire, State>);

impl Wires {
    fn new(puzzle: &Puzzle) -> Self {
        let mut map: HashMap<Wire, State> = HashMap::new();
        for &FixedWire { wire, state } in &puzzle.fixed_wires {
            map.insert(wire, State::Fixed(state.into()));
        }
        for &gate in &puzzle.gates {
            map.insert(gate.output, State::Undetermined(gate));
        }
        Self(map)
    }
    fn get(&mut self, &wire: &Wire) -> Option<WireState> {
        let &state = self.0.get(&wire)?;
        Some(match state {
            State::Fixed(value) => value,
            State::Undetermined(gate) => {
                let value = gate.op.apply(self.get(&gate.a)?, self.get(&gate.b)?);
                self.0.insert(wire, State::Fixed(value));
                value
            }
        })
    }
}

pub fn run(input: &str) -> u128 {
    let puzzle: Puzzle = input.parse().expect("parse failed");
    let mut wires = Wires::new(&puzzle);
    let mut result: u128 = 0;
    for gate in &puzzle.gates {
        let wire = gate.output;
        if wire.name[0] == AsciiChar::z
            && wire.name[1].is_ascii_digit()
            && wire.name[2].is_ascii_digit()
        {
            let wire_state = wires.get(&wire).unwrap();
            if wire_state == WireState::On {
                let bit = wire.name[1..=2]
                    .as_ascii_str()
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap();
                result |= 1 << bit;
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::run;

    const EXAMPLE1: &'static str = include_str!("../../data/example1");
    const EXAMPLE2: &'static str = include_str!("../../data/example2");

    #[test]
    fn test1() {
        assert_eq!(run(EXAMPLE1), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(run(EXAMPLE2), 2024);
    }
}
