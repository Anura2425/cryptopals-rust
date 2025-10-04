use std::io;

fn main() {
    println!("Enter the number corresponding to the challenge you wish to run:");
    println!("(1) Convert Hex to Base64");
    println!("(2) Fixed XOR");
    println!("(3) Single-Byte XOR cipher");

    let mut input: String = String::new(); // input is read as a string initially
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let challenge_num: i32 = input.trim().parse().expect("Please enter valid integer");

    match challenge_num{
        1=>cryptopals_rust::set1::challenge1::run(),
        2=>cryptopals_rust::set1::challenge2::run(),
        3=>cryptopals_rust::set1::challenge3::run(),
        _=>println!("Select a challenge.")
    }
}
