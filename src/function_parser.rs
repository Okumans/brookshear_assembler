use once_cell::sync::Lazy;
use std::fmt;
use std::num::ParseIntError;
use std::{collections::HashMap, iter};

#[derive(Debug)]
pub enum ArgumentType {
    Register,
    MemoryAddress,
    Hexadecimal,
    SingleDigitHexadecimal,
    ZeroPadding,
}

pub struct Parameter {
    pub argument_numbers: usize,
    pub args: Vec<ArgumentType>,
}

pub struct Instruction {
    pub instruction_code: usize,
    pub parameter: Parameter,
}

pub struct ParserResult {
    pub bits: Vec<u8>,
    pub address: Option<u8>,
}

#[derive(Debug)]
pub enum ParserError {
    InvalidFormat(String),
    OutOfRange(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidFormat(details) => write!(f, "Invalid format: {}", details),
            ParserError::OutOfRange(value) => write!(f, "Out of range: {}", value),
        }
    }
}

impl std::error::Error for ParserError {}

impl From<ParseIntError> for ParserError {
    fn from(item: ParseIntError) -> Self {
        ParserError::InvalidFormat(item.to_string())
    }
}

static INSTRUCTIONS: Lazy<HashMap<&'static str, Instruction>> = Lazy::new(|| {
    HashMap::from([
        (
            "load",
            Instruction {
                instruction_code: 0x01,
                parameter: Parameter {
                    argument_numbers: 2,
                    args: vec![ArgumentType::Register, ArgumentType::MemoryAddress],
                },
            },
        ),
        (
            "loadi",
            Instruction {
                instruction_code: 0x02,
                parameter: Parameter {
                    argument_numbers: 2,
                    args: vec![ArgumentType::Register, ArgumentType::Hexadecimal],
                },
            },
        ),
        (
            "store",
            Instruction {
                instruction_code: 0x03,
                parameter: Parameter {
                    argument_numbers: 2,
                    args: vec![ArgumentType::Register, ArgumentType::MemoryAddress],
                },
            },
        ),
        (
            "move",
            Instruction {
                instruction_code: 0x04,
                parameter: Parameter {
                    argument_numbers: 2,
                    args: vec![
                        ArgumentType::ZeroPadding,
                        ArgumentType::Register,
                        ArgumentType::Register,
                    ],
                },
            },
        ),
        (
            "add",
            Instruction {
                instruction_code: 0x05,
                parameter: Parameter {
                    argument_numbers: 3,
                    args: vec![
                        ArgumentType::Register,
                        ArgumentType::Register,
                        ArgumentType::Register,
                    ],
                },
            },
        ),
        (
            "addf",
            Instruction {
                instruction_code: 0x06,
                parameter: Parameter {
                    argument_numbers: 3,
                    args: vec![
                        ArgumentType::Register,
                        ArgumentType::Register,
                        ArgumentType::Register,
                    ],
                },
            },
        ),
        (
            "or",
            Instruction {
                instruction_code: 0x07,
                parameter: Parameter {
                    argument_numbers: 3,
                    args: vec![
                        ArgumentType::Register,
                        ArgumentType::Register,
                        ArgumentType::Register,
                    ],
                },
            },
        ),
        (
            "and",
            Instruction {
                instruction_code: 0x08,
                parameter: Parameter {
                    argument_numbers: 3,
                    args: vec![
                        ArgumentType::Register,
                        ArgumentType::Register,
                        ArgumentType::Register,
                    ],
                },
            },
        ),
        (
            "xor",
            Instruction {
                instruction_code: 0x09,
                parameter: Parameter {
                    argument_numbers: 3,
                    args: vec![
                        ArgumentType::Register,
                        ArgumentType::Register,
                        ArgumentType::Register,
                    ],
                },
            },
        ),
        (
            "rotate",
            Instruction {
                instruction_code: 0x0A,
                parameter: Parameter {
                    argument_numbers: 2,
                    args: vec![
                        ArgumentType::Register,
                        ArgumentType::ZeroPadding,
                        ArgumentType::SingleDigitHexadecimal,
                    ],
                },
            },
        ),
        (
            "jump",
            Instruction {
                instruction_code: 0x0B,
                parameter: Parameter {
                    argument_numbers: 2,
                    args: vec![ArgumentType::Register, ArgumentType::MemoryAddress],
                },
            },
        ),
        (
            "halt",
            Instruction {
                instruction_code: 0x0C,
                parameter: Parameter {
                    argument_numbers: 0,
                    args: vec![
                        ArgumentType::ZeroPadding,
                        ArgumentType::ZeroPadding,
                        ArgumentType::ZeroPadding,
                    ],
                },
            },
        ),
    ])
});

fn dec_to_hex_push(num: u8, hex_vec: &mut Vec<u8>) {
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

fn is_n_digit_hexadecimal_with_prefix(s: &str, n: usize) -> bool {
    s.len() == n + 2 && s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit())
}

fn is_register(command: &str) -> bool {
    command.starts_with('R') && command[1..].parse::<u8>().is_ok()
}

fn is_address(command: &str) -> bool {
    command.len() >= 2
        && command.starts_with('[')
        && command.ends_with(']')
        && is_n_digit_hexadecimal_with_prefix(&command[1..command.len() - 1], 2)
}

// Do not check if the register is valid.
fn parse_register(command: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(&command[1..], 16)
}

// Do not check if the address is valid.
fn parse_address(command: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(&command[3..command.len() - 1], 16)
}

pub fn parser(line: &str, address_limit: usize) -> Result<Option<ParserResult>, ParserError> {
    // get the instruction without a comment
    let instruction = if let Some((instruction, _)) = line.split_once(';') {
        instruction.trim()
    } else {
        line.trim()
    };

    // get the instruction without a MemoryAddress specifier
    let (command_address, instruction) =
        if let Some((command_address, instruction)) = instruction.split_once(':') {
            if !is_n_digit_hexadecimal_with_prefix(command_address.trim(), 2) {
                return Err(ParserError::InvalidFormat(format!(
                    "Address \"{}\" is not in a format of 0x000.",
                    command_address.trim()
                )));
            }

            let command_address = u8::from_str_radix(&command_address.trim()[2..], 16).ok();
            if let Some(command_address) = command_address {
                if command_address as usize >= address_limit {
                    return Err(ParserError::OutOfRange(format!(
                        "Address \"{:02x}\" is out of range 0-{}.",
                        command_address, address_limit
                    )));
                }
            }

            (command_address, instruction.trim())
        } else {
            (None, instruction.trim())
        };

    if instruction.is_empty() {
        return Ok(None);
    }

    let mut instruction_parts = instruction.splitn(2, char::is_whitespace);
    let instruction_command = instruction_parts.next().unwrap_or("").trim();
    let instruction_arguments = instruction_parts.next().unwrap_or("").trim();

    if !INSTRUCTIONS.contains_key(instruction_command) {
        return Err(ParserError::InvalidFormat(format!(
            "Instruction \"{}\" is not valid.",
            instruction_command
        )));
    }

    let target_instruction = &INSTRUCTIONS[instruction_command];
    let incoming_arguments: Vec<_> = instruction_arguments.split(',').map(|a| a.trim()).collect();

    if (incoming_arguments.len() != 1
        && target_instruction.parameter.argument_numbers != incoming_arguments.len())
        || (incoming_arguments.len() == 1 && !incoming_arguments[0].is_empty())
    {
        return Err(ParserError::InvalidFormat(format!(
            "Argument numbers for Instruction \"{}\" is not valid. (must be {})",
            instruction_command, target_instruction.parameter.argument_numbers
        )));
    }

    let mut result_instruction = vec![target_instruction.instruction_code as u8];
    let mut zero_padding_buffer = "";

    for (mut incoming_argument, target_argument_type) in incoming_arguments
        .iter()
        .chain(iter::repeat(&""))
        .copied()
        .zip(target_instruction.parameter.args.iter())
    {
        if !zero_padding_buffer.is_empty() {
            (incoming_argument, zero_padding_buffer) = (zero_padding_buffer, incoming_argument);
        }

        let is_valid = match target_argument_type {
            ArgumentType::Register => is_register(incoming_argument),
            ArgumentType::MemoryAddress => is_address(incoming_argument),
            ArgumentType::Hexadecimal => is_n_digit_hexadecimal_with_prefix(incoming_argument, 2),
            ArgumentType::SingleDigitHexadecimal => {
                is_n_digit_hexadecimal_with_prefix(incoming_argument, 1)
            }
            ArgumentType::ZeroPadding => {
                zero_padding_buffer = incoming_argument;
                true
            }
        };

        if !is_valid {
            return Err(ParserError::InvalidFormat(format!(
                "Arguments for Instruction \"{}\" is not valid.",
                instruction_command
            )));
        }

        // Parse the incoming argument into a u8 value.
        let parsed_value = match target_argument_type {
            ArgumentType::Register => parse_register(incoming_argument),
            ArgumentType::MemoryAddress => parse_address(incoming_argument),
            ArgumentType::Hexadecimal => u8::from_str_radix(&incoming_argument[2..], 16),
            ArgumentType::SingleDigitHexadecimal => u8::from_str_radix(&incoming_argument[2..], 16),
            ArgumentType::ZeroPadding => Ok(0),
        }?;

        match target_argument_type {
            ArgumentType::Register
            | ArgumentType::SingleDigitHexadecimal
            | ArgumentType::ZeroPadding => {
                result_instruction.push(parsed_value);
            } // Single digit if possible
            ArgumentType::MemoryAddress | ArgumentType::Hexadecimal => {
                if parsed_value <= 0xF {
                    result_instruction.push(0);
                    result_instruction.push(parsed_value);
                } else {
                    dec_to_hex_push(parsed_value, &mut result_instruction);
                }
                // format!("{:02x}", parsed_value)
            } // Double digit
        };
    }

    Ok(Some(ParserResult {
        address: command_address,
        bits: result_instruction,
    }))
}
