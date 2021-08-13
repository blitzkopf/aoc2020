pub struct Treecounter {
    hpos: usize,
    vpos: usize,
    hstep: usize,
    vstep: usize,
    pub tree_count: i64
}
impl Treecounter{
    pub fn new(hstep:usize,vstep:usize) -> Treecounter {
        Treecounter{hpos :0, vpos:0, vstep:vstep, hstep:hstep, tree_count:0}
    }
    pub fn next_line(&mut self,line:&str){
        if self.vpos % self.vstep == 0  {
            let pos = self.hpos%line.len();
            if &line[pos..pos+1]=="#" {
                self.tree_count += 1;
            }
            self.hpos += self.hstep;
        } else {
            println!("skippint");
        }
        self.vpos += 1;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const test_str:&str=
"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    fn do_run(hstep:usize, vstep:usize)->i64 {
        let mut counter = Treecounter::new(hstep,vstep);
        for line in test_str.split("\n") {
            counter.next_line(line);
        } 
        counter.tree_count
    } /*
    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2. 
    In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.   */

    #[test]
    fn test_tree_counting() {
        assert_eq!(do_run(1,1),2);
        assert_eq!(do_run(3,1),7);
        assert_eq!(do_run(5,1),3);
        assert_eq!(do_run(7,1),4);
        assert_eq!(do_run(1,2),2);

        
    }
}