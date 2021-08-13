pub fn count_ways(target:u64,current:u64)->u64 {
    if current == target {
         1
    } else if current > target {
         0
    } else {
        let mut res:u64=0;
        for n in 1..4 {
            res += count_ways(target,current+n);
        }
        println!("Res:{}",res);
        res
    }
}

pub fn get_combinations(mut adapters:Vec<u64>) -> u64 {
    adapters.sort();
    let mut last:u64 =0;
    let mut cnt1:u64 =0;
    let mut p:u64 = 1;
    for adapter in adapters {
        println!("{} - {} = {}",adapter,last,adapter - last);
        match adapter - last {
            1 => {cnt1 += 1},
            3 => { p *= count_ways(cnt1+1,1); cnt1=0 },
            _ => { println!("something happened")}
        }
        last = adapter;
    }
    p *= count_ways(cnt1+1,1);
    p

}

#[cfg(test)]
use super::string_to_uintvec;

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const TEST_STR:&str=  "16
10
15
5
1
11
7
19
6
12
4";
    const TEST_STR2:&str=  
"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    #[test]
    fn test_count_ways() {
        assert_eq!(count_ways(1,1),1); 
        assert_eq!(count_ways(2,1),1);
        assert_eq!(count_ways(3,1),2);
        assert_eq!(count_ways(4,1),4);
        assert_eq!(count_ways(5,1),7);
        assert_eq!(count_ways(6,1),13);
        assert_eq!(count_ways(7,1),24);
    }
    #[test]
    fn test_get_combinations() {
        let nvec = string_to_uintvec(TEST_STR,"\n");
        assert_eq!(get_combinations(nvec),8);
    }     
    #[test]
    fn test_get_combinations2() {
        let nvec = string_to_uintvec(TEST_STR2,"\n");
        assert_eq!(get_combinations(nvec),19208);
    }        
    
}