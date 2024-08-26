use std::num::ParseIntError;

pub fn dec_to_hex_push(num: u8, hex_vec: &mut Vec<u8>) {
    let high_nibble = (num >> 4) & 0x0F;
    let low_nibble = num & 0x0F;

    hex_vec.push(if high_nibble < 10 {
        high_nibble
    } else {
        high_nibble + 10 // 'A' corresponds to 10
    });

    hex_vec.push(if low_nibble < 10 {
        low_nibble
    } else {
        low_nibble + 10 // 'A' corresponds to 10
    });
}

pub fn is_n_digit_hexadecimal_with_prefix(s: &str, n: usize) -> bool {
    s.len() == n + 2 && s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit())
}

pub fn is_register(command: &str) -> bool {
    command.starts_with('R') && command[1..].parse::<u8>().is_ok()
}

pub fn is_address(command: &str) -> bool {
    command.len() >= 2
        && command.starts_with('[')
        && command.ends_with(']')
        && is_n_digit_hexadecimal_with_prefix(&command[1..command.len() - 1], 2)
}

// Do not check if the register is valid.
pub fn parse_register(command: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(&command[1..], 16)
}

// Do not check if the address is valid.
pub fn parse_address(command: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(&command[3..command.len() - 1], 16)
}
