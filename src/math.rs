//#[macro_use]
//extern crate approx;
extern crate nalgebra as na;

use self::na::*;

pub type scalar = f64;
pub type Point = Vector3<scalar>;
pub type Direction = Vector3<scalar>;
pub type RGB = Vector3<scalar>;

trait ColorAccessor
{
    
}