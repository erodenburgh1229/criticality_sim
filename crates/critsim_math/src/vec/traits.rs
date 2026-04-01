// Uses self and Self ensure that the types for the functions in the trait are guaranteed to be the same type
// self -> the object the function is operating on
// Self -> the type of the object being operated on

pub trait VectorOps {
    type Scalar;    // Allows this type to vary

    // Core arithmetic
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mult_scalar(&self, scalar: Self::Scalar) -> Self;
    fn neg(&self) -> Self;
}