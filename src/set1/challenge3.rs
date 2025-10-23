use crate::utils::*;
use std::io;

pub fn run(){
    let mut hex_string = String::new();
    println!("Input hex string that has been single-byte XOR'd: ");

    io::stdin().read_line(&mut hex_string).unwrap();
    let cleaned_hex_string = hex_string.trim_end();

    

}


//TODO: write test case for challenge given input
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passed(){

    }
}
