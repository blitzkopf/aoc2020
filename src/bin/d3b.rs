use structopt::StructOpt;
use aoc2020::toboggan_trajectory::Treecounter;
use aoc2020::read_lines;


// Read some lines of a file
#[derive(StructOpt)]
struct Cli {
    /// Input file to read
    file: String,
}
fn do_run(file:&String,hstep:usize, vstep:usize)->i64 {
    let mut counter = Treecounter::new(hstep,vstep);
    if let Ok(lines) = read_lines(file) {

        for line in lines {
            if let Ok(rline) = line {
                counter.next_line(&rline);

            }
        }
    }
    counter.tree_count
}
fn main()   {

    let args = Cli::from_args();
    let c1 = do_run(&args.file,1,1);
    let c2 = do_run(&args.file,3,1);
    let c3 = do_run(&args.file,5,1);
    let c4 = do_run(&args.file,7,1);
    let c5 = do_run(&args.file,1,2);

    println!("c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, ",c1,c2,c3,c4,c5);
    println!("multi :{}",c1*c2*c3*c4*c5);


}