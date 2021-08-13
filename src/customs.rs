use array_tool::vec::Union;
use array_tool::vec::Intersect;



pub fn process_group(input:&str) -> (usize,usize) {
    //let char_vec: Vec<char> = input.chars().filter(|c| *c == '\n').collect::<char>().sort().dedup();
    let mut any_vec:Vec<char> = Vec::new();
    let mut every_vec: Option<Vec<char>> = None;

    for answers in input.split("\n") {
        println!("Answers <{}>",answers);
        if answers.len()>0 {

            let ans_vec: Vec<char> = answers.chars().collect();
            let ans_vec2: Vec<char> = answers.chars().collect();
            
            any_vec  = any_vec.union(ans_vec);
            if let Some(ev) = every_vec {
                //println!("Answers some <{}>",answers);
                every_vec = Some(ev.intersect(ans_vec2));
            } else {
                //println!("Answers none <{}>",answers);

                every_vec = Some(ans_vec2);
            }
        }
    }

/*    let mut char_vec: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
    char_vec.sort();
    char_vec.dedup();
*/    
    (any_vec.len(),every_vec.unwrap().len())

}
pub fn process_batch(input:&str) -> (usize,usize) {
    let mut res:(usize,usize) = (0,0);
    
    for group in input.split("\n\n") {
        let yeses =  process_group(group);
        println!("group: <{}> :{} {}",group,yeses.0,yeses.1);
        res.0 += yeses.0;
        res.1 += yeses.1;
    }
    res
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_process_group() {
        assert_eq!(process_group("abc"),(3,3));
        assert_eq!(process_group("a
b
c"),(3,0));
        assert_eq!(process_group("ab
ac"),(3,1));
        assert_eq!(process_group("a
a
a
a"),(1,1));
        assert_eq!(process_group("b"),(1,1));
    }
    #[test]
    fn test_process_batch() {
        assert_eq!(process_batch("abc

a
b
c

ab
ac

a
a
a
a

b
"),(11,6));
    }   
}