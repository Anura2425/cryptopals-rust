use crate::utils::*;
// use std::io;

pub fn run(){
    let filename: &str = "src/challenge_files/4.txt";
    let (decrypted_string, key, score, line_number) = detect_single_character_xor(filename);
    println!("Decrypted String: {decrypted_string}\nKey used: {key}\nScore: {score}\nLine Number in text file: {line_number}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passed(){
        let (result_string, _, _, _) = detect_single_character_xor("src/challenge_files/4.txt");
        assert_eq!(result_string, "Now that the party is jumping\n");
    }
}
