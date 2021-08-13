use regex::Regex;
use std::collections::HashMap;
use array_tool::vec::Union;
//use array_tool::vec::Intersect;

#[derive(Debug, Clone)]
pub struct CItem {
    bag:String,
    count:usize
} 
impl PartialEq for CItem {
    fn eq(&self, other: &Self) -> bool {
        self.bag == other.bag &&
        self.count == other.count
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    bag:String,
    contents: Option<Vec<CItem>>
}
impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.bag == other.bag &&
        self.contents == other.contents
    }
}

pub struct LuggageProcessing {
    rules:Vec<Rule>,
    can_contain: HashMap<String,Vec<CItem>>,
    can_be_in: HashMap<String,Vec<String>>
}
impl LuggageProcessing {
    pub fn new() -> LuggageProcessing {
        LuggageProcessing {
            rules:Vec::new(),
            can_contain: HashMap::new(),
            can_be_in: HashMap::new()
        }
    }

    pub fn read_rule(&mut self,inp:&str) {
        let  rule = parse_bag_rule(inp);
        //self.rules.push(rule);
        if let Some(cont) = rule.contents.clone() {
            self.can_contain.insert(rule.bag.clone(), cont.clone());
        }
        if let Some(cont) = rule.contents {
            for content in cont {
            /*let mut iter = rule.contents.iter();
            while let Some(content) = iter.next() {*/
                if let  Some(  cbi_vec) = self.can_be_in.get_mut(&content.bag) {
                    cbi_vec.push(rule.bag.clone());
                } else {
                    let cbi_vec = vec![rule.bag.clone()];
                    self.can_be_in.insert(content.bag.clone(),cbi_vec);
                }
            }
        } 
    }
    pub fn read_rule_book(&mut self,inp:&str) {
        for line in inp.lines() {
            self.read_rule(line);
        }
    }

    pub fn who_can_contain(&self ,bag:&String) -> Vec<String> {
        let mut res:Vec<String> = Vec::new();
        //let mut iter = self.can_be_in.get(bag).iter() ;
        //while let  Some(cb) = iter.next() {
        if let Some(cbi) = self.can_be_in.get(bag) {
            for cb in cbi {
                println!("CB: {}",cb);
                if ! res.contains(cb) {
                    res.push(cb.to_string());
                    res = res.union(self.who_can_contain(cb))
                }
            }
        }
        res
    }
    pub fn how_many_does_it_contain(&self ,bag:&String) -> usize {
        let mut res:usize =0;
        if let Some(cc) = self.can_contain.get(bag) {
            for ci in cc {
                println!("Ci: {} {}",ci.bag,ci.count    );
                res += ci.count;
                res += ci.count*self.how_many_does_it_contain(&ci.bag);
            }
        }
        res
    }
}

pub fn parse_bag_rule(inp:&str) -> Rule
{
    let split:Vec<&str> = inp.split(" bags contain ").collect();
    //let (bag:&str,contents:&str) = (split[0],split[1]);
    let bag:&str = split[0];
    let contents:&str = split[1];
    if contents == "no other bags." {
        return Rule {bag:bag.to_string(), contents:None  };
    } else {
        println!("contents: {} ", contents);
    }   
    lazy_static! {
        // 3 faded blue bags, 4 dotted black bags.
        static ref ITEM_RE:Regex = Regex::new(r"(\d+) (\S+ \S+) bags?").unwrap();
    }
    let mut conts:Vec<CItem> = Vec::new();
    for cap in ITEM_RE.captures_iter(contents) {
        conts.push( CItem{bag:cap[2].to_string(),count:cap[1].parse::<usize>().unwrap()})
    }
    Rule {bag:bag.to_string(), contents:Some(conts) }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_parse_bag_rule() {
        assert_eq!(parse_bag_rule("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            Rule {bag:"light red".to_string(),
                contents:Some(vec![
                    CItem{bag:"bright white".to_string(),count:1},
                    CItem{bag:"muted yellow".to_string(),count:2},
                    ])});

        assert_eq!(parse_bag_rule("faded blue bags contain no other bags."),
            Rule {bag:"faded blue".to_string(),contents:None});
    }

    #[test]
    fn test_who_can_contain() {
        let mut lp = LuggageProcessing::new();
        lp.read_rule_book("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.");
        assert_eq!(lp.who_can_contain(&"shiny gold".to_string()).len(),4);
    }
    #[test]
    fn test_how_many_does_it_contain() {
        let mut lp = LuggageProcessing::new();
        lp.read_rule_book("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.");
        assert_eq!(lp.how_many_does_it_contain(&"shiny gold".to_string()),32);

        lp.read_rule_book("shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.");
                assert_eq!(lp.how_many_does_it_contain(&"shiny gold".to_string()),126);
    }
}
            