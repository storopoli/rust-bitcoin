use bitcoin::hashes::{sha256, HashEngine};
use honggfuzz::fuzz;

fn do_test(data: &[u8]) {
    let mut engine = sha256::Hash::engine();
    engine.input(data);
    let eng_hash = sha256::Hash::from_engine(engine);

    let hash = sha256::Hash::hash(data);
    assert_eq!(hash.as_byte_array(), eng_hash.as_byte_array());
}

fn main() {
    loop {
        fuzz!(|d| { do_test(d) });
    }
}

#[cfg(all(test, fuzzing))]
mod tests {
    fn extend_vec_from_hex(hex: &str, out: &mut Vec<u8>) {
        let mut b = 0;
        for (idx, c) in hex.as_bytes().iter().enumerate() {
            b <<= 4;
            match *c {
                b'A'..=b'F' => b |= c - b'A' + 10,
                b'a'..=b'f' => b |= c - b'a' + 10,
                b'0'..=b'9' => b |= c - b'0',
                _ => panic!("Bad hex"),
            }
            if (idx & 1) == 1 {
                out.push(b);
                b = 0;
            }
        }
    }

    #[test]
    fn duplicate_crash() {
        let mut a = Vec::new();
        extend_vec_from_hex("fff400610004", &mut a);
        super::do_test(&a);
    }
}
