# ray_tracer

## A Rust script for ray path tracing and image generation

Originally inspired by _Ray Tracing in One Weekend_ (Peter Shirley) 

### How to run this program

- Clone this repository
- If `rsmake` is not already executable then run `$ chmod u+x rsmake`
- `$ ./rsmake` will do a few things :
  - compile in release mode (debug mode is useless since rendering an image takes longer than compiling), make sure that `cargo` is in your `$PATH`
  - copy the executable to the root directory of the project as `exec`
  If `rsmake` fails, one common reason is the presence of multiple executables in `./release/deps/`. The problem can be fixed by deleting `./release/` before running `rsmake` again.
- Run `./exec`
- Open the newly generated `img.ppm`

### Creating a new scene

To create and render a scene, edit the `build_world` function in `main.rs`, then run as described above. A list of objects and functions to do so can be found below.

### Creating a new complex object

It is recommended to create a new module :
- Create a new file `composite_<object>.rs`
- Implement `<object>` by providing a `build` method (more information below)
- Integrate the new object with the rest of the program

### Modules

```rust
mod camera;                       // Abstraction for the camera
mod hitable;                      // Logic for managing ray/object interaction
mod primitives;                   // Basic objects
mod ray;                          // Wrapper, no important logic in this file
mod rgb;                          // Color struct
mod vec3;                         // Vector overloads

mod composite_axes;               // Axes for debugging purposes
mod composite_cradle;             // Newton's craddle
mod composite_die;                // Cubic die
mod composite_molecules;          // Miscellaneous molecules
mod composite_erlenmeyer;         // Glass erlenmeyer
```

### Imports and dependencies
```rust
std::ops
std::fmt
std::fs
std::io
std::process

rand
rayon
```

## Contents

### vec3.rs
```rust
pub struct Vec3 {         // Derives Copy
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(f64, f64, f64) -> Self;                    // New vector from coordinates
    pub fn len(&self) -> f64;                             // Length
    pub fn unit(&self) -> Self;                           // Unit vector with same direction
    pub fn dot(&self, &Self) -> f64;                      // Dot product
    pub fn dot_self(&self) -> f64;                        // Dot product with self
    pub fn cross(&self, &Self) -> Self;                   // Cross product
    pub fn reflect(&self, &Self) -> Self;                 // Calculate reflection using the surface normal
    pub fn refract(&self, &Self, f64) -> Option<Self>;    // Calculate refraction using the surface normal and quotient of optical indexes
}

impl ops::Add for Vec3;
impl ops::AddAssign for Vec3;
impl ops::Mul<Vec3> for Vec3;                              // Each coordinate separately 
impl ops::MulAssign<Vec3> for Vec3;                        // Each coordinate separately
impl ops::Mul<f64> for Vec3;
impl ops::MulAssign<f64> for Vec3;
impl ops::Sub for Vec3;
impl ops::SubAssign for Vec3;
impl ops::Div<Vec3> for Vec3;                              // Each coordinate separately
impl ops::DivAssign<Vec3> for Vec3;                        // Each coordinate separately
impl ops::Div<f64> for Vec3;
impl ops::DivAssign<f64> for Vec3;
impl ops::Neg for Vec3;
```

### rgb.rs
```rust
pub struct RGB {    // derives Copy
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl RGB {
    pub fn new(f64, f64, f64) -> Self;                    // New color from r/g/b values in [0.; 1.]
}

impl ops::Add for RGB;
impl ops::AddAssign for RGB;
impl ops::Mul<RGB> for RGB ;                              // Each value separately
impl ops::MulAssign<RGB> for RGB;                         // Each value separately
impl ops::Mul<f64> for RGB;
impl ops::MulAssign<f64> for RGB;
impl ops::Sub for RGB;
impl ops::SubAssign for RGB;
impl ops::Div<RGB> for RGB;                               // Each value separately
impl ops::DivAssign<RGB> for RGB;                         // Each value separately
impl ops::Div<f64> for RGB;
impl ops::DivAssign<f64> for RGB;
impl fmt::Display for RGB;                                // For ppm output : "{r} {g} {b}"
impl ops::Rem<usize> for RGB;                             // COLOR%n == COLOR * n as f64 / 100.

pub const RED: RGB;
pub const DKRED: RGB;
pub const LTRED: RGB;
pub const BLUE: RGB;
pub const DKBLUE: RGB;
pub const LTBLUE: RGB;
pub const CYAN: RGB;
pub const GREEN: RGB;
pub const DKGREEN: RGB;
pub const LTGREEN: RGB;
pub const PURPLE: RGB;
pub const MAGENTA: RGB;
pub const YELLOW: RGB;
pub const BROWN: RGB;
pub const ORANGE: RGB;
pub const TURQUOISE: RGB;
pub const BLACK: RGB;
pub const WHITE: RGB;
pub const GREY: RGB;
pub const DKGREY: RGB;
pub const LTGREY: RGB;
```

### ray.rs
```rust
pub struct Ray {       // derives Copy
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(Vec3, Vec3) -> Self;            // Create ray from origin and direction
    pub fn project(&self, f64) -> Vec3;        // r.project(t) == r.orig + r.dir * t
}
```

### camera.rs
```rust
pub struct Camera {     // derives Clone
    orig: Vec3,
    low_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new_absolute(
        Vec3,                // Look from
        Vec3,                // Look at
        Vec3,                // Vertical
        f64,                 // Field of view (degrees)
        f64,                 // Aspect ratio
    ) -> Self;

    pub fn new_relative(
        Vec3,                // Look at
        f64,                 // Angle around target (degrees)
        f64,                 // Angle above target (degrees)
        f64,                 // Distance from target
        f64,                 // Lateral tilt (degrees)
        f64,                 // Field of view (degrees)
        f64,                 // Aspect ratio
    ) -> Self;

    pub fn get_ray(&self, f64, f64) -> Ray;   // map [0.; 1.] x [0.; 1.] to rays going out of the camera
}
```
