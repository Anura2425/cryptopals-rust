use crate::utils::*;
use std::io;

pub fn run(){
    let filename: &str = "../challenge_files/4.txt";
    let(decrypted_string, key, score, line_number) = detect_single_character_xor(filename);
    println!("{decrypted_string}\n{key}\n{score}\n{line_number}")
}
