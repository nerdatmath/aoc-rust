use crate::directional_keypad::{self, DirectionalKeypad};
use crate::keypad::Keypad;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;
use std::hash::Hash;

// The cost for self to move from one key to another on the target keypad.
pub trait Costs<KP: Keypad> {
    fn cost(&self, keypad: &KP, source: KP::Key, target: KP::Key) -> usize;
}

#[derive(Debug)]
pub struct PrecomputedCosts<KP: Keypad>(pub HashMap<(KP::Key, KP::Key), usize>);

impl<KP: Keypad> PrecomputedCosts<KP> {
    pub fn new(keypad: &KP, costs: &dyn Costs<KP>) -> Self {
        PrecomputedCosts(
            enum_iterator::all::<KP::Key>()
                .flat_map(|source| {
                    enum_iterator::all::<KP::Key>()
                        .map(move |target| ((source, target), costs.cost(keypad, source, target)))
                })
                .collect(),
        )
    }
}

impl<KP: Keypad> Costs<KP> for PrecomputedCosts<KP> {
    fn cost(&self, _keypad: &KP, source: KP::Key, target: KP::Key) -> usize {
        self.0[&(source, target)]
    }
}

pub struct Human;

impl Human {
    pub fn new() -> Self {
        Self
    }
}

impl<KP: Keypad> Costs<KP> for Human {
    fn cost(&self, _keypad: &KP, _from: KP::Key, _to: KP::Key) -> usize {
        0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Robot<C: Costs<DirectionalKeypad>> {
    operator: C,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State<Key> {
    parent_key: directional_keypad::Key,
    key: Key,
}

impl<C: Costs<DirectionalKeypad>> Robot<C> {
    pub fn new(operator: C) -> Self {
        Self { operator }
    }

    // Next states and the costs to get there.
    fn successors<KP: Keypad>(
        &self,
        keypad: &KP,
        State { parent_key, key }: State<KP::Key>,
    ) -> impl IntoIterator<Item = (State<KP::Key>, usize)> {
        let mut v: Vec<(State<KP::Key>, usize)> = Vec::new();
        if let directional_keypad::Key::Move(direction) = parent_key {
            // Pushing the parent key moves us in the given direction.
            // The cost is 1 because all the rest of the parents are pointed at `A`.
            if let Some(key) = keypad.next_key(key, direction) {
                v.push((State { parent_key, key }, 1));
            }
        }
        // Costs to move the parent key somewhere else.
        for new_parent_key in enum_iterator::all::<directional_keypad::Key>() {
            if new_parent_key == parent_key {
                continue;
            }
            v.push((
                State {
                    parent_key: new_parent_key,
                    key,
                },
                self.operator
                    .cost(&DirectionalKeypad, parent_key, new_parent_key),
            ));
        }
        v
    }
}

impl<KP: Keypad, C: Costs<DirectionalKeypad>> Costs<KP> for Robot<C> {
    // The cost for a robot to move from `source` to `target`,
    // with the operator starting at `A` and returning to `A`.
    // The operator of the robot must push an appropriate series of keys
    // on the robot's keypad. This leaves the robot hovering over the key
    // but not pushing it. To then press the button `n` times, add `n` to
    // the result.
    fn cost(&self, keypad: &KP, source: KP::Key, target: KP::Key) -> usize {
        // Find the cost of the shortest path to the state where the
        // robot is hovering over `target`.
        let start: State<<KP as Keypad>::Key> = State {
            parent_key: DirectionalKeypad::ACTIVATE,
            key: source,
        };
        let target: State<<KP as Keypad>::Key> = State {
            parent_key: DirectionalKeypad::ACTIVATE,
            key: target,
        };
        let (_, cost) = dijkstra(
            &start,
            |&state| self.successors(keypad, state),
            |&state| state == target,
        )
        .unwrap();
        cost
    }
}

pub fn robot_stack(n: usize) -> Robot<PrecomputedCosts<DirectionalKeypad>> {
    let human: &dyn Costs<DirectionalKeypad> = &Human::new();
    let mut robot: Robot<PrecomputedCosts<DirectionalKeypad>> =
        Robot::new(PrecomputedCosts::new(&DirectionalKeypad, human));
    for _ in 1..n {
        robot = Robot::new(PrecomputedCosts::new(&DirectionalKeypad, &robot));
    }
    robot
}
