use regex::Regex;
use std::collections::HashMap;

pub struct Processor {
}
impl Processor {
    pub fn new()-> Processor {
        Processor {}
    }


    pub fn validate_passport(&self,input:&str) -> (bool,bool) {
        lazy_static! {
            /* ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                byr:1937 iyr:2017 cid:147 hgt:183cm */
            static ref SPLIT_RE: Regex = Regex::new(r"([[:alpha:]]{3}):(\S+)").unwrap();
        }
        lazy_static! {
                    static ref  REQUIRED:[String;7] = ["byr".to_string(),"iyr".to_string(),"eyr".to_string(),"hgt".to_string(), "hcl".to_string(), "ecl".to_string(), "pid".to_string() /*, "cid" */ ];
                    //static ref  REQUIRED:[str;7] = ["byr", "iyr", "eyr","hgt", "hcl", "ecl", "pid" /*, "cid" */ ];
        }
        let mut fields = HashMap::new();

        for cap in SPLIT_RE.captures_iter(input) {
            //println!("field: {} value: {}", &cap[1], &cap[2]);
            fields.insert(cap[1].to_string(), cap[2].to_string());
        }
        let mut fields_valid = true;
        for i in 0.. REQUIRED.len() {
            let field = &REQUIRED[i];
            if !fields.contains_key(&REQUIRED[i]) {
                return (false,false);
            } 
            let value = fields.get(field);
            match &field[0..] {
                "byr" => {fields_valid &=  Processor::validate_byr(value.unwrap())},
                "iyr" => {fields_valid &=  Processor::validate_iyr(value.unwrap())},
                "eyr" => {fields_valid &=  Processor::validate_eyr(value.unwrap())},
                "hgt" => {fields_valid &=  Processor::validate_hgt(value.unwrap())},
                "hcl" => {fields_valid &=  Processor::validate_hcl(value.unwrap())},
                "ecl" => {fields_valid &=  Processor::validate_ecl(value.unwrap())},
                "pid" => {fields_valid &=  Processor::validate_pid(value.unwrap())},
                "cid" => {},
                _ => {}

            }
        }
        (true,fields_valid)
    }

    pub fn validate_batch(&self,input:&str) -> (i32,i32) {
        let mut res:i32 = 0;
        let mut resb:i32 = 0;
        
        /*let mut passport = "";
        for line in input.lines() {
            println!("line <{}>",line);
            if line == "" {
                println!("passport <{}>",passport);
                if processor.validate_passport(passport) {
                    res +=1;
                }
                passport = "";
            }

        }
        if processor.validate_passport(passport) {
            res +=1;
        }*/

        for passport in input.split("\n\n") {
            println!("passport <{}>",passport);
            let (v1,v2) =  self.validate_passport(passport);
            if v1 {
                res +=1;
            }
            if v2 {
                resb +=1;
            }
        }
        (res,resb)
    }

    fn validate_number(inp:&str,min:i32,max:i32) -> bool {

        
        if let Ok(numb)  = inp.parse::<i32>() {
            if numb>= min && numb <= max {
                return true;
            }
        }
        false
    }

    
    fn validate_byr(inp:&str)  -> bool {
        Processor::validate_number(inp,1920,2002) 
    }
    fn validate_iyr(inp:&str)  -> bool {
        Processor::validate_number(inp,2010,2020) 
    }
    fn validate_eyr(inp:&str)  -> bool {
        Processor::validate_number(inp,2020,2030) 
    }
    fn validate_hgt(inp:&str)  -> bool {
        lazy_static! {
            static ref RE_HGT: Regex = Regex::new(r"^(\d{2,3})(cm|in)").unwrap();
        }
        if  let Some(caps) = RE_HGT.captures(inp) {
            let hgt = caps[1].parse::<u32>().unwrap();
            if &caps[2] == "cm" {
                if hgt >= 150 && hgt <= 193 {
                    return true
                }
            } else if &caps[2] == "in" {
                if hgt >= 59 && hgt <= 76 {
                    return true
                }
            }
        }
        false
    }

    fn validate_hcl(inp:&str)  -> bool {
        lazy_static! {
            static ref RE_HCL: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        }

        RE_HCL.is_match(inp)
    }

    fn validate_ecl(inp:&str)  -> bool {
        lazy_static! {
            static ref RE_ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        }

        RE_ECL.is_match(inp)
    }

    fn validate_pid(inp:&str)  -> bool {
        lazy_static! {
            static ref RE_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        RE_PID.is_match(inp)
    }


}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_validate_passport() {
        let   processor = Processor::new();

        assert_eq!(processor.validate_passport("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm"),(true,true));
        assert_eq!(processor.validate_passport("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929"),(false,false));
        assert_eq!(processor.validate_passport("hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm"),(true,true));
        assert_eq!(processor.validate_passport("hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"),(false,false));

    }
    #[test]
    fn test_validate_batch() {
        let processor = Processor::new();

        assert_eq!(processor.validate_batch("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"),(2,2));
    }       
    #[test]
    fn test_validate_byr() {
        assert_eq!(Processor::validate_byr("2000"),true);
        assert_eq!(Processor::validate_byr("2020"),false);


    }
    #[test]
    fn test_validate_hgt() {
        assert_eq!(Processor::validate_hgt("180cm"),true);
        assert_eq!(Processor::validate_hgt("200cm"),false);
        assert_eq!(Processor::validate_hgt("60in"),true);
        assert_eq!(Processor::validate_hgt("0in"),false);
        assert_eq!(Processor::validate_hgt("180"),false);
        assert_eq!(Processor::validate_hgt("cm"),false);
    }

    #[test]
    fn test_validate_hcl() {
        assert_eq!(Processor::validate_hcl("#cc7788"),true);
        assert_eq!(Processor::validate_hcl("#aaaaaa"),true);
        assert_eq!(Processor::validate_hcl("1236548"),false);
        assert_eq!(Processor::validate_hcl("12#001100"),false);
        assert_eq!(Processor::validate_hcl("180"),false);
        assert_eq!(Processor::validate_hcl("cm"),false);
    }
}