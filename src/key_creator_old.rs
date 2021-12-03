use std::collections::BinaryHeap;
use std::cmp::Reverse;
use either::Either::{self, *};
use array_init::array_init;
use bitvec::prelude::*;


fn byte_disposition<I>(byte_iter : I) -> [usize;256]
where I: Iterator<Item = u8> {
    let mut byte_disposition = [0usize;256];
    
    for byte in byte_iter {
        byte_disposition[byte as usize] += 1;
    }
    
    byte_disposition
}


fn huffman_tree_to_array(huffman_tree : &[Either<u8, (u16, u16)>;2*256], cur_node : u16, cur_path : BitVec, output : &mut [BitVec;256]) {
    match huffman_tree[cur_node as usize] {
        Left(byte) => output[byte as usize] = cur_path,
        Right((path1, path2)) => {
            let mut tree_path1 = cur_path.clone();
            let mut tree_path2 = cur_path;
            tree_path1.push(false);
            tree_path2.push(true);
            huffman_tree_to_array(huffman_tree, path1, tree_path1, output);
            huffman_tree_to_array(huffman_tree, path2, tree_path2, output);
        }
    }
}

fn key_creator(byte_disposition : [usize;256]) -> Key {
    let mut tree_array : [Either<u8, (u16, u16)>;2*256] = [Left(0);2*256]; //Redo and make cleaner
    let mut tree_heap : BinaryHeap<Reverse<(usize, u16)>> = BinaryHeap::with_capacity(2*256);

    for i in 0..256usize {
        tree_array[i] = Left(i as u8);
        tree_heap.push(Reverse((byte_disposition[i], i as u16)));
    }

    for i in 256..2*255 {
        let Reverse(node1) = tree_heap.pop().unwrap();
        let Reverse(node2) = tree_heap.pop().unwrap();

        tree_array[i] = Right((node1.1, node2.1));
        tree_heap.push(Reverse((node1.0 + node2.0 , i as u16)));
    }
    
    let mut encode_array : [BitVec;256] = array_init(|_x| BitVec::with_capacity(0));;
    huffman_tree_to_array(&tree_array, 255, BitVec::new(), &mut encode_array);
    
    Key::new(encode_array, tree_array)
}


pub struct Key {
    encode_key : [BitVec;256],
    decode_key : [Either<u8, (u16, u16)>;2*256],
}

impl Key {
    fn new(encode_key : [BitVec;256], decode_key : [Either<u8, (u16, u16)>;2*256]) -> Key {
        Key{encode_key : encode_key, decode_key : decode_key}
    }
}
