use ascii::AsAsciiStr as _;
use parse_display::Display;
use parse_display::FromStr;
use parse_display_with::formats::delimiter;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseError;

#[derive(Debug, FromStr)]
#[display("{fixed_wires}\n\n{gates}")]
pub struct Puzzle {
    #[display(with=delimiter("\n"))]
    pub fixed_wires: Vec<FixedWire>,
    #[display(with=delimiter("\n"))]
    pub gates: Vec<Gate>,
}

#[derive(Clone, Copy, Debug, Display, FromStr)]
#[display("{wire}: {state}")]
pub struct FixedWire {
    pub wire: Wire,
    pub state: WireState,
}

#[derive(Clone, Copy, Debug, Display, FromStr, Hash, PartialEq, Eq)]
#[display("{a} {op} {b} -> {output}")]
#[from_str(new = Gate::new(a, op, b, output))]
pub struct Gate {
    pub a: Wire,
    pub op: Op,
    pub b: Wire,
    pub output: Wire,
}

impl Gate {
    fn new(a: Wire, op: Op, b: Wire, output: Wire) -> Self {
        let (a, b) = (a.min(b), a.max(b));
        Self { a, op, b, output }
    }
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Wire {
    pub name: [ascii::AsciiChar; 3],
}

#[allow(unused)]
impl Wire {
    pub fn x(n: u8) -> Self {
        let s = format!("x{n:02}");
        s.parse().expect("invalid number")
    }
    pub fn y(n: u8) -> Self {
        let s = format!("y{n:02}");
        s.parse().expect("invalid number")
    }
    pub fn z(n: u8) -> Self {
        let s = format!("z{n:02}");
        s.parse().expect("invalid number")
    }
    pub fn is_fixed(&self) -> bool {
        use ascii::AsciiChar::*;
        matches!(self.name[0], x | y)
    }
}

impl Display for Wire {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.as_ascii_str().unwrap().as_str())
    }
}

impl std::fmt::Debug for Wire {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wire!({self})")
    }
}

impl FromStr for Wire {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        {
            let s = s.as_ascii_str().map_err(|_| ParseError)?;
            if s.len() != 3 {
                return Err(ParseError);
            }
            Ok(Wire {
                name: [s[0], s[1], s[2]],
            })
        }
    }
}

#[macro_export]
macro_rules! wire {
    ($x:ident) => {
        <Wire as std::str::FromStr>::from_str(stringify!($x)).unwrap()
    };
}

#[macro_export]
macro_rules! gate {
    ($a:ident $op:ident $b:ident -> $o:ident) => {
        Gate {
            a: <Wire as std::str::FromStr>::from_str(stringify!($a)).unwrap(),
            op: <Op as std::str::FromStr>::from_str(stringify!($op)).unwrap(),
            b: <Wire as std::str::FromStr>::from_str(stringify!($b)).unwrap(),
            output: <Wire as std::str::FromStr>::from_str(stringify!($o)).unwrap(),
        }
    };
}

#[macro_export]
macro_rules! parse {
    ($t:ty, $x:literal) => {
        <$t>::from_str($x).unwrap()
    };
}

#[test]
fn test_wire_macro() {
    use ascii::AsciiChar::{a, b, c};
    let abc = wire!(abc);
    assert_eq!(abc, Wire { name: [a, b, c] });
}

#[derive(Clone, Copy, Debug, Display, FromStr, Hash, PartialEq, Eq)]
#[display(style = "UPPERCASE")]
pub enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    pub fn apply(&self, a: WireState, b: WireState) -> WireState {
        use Op::*;
        use WireState::*;
        match (a, self, b) {
            (On, And, On) => On,
            (_, And, _) => Off,
            (Off, Or, Off) => Off,
            (_, Or, _) => On,
            (Off, Xor, Off) => Off,
            (On, Xor, On) => Off,
            (_, Xor, _) => On,
        }
    }
}

#[derive(Clone, Copy, Debug, Display, FromStr, Hash, PartialEq, Eq)]
pub enum WireState {
    #[display("0")]
    Off,
    #[display("1")]
    On,
}
