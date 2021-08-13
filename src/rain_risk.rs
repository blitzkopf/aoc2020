use std::fmt;
use std::error::Error;


#[derive(Debug)]
pub struct NavigationError {
    action:String,
    value:i64,
    details: String
}

impl NavigationError {
    fn new(msg: &str,action:&str, value:i64) -> NavigationError {
        NavigationError{action:action.to_string(), value:value, details: msg.to_string()}
    }
}

impl fmt::Display for NavigationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for NavigationError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug,PartialEq)]
pub struct Ship {
    lat:i64,
    lon:i64,
    heading:i64,
}
impl Ship {
    //type Err = NavigationError;
    pub fn new()->Ship {
        Ship{lat:0, lon:0, heading:90 }
    }

    pub fn action( & mut self,instruction:&str) -> Result<&Self,  NavigationError>{
        let action = &instruction[0..1];
        let value:i64 = instruction[1..].parse().unwrap();
        
        match action {
            "N" => self.lat += value,
            "S" => self.lat -= value,
            "E" => self.lon += value,
            "W" => self.lon -= value,

            "L" => self.heading = ( self.heading - value +360 ) % 360,
            "R" => self.heading = ( self.heading + value +360 ) % 360,
            "F" => match self.heading {
                0   => self.lat += value,
                90  => self.lon += value,
                180 => self.lat -= value,
                270 => self.lon -= value,
                _ =>  return Err(NavigationError::new(&format!("Unknown heading {}",self.heading),action,value))
            }
            _ => return Err(NavigationError::new("Unknown action",action,value))


        }
        Ok(self)
    }
    pub fn navigate( & mut self,instructions:&str) -> Result<&Self,  NavigationError>{
        for instr in instructions.lines() {
            match self.action(instr) {
                Ok(s)  => {},
                Err(e) => return Err(e)
            }
        }
        Ok(self)
    }
    pub fn manhattan_dist(self) -> i64 {
         self.lat.abs()+self.lon.abs()
    }
}

#[derive(Debug,PartialEq)]
pub struct Ship2 {
    lat:i64,
    lon:i64,
    vector:(i64,i64)
}
impl Ship2 {
    //type Err = NavigationError;
    pub fn new()->Ship2 {
        Ship2{lat:0, lon:0, vector:(1,10) }
    }

    fn left(vector:(i64,i64),degrees:i64) ->(i64,i64) {
        if degrees==0 {
            vector
        } else {
            Ship2::left((vector.1,-vector.0),degrees-90)
        }
    } 
    fn right(vector:(i64,i64),degrees:i64) ->(i64,i64) {
        if degrees==0 {
            vector
        } else {
            Ship2::right((-vector.1,vector.0),degrees-90)
        }
    } 

    pub fn action( & mut self,instruction:&str) -> Result<&Self,  NavigationError>{
        let action = &instruction[0..1];
        let value:i64 = instruction[1..].parse().unwrap();
        
        match action {
            "N" => self.vector.0 += value,
            "S" => self.vector.0 -= value,
            "E" => self.vector.1 += value,
            "W" => self.vector.1 -= value,

            "L" => self.vector = Ship2::left(self.vector,value),
            "R" => self.vector = Ship2::right(self.vector,value),
            "F" =>  {
                    self.lat += self.vector.0 * value;
                    self.lon += self.vector.1 * value;
                }
            _ => return Err(NavigationError::new("Unknown action",action,value))
    }
        Ok(self)
    }
    pub fn navigate( & mut self,instructions:&str) -> Result<&Self,  NavigationError>{
        for instr in instructions.lines() {
            match self.action(instr) {
                Ok(s)  => {},
                Err(e) => return Err(e)
            }
            println!("Ship:{:?}",self);
        }
        Ok(self)
    }
    pub fn manhattan_dist(self) -> i64 {
         self.lat.abs()+self.lon.abs()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const TEST_STR:&str=  
"F10
N3
F7
R90
F11";

    #[test]
    fn test_action() {
        let mut ship  = Ship::new();
        assert_eq!(ship.action("F10").unwrap(),&Ship{lat:0, lon:10, heading:90 });
        assert_eq!(ship.action("N3").unwrap(),&Ship{lat:3, lon:10, heading:90 });
        assert_eq!(ship.action("F7").unwrap(),&Ship{lat:3, lon:17, heading:90 });
    }
    #[test]
    fn test_navigation() {
        let mut ship  = Ship::new();
        ship.navigate(TEST_STR);
        assert_eq!(ship.manhattan_dist(),25);
    }
    #[test]
    fn test_navigation2() {
        let mut ship  = Ship2::new();
        ship.navigate(TEST_STR);
        assert_eq!(ship.manhattan_dist(),286);
    }
}
