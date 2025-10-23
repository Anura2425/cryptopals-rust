use base64::engine::general_purpose;
use base64::Engine as _;
use hex::{decode, encode};

static FREQUENT_LETTERS: &str = "ETAOIN SHRDLU";

// SET 1:
// CHALLENGE 1:
pub fn hex_to_base64(hex: &str) -> String {
    let bytes: Vec<u8> = hex::decode(hex).expect("invalid hex");
    general_purpose::STANDARD.encode(bytes)
}

// CHALLENGE 2:
pub fn fixed_xor(buffer1: &str, buffer2: &str) -> String {
    let bytes1: Vec<u8> = decode(buffer1).unwrap();
    let bytes2: Vec<u8> = decode(buffer2).unwrap();

    let xor_bytes: Vec<u8> = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect();
    encode(xor_bytes)
}

// CHALLENGE 3:
pub fn score_character(input: char) -> i32 {
    let mut score: i32 = 0;
    if input.is_ascii(){
        if input.is_alphabetic(){
            if FREQUENT_LETTERS.contains(input.to_ascii_uppercase()){
                score += 5;
            }
            score += 3;
        }
        score += 1;
    }
    score
}

pub fn find_single_byte_xor_key(hex: &str) -> (&str, i32){
    ("hello", 10)
}

