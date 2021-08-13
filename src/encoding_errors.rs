use std::collections::VecDeque;
use super::string_to_uintvec;


pub struct Xmas {
    preample_size: usize,
    buffer:VecDeque<u64>,

}
impl Xmas {
    pub fn new(preample_size:usize) -> Xmas {
        Xmas {preample_size:preample_size, buffer:VecDeque::with_capacity(preample_size)} 
    }

    pub fn add_number(&mut self, number:u64 ) -> Result<(u64,u64),String> {
        if self.buffer.len() < self.preample_size {
            self.buffer.push_back(number);
            return Ok((0,0))
        } 

        let mut a:u64 = 0;
        let mut b:u64 = 0;
        for n in self.buffer.iter() {
            println!("checking for {} {}",n,number);
            if number != 2*n && *n < number && self.buffer.contains(&(number-n)) {
                
                a= *n;
                b= number - a;
                break;
            }
        }
        println!("found for {} {}",a,b);
        if a+b == number {
            self.buffer.pop_front();
            self.buffer.push_back(number);
            Ok((a,b))
        } else {
            Err(format!("Unable to get {}",number))
        }

    }
    pub fn add_numbers(&mut self, numbers:Vec<u64>) -> Result<(),String>
    {
        for number in numbers.iter() {
            if let Err(x) = self.add_number(*number) {
                return Err(x)
            }
        }
        Ok(())
    }

    pub fn find_contiguous_sum(target:u64, numbers:Vec<u64>)->Result<(u64,u64),String> {
        let mut total:u64 =0;
        let mut list:VecDeque<u64> = VecDeque::with_capacity(500);

        let mut iter = numbers.iter();
        while true {
            if total == target {
                println!("{:?}",list);
                break;
               
            }
            else if total<target {
                if let Some(n) = iter.next() {
                    total += n;
                    list.push_back(*n);
                }
            } else {
                if let Some(n)  = list.pop_front(){
                    total -= n;
                }
            }

        }
        if total == target {
            let a = list.iter().min().unwrap();
            let b = list.iter().max().unwrap();
            return Ok((*a,* b))
        } else {
            Err("No contiguus field found!".to_string())
        }
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const TEST_STR:&str=
"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    #[test]
    fn test_add_number() {
        let mut xmas = Xmas::new(5);
        assert_eq!(xmas.add_number(35),Ok((0,0)));
        assert_eq!(xmas.add_number(20),Ok((0,0)));
        assert_eq!(xmas.add_number(15),Ok((0,0)));
        assert_eq!(xmas.add_number(25),Ok((0,0)));
        assert_eq!(xmas.add_number(47),Ok((0,0)));
        assert_eq!(xmas.add_number(40),Ok((15,25)));
        assert_eq!(xmas.add_number(62),Ok((15,47)));
        assert_eq!(xmas.add_number(55),Ok((15,40)));
        xmas.add_number(65);
        xmas.add_number(95);
        xmas.add_number(102);
        xmas.add_number(117);
        xmas.add_number(150);
        xmas.add_number(182);
        assert_eq!(xmas.add_number(127),Err("Unable to get 127".to_string()));

    }

    #[test]
    fn test_add_numbers() {
        let nvec = string_to_uintvec(TEST_STR,"\n");
        let mut xmas = Xmas::new(5);
        assert_eq!(xmas.add_numbers(nvec),Err("Unable to get 127".to_string()));
    }

    #[test]
    fn test_find_contiguous_sum() {
        let nvec = string_to_uintvec(TEST_STR,"\n");
        let mut xmas = Xmas::new(5);
        assert_eq!(Xmas::find_contiguous_sum(127,nvec),Ok((15,47)));
    }
}
