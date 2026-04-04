// Uses self and Self ensure that the types for the functions in the trait are guaranteed to be the same type
// self -> the object the function is operating on
// Self -> the type of the object being operated on

pub trait VectorOps {
    type Scalar;    // Allows this type to vary

    // Core arithmetic
    fn dot(&self, other: Self) -> Self::Scalar;
    fn length_squared(&self) -> Self::Scalar;
}