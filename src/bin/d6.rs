use structopt::StructOpt;
use aoc2020::customs::process_batch;

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

    let yesses = process_batch(&content);
    println!("Valid: {} {}",yesses.0,yesses.1);
}