// use crate::utils::*; COMMENT BACK IN ONCE IN USE
use std::io;

pub fn run(){
    let mut cipher_text = String::new();
    println!("Input hex string that has been single-byte XOR'd: ");

    io::stdin().read_line(&mut cipher_text).unwrap();
    let cleaned_cipher_text = cipher_text.trim_end();

    println!("{}", cleaned_cipher_text); // placeholder
    //TODO: write functions for this challenge in utils
}


//TODO: write test case for challenge given input