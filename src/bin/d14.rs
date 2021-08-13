use clap::{Arg, App};
use aoc2020::docking_data::{CPU};
use aoc2020::string_to_uintvec;



fn main() {
    let matches = App::new("Day 14").version("1.0")
        .author("Yngvi <blitzkopf@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("FILE")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();
    let content = std::fs::read_to_string(matches.value_of("FILE").unwrap())
        .expect("could not read file");
    
    let mut cpu = CPU::new();

    cpu.run_code(content);
    println!("Samtals: {} ",cpu.sum());

}