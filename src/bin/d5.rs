use structopt::StructOpt;
use aoc2020::binary_boarding::bpcode_to_int;
use aoc2020::read_lines;


// Read some lines of a file
#[derive(StructOpt)]
struct Cli {
    /// Input file to read
    file: String,
}
fn main()   {
    let args = Cli::from_args();

    let mut max:u16 = 0;
    let mut vec = Vec::new();

    if let Ok(lines) = read_lines(&args.file) {

        for line in lines {
            if let Ok(rline) = line {
                let val=bpcode_to_int(rline);
                vec.push(val);
                if val > max {
                    max=val
                }

            }
        }
    }
    println!("Max: {}",max);
    vec.sort();
    let mut last =vec[0];
    for elem in vec.iter() {
        if *elem == last+2 {
            println!("single spot {}",last+1)
        }
        last = *elem;
    } 
}

