
type Timestamp = i128;

pub fn first_bus(time:Timestamp,schedules:Vec<Timestamp>) -> (Timestamp,Timestamp) {
    let next_departure:Vec<(Timestamp,Timestamp)> = schedules
        .iter()
        .map(|bus| (*bus,((time / bus)+1)*bus ))
        .collect();
    let first:(Timestamp,Timestamp) = next_departure.iter()
        .fold((0,time*2),|first,departure| if departure.1 < first.1 {*departure} else { first });

    first
}

pub struct SchedIter {
    slope:Timestamp,
    intersect:Timestamp, 
    counter:Timestamp
}
impl SchedIter {
    fn new(slope:Timestamp,intersect:Timestamp) -> SchedIter {
        SchedIter { slope:slope,intersect:intersect,counter: 0 }
    }
    fn next_above(&mut self,at_least:Timestamp) -> Option<Timestamp> {
        self.counter += 1;
        self.counter=(at_least-self.intersect)/self.slope;
        if self.counter*self.slope+self.intersect < at_least {
            self.counter += 1
        }
        // Check to see if we've finished counting or not.
        Some(self.counter*self.slope+self.intersect)
    }
}

impl Iterator for SchedIter {
    type Item = Timestamp;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        // Check to see if we've finished counting or not.
        Some(self.counter*self.slope+self.intersect)
    }
}

pub fn combined(mut i1:SchedIter,mut i2:SchedIter)->(Timestamp,Timestamp){    
    let mut v1 = i1.next().unwrap();
    let mut v2 = i2.next().unwrap();
    let first_intersect:Timestamp;    
    let second_intersect:Timestamp;    

    loop {
        if v1 == v2 {
            first_intersect=v1;
            break;
        } else  if v1 < v2  {
            v1 = i1.next_above(v2).unwrap();
        } else {
            v2 = i2.next_above(v1).unwrap();
        }
        
    }
    v1 = i1.next().unwrap();
    v2 = i2.next().unwrap();
    loop {
        if v1 == v2 {
            second_intersect=v1;
            break;
        } else  if v1 < v2  {
            v1 = i1.next_above(v2).unwrap();
        } else {
            v2 = i2.next_above(v1).unwrap();
        }
        
    }
    println!("first {} second {}",first_intersect,second_intersect);
    let slope = second_intersect-first_intersect;
    let offset = first_intersect - slope;
    println!("slope  {} offset {}",slope,offset);
    
    (slope,offset) 
    //(first_intersect,second_intersect)
}

pub fn find_first_sequential_time(mut schedules:Vec<(Timestamp,Timestamp)>)-> Timestamp {
    let mut iter = schedules.iter();
    let mut sched1 = *iter.next().unwrap();
    let mut adder = sched1.1;
    while let Some(sched2) = iter.next() {
        let mut sp1 = SchedIter::new(sched1.0,sched1.1);
        adder += sched2.1;
        let mut sp2 = SchedIter::new(sched2.0,sched2.1);
        sched1 = combined(sp1,sp2);


    }
    println!("adder = {}",adder);
    //sched1.0+sched1.1-(adder)
    sched1.0+sched1.1
}  

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const TEST_STR:&str=  
"939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_action() {
        assert_eq!(first_bus(939,vec!(7,13,59,31,19)),(59,944));
    }

    #[test]
    fn test_sched_iter() {
        let mut sp = SchedIter::new(5,1);

        assert_eq!(sp.next().unwrap(),6);
        assert_eq!(sp.next().unwrap(),11);
    }

    #[test]
    fn test_combined() {
        let mut sp1 = SchedIter::new(5,0);
        let mut sp2 = SchedIter::new(7,-1);

        assert_eq!(combined(sp1,sp2),(35,-15));

    }
    #[test]
    fn test_find_first_sequential_time() {

        assert_eq!(find_first_sequential_time(vec![(5,0),(7,-1)]),20);
        //7,13,x,x,59,x,31,19
        //assert_eq!(find_first_sequential_time(vec![(7,0),(13,-1),(59,-4),(31,-6),(19,-7)]),1068781);
        //17,x,13,19 -> 3417
        assert_eq!(find_first_sequential_time(vec![(17,0),(13,-2),(19,-3)]),3417);
        //67,7,59,61 first occurs at timestamp 754018.
        assert_eq!(find_first_sequential_time(vec![(67,0),(7,-1),(59,-2),(61,-3)]),754018);
        //67,x,7,59,61 first occurs at timestamp 779210.
        assert_eq!(find_first_sequential_time(vec![(67,0),(7,-2),(59,-3),(61,-4)]),779210);
        //67,7,x,59,61 first occurs at timestamp 1261476.
        assert_eq!(find_first_sequential_time(vec![(67,0),(7,-1),(59,-3),(61,-4)]),1261476);
        //1789,37,47,1889 first occurs at timestamp 1202161486.

    }
}