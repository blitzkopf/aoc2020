use structopt::StructOpt;
use aoc2020::expense_report::ExpRep;

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
    let mut exprep = ExpRep::load_expenses(&content);
    let (v1,v2) = exprep.find_pair(2020);
    println!("{},{},{}",v1,v2,v1*v2);
    let (t1,t2,t3) = exprep.find_triple(2020);
    println!("{},{},{},{}",t1,t2,t3,t1*t2*t3);
}