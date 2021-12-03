use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn byte_disposition<I>(byte_iter : I) -> [usize;256]
where I: Iterator<Item = u8> {
    let mut byte_disposition = [0usize;256];
    
    for byte in byte_iter {
        byte_disposition[byte as usize] += 1;
    }
    
    byte_disposition
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum TreeNode {
    Node(u16, u16),
    Byte(u8)
}
use TreeNode::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Tree {
    parent : u16,
    children : TreeNode
}

impl Tree {
    fn new(parent : u16, tree_node : TreeNode) -> Self {
        Tree{parent : parent, children : tree_node}
    }

    fn set_parent(&mut self, parent : u16) {
        self.parent = parent;
    }
}

pub fn key_creator(byte_disposition : [usize;256]) -> Key {
    let mut tree_array : [Tree;2*256-1] = [Tree::new(0, Byte(0));2*256-1]; //The values assigned here are temporary, but it can be gauranteed that they will be changed.
    let mut tree_heap : BinaryHeap<Reverse<(usize, u16)>> = BinaryHeap::with_capacity(2*256-1);

    for i in 0..256usize {
        tree_array[i] = Tree::new(0, Byte(i as u8));
        tree_heap.push(Reverse((byte_disposition[i], i as u16)));
    }

    for i in 256..2*256-1 {
        let Reverse(node1) = tree_heap.pop().unwrap();
        let Reverse(node2) = tree_heap.pop().unwrap();

        tree_array[i] = Tree::new(0, Node(node1.1, node2.1));
        tree_array[node1.1 as usize].set_parent(i as u16);
        tree_array[node2.1 as usize].set_parent(i as u16);

        tree_heap.push(Reverse((node1.0 + node2.0 , i as u16)));
    }

    Key::new(tree_array)
}


pub struct Key ([Tree;2*256-1]);

impl Key {
    fn new(key : [Tree;2*256-1]) -> Key {
        Key(key)
    }

    pub(crate) fn get_parent(&self, index : u16) -> u16 {
        self.0[index as usize].parent
    }

    pub(crate) fn get_children(&self, index : u16) ->  TreeNode {
        self.0[index as usize].children
    }

    pub(crate) fn is_byte(&self, index : u16) -> bool {
        match self.0[index as usize].children {
            Node(_, _) => false,
            Byte(_) => true
        }
    }
}
