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
pub fn score_character(c: u8) -> i32 {
    let ch = c as char;

    // Penalize non printable characters
    if !ch.is_ascii() || c < 0x20 || c > 0x7E {
        return -5;
    }

    let ch_upper = ch.to_ascii_uppercase();

    // Frequent English letters get highest score
    if FREQUENT_LETTERS.contains(ch_upper) {
        return 5;
    }

    if ch.is_ascii_alphabetic() {
        return 3;
    }

    if ch == ' ' {
        return 3;
    }

    // Common punctuation
    if ",.'!?;:".contains(ch) {
        return 1;
    }

    0
}

pub fn single_byte_xor_cipher(hex: &str) -> (String, char) {
    // try every single-byte key and pick the best-scoring plaintext

    let mut best_key: char = '\0';
    let mut best_output: String = String::new();
    let mut best_score: i32 = 0;

    let hex_string: Vec<u8> = decode(hex).unwrap();

    for ascii_value in 0u8..=255u8 {
        let xored_vector: Vec<u8> = hex_string
            .iter()
            .map(|&value| value ^ ascii_value) // XOR each value with current ascii_value (key)
            .collect();

        let curr_score: i32 = xored_vector
            .iter()
            .map(|&b| score_character(b))
            .sum();
        
        if curr_score > best_score {
            best_score = curr_score;
            best_key = ascii_value as char;
            best_output = String::from_utf8_lossy(&xored_vector).to_string();
        }
    }

    (best_output, best_key)
}

