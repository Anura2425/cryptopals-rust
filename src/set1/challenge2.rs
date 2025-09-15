use crate::utils::*;

pub fn run(){
    let buffer1 = "1c0111001f010100061a024b53535009181c";
    let buffer2 = "686974207468652062756c6c277320657965";
    println!("{}", fixed_xor(buffer1, buffer2));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passed(){
        let result: String = fixed_xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965");
        assert_eq!(result, "746865206b696420646f6e277420706c6179")
    }
}
