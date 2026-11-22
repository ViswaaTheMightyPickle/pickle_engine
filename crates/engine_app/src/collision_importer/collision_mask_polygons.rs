#[derive(PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

pub struct CollisionMaskPolygons {
    pub triangles: Vec<Triangle>,
    pub intersection_points: Vec<Point>,
}