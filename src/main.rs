mod key_creator;
mod encode;
mod decode;

fn main() {
    println!("Hello, world!");
}






#[cfg(test)]
mod tests {
    use std::ops::Index;

    use crate::key_creator;

    use super::{encode::*, decode::*, key_creator::*};
    #[test]
    fn sherlock() {
        let sherlock = "I have the advantage of knowing your habits, my dear Watson, said he. When your round is a short one you walk, and when it is a long one you use a hansom. As I perceive that your boots, although used, are by no means dirty, I cannot doubt that you are at present busy enough to justify the hansom.

        Excellent! I cried.
        
        Elementary, said he. It is one of those instances where the reasoner can produce an effect which seems remarkable to his neighbour, because the latter has missed the one little point which is the basis of the deduction. The same may be said, my dear fellow, for the effect of some of these little sketches of yours, which is entirely meretricious, depending as it does upon your retaining in your own hands some factors in the problem which are never imparted to the reader.";
    
        let mut bytes : Vec<u8> = sherlock.as_bytes().into_iter().map(|x|*x).collect();

        let byte_disposition = byte_disposition(bytes.iter().map(|x| *x));
        let key = key_creator(byte_disposition);

        bytes.reverse();

        let mut encoded : Vec<bool> = key.encode_rev(bytes.into_iter()).collect();
        encoded.reverse();

        let encoded_len = encoded.len();

        let decoded : Vec<u8> = key.decode(encoded.into_iter()).collect();

        let decoded_string : String = decoded.iter().map(|&x| x as char).collect();

        println!("OG : {}, COMP : {}", sherlock.len()*8, encoded_len);

        assert_eq!(sherlock.to_string(), decoded_string);
    
    }
}