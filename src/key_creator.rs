use std::collections::BinaryHeap;
use std::cmp::Reverse;
use either::Either::*;
use either::Either;
use bit_vec::BitVec;

fn byte_disposition<I>(byte_iter : I) -> [usize;256]
where I: Iterator<Item = u8> {
    let mut byte_disposition = [0usize;256];
    
    for byte in byte_iter {
        byte_disposition[byte as usize] += 1;
    }
    
    byte_disposition
}


fn key_creator(byte_disposition : [usize;256]) -> [Either<u8, (usize, usize)>;2*256] {
    let mut tree_vec : [Either<u8, (usize, usize)>;2*256] = [Left(0);2*256];
    let mut tree_heap : BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::with_capacity(2*256);

    for i in 0..256usize {
        tree_vec[i] = Left(i as u8);
        tree_heap.push(Reverse((byte_disposition[i], i)));
    }

    for i in 256..2*255 {
        let Reverse(node1) = tree_heap.pop().unwrap();
        let Reverse(node2) = tree_heap.pop().unwrap();

        tree_vec[i] = Right((node1.1, node2.1));
        tree_heap.push(Reverse((node1.0 + node2.0 , i)));
    }
    
    tree_vec
}


struct Key {
    encode_key : [BitVec;256],
    decode_key : [Either<u8, (usize, usize)>;2*256],
}

impl Key {
    fn new(encode_key : [BitVec;256], decode_key : [Either<u8, (usize, usize)>;2*256]) -> Key {
        Key{encode_key : encode_key, decode_key : decode_key}
    }

    fn encode(self, Vec<u8>)
}