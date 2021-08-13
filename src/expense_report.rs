use super::string_to_intvec;

pub struct ExpRep {
    expenses: Vec<i64>
}

impl ExpRep {
    pub fn load_expenses (string:&str) -> ExpRep {
        ExpRep {  
            expenses:string_to_intvec(string,"\n")
         }       
    }
    pub fn find_pair(&mut self, target: i64 ) -> (i64, i64) {
        for i in 0..self.expenses.len()-1 {
            let v1 = self.expenses[i];
            for j in i+1 .. self.expenses.len() {
                let v2 = self.expenses[j];
                if v1 + v2 == target {
                    return (v1,v2)
                }
            }
        }
        (0,0)
    }
    pub fn find_triple(&mut self, target: i64 ) -> (i64, i64, i64) {
        for i in 0..self.expenses.len()-2 {
            let v1 = self.expenses[i];
            for j in i+1 .. self.expenses.len()-1 {
                let v2 = self.expenses[j];
                let s1 = v1+v2;
                if s1 < 2020 {
                    for k in j+1 .. self.expenses.len() {
                        let v3 = self.expenses[k];
                        if s1 + v3 == target {
                            return (v1,v2,v3)
                        }
                    }
                                    
                }
            }
        }
        (0,0,0)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_find_pair() {
        let mut exp  = ExpRep::load_expenses("1721
        979
        366
        299
        675
        1456");
        assert_eq!(exp.find_pair(2020),( 1721,299));
    }
    #[test]
    fn test_find_triple() {
        let mut exp  = ExpRep::load_expenses("1721
        979
        366
        299
        675
        1456");
        assert_eq!(exp.find_triple(2020),( 979, 366,675));
    }
}