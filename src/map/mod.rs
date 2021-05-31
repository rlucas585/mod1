use super::render::Coord;
use crate::delauney::{delauney_triangulation, Triangle, Vec2};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

macro_rules! get_edge_val {
    ($minmax:ident, $vec:ident, $axis:ident) => {
        *$vec
            .iter()
            .$minmax(|vertex| *vertex.$axis() as i32)
            .unwrap()
            .$axis();
    };
}

macro_rules! find_index {
    ($vertex:ident, $vertices:ident, $triangle:ident) => {
        $vertices
            .iter()
            .position(|&coord| {
                coord.position.0 == $triangle.$vertex.x && coord.position.1 == $triangle.$vertex.y
            })
            .unwrap()
    };
}

const SCALE_FACTOR: f32 = 6.0;

pub struct Map {
    pub vertices: Vec<Coord>,
    center: Coord,
    pub indices: Vec<u16>,
    pub scale: usize,
}

impl Map {
    pub fn new_from_file(filename: &str) -> Result<Self, io::Error> {
        let reader = BufReader::new(File::open(filename)?);
        let mut vertices: Vec<Coord> = Vec::new();
        for line in reader.lines() {
            if let Ok(new_coord) = line.unwrap().parse::<Coord>() {
                vertices.push(new_coord);
            } else {
                return Err(io::Error::new(io::ErrorKind::Other, "Invalid Coord!"));
            }
        }
        let (center, scale) = Map::add_edges(&mut vertices);

        let points: Vec<Vec2> = vertices.iter().map(|&coord| coord.vec2()).collect();
        let triangulation = delauney_triangulation(
            &points,
            Triangle::new(
                Vec2::new(0.0, 200.0 * scale as f32),
                Vec2::new(200.0 * scale as f32, -(200.0 * scale as f32)),
                Vec2::new(-(200.0 * scale as f32), -(200.0 * scale as f32)),
            ),
        );
        for triangle in triangulation.iter() {
            println!("{}", triangle);
        }
        let indices = Map::calculate_indices(triangulation, &vertices);

        // Scaling may be useful to add at some point, to enable a wider variety of maps
        Ok(Self {
            vertices,
            center,
            indices,
            scale,
        })
    }

    pub fn center(&self) -> Coord {
        return self.center;
    }

    fn add_edges(vertices: &mut Vec<Coord>) -> (Coord, usize) {
        let max_x = get_edge_val!(max_by_key, vertices, x);
        let min_x = get_edge_val!(min_by_key, vertices, x);
        let max_y = get_edge_val!(max_by_key, vertices, y);
        let min_y = get_edge_val!(min_by_key, vertices, y);
        let y_range = if max_y - min_y == 0.0 {
            10.0
        } else {
            (max_y - min_y) / 2.0
        };
        let x_range = if max_x - min_x == 0.0 {
            10.0
        } else {
            (max_x - min_x) / 2.0
        };
        vertices.push(Coord::new(
            min_x as f32 - x_range,
            min_y as f32 - y_range,
            0.0,
        ));
        vertices.push(Coord::new(
            min_x as f32 - x_range,
            max_y as f32 + y_range,
            0.0,
        ));
        vertices.push(Coord::new(
            max_x as f32 + x_range,
            min_y as f32 - y_range,
            0.0,
        ));
        vertices.push(Coord::new(
            max_x as f32 + x_range,
            max_y as f32 + y_range,
            0.0,
        ));
        let max_x = get_edge_val!(max_by_key, vertices, x);
        let min_x = get_edge_val!(min_by_key, vertices, x);
        let max_y = get_edge_val!(max_by_key, vertices, y);
        let min_y = get_edge_val!(min_by_key, vertices, y);
        let y_range = max_y - min_y;
        let x_range = max_x - min_x;
        (
            Coord::new((max_x + min_x) / 2.0, (max_y + min_y) / 2.0, 0.0),
            ((x_range + y_range) / (2.0 * SCALE_FACTOR)) as usize,
        )
    }

    fn calculate_indices(triangulation: Vec<Triangle>, vertices: &Vec<Coord>) -> Vec<u16> {
        let mut indices = Vec::new();
        for triangle in triangulation.iter() {
            indices.push(find_index!(a, vertices, triangle) as u16);
            indices.push(find_index!(b, vertices, triangle) as u16);
            indices.push(find_index!(c, vertices, triangle) as u16);
        }
        indices
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for vertex in self.vertices.iter() {
            writeln!(f, "{}", vertex)?;
        }
        writeln!(f, "center: {}", self.center)?;
        writeln!(f, "scale: {}", self.scale)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn create_map_from_file() -> Result<(), std::io::Error> {
        let map = Map::new_from_file("src/map/demo_c.mod1")?;

        print!("{}", map);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Invalid Coord!")]
    fn invalid_map() {
        let map = Map::new_from_file("src/map/invalid_a.mod1").unwrap();

        print!("{}", map);
    }
}
