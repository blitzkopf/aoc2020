use clap::{Arg, App};
use aoc2020::adapter_array::get_combinations;
use aoc2020::string_to_uintvec;



fn main() {
    let matches = App::new("Day 10").version("1.0")
        .author("Yngvi <blitzkopf@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("FILE")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();
    let content = std::fs::read_to_string(matches.value_of("FILE").unwrap())
        .expect("could not read file");
    let nvec = string_to_uintvec(&content,"\n");
    let sol = get_combinations(nvec);
    println!("Combinations {} ",sol);
}