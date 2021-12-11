use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, Clone, Copy)]
pub struct Key {
    parents : [u16;2*256-1],
    children : [[u16;2];256-1],
}

impl Key {
    fn default() -> Key {
        Key{parents : [0;2*256-1], children : [[0, 0];256-1]}
    }
    
    pub(crate) fn get_parent(&self, index : u16) -> u16 {
        self.parents[index as usize]
    }
    
    fn set_parent(&mut self, index : u16, parent : u16) {
        self.parents[index as usize] = parent;
    }

    fn set_children(&mut self, index : u16, node : [u16;2]) {
        self.children[index as usize - 256] = node;
    }

    pub(crate) fn get_children(&self, index : u16) -> [u16;2] {
        self.children[index as usize -256]
    }


    pub fn new(byte_disposition : [usize;256]) -> Key {
        let mut key = Key::default();
        let mut heap : BinaryHeap<Reverse<(usize, u16)>> = BinaryHeap::with_capacity(2*256-1);
    
        for i in 0..256 {
            heap.push(Reverse((byte_disposition[i as usize], i)));
        }
    
        for i in 256..2*256-1 {
            let Reverse(node1) = heap.pop().unwrap();
            let Reverse(node2) = heap.pop().unwrap();
    
            key.set_children(i, [node1.1, node2.1]);
            key.set_parent(node1.1, i);
            key.set_parent(node2.1, i);
    
            heap.push(Reverse((node1.0 + node2.0, i)));
        }
        key
    }
}

pub fn byte_disposition<I>(byte_iter : I) -> [usize;256]
where I: Iterator<Item = u8> {
    let mut byte_disposition = [0usize;256];
    
    for byte in byte_iter {
        byte_disposition[byte as usize] += 1;
    }
    byte_disposition
}
