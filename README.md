# glium_shapes

Prefabricated shapes for the glium OpenGL wrapper.


## Getting Started

- Import the `glium_shapes` crate:

  ```rust
  extern crate glium_shapes;
  ```

- Build a 2x3x4 `Cuboid` with its centre-of-mass at the origin and draw it:

  ```rust
  let cuboid = glium_shapes::cuboid::CuboidBuilder()
               .scale(2.0, 3.0, 4.0)
               .build(display)
               .expect("Failed to build cuboid shape");
  frame.draw( &cuboid, &cuboid, your_shader_program, your_uniforms, your_draw_params );
  ```


## Technical Details

* Each shape is constructed using a builder object, which provides methods for customising
  your new shape

* By default, the geometry is constructed to suit the standard OpenGL context defaults:

  * Right-handed coordinate system (x = right, y = up, z = out of screen)
  * Front-faces taken to be counter-clock-wise

* By default, each shape is constructed with its centre-of-mass at the origin


## Development Status

This module is still work-in-progress and hasn't reached an initial release yet. The current plan is to provide an initial 0.1.0 release with the following shapes:

* Cuboid (done)
* Axes
* Sphere
* Cylinder
* Cone


## Contributors

James Bird (@jbrd)
