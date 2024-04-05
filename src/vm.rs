use super::instruction::Opcode;
pub struct VM{
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,//vector of bytes
    remainder: u32,


}


impl VM{
    pub fn new() -> VM{
        VM{
            registers: [0;32],
            program: vec![],
            pc: 0,
            remainder: 0
        }
    }
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }
    fn next8bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next16bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }

    fn execute_instruction(&mut self)-> bool{
            if self.pc >= self.program.len(){
               return false;
            }
            match self.decode_opcode(){
                Opcode::HLT => {
                    println!("HLT Encountered");
                    return false;
                },
                Opcode::LOAD => {
                    let register = self.next8bits() as usize;
                    let number = self.next16bits() as u16;
                    self.registers[register] = number as i32;
                    // continue;

                },
                Opcode::ADD => {
                    let register1 = self.registers[self.next8bits() as usize];
                    let register2 = self.registers[self.next8bits() as usize];
                    self.registers[self.next8bits() as usize] = register1 + register2;

                },
                Opcode::SUB => {
                    let register1 = self.registers[self.next8bits() as usize];
                    let register2 = self.registers[self.next8bits() as usize];
                    self.registers[self.next8bits() as usize] = register1 - register2;

                },
                Opcode::MUL => {
                    let register1 = self.registers[self.next8bits() as usize];
                    let register2 = self.registers[self.next8bits() as usize];
                    self.registers[self.next8bits() as usize] = register1 * register2;

                },
                Opcode::DIV => {
                    let register1 = self.registers[self.next8bits() as usize];
                    let register2 = self.registers[self.next8bits() as usize];
                    self.registers[self.next8bits() as usize] = register1 / register2;
                    self.remainder = (register1 % register2) as u32;
                },

                _ => {
                    println!("unrecognized opcode, terminating");
                    
                    return false;
                }
            }
            true

    }
    pub fn run_once(&mut self){
        self.execute_instruction();
    }

    pub fn run(&mut self){
        let mut done = false;
        while !done{
           done = self.execute_instruction();
            
        }
            // loop{
            // if self.pc >= self.program.len(){
                // break;
            // }
            // match self.decode_opcode(){
                // Opcode::HLT => {
                    // println!("HLT Encountered");
                    // return;
                // },
                // Opcode::LOAD => {
                    // let register = self.next8bits() as usize;
                    // let number = self.next16bits() as u16;
                    // self.registers[register] = number as i32;
                    // continue;
                // },
                // _ => {
                    // println!("unrecognized opcode, terminating");
                    // return;
                // }
            // }
        // }
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
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl(){
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 200, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }
    
    #[test]
    fn test_opcode_load(){
        let mut test_vm = VM::new();
        let test_bytes = vec![1, 0, 1, 244];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 500);
    }
}
