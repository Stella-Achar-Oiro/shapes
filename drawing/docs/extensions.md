# Extensions Guide

This guide explains how to extend the geometric drawing library with new features.

## Adding Color Support

### Implementation Strategy

1. Add color fields to shape structures
2. Create constructors that accept color parameters
3. Override the `color()` method in the `Drawable` trait

### Example Approach

For any shape structure:
- Add an optional color field
- Add a `with_color` constructor method
- Implement the color method to return the custom color if set

## Implementing Filled Shapes

### Extension Strategy

1. Add a `draw_filled()` method to the `Drawable` trait
2. Implement specific fill algorithms for each shape type

### Fill Algorithms By Shape

#### Rectangle Fill

Simplest approach - loop through all internal pixels.

#### Triangle Fill

Use a scan-line algorithm:
1. Process one horizontal line at a time
2. Find where scan line intersects triangle edges
3. Fill between intersection points

#### Circle Fill

Two main approaches:
1. Draw concentric circles with decreasing radius
2. For each y-value, calculate x-range and fill horizontally

## Adding Thickness to Lines

### Implementation Strategy

1. Add a thickness parameter to Line and other structures
2. For thin lines (1-2 pixels), use the standard algorithms
3. For thicker lines, implement these approaches:
   - Draw multiple offset parallel lines
   - Use a brush technique (filled circle at each point)
   - Implement a proper antialiased thick line algorithm

## Performance Optimizations

### Clipping Optimization

Pre-compute shape bounds to avoid unnecessary pixel calculations:

1. Calculate the bounding box of each shape
2. Check if it's completely outside the image
3. For partially visible shapes, only process the overlapping region

### Batch Rendering

Implement a method to render multiple pixels at once:

1. Collect all pixel operations into a buffer
2. Process the buffer in a single operation
3. Reduces function call overhead

### Multi-threading

For large images with many shapes:

1. Divide the image into regions
2. Process each region on a separate thread
3. Combine the results

## Implementing Bonus Shapes

### Pentagon

Implementation approach:
1. Define the pentagon with a center point, radius, and rotation
2. Calculate the five vertices (using trigonometry)
3. Draw lines between consecutive vertices
4. For filling, divide into triangles from center

### Cube (3D)

Implementation steps:
1. Define the cube with a center, size, and rotation angles
2. Create a 3D point structure with x, y, z coordinates
3. Implement rotation matrices for 3D transformations
4. Add projection methods (perspective or isometric)
5. Draw the 12 edges connecting the 8 vertices
6. For more realism, implement hidden face removal

## Animation Support

To add simple animation:

1. Implement methods to update shape properties (position, size, rotation)
2. Create a loop that:
   - Clears the previous frame
   - Updates shape properties
   - Renders shapes
   - Saves or displays the frame
   - Repeats

## User Interaction

For interactive applications:

1. Capture keyboard/mouse input
2. Map inputs to shape transformations
3. Implement selection and modification of shapes
4. Add UI elements for color or property selection

## Extending the Traits

For more advanced features, consider extending the core traits:

1. Add new methods to `Drawable`:
   - `transform(&mut self, tx: f64, ty: f64, scale: f64, rotation: f64)`
   - `contains(&self, point: &Point) -> bool`

2. Enhance `Displayable`:
   - `display_multiple(&mut self, points: &[(i32, i32)], color: Color)`
   - `clear(&mut self, color: Color)`