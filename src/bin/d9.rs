use clap::{Arg, App};
use aoc2020::encoding_errors::Xmas;
use aoc2020::string_to_uintvec;



fn main() {
    let matches = App::new("Day 9").version("1.0")
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
    let mut xmas = Xmas::new(25);
    if let Err(msg) = xmas.add_numbers(nvec) {
        println!("Error msg {}",msg);
    }
    let nvec2 = string_to_uintvec(&content,"\n");
    if let Ok((a,b)) =  Xmas::find_contiguous_sum(144381670,nvec2) {
        println!("found series {} +{} = {}",a,b,a+b);
    }
}