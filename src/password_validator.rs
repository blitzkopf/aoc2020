use regex::Regex;

pub struct Validator 
{
}
impl Validator {
    pub fn new() -> Validator {
        Validator {
        }
    }

    pub fn validate_line(& self,line:&str ) -> bool {
        /*
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
        */            
        lazy_static! {
            static ref SPLIT_RE: Regex = Regex::new(r"((\d+)-(\d+) ([[:alpha:]])): ([[:alpha:]]+)").unwrap();
        }
        let caps = SPLIT_RE.captures(line).unwrap();
        let(_rule,min,max,letter,pwd) = (&caps[1],&caps[2],&caps[3],&caps[4],&caps[5]);
        /*if !self.rule_re.contains_key(&rule.to_string()) {
            let newre = Regex::new(&*format!("[{}]{{{},{}}}",letter,min,max)).unwrap();
            self.rule_re.insert(rule.to_string(),newre);
        }
        let re = self.rule_re.get(&rule.to_string()).unwrap();
        re.is_match(pwd)*/
        let c = pwd.matches(letter).count();
        //token.trim().parse::<i64>().unwrap()
        if c >= min.trim().parse::<usize>().unwrap() && c <= max.trim().parse::<usize>().unwrap()  {
            return true;
        } else  {
            return false;
        }




    }

    pub fn validate_line_b(& self,line:&str ) -> bool {
        lazy_static! {
            static ref SPLIT_RE: Regex = Regex::new(r"((\d+)-(\d+) ([[:alpha:]])): ([[:alpha:]]+)").unwrap();
        }
           
        let caps = SPLIT_RE.captures(line).unwrap();
        let(_rule,p1,p2,letter,pwd) = (&caps[1],&caps[2].parse::<usize>().unwrap() ,&caps[3].parse::<usize>().unwrap() ,&caps[4],&caps[5]);
        println!("{}={}={}",letter,pwd.get(*p1..p1+1).unwrap(),pwd.get(p2-1..*p2).unwrap());
        if ( pwd.get(p1-1..*p1).unwrap() == letter ) ^ (pwd.get(p2-1..*p2).unwrap() == letter )  {
            return true;
        } else  {
            return false;
        }

    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_validate_line() {
        let    validator = Validator::new();

        assert_eq!(validator.validate_line("1-3 a: abcde"),true);
        assert_eq!(validator.validate_line("1-3 b: cdefg"),false);
        assert_eq!(validator.validate_line("2-9 c: ccccccccc"),true);

    }
    #[test]
    fn test_validate_line_b() {
        let    validator = Validator::new();

        assert_eq!(validator.validate_line_b("1-3 a: abcde"),true);
        assert_eq!(validator.validate_line_b("1-3 b: cdefg"),false);
        assert_eq!(validator.validate_line_b("2-9 c: ccccccccc"),false);

    }
}