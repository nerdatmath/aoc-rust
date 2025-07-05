use elsa::FrozenMap;
use lazy_regex::regex_if;
use std::cell::OnceCell;
use std::collections::HashMap;
use std::default::Default;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name([u8; 3]);

impl Name {
    pub fn is_source(&self) -> bool {
        self.0[2] == b'A'
    }

    pub fn is_target(&self) -> bool {
        self.0[2] == b'Z'
    }
}

impl FromStr for Name {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(ParseError);
        }
        Ok(Name(s.as_bytes().try_into().map_err(|_| ParseError)?))
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(&self.0).unwrap())
    }
}

#[derive(Debug)]
struct Node {
    name: Name,
    l: Name,
    r: Name,
}

impl FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        regex_if!(
            r#"(?<name>\w{3}) = \((?<l>\w{3}), (?<r>\w{3})\)"#,
            s,
            Self {
                name: name.parse()?,
                l: l.parse()?,
                r: r.parse()?,
            }
        )
        .ok_or(ParseError)
    }
}

#[derive(Debug)]
pub struct Nodes(pub HashMap<Name, (Name, Name)>);

impl FromStr for Nodes {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Nodes(
            s.lines()
                .map(|s| {
                    let node: Node = s.parse()?;
                    Ok((node.name, (node.l, node.r)))
                })
                .collect::<Result<_, Self::Err>>()?,
        ))
    }
}

#[derive(Debug)]
pub struct CyclicNode<'arena> {
    pub name: Name,
    pub l: OnceCell<CyclicNodeRef<'arena>>,
    pub r: OnceCell<CyclicNodeRef<'arena>>,
}

impl<'a> CyclicNode<'a> {
    fn new(name: Name) -> Self {
        Self {
            name,
            l: Default::default(),
            r: Default::default(),
        }
    }

    pub fn is_target(&'a self) -> bool {
        self.name.is_target()
    }
}

impl<'a> FromStr for CyclicNode<'a> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name: Name = s.parse()?;
        Ok(CyclicNode::new(name))
    }
}

pub type CyclicNodeRef<'a> = &'a CyclicNode<'a>;

pub struct Graph<'a> {
    pub nodes: FrozenMap<Name, Box<CyclicNode<'a>>>,
}

impl<'arena> Graph<'arena> {
    fn link_nodes(&'arena self, name: &Name, l: &Name, r: &Name) {
        let v = self.nodes.get(name).unwrap();
        let l = self.nodes.get(l).unwrap();
        let r = self.nodes.get(r).unwrap();
        v.l.set(l).unwrap();
        v.r.set(r).unwrap();
    }

    pub fn new() -> Graph<'arena> {
        Graph {
            nodes: FrozenMap::new(),
        }
    }

    pub fn add_nodes(&'arena self, Nodes(links): &Nodes) {
        for (&name, _) in links {
            self.nodes.insert(name, Box::new(CyclicNode::new(name)));
        }
        for (name, (l, r)) in links {
            self.link_nodes(name, l, r);
        }
    }
}
