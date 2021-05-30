use glium;

use crate::delauney::Vec2;
use std::fmt;
use std::ops::Index;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Coord {
    pub position: (f32, f32, f32),
}

impl Coord {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: (x, y, z),
        }
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.position.0
    }
    pub fn x(&self) -> &f32 {
        &self.position.0
    }
    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.position.1
    }
    pub fn y(&self) -> &f32 {
        &self.position.1
    }
    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.position.2
    }
    pub fn z(&self) -> &f32 {
        &self.position.2
    }

    pub fn vec2(&self) -> Vec2 {
        Vec2::new(self.position.0, self.position.1)
    }

    pub fn vec3(&self) -> [f32; 3] {
        [self.position.0, self.position.1, self.position.2]
    }
}

// Could change to enum, and make ParseCoordError with 'kind' parameter, for more accurate error
// output, if there is time
#[derive(Debug, PartialEq)]
pub struct CoordError;

impl fmt::Display for CoordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Coordinate")
    }
}

impl From<std::num::ParseFloatError> for CoordError {
    fn from(_: std::num::ParseFloatError) -> Self {
        Self
    }
}

impl std::error::Error for CoordError {}

// Implicitly implements ToString
impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x(), self.y(), self.z())
    }
}

impl FromStr for Coord {
    type Err = CoordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 7 {
            return Err(CoordError);
        }
        if s.chars().next().unwrap() != '(' || s.chars().last().unwrap() != ')' {
            return Err(CoordError);
        }
        let vals: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();
        if vals.len() != 3 {
            return Err(CoordError);
        }
        let x = vals[0].trim().parse::<f32>()?;
        let y = vals[1].trim().parse::<f32>()?;
        let z = vals[2].trim().parse::<f32>()?;
        Ok(Coord::new(x, y, z))
    }
}

impl Index<usize> for Coord {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.position.0,
            1 => &self.position.1,
            2 => &self.position.2,
            _ => panic!("Invalid indexing of Coord"),
        }
    }
}

glium::implement_vertex!(Coord, position);

#[derive(Copy, Clone, Debug)]
pub struct CameraMatrix {
    pub zoom: f32,
    pub position: Coord,
    pub direction: Coord,
    pub up: Coord,
}

impl CameraMatrix {
    pub fn mat4(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };

        let s = [
            self.up[1] * f[2] - self.up[2] * f[1],
            self.up[2] * f[0] - self.up[0] * f[2],
            self.up[0] * f[1] - self.up[1] * f[0],
        ];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };

        let u = [
            f[1] * s_norm[2] - f[2] * s_norm[1],
            f[2] * s_norm[0] - f[0] * s_norm[2],
            f[0] * s_norm[1] - f[1] * s_norm[0],
        ];

        let p = [
            -self.position[0] * s_norm[0]
                - self.position[1] * s_norm[1]
                - self.position[2] * s_norm[2],
            -self.position[0] * u[0] - self.position[1] * u[1] - self.position[2] * u[2],
            -self.position[0] * f[0] - self.position[1] * f[1] - self.position[2] * f[2],
        ];

        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }
}

#[derive(Default)]
pub struct CameraBuilder {
    zoom: f32,
    position: Coord,
    direction: Coord,
    up: Coord,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn zoom(mut self, zoom: f32) -> CameraBuilder {
        self.zoom = zoom;
        self
    }
    pub fn position(mut self, position: Coord) -> CameraBuilder {
        self.position = position;
        self
    }
    pub fn direction(mut self, direction: Coord) -> CameraBuilder {
        self.direction = direction;
        self
    }
    pub fn up(mut self, up: Coord) -> CameraBuilder {
        self.up = up;
        self
    }
    pub fn build(self) -> CameraMatrix {
        CameraMatrix {
            zoom: self.zoom,
            position: self.position,
            direction: self.direction,
            up: self.up,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn coords_can_be_parsed() {
        let coord1 = "(15,20,15)".parse::<Coord>();
        assert!(coord1.is_ok());
        assert_eq!(coord1.unwrap(), Coord::new(15.0, 20.0, 15.0));

        let coord2 = "(-20.54, 15.728, 120.7)".parse::<Coord>();
        assert!(coord2.is_ok());
        assert_eq!(coord2.unwrap(), Coord::new(-20.54, 15.728, 120.7));

        let coord3 = "(-20.five, 15.728, 120.7)".parse::<Coord>();
        assert!(coord3.is_err());
    }
}
