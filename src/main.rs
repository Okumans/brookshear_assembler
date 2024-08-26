use sml_assembler::function_parser::{parser, ParserResult};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");

    if !Path::new(&path).exists() {
        println!("Path {:?} is'nt exists.", path);
        return;
    }

    let file = File::open(path).expect("Unable to open file.");
    let reader = BufReader::new(file);

    println!("{}", asm_transformer(reader));
}

fn asm_transformer(reader: BufReader<File>) -> String {
    let mut memory_address: [u8; 256] = [0; 256];
    let mut program_counter: u8 = 0x00;

    for (line_number, line) in reader.lines().enumerate() {
        let Ok(line) = line else {
            break;
        };

        if line.trim().is_empty() {
            continue;
        }

        let Some(ParserResult {
            bits: parsed,
            address,
        }) = parser(&line, 256).unwrap_or_else(|err| {
            panic!("Failed to parse line {}. becuase {}", line_number + 1, err)
        })
        else {
            continue;
        };

        if address.is_some() {
            program_counter = address.unwrap();
        }

        for (values, current_address) in parsed.chunks(2).zip(program_counter..program_counter + 2)
        {
            memory_address[current_address as usize] = (values[0] * 16) + values[1];
        }

        program_counter += 2;
    }

    let result_memory_address = memory_address
        .map(|a| format!("{:02x}", a))
        .join("")
        .trim_end_matches('0')
        .to_string();

    if result_memory_address.ends_with('c') {
        format!("{}0", result_memory_address)
    } else {
        result_memory_address
    }
}
