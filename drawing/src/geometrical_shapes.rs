use raster::Color;
use rand::Rng;

// Core traits
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw<I: Displayable>(&self, image: &mut I);
    fn color(&self) -> Color {
        Color::white() // Default implementation returns white
    }
}

// ========== Point ==========
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x as f64 - other.x as f64).powf(2.0) + 
         (self.y as f64 - other.y as f64).powf(2.0)).sqrt()
    }
    
    pub fn random(limit_x: i32, limit_y: i32) -> Point {
        Point {
            x: rand::thread_rng().gen_range(0..limit_x),
            y: rand::thread_rng().gen_range(0..limit_y),
        }
    }
}

impl Drawable for Point {
    fn draw<I: Displayable>(&self, image: &mut I) {
        image.display(self.x, self.y, self.color())
    }
}

// ========== Line ==========
#[derive(Debug)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    pub fn new(beg: &Point, end: &Point) -> Line {
        Line {
            a: beg.clone(),
            b: end.clone(),
        }
    }

    pub fn random(limit_x: i32, limit_y: i32) -> Line {
        Line::new(
            &Point::random(limit_x, limit_y),
            &Point::random(limit_x, limit_y),
        )
    }
}

// ========== Square ==========
#[derive(Debug)]
pub struct Square {
    pub top_left: Point,
    pub side_length: i32,
}

impl Square {
    pub fn new(top_left: &Point, side_length: i32) -> Square {
        Square {
            top_left: top_left.clone(),
            side_length,
        }
    }
}

impl Drawable for Square {
    fn draw<I: Displayable>(&self, image: &mut I) {
        // Create a rectangle with equal width and height
        let bottom_right = Point {
            x: self.top_left.x + self.side_length,
            y: self.top_left.y + self.side_length,
        };
        
        Rectangle::new(&self.top_left, &bottom_right).draw(image);
    }
}

// ========== Rectangle ==========
#[derive(Debug)]
pub struct Rectangle {
    pub upper_left_corner: Point,
    pub lower_right_corner: Point,
}

impl Rectangle {
    pub fn new(point1: &Point, point2: &Point) -> Rectangle {
        Rectangle {
            upper_left_corner: point1.clone(),
            lower_right_corner: point2.clone(),
        }
    }
}

impl Drawable for Rectangle {
    fn draw<I: Displayable>(&self, image: &mut I) {
        // Create the other two corners
        let upper_right_corner = Point {
            x: self.lower_right_corner.x,
            y: self.upper_left_corner.y,
        };
        
        let lower_left_corner = Point {
            x: self.upper_left_corner.x,
            y: self.lower_right_corner.y,
        };
        
        // Draw the four sides as lines
        Line::new(&self.upper_left_corner, &upper_right_corner).draw(image);
        Line::new(&upper_right_corner, &self.lower_right_corner).draw(image);
        Line::new(&self.lower_right_corner, &lower_left_corner).draw(image);
        Line::new(&lower_left_corner, &self.upper_left_corner).draw(image);
    }
}

// ========== Triangle ==========
#[derive(Debug)]
pub struct Triangle {
    pub vertices: (Point, Point, Point),
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Triangle {
        Triangle {
            vertices: (a.clone(), b.clone(), c.clone()),
        }
    }
}

impl Drawable for Triangle {
    fn draw<I: Displayable>(&self, image: &mut I) {
        // Draw the three sides as lines
        Line::new(&self.vertices.0, &self.vertices.1).draw(image);
        Line::new(&self.vertices.1, &self.vertices.2).draw(image);
        Line::new(&self.vertices.2, &self.vertices.0).draw(image);
    }
}

// ========== Circle ==========
use std::f64::consts;

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
    pub color: Color,
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: i32) -> Circle {
        Circle {
            center: Point { x, y },
            radius,
            color: Color::white(),
        }
    }
    
    pub fn area(&self) -> f64 {
        consts::PI * self.radius as f64 * self.radius as f64
    }
    
    pub fn diameter(&self) -> i32 {
        2 * self.radius
    }
    
    pub fn intersect(&self, c: &Circle) -> bool {
        self.center.distance(&c.center) < c.radius as f64 + self.radius as f64
    }
    
    pub fn random(limit_x: i32, limit_y: i32) -> Circle {
        let red = rand::thread_rng().gen_range(0..255);
        let blue = rand::thread_rng().gen_range(0..255);
        let green = rand::thread_rng().gen_range(0..255);
        
        Circle {
            center: Point::random(limit_x, limit_y),
            radius: rand::thread_rng().gen_range(9..500),
            color: raster::Color::rgb(red, green, blue),
        }
    }
}

impl Drawable for Circle {
    fn draw<I: Displayable>(&self, image: &mut I) {
        // Midpoint Circle Algorithm
        let mut x = 0;
        let mut y = self.radius;
        let mut d: i64 = 3 - 2 * self.radius as i64;
        
        while x < y {
            // Draw 8 octants
            image.display(y + self.center.x, x + self.center.y, self.color());
            image.display(x + self.center.x, y + self.center.y, self.color());
            image.display(-x + self.center.x, y + self.center.y, self.color());
            image.display(-y + self.center.x, x + self.center.y, self.color());
            image.display(-y + self.center.x, -x + self.center.y, self.color());
            image.display(-x + self.center.x, -y + self.center.y, self.color());
            image.display(x + self.center.x, -y + self.center.y, self.color());
            image.display(y + self.center.x, -x + self.center.y, self.color());
            
            if d < 0 {
                d = d + 4 * x as i64 + 6;
                x = x + 1;
            } else {
                d = d + 4 * (x - y) as i64 + 10;
                x = x + 1;
                y = y - 1;
            }
        }
    }
    
    fn color(&self) -> Color {
        self.color.clone()
    }
}

// ========== Line ==========
impl Drawable for Line {
    fn draw<I: Displayable>(&self, image: &mut I) {
        // Implementation of Bresenham's line algorithm
        use std::cmp;
        
        let beg = self.a.clone();
        let end = self.b.clone();
        let dx = (end.x - beg.x).abs();
        let dy = (end.y - beg.y).abs();
        let is_a_vertical_line = dx == 0;

        if is_a_vertical_line {
            let start = cmp::min(beg.y, end.y);
            let fin = cmp::max(beg.y, end.y);
            for y in start..fin + 1 {
                image.display(self.a.x, y, self.color());
            }
            return;
        }

        let slope = (beg.y - end.y) as f64 / (beg.x - end.x) as f64;
        let inc = if slope < 0.0 { -1 } else { 1 };
        
        // Handle slope between 0 and 1
        if slope.abs() <= 1.0 {
            let a = 2 * dy;
            let b = a - 2 * dx;
            let mut p = a - dx;
            let start = cmp::min(beg.x, end.x);
            let fin = cmp::max(beg.x, end.x) + 1;
            let mut y = if start == beg.x { beg.y } else { end.y };
            
            for x in start..fin {
                image.display(x, y, self.color());
                if p < 0 {
                    p += a;
                } else {
                    y += inc;
                    p += b;
                }
            }
        } 
        // Handle slope greater than 1
        else {
            let a = 2 * dx;
            let b = a - 2 * dy;
            let mut p = a - dy;
            let start = cmp::min(beg.y, end.y);
            let fin = cmp::max(beg.y, end.y) + 1;
            let mut x = if start == beg.y { beg.x } else { end.x };
            
            for y in start..fin {
                image.display(x, y, self.color());
                if p < 0 {
                    p += a;
                } else {
                    x += inc;
                    p += b;
                }
            }
        }
    }
}

// ========== Pentagon ==========
#[derive(Debug)]
pub struct Pentagon {
    pub center: Point,
    pub radius: i32,
    pub rotation: f64,
}

impl Pentagon {
    pub fn new(center: &Point, radius: i32, rotation: f64) -> Pentagon {
        Pentagon {
            center: center.clone(),
            radius,
            rotation,
        }
    }
    
    // Calculate the five vertices of the pentagon
    fn vertices(&self) -> [Point; 5] {
        let mut vertices = [
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
        ];
        
        // Calculate the five vertices
        for i in 0..5 {
            let angle = self.rotation + (i as f64) * 2.0 * consts::PI / 5.0;
            let x = self.center.x + (self.radius as f64 * angle.cos()) as i32;
            let y = self.center.y + (self.radius as f64 * angle.sin()) as i32;
            vertices[i] = Point { x, y };
        }
        
        vertices
    }
    
    pub fn random(limit_x: i32, limit_y: i32) -> Pentagon {
        Pentagon {
            center: Point::random(limit_x, limit_y),
            radius: rand::thread_rng().gen_range(20..200),
            rotation: rand::thread_rng().gen_range(0.0..consts::PI * 2.0),
        }
    }
}

impl Drawable for Pentagon {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let vertices = self.vertices();
        
        // Draw the five sides as lines
        for i in 0..5 {
            let next = (i + 1) % 5;
            Line::new(&vertices[i], &vertices[next]).draw(image);
        }
    }
}

// ========== 3D Point for Cube ==========
#[derive(Clone, Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }
    
    // Project 3D point to 2D using perspective projection
    fn project(&self, distance: f64) -> Point {
        let scale = distance / (distance + self.z);
        let projected_x = (self.x * scale) as i32;
        let projected_y = (self.y * scale) as i32;
        
        Point::new(projected_x, projected_y)
    }
    
    // Rotate point around X-axis
    fn rotate_x(&mut self, angle: f64) {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        
        let new_y = self.y * cos_angle - self.z * sin_angle;
        let new_z = self.y * sin_angle + self.z * cos_angle;
        
        self.y = new_y;
        self.z = new_z;
    }
    
    // Rotate point around Y-axis
    fn rotate_y(&mut self, angle: f64) {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        
        let new_x = self.x * cos_angle + self.z * sin_angle;
        let new_z = -self.x * sin_angle + self.z * cos_angle;
        
        self.x = new_x;
        self.z = new_z;
    }
    
    // Rotate point around Z-axis
    fn rotate_z(&mut self, angle: f64) {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        
        let new_x = self.x * cos_angle - self.y * sin_angle;
        let new_y = self.x * sin_angle + self.y * cos_angle;
        
        self.x = new_x;
        self.y = new_y;
    }
}

// ========== Cube ==========
#[derive(Debug)]
pub struct Cube {
    pub center: Point,
    pub size: i32,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
}

impl Cube {
    pub fn new(center: &Point, size: i32) -> Cube {
        Cube {
            center: center.clone(),
            size,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
        }
    }
    
    pub fn with_rotation(center: &Point, size: i32, rx: f64, ry: f64, rz: f64) -> Cube {
        Cube {
            center: center.clone(),
            size,
            rotation_x: rx,
            rotation_y: ry,
            rotation_z: rz,
        }
    }
    
    // Calculate the 8 vertices of the cube
    fn vertices(&self) -> [Point3D; 8] {
        let half_size = self.size as f64 / 2.0;
        
        // Define the 8 corners of the cube
        let mut vertices = [
            Point3D::new(-half_size, -half_size, -half_size), // 0: front bottom left
            Point3D::new(half_size, -half_size, -half_size),  // 1: front bottom right
            Point3D::new(half_size, half_size, -half_size),   // 2: front top right
            Point3D::new(-half_size, half_size, -half_size),  // 3: front top left
            Point3D::new(-half_size, -half_size, half_size),  // 4: back bottom left
            Point3D::new(half_size, -half_size, half_size),   // 5: back bottom right
            Point3D::new(half_size, half_size, half_size),    // 6: back top right
            Point3D::new(-half_size, half_size, half_size),   // 7: back top left
        ];
        
        // Apply rotations to each vertex
        for vertex in &mut vertices {
            vertex.rotate_x(self.rotation_x);
            vertex.rotate_y(self.rotation_y);
            vertex.rotate_z(self.rotation_z);
            
            // Translate to center position
            vertex.x += self.center.x as f64;
            vertex.y += self.center.y as f64;
        }
        
        vertices
    }
    
    // Project 3D vertices to 2D points
    fn projected_vertices(&self) -> [Point; 8] {
        let vertices_3d = self.vertices();
        let mut vertices_2d = [
            Point::new(0, 0), Point::new(0, 0), Point::new(0, 0), Point::new(0, 0),
            Point::new(0, 0), Point::new(0, 0), Point::new(0, 0), Point::new(0, 0),
        ];
        
        // Using perspective projection with a view distance of 1000
        for i in 0..8 {
            vertices_2d[i] = vertices_3d[i].project(1000.0);
        }
        
        vertices_2d
    }
    
    pub fn random(limit_x: i32, limit_y: i32) -> Cube {
        Cube {
            center: Point::random(limit_x, limit_y),
            size: rand::thread_rng().gen_range(50..200),
            rotation_x: rand::thread_rng().gen_range(0.0..consts::PI * 2.0),
            rotation_y: rand::thread_rng().gen_range(0.0..consts::PI * 2.0),
            rotation_z: rand::thread_rng().gen_range(0.0..consts::PI * 2.0),
        }
    }
}

impl Drawable for Cube {
    fn draw<I: Displayable>(&self, image: &mut I) {
        // Get the projected 2D points
        let vertices = self.projected_vertices();
        
        // Define the 12 edges of a cube
        let edges = [
            // Front face
            (0, 1), (1, 2), (2, 3), (3, 0),
            // Back face
            (4, 5), (5, 6), (6, 7), (7, 4),
            // Connecting edges
            (0, 4), (1, 5), (2, 6), (3, 7),
        ];
        
        // Draw each edge
        for (start, end) in edges.iter() {
            Line::new(&vertices[*start], &vertices[*end]).draw(image);
        }
    }
}