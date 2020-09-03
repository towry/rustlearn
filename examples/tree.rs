#![feature(get_mut_unchecked)]

// I know I should use Box instead of Arc.

extern crate rustlearn;

use std::io;
use std::sync::Arc;
use std::fs::{self, DirEntry};
use std::path::Path;
use rustlearn::util;

pub struct Node {
    pub name: String,
    pub is_dir: bool,
    pub is_file: bool,
    child: Vec<Arc<Node>>,
    parent: Option<Arc<Node>>,
    pub path: String
}

pub struct Tree {
    base: String,
    tree: Option<Arc<Node>>
}

fn callback(entry: &DirEntry, parent_node: &Arc<Node>) -> io::Result<Arc<Node>> {
    let is_file: bool;

    // unwrap 
    if fs::metadata(entry.path())?.is_file() {
        is_file = true;
    } else {
        is_file = false;
    }

    return Node::new(String::from(entry.path().to_str().unwrap()), Some(parent_node.clone()), is_file);
}

impl Node {
    pub fn new(dir: String, mut parent: Option<Arc<Node>>, is_file: bool) -> io::Result<Arc<Node>> {
        let node = Node {
            name: util::path::basename(dir.clone()),
            is_file: is_file,
            is_dir: !is_file,
            path: dir,
            child: Vec::new(),
            parent: None
        };

        let mut node_ref = Arc::new(node);

        match parent.as_mut() {
            Some(mut x) => {
                // Returns a mutable reference into the given Arc, if there are no other Arc or Weak pointers to the same allocation.
                // Returns None otherwise, because it is not safe to mutate a shared value.
                (*Arc::get_mut(&mut node_ref).unwrap()).parent = Some(x.clone());
                unsafe {
                    Arc::get_mut_unchecked(&mut x).push(node_ref.clone());
                }
            },
            None => ()
        }

        if is_file {
            return Ok(node_ref);
        }

        // if is dir 
        // read dir and create the tree 
        let path = Path::new(&*node_ref.path);

        visit_dirs(&path, &callback, &node_ref).unwrap();

        Ok(node_ref)
    }

    pub fn push(&mut self, node: Arc<Node>) {
        self.child.push(node);
    }

    pub fn children(&self) -> &Vec<Arc<Node>> {
        return &self.child;
    }
}

impl Tree {
    pub fn search(&mut self, _path: String) -> Option<Node> {
        self.fresh();

        None
    }

    pub fn fresh(&mut self) {
        self.tree = Node::new(self.base.clone(), None, false).ok();
    }
}


fn visit_dirs<'a> (dir: &Path, cb: &dyn Fn(&DirEntry, &Arc<Node>) -> io::Result<Arc<Node>>, node: &Arc<Node>) -> io::Result<()> {
    if fs::metadata(dir)?.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();

            if let Err(e) = cb(&entry, node) {
                println!("error: {}", e);
                std::process::exit(1);
            }
        }
    } 

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_fresh() {
        let mut tree = Tree {
            base: String::from("/Users/towry/Projects/mobile-flight/build"),
            tree: None
        };

        tree.fresh();
    }
}

fn main() {
    let mut tree = Tree {
        base: String::from("/Users/towry/workspace"),
        tree: None
    };

    tree.fresh();
}
