use crate::direction::Direction;
use crate::instructions::Instructions;
use crate::nodes::CyclicNodeRef;
use crate::nodes::Name;
use std::collections::HashSet;

#[derive(Debug)]
pub struct State<'a> {
    instructions: &'a [Direction],
    pos: usize, // position in the instructions list
    cnode: CyclicNodeRef<'a>,
}

impl<'a> State<'a> {
    pub fn new(instructions: &'a Instructions, cnode: CyclicNodeRef<'a>) -> Self {
        Self {
            instructions: &instructions.0,
            pos: 0,
            cnode,
        }
    }

    fn step(&mut self) {
        let dir = self.instructions[self.pos];
        self.pos = (self.pos + 1) % self.instructions.len();
        use Direction::*;
        self.cnode = match dir {
            Left => self.cnode.l.get().unwrap(),
            Right => self.cnode.r.get().unwrap(),
        }
    }

    // Step to the next target node and return the # of steps taken.
    pub fn target_distance(&mut self, is_target: impl Fn(CyclicNodeRef) -> bool) -> usize {
        let mut count = 0usize;
        loop {
            self.step();
            count += 1;
            if is_target(self.cnode) {
                break count;
            }
        }
    }

    fn location(&self) -> (usize, Name) {
        (self.pos, self.cnode.name)
    }

    // Find the distances to each target node, ending when we find a loop.
    pub fn find(&mut self, is_target: impl Fn(CyclicNodeRef) -> bool) -> Option<usize> {
        let mut seen: HashSet<(usize, Name)> = HashSet::new();
        let distance = self.target_distance(&is_target);
        loop {
            if seen.contains(&self.location()) {
                break Some(distance);
            }
            seen.insert(self.location());
            if self.target_distance(&is_target) != distance {
                break None;
            }
        }
    }
}
