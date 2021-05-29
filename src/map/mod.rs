use super::render::Coord;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Map {
    pub vertices: Vec<Coord>,
}

impl Map {
    // pub fn new_from_file(filename: &str) -> Result<Self, String> {
    //     let reader = BufReader::new(File::open(filename)?);
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_file() -> Result<(), std::io::Error> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let reader = BufReader::new(File::open("src/map/demo_a.mod1")?);

        for line in reader.lines() {
            assert_eq!("(0,0,20)", line.unwrap());
        }
        Ok(())
    }
}
