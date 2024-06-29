// Define a trait named `IsEven` that has a method `is_even` that returns a `true` if `self` is
// even, otherwise `false`.
//
// Then implement the trait for `u32` and `i32`.

trait IsEven {
    fn is_even(self) -> bool;
}

fn is_even<T>(number: T) -> bool
where
    T: std::ops::Rem<Output = T> + PartialEq + From<u8>,
{
    number % T::from(2u8) == T::from(0u8)
}

impl IsEven for u32 {
    fn is_even(self) -> bool {
        is_even(self)
    }
}

impl IsEven for i32 {
    fn is_even(self) -> bool {
        is_even(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42u32.is_even());
        assert!(!43u32.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42i32.is_even());
        assert!(!43i32.is_even());
        assert!(0i32.is_even());
        assert!(!(-1i32).is_even());
    }
}
