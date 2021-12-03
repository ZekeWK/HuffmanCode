use crate::key_creator::{*, TreeNode::*};

pub struct DecodeIter <'a, T> where T: Iterator<Item = bool> { //Must be given the reverse of the message, and gives the reverse of the message.
    to_decode : T,
    key : &'a Key,
    cur_pos : u16
}

impl Key {
    pub fn decode<T>(&self, to_decode : T) -> DecodeIter <T> where T: Iterator<Item = bool> {
        DecodeIter{to_decode : to_decode, key : self, cur_pos : 256*2-2}
    }
}

impl <'a, T> Iterator for DecodeIter<'a, T> 
where T: Iterator<Item = bool>
{
    type Item = u8;
    fn next(&mut self) -> Option<<Self as Iterator>::Item>{//Make it break properly
        let next_byte = loop {
            match self.key.get_children(self.cur_pos) {
                Node(child1, child2) => self.cur_pos = {
                    if !match self.to_decode.next() {Some(val) => val, None => return None} {
                        child1
                    } 
                    else {
                        child2
                    }
                },
                Byte(byte) => break byte
            }
        };
        
        self.cur_pos = 256*2-2;
        
        Some(next_byte)
    }
}