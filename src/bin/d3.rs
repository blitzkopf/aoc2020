use structopt::StructOpt;
use aoc2020::toboggan_trajectory::Treecounter;
use aoc2020::read_lines;


// Read some lines of a file
#[derive(StructOpt)]
struct Cli {
    /// Input file to read
    file: String,
}

fn main()   {

    let args = Cli::from_args();
    let mut counter = Treecounter::new(3,1);
    if let Ok(lines) = read_lines(&args.file) {

        for line in lines {
            if let Ok(rline) = line {
                counter.next_line(&rline);

            }
        }
    }
    println!("Trees :{}",counter.tree_count);

}