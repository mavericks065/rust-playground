fn get_result(number: u16) -> String {
    if number % 15 == 0 {
        return String::from("fizzbuzz");
    }
    if number % 3 == 0 {
        return String::from("fizz");
    }
    if number % 5 == 0 {
        return String::from("buzz");
    }
    return number.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_fizz_if_divisible_by_3() {
        // Given
        let input = 6;

        // When
        let result = get_result(input);

        // Then
        assert_eq!(result, "fizz");
    }

    #[test]
    fn should_return_buzz_if_divisible_by_5() {
        // Given
        let input = 10;

        // When
        let result = get_result(input);

        // Then
        assert_eq!(result, "buzz");
    }

    #[test]
    fn should_return_fizzbuzz_if_divisible_by_15() {
        // Given
        let input = 30;

        // When
        let result = get_result(input);

        // Then
        assert_eq!(result, "fizzbuzz");
    }

    #[test]
    fn should_return_nunmber_as_string_otherwise() {
        // Given
        let input = 4;

        // When
        let result = get_result(input);

        // Then
        assert_eq!(result, "4");
    }
}