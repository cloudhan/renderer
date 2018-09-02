//#[macro_use]
//extern crate approx;

extern crate nalgebra as na;
extern crate num_traits;
use self::na::*;

//pub use std::f64;
pub use self::num_traits::Float;

#[allow(non_camel_case_types)]
pub type scalar = f64;
pub type Vec3 = Vector3<scalar>;


trait ColorAccessor
{
    
}