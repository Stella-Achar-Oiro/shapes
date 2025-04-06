# Drawing Algorithms

This document explains the key algorithms used in our geometric drawing library.

## Bresenham's Line Algorithm

### Overview

Bresenham's algorithm efficiently draws lines on a pixel grid using only integer arithmetic. It determines which pixels should be colored to form the closest approximation to a straight line.

### How It Works

1. Calculate the differences between endpoints: Δx and Δy
2. Determine the primary direction (x or y) based on which difference is larger
3. Initialize an error term based on the differences
4. For each step along the primary direction:
   - Plot the current pixel
   - Update the error term
   - Step in the secondary direction if needed (based on the error term)

### Advantages

- Uses only integer addition, subtraction, and bit shifting (no floating point)
- Produces the best possible approximation on a pixel grid
- Efficient for all line angles

### Cases Handled

- Shallow slope (|m| < 1)
- Steep slope (|m| > 1)
- Horizontal lines
- Vertical lines

## Midpoint Circle Algorithm

### Overview

The Midpoint Circle Algorithm (also known as Bresenham's Circle Algorithm) efficiently draws circles by calculating points for only 1/8 of the circle and using symmetry for the rest.

### How It Works

1. Start from (0, r) where r is the radius
2. Use a decision parameter to determine which pixel is closer to the true circle
3. Calculate points for the first octant (45° arc)
4. Use 8-way symmetry to plot all 8 octants simultaneously
5. Continue until the octant is complete (until x ≥ y)

### Advantages

- Uses only integer arithmetic
- Leverages circle symmetry to minimize calculations
- Creates smooth, continuous circles
- More efficient than trigonometric approaches

### Visual Explanation

For each point (x, y) calculated in the first octant, we plot:
- (x, y)
- (y, x)
- (-x, y)
- (-y, x)
- (-x, -y)
- (-y, -x)
- (x, -y)
- (y, -x)

## Shape Filling Algorithms

### Scan-Line Algorithm

Used for filling triangles and other polygons:

1. Find the minimum and maximum y-coordinates
2. For each horizontal scan line (from yMin to yMax):
   - Find intersections with all edges
   - Sort intersections by x-coordinate
   - Fill pixels between each pair of intersections

### Flood Fill Algorithm

Used for filling enclosed areas:

1. Start at a seed point inside the shape
2. Add it to a queue of pixels to process
3. For each pixel in the queue:
   - Color the pixel
   - Add its uncolored neighbors to the queue
4. Continue until the queue is empty

## 3D Projection (For Cube)

### Basic Principle

To display 3D objects on a 2D screen, we project their coordinates.

### Perspective Projection

Objects appear smaller as they get farther away:
- Scale factor = viewing distance / (viewing distance + z)
- x' = x * scale factor
- y' = y * scale factor

### Isometric Projection

No perspective distortion, maintains parallel lines:
- x' = x cos(30°) - z cos(30°)
- y' = y + x sin(30°) + z sin(30°)

## Optimization Techniques

### Clipping

Skip rendering shapes or portions outside the visible area:
1. Calculate shape's bounding box
2. Check if it intersects with screen boundaries
3. Only process pixels within the visible region

### Batch Processing

Process multiple pixels at once to reduce overhead.

### Memory Efficiency

Reuse memory and avoid unnecessary allocations.