# Shapes Documentation

## Point

A single pixel in 2D space.

### Properties

- `x`: Horizontal coordinate (i32)
- `y`: Vertical coordinate (i32)

### Methods

- `new(x: i32, y: i32)`: Create a new point
- `distance(&self, other: &Point)`: Calculate distance to another point
- `random(limit_x: i32, limit_y: i32)`: Create a random point within limits

### Usage

```rust
let p1 = Point::new(100, 200);
```

## Line

A straight line connecting two points.

### Properties

- `a`: Starting point
- `b`: Ending point

### Methods

- `new(beg: &Point, end: &Point)`: Create a new line
- `random(limit_x: i32, limit_y: i32)`: Create a random line within limits

### Algorithm

Lines are drawn using Bresenham's line algorithm, which:
- Uses only integer arithmetic for efficiency
- Handles all slope cases (shallow, steep, horizontal, vertical)
- Produces the closest approximation to a straight line on a pixel grid

### Usage

```rust
let line = Line::new(&Point::new(10, 10), &Point::new(200, 50));
```

## Rectangle

A four-sided shape with all angles at 90 degrees.

### Properties

- `upper_left_corner`: Top-left point
- `lower_right_corner`: Bottom-right point

### Methods

- `new(point1: &Point, point2: &Point)`: Create a new rectangle

### Usage

```rust
let rect = Rectangle::new(&Point::new(100, 100), &Point::new(300, 200));
```

## Triangle

A shape with three sides and three vertices.

### Properties

- `vertices`: Tuple of three points (a, b, c)

### Methods

- `new(a: &Point, b: &Point, c: &Point)`: Create a new triangle

### Usage

```rust
let triangle = Triangle::new(
    &Point::new(100, 100),
    &Point::new(200, 300),
    &Point::new(300, 150)
);
```

## Circle

A shape where all points are equidistant from the center.

### Properties

- `center`: Center point
- `radius`: Distance from center to edge
- `color`: Color of the circle

### Methods

- `new(x: i32, y: i32, radius: i32)`: Create a new circle
- `area(&self)`: Calculate the area
- `diameter(&self)`: Get the diameter
- `intersect(&self, c: &Circle)`: Check if intersects with another circle
- `random(limit_x: i32, limit_y: i32)`: Create a random circle

### Algorithm

Circles are drawn using the Midpoint Circle Algorithm, which:
- Calculates which pixels best approximate a circle
- Uses 8-way symmetry to reduce calculations
- Uses only integer arithmetic for efficiency

### Usage

```rust
let circle = Circle::new(400, 300, 100);
```

## Bonus Shapes

### Pentagon (Optional)

A five-sided polygon.

### Implementation Idea

Define a center point, radius (distance to vertices), and a rotation angle.

### Cube (Optional)

A 3D shape with 6 faces, 8 vertices, and 12 edges.

### Implementation Idea

Define the cube with:
- Center point
- Size (edge length)
- Rotation angles (x, y, z)

A 3D-to-2D projection will be needed to display on the 2D canvas.

## Extension Ideas

Possible ways to extend the shape system:

1. **Filled shapes**: Implement interior filling for closed shapes
2. **Color customization**: Allow custom colors for each shape
3. **Thickness**: Add line thickness for shapes' outlines
4. **Transformations**: Scaling, rotation for all shapes
5. **Composite shapes**: Group shapes together