//#[macro_use]
//extern crate approx;
extern crate nalgebra as na;

use self::na::*;
pub use std::f64;

pub type scalar = f64;
pub type Vec3 = Vector3<scalar>;


trait ColorAccessor
{
    
}