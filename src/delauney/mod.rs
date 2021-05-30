use std::fmt;

fn circumcenter(a: &Vec2, b: &Vec2, c: &Vec2) -> Vec2 {
    let ad = a.x.powi(2) + a.y.powi(2);
    let bd = b.x.powi(2) + b.y.powi(2);
    let cd = c.x.powi(2) + c.y.powi(2);
    let dis = 2.0 * (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y));
    Vec2::new(
        1.0 / dis * (ad * (b.y - c.y) + bd * (c.y - a.y) + cd * (a.y - b.y)),
        1.0 / dis * (ad * (c.x - b.x) + bd * (a.x - c.x) + cd * (b.x - a.x)),
    )
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Eq for Vec2 {}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance(a: &Self, b: &Self) -> f32 {
        ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
    }

    pub fn distance_to(&self, b: &Self) -> f32 {
        Vec2::distance(&self, b)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    pub points: [Vec2; 2],
}

impl Edge {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        Self { points: [p1, p2] }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.points[0] == other.points[0] && self.points[1] == other.points[1])
            || (self.points[0] == other.points[1] && self.points[1] == other.points[0])
    }
}

impl Eq for Edge {}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Triangle {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
    pub edges: [Edge; 3],
    pub circumcenter: Vec2,
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
        let edges = [Edge::new(a, b), Edge::new(b, c), Edge::new(c, a)];
        Self {
            a,
            b,
            c,
            edges,
            circumcenter: circumcenter(&a, &b, &c),
        }
    }

    pub fn contains_point_in_circumcircle(&self, point: &Vec2) -> bool {
        self.a.distance_to(&self.circumcenter) > point.distance_to(&self.circumcenter)
    }

    pub fn has_vertex(&self, point: &Vec2) -> bool {
        self.a == *point || self.b == *point || self.c == *point
    }

    pub fn shares_vertex(&self, other: &Self) -> bool {
        self.has_vertex(&other.a) || self.has_vertex(&other.b) || self.has_vertex(&other.c)
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[a: {:?}, b: {:?}, c: {:?}]", self.a, self.b, self.c)
    }
}

fn edge_is_not_shared_by_other_triangles(
    edge: Edge,
    current: &Triangle,
    triangles: &Vec<Triangle>,
) -> bool {
    for triangle in triangles {
        if triangle == current {
            continue;
        }
        for other_edge in triangle.edges.iter() {
            if *other_edge == edge {
                return false;
            }
        }
    }
    true
}

// Using Bowyer-Watson algorithm (https://en.wikipedia.org/wiki/Bowyer%E2%80%93Watson_algorithm)
pub fn delauney_triangulation(points: &Vec<Vec2>, super_triangle: Triangle) -> Vec<Triangle> {
    let mut triangulation = Vec::new();

    triangulation.push(super_triangle);

    for point in points.iter() {
        let mut bad_triangles = Vec::new();

        // If triangle contains a point in its circumcircle, it is "bad" and will be removed
        for triangle in triangulation.iter() {
            if triangle.contains_point_in_circumcircle(point) {
                bad_triangles.push(*triangle);
            }
        }

        // Find the boundary of the polygonal hole
        let mut polygon = Vec::new();
        for triangle in bad_triangles.iter() {
            for edge in triangle.edges.iter() {
                if edge_is_not_shared_by_other_triangles(*edge, triangle, &bad_triangles) {
                    polygon.push(edge);
                }
            }
        }

        // Remove bad triangles
        triangulation = triangulation
            .into_iter()
            .filter(|&triangle| bad_triangles.contains(&triangle) == false)
            .collect();

        // Re-triangulate the polygonal hole
        for edge in polygon.into_iter() {
            let triangle = Triangle::new(edge.points[0], edge.points[1], *point);
            triangulation.push(triangle);
        }
    }

    // Done inserting points, now clean up (remove triangles that contain a vertex of super
    // triangle)
    triangulation
        .into_iter()
        .filter(|&triangle| !triangle.shares_vertex(&super_triangle))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn does_share_vertex() {
        let super_triangle = Triangle::new(
            Vec2::new(5.0, 50.0),
            Vec2::new(50.0, -40.0),
            Vec2::new(-50.0, -40.0),
        );
        let smaller_triangle = Triangle::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(50.0, -40.0),
        );
        assert!(smaller_triangle.shares_vertex(&super_triangle));
    }

    #[test]
    fn basic_triangulation() {
        let vertices = vec![
            Vec2::new(50.0, 50.0),
            Vec2::new(100.0, 100.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(100.0, 0.0),
            Vec2::new(0.0, 100.0),
        ];
        let triangulation = delauney_triangulation(
            &vertices,
            Triangle::new(
                Vec2::new(50.0, 600.0),
                Vec2::new(-200.0, -200.0),
                Vec2::new(400.0, -200.0),
            ),
        );
        for triangle in triangulation.iter() {
            println!("{}", triangle);
        }
        assert_eq!(triangulation.len(), 4);
    }

    #[test]
    fn triangulation_is_successful() {
        let vertices = vec![
            Vec2::new(50.0, 50.0),
            Vec2::new(150.0, 50.0),
            Vec2::new(50.0, 150.0),
            Vec2::new(150.0, 150.0),
            Vec2::new(100.0, 50.0),
            Vec2::new(100.0, 150.0),
            Vec2::new(50.0, 100.0),
            Vec2::new(150.0, 100.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 200.0),
            Vec2::new(200.0, 0.0),
            Vec2::new(200.0, 200.0),
        ];
        let triangulation = delauney_triangulation(
            &vertices,
            Triangle::new(
                Vec2::new(100.0, 600.0),
                Vec2::new(-200.0, -200.0),
                Vec2::new(400.0, -200.0),
            ),
        );
        for triangle in triangulation.iter() {
            println!("{}", triangle);
        }
    }
}
