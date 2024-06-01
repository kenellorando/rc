fn main() {
    let hex: u32 = 0xa000;
    let mut decimal: u32 = 0;
    let mut power: u32 = 0;

    for digit in format!("{:x}", hex).chars().rev() {
        decimal += digit.to_digit(16).unwrap() * 16_u32.pow(power);
        power += 1;
    }

    println!("{}", decimal);


    // let binary: u32 = 0b0011000000111001;
    // let mut decimal: u32 = 0;
    // let mut power: u32 = 0;

    // for digit in format!("{:b}", binary).chars().rev() {
    //     decimal += digit.to_digit(2).unwrap() * 2_u32.pow(power);
    //     power += 1;
    // }

    // println!("{}", decimal);
}
