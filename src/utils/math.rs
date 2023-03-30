pub fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
    if dividend == 0 {
        return false;
    }

    dividend % divisor == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_devisible_by() {
        assert_eq!(is_divisible_by(0, 1), false);
        assert_eq!(is_divisible_by(1, 1), true);
        assert_eq!(is_divisible_by(3, 2), false);
        assert_eq!(is_divisible_by(4, 2), true);

    }
}
