use crate::key::*;

//Must be given the reverse of the message, and gives the reverse of the message.
pub struct EncodeIterRev <'a, T> where T: Iterator<Item = u8> { 
    to_encode : T,
    key : &'a Key,
    cur_pos : u16
}

impl <'a, T> EncodeIterRev<'a, T> where T : Iterator<Item = u8> {
    fn new(to_encode : T, key : &'a Key) -> Self {
        EncodeIterRev{to_encode : to_encode, key : key, cur_pos : 2*256-2}
    }
}

pub trait EncodableRev <T> where T : Iterator<Item = u8> {
    fn encode(self, key : &Key) -> EncodeIterRev<T>;
}

impl <T> EncodableRev<T> for T where T : Iterator<Item = u8> {
    fn encode(self, key : &Key) -> EncodeIterRev<T> {
        EncodeIterRev::new(self, key)
    }
}

impl <'a, T> Iterator for EncodeIterRev<'a, T> 
where T: Iterator<Item = u8>
{
    type Item = bool;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.cur_pos == 2*256-2 {
            match self.to_encode.next() {
                Some(byte) => self.cur_pos = byte as u16,
                None => return None,
            }
        };

        let last_pos = self.cur_pos;
        self.cur_pos = self.key.get_parent(self.cur_pos);

        let [child1, _child2] = self.key.get_children(self.cur_pos);
        
        Some(last_pos != child1)
    }
}