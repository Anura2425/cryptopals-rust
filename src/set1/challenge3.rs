use crate::utils::*;
use std::io;

pub fn run(){
    let mut hex_string = String::new();
    println!("Input hex string that has been single-byte XOR'd: ");

    io::stdin().read_line(&mut hex_string).unwrap();
    let cleaned_hex_string = hex_string.trim_end();

    let results: (String, char) = single_byte_xor_cipher(cleaned_hex_string);

    let (decrypted_string, key) = results;
    
    println!("The single byte key '{key}' Output decrypted string: \n{decrypted_string}")
}


//TODO: write test case for challenge given input
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passed(){
        let (result_string, result_key) = single_byte_xor_cipher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        assert_eq!(result_key, 'X');
        assert_eq!(result_string, "Cooking MC's like a pound of bacon");
    }
}
