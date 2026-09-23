// Test functions for the calculator library src/lib.rs
use calc::add;
use calc::subtract;
use calc::multiply;
use calc::divide;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2.0, 3.0), 6.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6.0, 2.0), Ok(3.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(6.0, 0.0), Err("Denominator cannot be zero"));
    }
}