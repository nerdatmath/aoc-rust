use crate::instructions::Instructions;
use crate::nodes::{Graph, Nodes};
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseError;

impl From<<Nodes as FromStr>::Err> for ParseError {
    fn from(_value: <Nodes as FromStr>::Err) -> Self {
        Self {}
    }
}

impl From<<Instructions as FromStr>::Err> for ParseError {
    fn from(_value: <Instructions as FromStr>::Err) -> Self {
        Self {}
    }
}

pub struct Puzzle {
    pub instructions: Instructions,
    pub nodes: Nodes,
}

impl Puzzle {
    fn new(instructions: Instructions, nodes: Nodes) -> Self {
        Self {
            instructions,
            nodes,
        }
    }

    pub fn add_to_graph<'arena>(&self, graph: &'arena Graph<'arena>) {
        graph.add_nodes(&self.nodes);
    }
}

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instructions, nodes) = s.split_once("\n\n").ok_or(ParseError)?;
        let instructions: Instructions = instructions.parse()?;
        let nodes: Nodes = nodes.parse()?;
        Ok(Self::new(instructions, nodes))
    }
}
