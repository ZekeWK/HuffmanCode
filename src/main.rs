mod key;
mod encode;
mod decode;

fn main() {

}

#[cfg(test)]
mod tests {
    use crate::{encode::*, decode::*, key::*};

    #[test]
    fn sherlock() {
        let sherlock = "You will not apply my precept,\" he said, shaking his head. \"How often have I said to you that when you have eliminated the impossible, whatever remains, however improbable, must be the truth? We know that he did not come through the door, the window, or the chimney. We also know that he could not have been concealed in the room, as there is no concealment possible. When, then, did he come?";
        let byte_iter = sherlock.as_bytes().into_iter().map(|x| *x);

        let byte_disposition = byte_disposition(byte_iter.clone());
        let key = Key::new(byte_disposition);

        let mut encoded : Vec<bool> = byte_iter.rev().encode(&key).collect(); encoded.reverse();

        let decoded_bytes : Vec<u8> = encoded.clone().into_iter().decode(&key).collect();

        let decoded : String = decoded_bytes.into_iter().map(|x| x as char).collect();
        println!("Original : {}, Encoded : {}", sherlock.len()*8, encoded.len());
        assert_eq!(sherlock, decoded)

    }

    #[test]
    fn all_chars() {
        let key = Key::new([0;256]);

        for byte in 0..=255u8 {
            let encoded : Vec<bool> = [byte].into_iter().map(|x|*x).encode(&key).collect();
            println!("Byte : {}, Encoded : {:?}", byte, encoded);
        }
    }
}