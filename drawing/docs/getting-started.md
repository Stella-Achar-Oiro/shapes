# Getting Started

This guide will help you set up and start using the Geometric Drawing Library.

## Setup

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Installation

1. Clone the repository:
```
git clone https://learn.zone01kisumu.ke/git/steoiro/drawing.git
cd drawing
```

2. Build the project:
```
cargo build
```

3. Run the example:
```
cargo run
```

This will generate an `image.png` file with various shapes drawn on it.

## Project Structure

- `src/main.rs`: Entry point and usage examples
- `src/geometrical_shapes.rs`: Core library implementation
- `Cargo.toml`: Project configuration and dependencies

## Core Concepts

### Traits

The library is built around two main traits:

1. **Displayable**: Objects that can display pixels
   - Implemented for the `Image` type from the raster crate
   - Allows drawing pixels at specific coordinates

2. **Drawable**: Objects that can be drawn on a Displayable
   - Implemented for all shapes
   - Provides methods to draw shapes and specify colors

### Basic Usage Pattern

```rust
// Create a canvas
let mut image = Image::blank(1000, 1000);

// Create a shape
let shape = gs::Circle::new(500, 500, 100);

// Draw the shape to the canvas
shape.draw(&mut image);

// Save the result
raster::save(&image, "output.png").unwrap();
```

## Shapes Overview

![Basic Shapes](https://i.postimg.cc/fRwDGnv1/basic-shapes.png)

The library includes these basic shapes:

- **Point**: A single pixel
- **Line**: Connects two points 
- **Triangle**: Defined by three points
- **Rectangle**: Defined by two corner points
- **Circle**: Defined by center point and radius

Each shape has a `new()` constructor and implements the `Drawable` trait.

## Random Shapes

Some shapes support a `random()` method to create randomized instances:

```rust
// Create a random circle
let random_circle = gs::Circle::random(width, height);
```

## Next Steps

- Learn about each shape in detail in [Shapes Documentation](shapes.md)
- Understand the algorithms used in [Algorithms](algorithms.md)
- Explore how to extend the library in [Extensions Guide](extensions.md)