use std::usize;

use super::instruction::Opcode;
pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>, //vector of bytes
    remainder: u32,
    equal_flag: bool, //flag to check if two values are equal
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
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

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT Encountered");
                return false;
            }
            Opcode::LOAD => {
                let register = self.next8bits() as usize;
                let number = self.next16bits() as u16;
                self.registers[register] = number as i32;
                // continue;
            }
            Opcode::ADD => {
                let register1 = self.next8bits() as usize;
                let register2 = self.next8bits() as usize;
                let result_register = self.next8bits() as usize;
                self.registers[result_register] =
                    self.registers[register1] + self.registers[register2];
            }
            Opcode::SUB => {
                let register1 = self.next8bits() as usize;
                let register2 = self.next8bits() as usize;
                let result_register = self.next8bits() as usize;
                self.registers[result_register] =
                    self.registers[register1] - self.registers[register2];
            }
            Opcode::MUL => {
                let register1 = self.next8bits() as usize;
                let register2 = self.next8bits() as usize;
                let result_register = self.next8bits() as usize;
                self.registers[result_register] =
                    self.registers[register1] * self.registers[register2];
            }
            Opcode::DIV => {
                let register1 = self.next8bits() as usize;
                let register2 = self.next8bits() as usize;
                let result_register = self.next8bits() as usize;
                if self.registers[register2] == 0 {
                    println!("Error: Division by zero");
                    return false;
                }
                self.registers[result_register] =
                    self.registers[register1] / self.registers[register2];
                self.remainder = (self.registers[register1] % self.registers[register2]) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next8bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next8bits() as usize];
                self.pc += value as usize;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next8bits() as usize];
                self.pc -= value as usize;
            }
            Opcode::EQ => {
                let v1 = self.registers[self.next8bits() as usize];
                let v2 = self.registers[self.next8bits() as usize];
                self.equal_flag = v1 == v2;
                self.next8bits();
            }

            _ => {
                println!("unrecognized opcode, terminating");

                return false;
            }
        }
        true
    }
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            done = self.execute_instruction();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    pub fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        // test_vm.float_registers[0] = 5.0;
        // test_vm.float_registers[1] = 10.0;
        test_vm
    }
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
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 200, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        let test_bytes = vec![1, 0, 1, 244];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 500);
    }
    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 20;
        test_vm.program = vec![2, 0, 1, 2, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 30);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![3, 0, 1, 2, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 10);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 20;
        test_vm.program = vec![4, 0, 1, 2, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 200);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![5, 0, 1, 2, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 2);
    }

    #[test]
    fn test_div_opcode_remainder() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 3;
        test_vm.program = vec![5, 0, 1, 2, 0];
        test_vm.run_once();
        assert_eq!(test_vm.remainder, 2);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![8, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }
}
