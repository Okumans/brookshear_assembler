# Brookshear Assembler

## Overview

The Brookshear Assembler is a tool that converts assembly language code into binary format for use with the [joeledstrom Brookshear-Emulator](https://joeledstrom.github.io/brookshear-emu/#). This assembler translates human-readable assembly code into machine code that can be executed by the emulator.

## Syntax

The syntax for the assembly code is based on the provided example. The assembly instructions are designed to be straightforward and include operations such as loading values, performing arithmetic operations, and handling program flow.

### Example Assembly Code

```assembly
; X is stored in 0x50
; n is stored in 0x51
; result will be saved in 0x52

; R1 : Adder (loaded from address 0x50)
; R2 : Counter
; R3 : Sum
; R4 : Temporary storage for intermediate results
; R0 : Limit (n) (loaded from address 0x51)
; R5 : Constant value 1

; Initialization
0x00: load R0, [0x51] ; Load n (the limit) from memory address 0x51
0x02: load R1, [0x50] ; Load adder value from memory address 0x50
0x04: loadi R2, 0x00 ; Initialize counter (R2) to 0
0x06: loadi R3, 0x00 ; Initialize sum (R3) to 0
0x08: loadi R5, 0x01 ; Initialize constant value 1 (R5) to 1

; Increment n
0x0A: add R0, R0, R5 ; Increment n (R0) by 1

; Main loop
0x0C: add R4, R3, R1 ; Add Sum (R3) to Adder (R1) and store in temporary buffer (R4)
0x0E: add R3, R4, R2 ; Add Counter (R2) to (Sum + Adder) and store result in Sum (R3)
0x10: add R2, R5, R2 ; Increment Counter (R2) by 1

; Loop control
jump R2, [0x30] ; If Counter (R2) is non-zero, jump to address 0x30 (end of loop)
jump R0, [0x0C] ; If n (R0) is non-zero, jump to address 0x0C (continue loop)

; End of program
0x30: store R3, [0x52] ; Store the final result (Sum in R3) at memory address 0x52
0x32: halt ; Terminate the program
```

## Instruction Set

The assembler translates instructions based on the [Brookshear SML Instruction Set](https://www.mycourseville.com/sites/all/modules/courseville/files/uploads/2016_1/2110221/materials/sml_instruction_set.333.1471674877.pdf). The key instructions include:

| Op-code | Operand | Description                                                                                                                                                                                                                                                                                               |
| ------- | ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1       | RXY     | LOAD the register R with the bit pattern found in the memory cell whose address is XY.                                                                                                                                                                                                                    |
| 2       | RXY     | LOAD the register R with the bit pattern XY.                                                                                                                                                                                                                                                              |
| 3       | RXY     | STORE the bit pattern found in register R in the memory cell whose address is XY.                                                                                                                                                                                                                         |
| 4       | 0RS     | MOVE the bit pattern found in register R to register S.                                                                                                                                                                                                                                                   |
| 5       | RST     | ADD the bit patterns in registers S and T as though they were two’s complement representations and leave the result in register R.                                                                                                                                                                        |
| 6       | RST     | ADD the bit patterns in registers S and T as though they represented values in floating-point notation and leave the floating-point result in register R.                                                                                                                                                 |
| 7       | RST     | OR the bit patterns in registers S and T and place the result in register R.                                                                                                                                                                                                                              |
| 8       | RST     | AND the bit patterns in registers S and T and place the result in register R.                                                                                                                                                                                                                             |
| 9       | RST     | EXCLUSIVE OR the bit patterns in registers S and T and place the result in register R.                                                                                                                                                                                                                    |
| A       | R0X     | ROTATE the bit pattern in register R one bit to the right X times. Each time place the bit that started at the low-order end at the high-order end.                                                                                                                                                       |
| B       | RXY     | JUMP to the instruction located in the memory cell at address XY if the bit pattern in register R is equal to the bit pattern in register number 0. Otherwise, continue with the normal sequence of execution. (The jump is implemented by copying XY into the program counter during the execute phase.) |
| C       | 000     | HALT execution.                                                                                                                                                                                                                                                                                           |

## Usage

1. **Write Assembly Code:**
   Create your assembly code following the syntax provided in the example section.

2. **Assemble Code:**
   Use the Brookshear Assembler to convert your assembly code to binary format. The assembler reads the assembly code and generates a binary file suitable for the [Brookshear Emulator](https://joeledstrom.github.io/brookshear-emu/#).

3. **Run on Emulator:**
   Load the generated binary string into the [Brookshear Emulator](https://joeledstrom.github.io/brookshear-emu/#) to execute the program.

## Installation

To install the Brookshear Assembler, follow these steps:

1. Clone the repository:

   ```sh
   git clone https://github.com/okumans/brookshear_assembler.git
   ```

2. Navigate to the project directory:

   ```sh
   cd brookshear_assembler
   ```

3. Build the assembler:

   ```sh
   cargo build --release
   ```

4. Run the assembler with your assembly code:

   ```sh
   ./sml_assembler input.asm output.bin
   ```
