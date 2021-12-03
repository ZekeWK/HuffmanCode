use crate::key_creator::{*, TreeNode::*};

pub struct EncodeIterRev <'a, T> where T: Iterator<Item = u8> { //Must be given the reverse of the message.
    to_encode : T,
    key : &'a Key,
    cur_pos : u16
}

impl Key {
    pub fn encode_rev<T>(&self, to_encode : T) -> EncodeIterRev<T> where T: Iterator<Item = u8> {
        EncodeIterRev{to_encode : to_encode, key : self, cur_pos : 256*2-2}
    }
}

impl <'a, T> Iterator for EncodeIterRev<'a, T> 
where T: Iterator<Item = u8>
{
    type Item = bool;
    fn next(&mut self) -> Option<<Self as Iterator>::Item>{
        if self.cur_pos == 2*256-2 {
            match self.to_encode.next() {
                Some(byte) => self.cur_pos = byte as u16,
                None => return None,
            }
        };

        let last_pos = self.cur_pos;

        self.cur_pos = self.key.get_parent(self.cur_pos);

        match self.key.get_children(self.cur_pos) {
            Byte(_) => unreachable!(),
            Node(child1, child2) => Some(last_pos != child1)
        }
    }
}