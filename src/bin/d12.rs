use clap::{Arg, App};
use aoc2020::rain_risk::{Ship,Ship2};
use aoc2020::string_to_uintvec;



fn main() {
    let matches = App::new("Day 12").version("1.0")
        .author("Yngvi <blitzkopf@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("FILE")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();
    let content = std::fs::read_to_string(matches.value_of("FILE").unwrap())
        .expect("could not read file");

        let mut ship = Ship::new();
        ship.navigate(&content).unwrap();
        println!("Manhattan distance: {} ",ship.manhattan_dist());

        let mut ship2 = Ship2::new();
        ship2.navigate(&content).unwrap();
        println!("Manhattan distance: {} ",ship2.manhattan_dist());

}