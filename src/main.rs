fn main() {
    let binary: u32 = 0b1101;
    let mut decimal: u32 = 0;
    let mut power: u32 = 0;

    for digit in format!("{:b}", binary).chars().rev() {
        decimal += digit.to_digit(2).unwrap() * 2_u32.pow(power);
        power += 1;
    }

    println!("{}", decimal);
}
