use std::{borrow::Borrow, collections::HashSet};

pub(crate) mod parsers;
#[cfg(test)]
mod tests;

// stack - keep track of current position in fs
// fs tree - image of whole fs
// tree node - contains refs to inner dirs and has a collection of contained files

#[derive(Debug)]
pub enum Command<'a> {
    Cd { name: &'a str },
    Ls { contents: &'a str },
}

#[derive(Eq)]
pub struct File {
    name: String,
    size: usize,
}
use std::hash::Hash;
impl Hash for File {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
type Name = str;
impl Borrow<Name> for File {
    fn borrow(&self) -> &Name {
        self.name.as_ref()
    }
}

pub enum LsEntry<'a> {
    File(File),
    Directory(&'a str),
}
#[derive(Eq)]
struct DirectoryNode {
    name: String,
    files: HashSet<File>,
    children: HashSet<DirectoryNode>,
}
impl DirectoryNode {
    fn new(name: &str) -> DirectoryNode {
        DirectoryNode {
            name: name.into(),
            files: HashSet::new(),
            children: HashSet::new(),
        }
    }
    fn add_child(&mut self, child: DirectoryNode) {
        self.children.insert(child);
    }
    fn add_file(&mut self, file: File) {
        self.files.insert(file);
    }
    fn zipper(self) -> NodeZipper {
        NodeZipper {
            node: self,
            parent: None,
        }
    }
}
impl Hash for DirectoryNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl PartialEq for DirectoryNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Borrow<Name> for DirectoryNode {
    fn borrow(&self) -> &Name {
        self.name.as_ref()
    }
}

struct NodeZipper {
    node: DirectoryNode,
    parent: Option<Box<NodeZipper>>,
}
impl NodeZipper {
    fn child(mut self, name: &str) -> NodeZipper {
        let child = self.node.children.take(name).unwrap();

        NodeZipper {
            node: child,
            parent: Some(Box::new(self)),
        }
    }
    fn parent(self) -> NodeZipper {
        let NodeZipper { node, parent } = self;

        let NodeZipper {
            node: mut parent_node,
            parent: parent_parent,
        } = *parent.unwrap();
        parent_node.children.insert(node);

        NodeZipper {
            node: parent_node,
            parent: parent_parent,
        }
    }
}

fn main() {
    let content = include_str!("../example.txt");
    println!("{}", content)
}
