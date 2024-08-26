use once_cell::sync::Lazy;
use std::collections::HashMap;

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

pub static INSTRUCTIONS: Lazy<HashMap<&'static str, Instruction>> = Lazy::new(|| {
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
