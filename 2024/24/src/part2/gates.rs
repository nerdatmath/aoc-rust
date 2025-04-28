use crate::puzzle::{Gate, Op, Wire};
use parse_display::Display;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Display)]
#[display("sum: {sum}, carry: {carry}")]
pub struct AdderOutput {
    pub sum: Wire,
    pub carry: Wire,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Display)]
#[display("HalfAdder(({inputs}) -> {output})")]
pub struct HalfAdder {
    #[display("{0}, {1}")]
    pub inputs: (Wire, Wire),
    pub output: AdderOutput,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Display)]
#[display("FullAdder({half_adders}, input_carry = {input_carry}, output_carry = {output_carry})")]
pub struct FullAdder {
    #[display("{0}, {1}")]
    pub half_adders: (HalfAdder, HalfAdder),
    pub input_carry: Wire,
    pub output_carry: Wire,
}

#[derive(Debug)]
pub struct Gates {
    by_inputs_and_op: HashMap<((Wire, Wire), Op), Wire>,
}

impl Gates {
    fn new() -> Self {
        Self {
            by_inputs_and_op: HashMap::new(),
        }
    }
}

impl<'a> FromIterator<&'a Gate> for Gates {
    fn from_iter<T: IntoIterator<Item = &'a Gate>>(iter: T) -> Self {
        let mut gates = Gates::new();
        for gate in iter.into_iter().copied() {
            let inputs = (gate.a.min(gate.b), gate.b.max(gate.a));
            gates
                .by_inputs_and_op
                .insert((inputs, gate.op), gate.output);
        }
        gates
    }
}

#[derive(Debug, Display)]
pub enum GateNotFoundError {
    #[display("missing gate [{0.0} {1} {0.1} -> ?]")]
    MissingInputsAndOp((Wire, Wire), Op),
}

#[derive(Debug, Display)]
pub enum FullAdderNotFoundError {
    #[display("{0}")]
    GateNotFound(GateNotFoundError),
    #[display("ok with swap of wires ({0}, {1}")]
    OkWithSwap(FullAdder, Wire, Wire),
}

impl From<GateNotFoundError> for FullAdderNotFoundError {
    fn from(value: GateNotFoundError) -> Self {
        FullAdderNotFoundError::GateNotFound(value)
    }
}

impl Gates {
    // Find a gate with the given inputs and op and return the output.
    fn find_gate_output_with_inputs_and_op(
        &self,
        inputs: (Wire, Wire),
        op: Op,
    ) -> Result<Wire, GateNotFoundError> {
        let inputs = (inputs.0.min(inputs.1), inputs.0.max(inputs.1));
        Ok(*self
            .by_inputs_and_op
            .get(&(inputs, op))
            .ok_or(GateNotFoundError::MissingInputsAndOp(inputs, op))?)
    }

    // Find a half adder with the given inputs.
    pub fn find_half_adder_with_inputs(
        &self,
        inputs: (Wire, Wire),
    ) -> Result<HalfAdder, GateNotFoundError> {
        let sum = self.find_gate_output_with_inputs_and_op(inputs, Op::Xor)?;
        let carry = self.find_gate_output_with_inputs_and_op(inputs, Op::And)?;
        Ok(HalfAdder {
            inputs,
            output: AdderOutput { sum, carry },
        })
    }

    fn finish_full_adder(
        &self,
        (first, second): (HalfAdder, HalfAdder),
        input_carry: Wire,
    ) -> Result<FullAdder, FullAdderNotFoundError> {
        let a = first.output.carry;
        let b = second.output.carry;
        let op = Op::Or;
        let output = self.find_gate_output_with_inputs_and_op((a, b), op)?;
        Ok(FullAdder {
            half_adders: (first, second),
            input_carry,
            output_carry: output,
        })
    }

    // Find a full adder with the given inputs.
    fn find_full_adder_for_inputs_and_sum(
        &self,
        a: Wire,
        b: Wire,
        input_carry: Wire,
        sum: Wire,
    ) -> Result<FullAdder, FullAdderNotFoundError> {
        let mut first = self.find_half_adder_with_inputs((a, b).into())?;
        match self.find_half_adder_with_inputs((first.output.sum, input_carry).into()) {
            Ok(mut second) => {
                let mut other: Option<Wire> = None;
                if first.output.carry == sum {
                    first.output.carry = second.output.sum;
                    second.output.sum = sum;
                    other = Some(first.output.carry);
                }
                if second.output.carry == sum {
                    second.output.carry = second.output.sum;
                    second.output.sum = sum;
                    other = Some(second.output.carry);
                }
                let mut full_adder = self.finish_full_adder((first, second), input_carry)?;
                if full_adder.output_carry == sum {
                    full_adder.output_carry = full_adder.half_adders.1.output.sum;
                    full_adder.half_adders.1.output.sum = sum;
                    other = Some(full_adder.output_carry);
                }
                match other {
                    Some(other) => Err(FullAdderNotFoundError::OkWithSwap(full_adder, sum, other)),
                    None => Ok(full_adder),
                }
            }
            Err(err) => {
                if let Ok(second) =
                    self.find_half_adder_with_inputs((first.output.carry, input_carry).into())
                {
                    let (carry, sum) = (first.output.sum, first.output.carry);
                    first.output.sum = sum;
                    first.output.carry = carry;
                    let full_adder = self.finish_full_adder((first, second), input_carry)?;
                    return Err(FullAdderNotFoundError::OkWithSwap(full_adder, sum, carry));
                }
                Err(err.into())
            }
        }
    }

    // Find a full adder for the given bit and input carry.
    pub fn find_full_adder_for_bit_and_input_carry(
        &self,
        bit: u8,
        input_carry: Wire,
    ) -> Result<FullAdder, FullAdderNotFoundError> {
        self.find_full_adder_for_inputs_and_sum(
            Wire::x(bit),
            Wire::y(bit),
            input_carry,
            Wire::z(bit),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::puzzle::Wire;
    use crate::{gate, wire};

    fn swap(target: &mut Wire, (a, b): (Wire, Wire)) {
        if target == &a {
            *target = b
        } else if target == &b {
            *target = a
        }
    }

    fn make_full_adder(swapping: Option<(Wire, Wire)>) -> Vec<Gate> {
        let mut gates = vec![
            gate!(ina XOR inb -> sma),
            gate!(ina AND inb -> cca),
            gate!(sma XOR cci -> smo),
            gate!(sma AND cci -> ccb),
            gate!(cca OR ccb -> cco),
        ];
        if let Some((a, b)) = swapping {
            for gate in &mut gates {
                swap(&mut gate.output, (a, b));
            }
        }
        gates
    }

    #[test]
    fn test_half_adder() {
        let gates: Gates = [
            // formatting
            gate!(ina XOR inb -> smo),
            gate!(ina AND inb -> cco),
        ]
        .iter()
        .collect();
        let half_adder_0 = gates
            .find_half_adder_with_inputs((wire!(ina), wire!(inb)).into())
            .unwrap();
        assert_eq!(half_adder_0.output.sum, wire!(smo));
        assert_eq!(half_adder_0.output.carry, wire!(cco));
    }

    #[test]
    fn test_full_adder() {
        let gates: Gates = make_full_adder(None).iter().collect();
        let full_adder = gates
            .find_full_adder_for_inputs_and_sum(wire!(ina), wire!(inb), wire!(cci), wire!(smo))
            .unwrap();
        assert_eq!(full_adder.output_carry, wire!(cco));
    }

    #[test]
    fn test_full_adder_swapping() {
        for swapping in [
            (wire!(smo), wire!(cca)),
            (wire!(smo), wire!(ccb)),
            (wire!(smo), wire!(cco)),
            (wire!(sma), wire!(cca)),
        ] {
            match make_full_adder(Some(swapping))
                .iter()
                .collect::<Gates>()
                .find_full_adder_for_inputs_and_sum(wire!(ina), wire!(inb), wire!(cci), wire!(smo))
                .unwrap_err()
            {
                FullAdderNotFoundError::OkWithSwap(_, a, b) => {
                    assert_eq!((a, b), swapping);
                }
                err => panic!("unexpected error {err}"),
            }
        }
    }
}
