use structopt::StructOpt;
use aoc2020::handy_haversack::LuggageProcessing;

// Read some lines of a file
#[derive(StructOpt)]
struct Cli {
    /// Input file to read
    file: String,
}

fn main()   {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.file)
        .expect("could not read file");

    let mut lp = LuggageProcessing::new();
    lp.read_rule_book(&content);
    let list = lp.who_can_contain(&"shiny gold".to_string());
    let resb = lp.how_many_does_it_contain(&"shiny gold".to_string());
    println!("count:{} ", list.len());
    println!("count b :{} ", resb );

}