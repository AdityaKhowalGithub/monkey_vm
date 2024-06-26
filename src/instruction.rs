//an opcode is an integer between 0 and 255 in this case because we are using 8 bits for each code
//rust enums are the bees knees
// op code just represents different actions comp can do
#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,  //halt
    LOAD, //load into register
    IGL,  //illegal
    ADD,  //add
    SUB,  //subtract
    MUL,  //multiply
    DIV,  //divide
    JMP,  //jump
    JMPF,
    JMPB,
    EQ,  //EQUAL
    NEQ, //NOT EQUAL
    GT,  //GREATER THAN
    LT,  //LESS THAN
    GTE, //GREATER THAN OR EQUAL
    LTE, //LESS THAN OR EQUAL
    JEQ, //JUMP IF EQUAL
}

//an instruction itself is 32 bits
pub struct Instruction {
    opcode: Opcode,
}
impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

//convert from a byte to an opcode
impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTE,
            14 => Opcode::LTE,
            15 => Opcode::JEQ,
            _ => Opcode::IGL,
        }
    }
}
//right now 0 halts and anything else gives illegal
////in decode opcode we increment PC by 1 because once we decode an opcode we have to move the
///counter to the next byte

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
