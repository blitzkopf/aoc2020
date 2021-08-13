use clap::{Arg, App};
use aoc2020::shuttle_schedule::{first_bus,find_first_sequential_time};

/*
1002632
23,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,829,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,677,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19
*/ 

fn main() {
    let earliest = 1002632;
    let (bus,time) = first_bus(earliest,vec!(23,41,829,13,17,29,677,37,19));
    println!("Bus:{} time{}, result {}",bus,time,bus*(time-earliest));
   
    let second_time = find_first_sequential_time(vec![(23,0),(41,-13),(829,-23),(13,-36),(17,-37),(29,-52),(677,-54),(37,-60),(19,-73)]);
    println!("timestamp:{}",second_time);

}