use structopt::StructOpt;
use aoc2020::password_validator::Validator;
use aoc2020::read_lines;


// Read some lines of a file
#[derive(StructOpt)]
struct Cli {
    /// Input file to read
    file: String,
}

fn main()   {

    let args = Cli::from_args();
    let mut valids_a:i32 =0;
    let mut valids_b:i32 =0;

    let validator = Validator::new();
    if let Ok(lines) = read_lines(&args.file) {

        for line in lines {
            if let Ok(rline) = line {
                if validator.validate_line(&rline) {
                    valids_a +=1;
                }
                if validator.validate_line_b(&rline) {
                    valids_b +=1;
                }

            }

        }
    }
    println!("Valids a :{}",valids_a);
    println!("Valids b:{}",valids_b);

}