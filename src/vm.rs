use super::instruction::Opcode;
pub struct VM{
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8> //vector of bytes

}


impl VM{
    pub fn new() -> VM{
        VM{
            registers: [0;32],
            program: vec![],
            pc: 0,
        }
    }
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    pub fn run(&mut self){
        loop{
            if self.pc >= self.program.len(){
                break;
            }
            match self.decode_opcode(){
                Opcode::HLT => {
                    println!("HLT Encountered");
                    return;
                }, 
                _ => {
                    println!("unrecognized opcode, terminating");
                    return;
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl(){
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 200, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
}
