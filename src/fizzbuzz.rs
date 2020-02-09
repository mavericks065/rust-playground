fn getResult(number: u16) -> String {
    return String::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_fizz_if_divisible_by_3() {
        // Given
        let input = 6;

        // When
        let result = getResult(input);

        // Then
        assert_eq!(result, "fizz");
    }
}