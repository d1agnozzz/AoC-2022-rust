use std::{borrow::Borrow, collections::HashSet};

pub(crate) mod parsers;
#[cfg(test)]
mod tests;

use parsers::*;
// stack - keep track of current position in fs
// fs tree - image of whole fs
// tree node - contains refs to inner dirs and has a collection of contained files

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
    fn total_size(&self) -> usize {
        self.files.iter().map(|e| e.size).sum::<usize>()
            + self.children.iter().map(Self::total_size).sum::<usize>()
    }
    fn all_dirs(&self) -> Box<dyn Iterator<Item = &DirectoryNode> + '_> {
        Box::new(std::iter::once(self).chain(self.children.iter().flat_map(Self::all_dirs)))
    }
    fn traverse_whole_tree(&self, level: usize) {
        println!("{}{}", "    ".repeat(level), self.name);
        for child in &self.children {
            child.traverse_whole_tree(level + 1);
        }
    }
    fn sum_size_of_at_most(&self, limit: usize, accumulator: usize, sum: &mut usize) -> usize {
        let files_size_sum: usize = self.files.iter().map(|e| e.size).sum();
        let mut accumulated_size = 0;
        let mut true_self_size = files_size_sum;
        for child in &self.children {
            let child_size = child.sum_size_of_at_most(limit, accumulator, sum);
            println!("child size: {child_size}");
            if accumulated_size + child_size <= limit {
                accumulated_size += child_size;
            }
            true_self_size += child_size;
        }

        if accumulated_size + files_size_sum <= limit {
            accumulated_size += files_size_sum;
        }

        if true_self_size <= limit {
            *sum += true_self_size;
        }
        dbg!(accumulated_size);
        dbg!(sum);

        // if whole_self_size >= limit {
        //     return accumulator;
        // }
        accumulator + true_self_size
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
// impl Borrow<Name> for DirectoryNode {
//     fn borrow(&self) -> &Name {
//         self.name.as_ref()
//     }
// }
impl std::fmt::Debug for DirectoryNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dirnode {}", self.name)
    }
}

struct NodeZipper {
    node: DirectoryNode,
    parent: Option<Box<NodeZipper>>,
}
impl NodeZipper {
    fn child(mut self, name: &str) -> NodeZipper {
        let child = self
            .node
            .children
            .take(&DirectoryNode {
                name: name.into(),
                files: HashSet::new(),
                children: HashSet::new(),
            })
            .unwrap();

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
    fn finish(mut self) -> DirectoryNode {
        while let Some(_) = self.parent {
            self = self.parent();
        }
        self.node
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut parse_subject = input.clone();

    let mut fs_stack = std::path::PathBuf::new();

    let mut root_node = DirectoryNode::new("root");
    root_node.add_child(DirectoryNode::new("/"));
    let mut zipper = root_node.zipper();

    while let Ok((remains_main, cmd)) = parsers::parse_cmd_line(parse_subject) {
        match cmd {
            Command::Cd { name: dir_name } => {
                println!("cd {dir_name}");
                if let ".." = dir_name {
                    fs_stack.pop();
                    zipper = zipper.parent();
                } else {
                    fs_stack.push(dir_name);
                    zipper = zipper.child(dir_name);
                }
            }
            Command::Ls { mut contents } => {
                println!("ls {}", fs_stack.display());
                while let Ok((remains_ls_contents, entry)) = parsers::parse_ls_entry(contents) {
                    match entry {
                        LsEntry::File(file) => {
                            println!("file {} {}", file.name, file.size);
                            zipper.node.add_file(File {
                                name: file.name,
                                size: file.size,
                            });
                            // add file to node
                        }
                        LsEntry::Directory(name) => {
                            println!("dir {name}");
                            zipper.node.add_child(DirectoryNode::new(name));
                            // add child to node
                        }
                    }
                    contents = remains_ls_contents;
                }
            }
        }
        parse_subject = remains_main;
    }
    root_node = zipper.finish();
    let total_size = 70_000_000;
    let required_space = 30_000_000;
    let used_space = root_node.total_size();
    let unused_space = total_size - used_space;
    let need_to_free_space = required_space - unused_space;

    let answer_p1 = root_node
        .all_dirs()
        .map(DirectoryNode::total_size)
        .filter(|s| *s <= 100_000)
        .sum::<usize>();

    let answer_p2 = root_node
        .all_dirs()
        .map(DirectoryNode::total_size)
        .filter(|s| *s >= need_to_free_space)
        .min();

    println!("Answer to p1 is presumably: {}", answer_p1);
    println!("Answer to p2 is {}", answer_p2.unwrap());
}
