use crate::key::*;

pub struct DecodeIter <'a, T> where T: Iterator<Item = bool> {
    to_decode : T,
    key : &'a Key,
}

impl <'a, T> DecodeIter<'a, T> where T : Iterator<Item = bool> {
    fn new(to_decode : T, key : &'a Key) -> DecodeIter <T> where T : Iterator<Item = bool> {
        DecodeIter{to_decode : to_decode, key : key}
    }
}

pub trait Decodable <T> where T : Iterator<Item = bool> {
    fn decode(self, key : &Key) -> DecodeIter<T>;
}

impl <T> Decodable<T> for T where T : Iterator<Item = bool> {
    fn decode(self, key : &Key) -> DecodeIter<T> {
        DecodeIter::new(self, key)
    }
}

impl <'a, T> Iterator for DecodeIter<'a, T> 
where T: Iterator<Item = bool>
{
    type Item = u8;
    fn next(&mut self) -> Option<<Self as Iterator>::Item>{//Make it break properly
        let mut cur_pos = 256*2-2;

        let mut dir = match self.to_decode.next() {
            Some(dir) => dir,
            None => return None,
        };

        loop {
            //println!("sds {}", cur_pos);
            cur_pos = self.key.get_children(cur_pos)[dir as usize];
            if cur_pos <= 256 {
                return Some(cur_pos as u8);
            }
            else {
                dir = self.to_decode.next().expect("Message does not decode to bytes.");
            }
        }
    }
}