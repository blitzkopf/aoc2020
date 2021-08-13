use regex::Regex;
use std::collections::HashMap;

pub struct CPU {
    memory:HashMap<usize, u64>,
    ormask:u64,
    andmask:u64
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: HashMap::new(),
            ormask:0,
            andmask: 0b1_11111_11111_11111_11111_11111_11111_11111u64,
        }
    }
    pub fn parse_line(&mut self, line :&str) {
        /*mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0 
*/
        lazy_static! {
            static ref  MASK_RE:Regex =Regex::new(r"mask = ([01X]{36})").unwrap();
            static ref MEM_RE:Regex =Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        }
        match MASK_RE.captures(line) {
            Some(c) => {
                self.set_mask(&c[1])
            },
            None => { 
                match MEM_RE.captures(line) {
                    Some(c2) => {
                        self.set_mem(c2[1].parse::<usize>().unwrap(),c2[2].parse::<u64>().unwrap())
                    },
                    None=> {println!("Unknown line {}",line)}

                }
            }
        }

    }
    pub fn set_mask(&mut self, mask :&str) {
        let ormask_str = mask.replace("X","0");
        self.ormask = u64::from_str_radix(&ormask_str,2).unwrap();
        let andmask_str = mask.replace("X","1");
        self.andmask = u64::from_str_radix(&andmask_str,2).unwrap();
    }

    pub fn set_mem(&mut self, address :usize,value:u64 ) {
        self.memory.insert(address, (value & self.andmask ) | self.ormask );
    }
    pub fn run_code(&mut self,program:String) {
        for line in program.lines() {
            self.parse_line(line);
        }
    }
    pub fn sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const TEST_STR:&str=
"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    #[test]
    fn test_set_mask() {
        let mut cpu = CPU::new();
        cpu.set_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(cpu.ormask,0b10_00000);
        assert_eq!(cpu.andmask,0b1_11111_11111_11111_11111_11111_11111_11101);
    }

    #[test]
    fn test_set_mem() {
        let mut cpu = CPU::new();
        cpu.set_mem(8,101);
        assert_eq!(cpu.memory[&8],101);
    
    }

    #[test]
    fn test_masked_set_mem() {
        let mut cpu = CPU::new();
        cpu.set_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        /*mem[8] = 11
        mem[7] = 101
        mem[8] = 0*/
        cpu.set_mem(8,11);
        assert_eq!(cpu.memory[&8],73);
        cpu.set_mem(7,101);
        cpu.set_mem(8,0);

        assert_eq!(cpu.memory[&7],101);
        assert_eq!(cpu.memory[&8],64);
    
    }

    #[test]
    fn test_run_code() {
        let mut cpu = CPU::new();

        cpu.run_code(TEST_STR.to_string());
        assert_eq!(cpu.sum(),165)
    
    }
}