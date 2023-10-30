pub trait Drawable {
    fn is_colored(&self, x: u32, y: u32) -> bool;
}

trait Location {
    fn dist(&self, other: Point) -> f32;
}

pub type Point = (f32, f32);

pub struct Circle {
    pub center: Point,
    pub rad: f32,
}

pub struct Triangle {
    points: [Point; 3],
}

pub struct Segment {
    points: [Point; 2],
}


impl Location for Point {
    fn dist(&self, other: Point) -> f32 {
        let (x1, y1) = self;
        let (x2, y2) = other;

        let dx = x2 - x1;
        let dy = y2 - y1;

        (dx*dx + dy*dy).sqrt()
    }
}


impl Drawable for Point {
    fn is_colored(&self, x: u32, y: u32) -> bool {
        self.0.floor() as u32 == x && self.1.floor() as u32 == y
    }
}

impl Drawable for Circle {
    fn is_colored(&self, x: u32, y: u32) -> bool {
        let mut inside = false;
        let mut outside = false;

        let x = x as f32;
        let y = y as f32;

        let center = self.center;
        let rad = self.rad;

        if center.dist((x, y)) > rad {
            inside = true;
        } else {
            outside = true;
        }

        if center.dist((x+1.0, y)) > rad {
            inside = true;
        } else {
            outside = true;
        }

        if center.dist((x, y+1.0)) > rad {
            inside = true;
        } else {
            outside = true;
        }
        if center.dist((x+1.0, y+1.0)) > rad {
            inside = true;
        } else {
            outside = true;
        }

        inside && outside
    }
}
