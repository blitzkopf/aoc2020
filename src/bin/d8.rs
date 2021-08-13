use clap::{Arg, App};
use aoc2020::handheld::{Handheld, HaltingCondition,Operation};


fn main() {
    let matches = App::new("Day 8").version("1.0")
    .author("Yngvi  <blitzkopf@gmail.com>")
    .about("Does awesome things")
    .arg(Arg::with_name("FILE")
    .help("Sets the input file to use")
    .required(true)
    .index(1))
        .get_matches();
    let content = std::fs::read_to_string(matches.value_of("FILE").unwrap())
        .expect("could not read file");
    let mut hh = Handheld::new(content.to_string());
    let result = hh.run();
    match result {
        HaltingCondition::EndlessLoop(a) =>      println!("EndlessLoop: {}", a),
        HaltingCondition::Finished(a)    =>      println!("Finished: {}",    a),
    }
    hh.search_for_error();

}