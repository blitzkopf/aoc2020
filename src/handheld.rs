#[derive(Debug,PartialEq,Clone,Copy)]
pub enum Operation {
    Acc,
    Jmp,
    Nop
}
#[derive(Debug,PartialEq)]
pub struct Instruction {
    operation: Operation ,
    argument: i64, 
    executions: u64,
}

pub struct Handheld {
    accumulator: i64,
    pub boot_code: Vec<Instruction>,
    pc: usize,
}

#[derive(Debug,PartialEq)]
pub enum HaltingCondition {
    Finished(i64),
    EndlessLoop(i64),
}

impl Handheld {
    pub fn new(program: String)->Handheld {
        Handheld{accumulator:0, boot_code:Handheld::load_boot_code(program), pc:0 }
    }

    fn load_boot_code(program:String) -> Vec<Instruction>{
        let mut boot_code:Vec<Instruction> = Vec::new();
        for line in program.lines() {
           if let Ok(inst) = Handheld::parse_instruction(line) {
               boot_code.push(inst)
           } 
        }
        boot_code
    }
    fn parse_instruction(line :&str) -> Result<Instruction,String>{
        let opstring = &line[0..3];
        let argument = &line[4..].parse::<i64>().unwrap();
        let opcode = match opstring {
            "acc"=> Ok(Operation::Acc),
            "jmp"=> Ok(Operation::Jmp),
            "nop"=> Ok(Operation::Nop),
            &_ => Err(format!("Unknown Opcode {}",opstring))

        };
        match opcode {
            Ok(operation) => Ok(Instruction{operation: operation, argument: *argument, executions:0}),
            Err(e) => Err(e),        
        }
    }

    pub fn step(&mut self) {
        if let  Some(inst) =  self.boot_code.get_mut(self.pc) {
            inst.executions += 1;
            let new_pc = match inst.operation {
                Operation::Acc => { self.accumulator += inst.argument; 1}
                Operation::Jmp => { inst.argument  }
                Operation::Nop => {1}

            } + self.pc as i64;
            self.pc = new_pc as usize;
        }
    }
    pub fn run(&mut self)->  HaltingCondition {
        let size = self.boot_code.len();
        while self.pc < size {
            if self.boot_code[self.pc].executions > 0 {
                return HaltingCondition::EndlessLoop(self.accumulator)
            }

            self.step();
            //println!("PC:{} A:{}",self.pc,self.accumulator);
        }
        HaltingCondition::Finished(self.accumulator)
    }

    pub fn reset(&mut self) {
        self.accumulator=0;
        self.pc=0;
        for instr in self.boot_code.iter_mut() {
            instr.executions = 0;
        }        
    }
    pub fn search_for_error(&mut self) {
        for i in 0..self.boot_code.len() {
            self.reset();
            if let  Some(instr) = self.boot_code.get_mut(i) {
                let prev_op = instr.operation;
                match prev_op {
                    Operation::Jmp => { instr.operation = Operation::Nop},
                    Operation::Nop => { instr.operation = Operation::Jmp},
                    Operation::Acc => {},
                }
                if prev_op != instr.operation {
                    if let HaltingCondition::Finished(a) = self.run() {
                        println!("Finished: {}",    a);
                        return
                    }
                }
                if let  Some(rinstr) = self.boot_code.get_mut(i) {
                    rinstr.operation = prev_op;
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const TEST_STR:&str=
"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    #[test]
    fn test_parse_instruction() {
        assert_eq!(Handheld::parse_instruction("nop +0").unwrap(),Instruction{operation:Operation::Nop,argument:0, executions:0});
        assert_eq!(Handheld::parse_instruction("acc +1").unwrap(),Instruction{operation:Operation::Acc,argument:1, executions:0});
        assert_eq!(Handheld::parse_instruction("jmp -3").unwrap(),Instruction{operation:Operation::Jmp,argument:-3, executions:0});

    }
    #[test]
    fn test_load_boot_code() {
        assert_eq!(Handheld::load_boot_code(TEST_STR.to_string()),vec![Instruction{operation:Operation::Nop,argument:0, executions:0},
                Instruction{operation:Operation::Acc,argument:1, executions:0},
                Instruction{operation:Operation::Jmp,argument:4, executions:0},
                Instruction{operation:Operation::Acc,argument:3, executions:0},
                Instruction{operation:Operation::Jmp,argument:-3, executions:0},
                Instruction{operation:Operation::Acc,argument:-99, executions:0},
                Instruction{operation:Operation::Acc,argument:1, executions:0},
                Instruction{operation:Operation::Jmp,argument:-4, executions:0},
                Instruction{operation:Operation::Acc,argument:6, executions:0},
                
                ]);
    }
    #[test]
    fn test_step() {
        let mut hh = Handheld::new(TEST_STR.to_string());
        assert_eq!(hh.boot_code[0].executions , 0);
        hh.step();
        assert_eq!(hh.boot_code[0].executions , 1);
    }
    #[test]
    fn test_run_until_repeat() {
        let mut hh = Handheld::new(TEST_STR.to_string());
        assert_eq!(hh.run() , HaltingCondition::EndlessLoop(5));
    }

}

