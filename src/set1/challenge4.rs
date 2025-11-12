use crate::utils::*;
// use std::io;

pub fn run(){
    let filename: &str = "src/challenge_files/4.txt";
    let (decrypted_string, key, score, line_number) = detect_single_character_xor(filename);
    println!("{decrypted_string}\n{key}\n{score}\n{line_number}")
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn passed(){
//         let (result_string, result_key, best_score, line_number) = detect_single_character_xor("../challenge_files/4.txt");
//         assert_eq!(result_key, 'X');
//         assert_eq!(result_string, "Cooking MC's like a pound of bacon");
//         println!("{best_score}");
//     }
// }
