use crate::result::Error;
use crate::rule::Rule;
use crate::Refined;
use regex::Regex;

/// A type that holds a value satisfying the `AlphaDigitRule`
pub type AlphaDigit = Refined<AlphaDigitRule>;

/// Rule where the input `String` contains only alphabet and digit characters
pub struct AlphaDigitRule;

impl Rule for AlphaDigitRule {
    type Item = String;
    fn validate(target: &Self::Item) -> Result<(), Error> {
        let regex = Regex::new(r"[0-9a-zA-Z]*").expect("Invalid regex");
        let is_valid = regex
            .find(target.as_str())
            .is_some_and(|matched| matched.as_str() == target.as_str());
        if is_valid {
            Ok(())
        } else {
            Err(Error::new(
                "The input `String` have some alpha_digit characters",
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::rule::AlphaDigit;

    #[test]
    fn test_alpha_digit_ok_1() {
        let alpha_digit = AlphaDigit::new("1234567890".to_string());
        assert!(alpha_digit.is_ok());
    }

    #[test]
    fn test_alpha_digit_ok_2() {
        let alpha_digit = AlphaDigit::new("".to_string());
        assert!(alpha_digit.is_ok());
    }

    #[test]
    fn test_alpha_digit_ok_3() {
        let alpha_digit = AlphaDigit::new("1234567890abc".to_string());
        assert!(alpha_digit.is_ok());
    }

    #[test]
    fn test_alpha_digit_err() {
        let alpha_digit = AlphaDigit::new("1234567890abcこんにちは".to_string());
        assert!(alpha_digit.is_err());
    }
}
