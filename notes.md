# Struct VM:

The `VM` struct represents a Virtual Machine, which is a software component that can execute programs. It has three main components:

- `registers`: An array of 32 `i32` (32-bit integer) values, which are used to store and manipulate data.
- `pc`: The "program counter", which keeps track of the current position in the program being executed.
- `program`: A `Vec<u8>` (vector of bytes), which represents the program code that the Virtual Machine will execute.

## Impl VM:

This section contains the implementation of the `VM` struct.

- `new()`: This is a constructor function that creates a new VM instance, initializing the registers to 0, the program to an empty vector, and the program counter (pc) to 0.
- `decode_opcode()`: This function reads the next byte from the program vector, interprets it as an `Opcode` (which we'll explain soon), and advances the pc by 1.
- `run()`: This function is the main execution loop of the Virtual Machine. It repeatedly decodes the next opcode and processes it until it encounters the HLT (Halt) opcode, at which point it exits the loop and returns.

### Registers:

Registers are temporary storage locations within the Virtual Machine that can hold data or addresses. In this case, the VM struct has 32 registers, each of which can store a 32-bit integer (`i32`). Registers are used to store and manipulate data during the execution of the program.

### Program Counter (pc):

The program counter (pc) is a special register that keeps track of the current position in the program being executed. It is used to fetch the next instruction (opcode) from the program vector. As the program executes, the pc is incremented to point to the next instruction.

### Opcode:

An opcode (short for "operation code") is a numeric code that represents a specific instruction or operation that the Virtual Machine can execute. In this code, the Opcode type is not defined, but it is mentioned that the `decode_opcode()` function returns an Opcode value. The Opcode type is likely defined elsewhere in the code (in the instruction module, as indicated by the `use super::instruction::Opcode;` line)

### Jumps:
#### Absolute jump:
- Jumps to memory adress, JMP 100 will jump to memory adress 100
- limited to max number that can be represented by available bits in instruction
- JMP 10000000000000000000000000000000 will not work because our 32 - 8 bit (for JMP opcode) gives us 24 bits to work with, so max number is 2^24 or 16777216

#### Relative jump:
- jumps to memory address relative to current memory address
- JMP +5 will jump 5 memory addresses forward
- JMP -5 will jump 5 memory addresses back
- JMP back is useful for loops

#### Jumps to labels:
- Jump to a labeled or tagged bit, without having to know the memory address
- Labels are like named bookmarks in your code, and the assembler or compiler will figure out the correct memory address for you.
- This makes the code more readable and easier to maintain, as you don't have to worry about the exact memory addresses.
