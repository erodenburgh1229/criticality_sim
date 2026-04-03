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

// ------ Basic math operator traits ------
// Implement the Add trait for cleaner and more idiomatic API approach
// The same is being done for Sub, Mul (with scalar only), Div (with scalar only)
use std::ops::Add;

impl<T> Add for Vec2<T>
where 
    T: Copy + std::ops::Add<Output=T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2 { 
            x: self.x + other.x, 
            y: self.y + other.y
        }
    }
}

use std::ops::Sub;

impl<T> Sub for Vec2<T> 
where 
    T: Copy + std::ops::Sub<Output=T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec2 { 
            x: self.x - other.x, 
            y: self.y - other.y 
        }
    }
}

// Because we are implementing the Mul trait for only scalars, we need to constrain the rhs type
// to be the same type as the data held within Vec2<T>
use std::ops::Mul;

impl<T> Mul<T> for Vec2<T>
where 
    T: Copy + std::ops::Mul<Output=T>
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Vec2 { 
            x: self.x * scalar, 
            y: self.y * scalar 
        }
    }
}

use std::ops::Div;

impl<T> Div<T> for Vec2<T>
where 
    T: Copy + std::ops::Div<Output=T>
{
    type Output = Self;

    fn div(self, scalar: T) -> Self{
        Vec2{
            x: self.x / scalar,
            y: self.y / scalar
        }  
    }
}

use std::ops::Neg;

impl<T> Neg for Vec2<T>
where
    T: Copy + std::ops::Neg<Output=T>
{
    type Output = Self;
    fn neg(self) -> Self {
        Vec2 {
            x: -self.x,
            y: -self.y
        }
    }
}

// ------ Tests ------
#[cfg(test)]
mod tests {
    mod add {
        use super::super::*;
        #[test]
        fn add_positives_i32(){
            let a = Vec2::new(1, 2);
            let b = Vec2::new(1, 2);

            assert_eq!(a + b, Vec2::new(2, 4));
        }

        #[test]
        fn add_zeroes_i32(){
            let a = Vec2::new(1, 2);
            let b = Vec2::new(0, 0);

            assert_eq!(a + b, Vec2::new(1, 2));
        }

        #[test]
        fn add_negatives_i32(){
            let a = Vec2::new(1, 2);
            let b = Vec2::new(-1, -2);

            assert_eq!(a + b, Vec2::new(0,0));
        }
    }

}