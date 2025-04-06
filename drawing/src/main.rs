mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    // Draw a random line
    gs::Line::random(image.width, image.height).draw(&mut image);

    // Draw a random point
    gs::Point::random(image.width, image.height).draw(&mut image);

    // Draw a square
    let square = gs::Square::new(&gs::Point::new(150, 150), 100); 
    square.draw(&mut image);

    // Draw a rectangle
    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 300), &gs::Point::new(50, 60));
    rectangle.draw(&mut image);

    // Draw a triangle
    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    // Draw random circles
    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }
    
    // Demonstrate methods - create a circle and check if it intersects with another
    let circle1 = gs::Circle::new(400, 400, 50);
    let circle2 = gs::Circle::new(450, 450, 60);
    if circle1.intersect(&circle2) {
        println!("Circles intersect!");
        println!("Circle 1 area: {}", circle1.area());
        println!("Circle 2 diameter: {}", circle2.diameter());
    }
    
    // Draw other shapes - Pentagon
    let pentagon = gs::Pentagon::new(&gs::Point::new(300, 200), 80, 0.2);
    pentagon.draw(&mut image);
    
    // Draw random pentagons
    gs::Pentagon::random(image.width, image.height).draw(&mut image);
    
    // Draw 3D Cube using new() method
    let basic_cube = gs::Cube::new(&gs::Point::new(700, 500), 80);
    basic_cube.draw(&mut image);
    // Draw 3D Cube
    let cube = gs::Cube::with_rotation(
        &gs::Point::new(700, 300),
        100,  // size
        0.3,  // x-rotation
        0.5,  // y-rotation
        0.1   // z-rotation
    );
    cube.draw(&mut image);
    
    // Draw random cube
    gs::Cube::random(image.width, image.height).draw(&mut image);

    // Save the image
    raster::save(&image, "image.png").unwrap();
    println!("Image has been saved as 'image.png'");
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}