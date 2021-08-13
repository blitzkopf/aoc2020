use structopt::StructOpt;
use aoc2020::passport_processing::Processor;

// Read some lines of a file
#[derive(StructOpt)]
struct Cli {
    /// Input file to read
    file: String,
}

fn main()   {
    let  processor = Processor::new();

    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.file)
        .expect("could not read file");

    let valids = processor.validate_batch(&content);
    println!("Valid: {} {}",valids.0,valids.1);
}