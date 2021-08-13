use clap::{Arg, App};
use aoc2020::seating_system::{Seating,Tile,Seat};
use aoc2020::string_to_uintvec;



fn main() {
    let matches = App::new("Day 11").version("1.0")
        .author("Yngvi <blitzkopf@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("FILE")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();
    let content = std::fs::read_to_string(matches.value_of("FILE").unwrap())
        .expect("could not read file");


        let mut seating = Seating::new(&content);
        let mut cnt = 0;
        while seating.round(4,1) {
            //println!("{:}",seating);
            cnt += 1;
        }
        println!("{:}",seating);

        println!("Occupied seats: {} ",seating.count(Tile::Seat(Seat::Occupied) ));

        let mut seating2 = Seating::new(&content);
        let mut cnt = 0;
        while seating2.round(5,99) {
            //println!("{:}",seating);
            cnt += 1;
        }
        println!("{:}",seating2);

        println!("Occupied seats: {} ",seating2.count(Tile::Seat(Seat::Occupied) ));

}