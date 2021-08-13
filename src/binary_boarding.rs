pub fn bpcode_to_int(s:String)->u16 {
    let bin_string:String = s.chars().map(|x| match x { 
        '!' => '?', 
        'B'|'R' => '1', 
        'F'|'L' => '0',
        _ => x
    }).collect();
    u16::from_str_radix(&bin_string, 2).unwrap()
    
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_bpcode_to_int() {
        assert_eq!(bpcode_to_int("BFFFBBFRRR".to_string()),567)
    }
}