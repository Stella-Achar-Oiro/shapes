# Geometric Drawing Library

A Rust library for drawing geometric shapes on raster images.

## Team Members
- [@steoiro](https://learn.zone01kisumu.ke/git/steoiro)
- [@najwang](https://learn.zone01kisumu.ke/git/najwang) 
- [@ebarsulai](https://learn.zone01kisumu.ke/git/ebarsulai)

## Project Overview

This library provides a framework for drawing various geometric shapes on a raster canvas. It implements key drawing algorithms such as Bresenham's line algorithm and the Midpoint circle algorithm.

![Example Output](https://i.postimg.cc/13P3yNyC/sample-output.png)

## Features

- Core shapes: Point, Line, Rectangle, Triangle, Circle
- Optional bonus shapes: Pentagon, Cube (3D)
- Random shape generation
- Customizable colors

## Documentation

See the [docs/](docs/) directory for detailed documentation:

- [Getting Started](docs/getting-started.md)
- [Shape Documentation](docs/shapes.md)
- [Algorithms](docs/algorithms.md)
- [Extensions Guide](docs/extensions.md)
- [3D Rendering](docs/3d-rendering.md)

## Example

```rust
use geometric_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);
    
    // Draw a triangle
    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);
    
    // Save the image
    raster::save(&image, "image.png").unwrap();
}
```

## Project Structure

```
drawing/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── geometrical_shapes.rs
└── docs/
    ├── images/
    ├── getting-started.md
    ├── shapes.md
    ├── algorithms.md
    ├── extensions.md
    └── 3d-rendering.md
```

## Development Roadmap

1. Implement core shapes and algorithms
2. Add color customization
3. Implement shape filling
4. Performance optimizations
5. Bonus shapes (Pentagon, Cube)

## License

The project is distributed under the terms of the [MIT license](LICENSE).