use base64::engine::general_purpose;
use base64::Engine as _;
use hex::{decode, encode};

pub fn hex_to_base64(hex: &str) -> String {
    let bytes: Vec<u8> = hex::decode(hex).expect("invalid hex");
    general_purpose::STANDARD.encode(bytes)
}

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