use std::fmt;
use std::str::FromStr;
use std::error::Error;
use vector2d::Vector2D;



#[derive(Debug)]
pub struct MapParseError {
    details: String
}

impl MapParseError {
    fn new(msg: &str) -> MapParseError {
        MapParseError{details: msg.to_string()}
    }
}

impl fmt::Display for MapParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MapParseError {
    fn description(&self) -> &str {
        &self.details
    }
}
#[derive(Debug,PartialEq)]
pub enum Seat {
    Empty,
    Occupied
}
#[derive(Debug,PartialEq)]
pub enum Tile   {
    Floor,
    Seat(Seat)
}
impl FromStr for Tile {
    type Err = MapParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tile = match s {    
            "." => Tile::Floor,
            "L" => Tile::Seat(Seat::Empty)  ,
            "#" => Tile::Seat(Seat::Occupied)  ,

            _ =>  { return Err(MapParseError::new("Unknown symbol"));}
        };
        Ok(tile)
    }
}   
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Tile::Floor => ".",
            Tile::Seat(x) => match x { Seat::Empty => "L", Seat::Occupied => "#"}
            //Seat=> "L",

        })
    }
}
#[derive( Debug)]
pub struct Seating {
    floor_map:Vec<Vec<Tile>>,
}
impl FromStr for Seating {
    type Err = MapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fmap= Vec::new();
        for line in s.lines() {
            let lvec :Vec<Tile> = line.chars().map(|c| c.to_string().parse::<Tile>().unwrap()).collect();
            fmap.push(lvec);
        }
        Ok(Seating {
            floor_map:fmap,
        })
    }
}
impl fmt::Display for Seating{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for lvec in self.floor_map.iter() {
            for tile in lvec.iter() {
                f.write_str(&tile.to_string()) ;
            }
            f.write_str("\n");
        }
        Ok(())
    }
}
impl Seating {
    pub fn new(map:&String) -> Seating {
        map.parse::<Seating>().unwrap()
    } 
    pub fn find_seat_in_direction(&self,pos:Vector2D<i64>,direction:Vector2D<i64>,maxdist:usize) -> Option<&Seat> {
        let mypos = pos+direction;
        //println!("sinking in maxdist:{}",maxdist);
        if maxdist > 0 {
            if let Some(v) = &self.floor_map.get(mypos.x as usize) {
                if let Some(t) = v.get(mypos.y as usize){
                    match t {
                        Tile::Seat(s) => Some(s) ,
                        _ => self.find_seat_in_direction(mypos,direction,maxdist-1)
                    }
                } else {
                    None
                }
            } else {
                None
            }

        } else {
            None
        }
    }

    pub fn get_occupied_around(&self,row:usize,seat:usize,maxdist:usize) -> u8 {

        let pos:Vector2D<i64> = Vector2D::new(row as i64,seat as i64);

        let mut cnt=0;


        for direction in vec!(
            (-1,-1),(-1,0),(-1,1),
            (0,-1),(0,1),
            (1,-1),(1,0),(1,1),
        ) {
            if let Some(seat) = self.find_seat_in_direction(pos,Vector2D::new(direction.0,direction.1),maxdist) {
                if *seat == Seat::Occupied {
                    cnt += 1;
                }
            }

        }

       /* let rm1 = row.checked_sub(1);
        let rp1 = row.checked_add(1);
        let sm1 = seat.checked_sub(1);
        let sp1 = seat.checked_add(1);

        for pos in vec!(
            (rm1,sm1),(rm1,Some(seat)),(rm1,sp1),
            (Some(row),sm1),(Some(row),sp1),
            (rp1,sm1),(rp1,Some(seat)),(rp1,sp1),
        ) {
                if let (Some(r),Some(s)) = pos {
                if let Some(v) = &self.floor_map.get(r) {
                    if let Some(t) = v.get(s){
                        if *t == Tile::Seat(Seat::Occupied) {
                            cnt += 1;
                        }
                    }
                }
            }
        }*/
        cnt
    }
    pub fn round ( &mut self ,maxocc:u8,maxdist:usize ) -> bool {
        let mut changed = false;
        let mut fmap= Vec::with_capacity(self.floor_map.len());
        for (row,row_vec) in self.floor_map.iter().enumerate() {
            let lvec :Vec<Tile> = row_vec.iter().enumerate().
                    map(|(seat,tile)| match tile  {
                        Tile::Floor => Tile::Floor,
                        Tile::Seat(x) => {
                            let occ = self.get_occupied_around(row,seat,maxdist);
                            match x { 
                                Seat::Empty => {
                                    if occ == 0 {
                                        changed = true;
                                        Tile::Seat(Seat::Occupied)
                                    } else {Tile::Seat(Seat::Empty)}}, 
                                Seat::Occupied => {
                                    if occ >= maxocc {
                                        changed = true;
                                        Tile::Seat(Seat::Empty)
                                    } else { Tile::Seat(Seat::Occupied) }
                                 }
                            }
                    }
                    }).collect();
            fmap.push(lvec);
        } 
        self.floor_map = fmap;
        changed
    }
    pub fn count(self,t_type:Tile ) -> usize {
        self.floor_map.iter().fold(0, |acc, v| acc + v.iter().fold(0, |acc2,i|acc2 + if *i == t_type { 1 } else {0}))
    } 
}

#[cfg(test)]
use super::string_to_uintvec;

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const TEST_STR:&str=  
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_seating() {
        let mut seating = Seating::new(&TEST_STR.to_string());
        println!("{:}",seating);
        println!("{:}",seating.get_occupied_around(0,0,1));
        let mut cnt = 0;
        while seating.round(4,1) {
            println!("{:}",seating);
            cnt += 1;
        }
        assert_eq!(cnt,5); 
        assert_eq!(seating.count(Tile::Seat(Seat::Occupied)),37); 

    }

    #[test]
    fn test_seating_direction() {
        let mut seating = Seating::new(&TEST_STR.to_string());
        println!("{:}",seating);
        println!("{:}",seating.get_occupied_around(0,0,10));
        let mut cnt = 0;
        while seating.round(5,10) {
            println!("{:}",seating);
            cnt += 1;
        }
        assert_eq!(cnt,6); 
        assert_eq!(seating.count(Tile::Seat(Seat::Occupied)),26); 

    }
}