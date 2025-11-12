use crate::utils::*;
use std::io;

pub fn run(){
    let mut input = String::new();
    println!("Enter an Input Hex String: ");
    
    io::stdin().read_line(&mut input).unwrap();
    let cleaned_input = input.trim_end();
    
    println!("\nOutput Base64 String:\n{}", hex_to_base64(&cleaned_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passed(){
        let result = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        assert_eq!(result, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
    }
}
