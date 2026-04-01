use crate::vec::traits::VectorOps;
use num_traits::Float;

// This makes the Vec2 struct copiable, cloneable, and easier to debug
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vec2<T> {

    pub fn new(_x: T, _y: T) -> Self {
        Self {x: _x, y: _y}
    }
}

// Restrict these functions to types that implement the num_traits::Float traits
impl<T> Vec2<T>
where 
    T: Copy + std::ops::Div<Output=T> + Float
{
    pub fn div_scalar(&self, scalar: T) -> Self {
        Vec2 {
            x: self.x / scalar, 
            y: self.y / scalar 
        }
    }
}

// The Copy trait says that a types values can be implicitly copied because no destructor or heap allocation is needed
// Types that are copy are implicitly copied when assigned or passed to functions
// Neg function could be moved out of this block if we want to allow for non-number types to be stored and negated (booleans)
impl <T> VectorOps for Vec2<T> 
where 
    T: Copy + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::ops::Neg<Output=T>
{
    type Scalar = T;
    fn add(&self, other: &Self) -> Self {
        Vec2 { 
            x: self.x + other.x, 
            y: self.y + other.y 
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Vec2 { 
            x: self.x - other.x,
            y: self.y - other.y 
         }
    }

    fn mult_scalar(&self, scalar: Self::Scalar) -> Self {
        Vec2 { 
            x: self.x * scalar, 
            y: self.y * scalar 
        }
    }

    fn neg(&self) -> Self {
        Vec2 {
            x: -self.x,
            y: -self.y
        }
    }
}