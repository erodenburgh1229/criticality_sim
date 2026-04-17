use num_traits::Float;

use crate::vec::traits::FloatVectorOps;
use crate::vec::traits::VectorOps;

// This makes the Vec2 struct copiable, cloneable, and easier to debug
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vec2<T> {
    pub fn new(_x: T, _y: T) -> Self {
        Self { x: _x, y: _y }
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

    fn div(self, scalar: T) -> Self {
        Vec2 {
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

// ------ VectorOps Trait ------
// Restricting this impl block to float types only
impl<T> VectorOps for Vec2<T>
where 
    T: Copy + std::ops::Add<Output=T> + std::ops::Mul<Output=T>
{
    type Scalar = T;

    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y + other.y
    }

    fn length_squared(&self) -> T {
        self.dot(self)
    }
}

// ------ FloatVectorOps Trait ------
impl<T> FloatVectorOps for Vec2<T>
where 
    T: Copy + std::ops::Add<Output=T> + std::ops::Mul<Output=T> + Float
{
    type Scalar = T;

    fn length(&self) -> Self::Scalar {
        self.length_squared().sqrt()
    }
}

// ------ Tests ------
#[cfg(test)]
mod tests {
    mod add {
        use std::f32::EPSILON;

        use super::super::*;
        #[test]
        fn add_positives_i32() {
            let a = Vec2::new(1, 2);
            let b = Vec2::new(1, 2);

            assert_eq!(a + b, Vec2::new(2, 4));
        }

        #[test]
        fn add_zeroes_i32() {
            let a = Vec2::new(1, 2);
            let b = Vec2::new(0, 0);

            assert_eq!(a + b, Vec2::new(1, 2));
        }

        #[test]
        fn add_negatives_i32() {
            let a = Vec2::new(1, 2);
            let b = Vec2::new(-1, -2);

            assert_eq!(a + b, Vec2::new(0, 0));
        }

        #[test]
        fn add_positives_f32() {
            let a = Vec2::new(1.25, 2.0);
            let b = Vec2::new(1.0, 2.25);

            let result = a + b;
            let expected = Vec2::new(2.25, 4.25);

            let diff = result - expected;

            assert!(diff.x.abs() < EPSILON && diff.y.abs() < EPSILON);
        }

        #[test]
        fn add_negatives_f32(){
            let a = Vec2::new(1.25, 2.0);
            let b = Vec2::new(-1.0, -2.25);

            let result = a + b;
            let expected = Vec2::new(0.25, -0.25);

            let diff = result - expected;

            assert!(diff.x.abs() < EPSILON && diff.y.abs() < EPSILON);
        }

        #[test]
        fn add_zeroes_f32(){
            let a = Vec2::new(1.25, 2.0);
            let b = Vec2::new(0.0, 0.0);

            let result = a + b;
            let expected = Vec2::new(1.25, 2.0);

            let diff = result - expected;

            assert!(diff.x.abs() < EPSILON && diff.y.abs() < EPSILON);
        }
    }

    mod sub{
        use std::f32::EPSILON;

        use super::super::*;

        #[test]
        fn sub_positive_i32(){
            let a = Vec2::new(4, 3);
            let b = Vec2::new(1, 2);

            assert_eq!(a - b, Vec2::new(3, 1));
        }

        #[test]
        fn sub_negatives_i32(){
            let a = Vec2::new(4, 3);
            let b = Vec2::new(-1, -2);

            assert_eq!(a - b, Vec2::new(5, 5));
        }

        #[test]
        fn sub_zeroes_i32(){
            let a = Vec2::new(4, 3);
            let b = Vec2::new(0, 0);

            assert_eq!(a - b, Vec2::new(4, 3));
        }

        #[test]
        fn sub_positive_f32(){
            let a = Vec2::new(4.0, 3.0);
            let b = Vec2::new(1.0, 2.0);

            let result = a - b;
            let expected = Vec2::new(3.0, 1.0);
            let diff = result - expected;

            assert!(diff.x.abs() < EPSILON && diff.y.abs() < EPSILON);
        }

        #[test]
        fn sub_negatives_f32(){
            let a = Vec2::new(4.0, 3.0);
            let b = Vec2::new(-1.0, -1.0);

            let result = a - b;
            let expected = Vec2::new(5.0, 4.0);
            let diff = result - expected;

            assert!(diff.x.abs() < EPSILON && diff.y.abs() < EPSILON);
        }

        #[test]
        fn sub_zeroes_f32(){
            let a = Vec2::new(4.0, 3.0);
            let b = Vec2::new(0.0, 0.0);

            let result = a - b;
            let diff = a - result;

            assert!(diff.x.abs() < EPSILON && diff.y.abs() < EPSILON);
        }
    }

    mod mul{
        use super::super::*;

        #[test]
        fn mul_positive_i32(){
            let a = Vec2::new(2, 3);
            let b = 2;

            assert_eq!(a * b, Vec2::new(4, 6));
        }

        #[test]
        fn mul_negative_i32(){
            let a = Vec2::new(2, 3);
            let b = -3;

            assert_eq!(a * b, Vec2::new(-6, -9));
        }

        #[test]
        fn mul_identity_i32(){
            let a = Vec2::new(2, 3);
            let b = 1;

            assert_eq!(a * b, a);
        }

        #[test]
        fn mul_zero_i32(){
            let a = Vec2::new(2, 3);
            let b = 0;

            assert_eq!(a * b, Vec2::new(0, 0));
        }
    }
}
