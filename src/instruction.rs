//an opcode is an integer between 0 and 255 in this case because we are using 8 bits for each code
//rust enums are the bees knees
// op code just represents different actions comp can do
#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT, //halt
    IGL //illegal
}

//an instruction itself is 32 bits
pub struct Instruction{
    opcode: Opcode
}
impl Instruction{
    pub fn new(opcode: Opcode) -> Instruction{
        Instruction{
            opcode: opcode
        }
    }
}

//convert from a byte to an opcode
impl From<u8> for Opcode{
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            _ => Opcode::IGL
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
