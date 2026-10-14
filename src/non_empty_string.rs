use std::fmt::Display;

/// Errors that can occur when manipulating or constructing a [`NonEmptyString`] and [`NonEmptyStringAllowWhitespace`].
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NonEmptyStringError {
    /// An operation attempted to create an empty string.
    #[error("empty string")]
    Empty,

    /// An operation attempted to create a whitespace-only string.
    #[error("whitespace only string")]
    WhitespaceOnly,
}

/// A string that is not empty.
///
/// # Type Parameters
/// - `ALLOW_WHITESPACE`: When `true`, allows whitespace-only strings. When `false`, rejects whitespace-only strings.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
pub struct NonEmptyStringBase<const ALLOW_WHITESPACE: bool>(String);

/// A non-empty string that rejects whitespace-only input.
pub type NonEmptyString = NonEmptyStringBase<false>;

/// A non-empty string that allows whitespace-only input.
pub type NonEmptyStringAllowWhitespace = NonEmptyStringBase<true>;

impl<const ALLOW_WHITESPACE: bool> AsRef<str> for NonEmptyStringBase<ALLOW_WHITESPACE> {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<const ALLOW_WHITESPACE: bool> Display for NonEmptyStringBase<ALLOW_WHITESPACE> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<const ALLOW_WHITESPACE: bool> TryFrom<String> for NonEmptyStringBase<ALLOW_WHITESPACE> {
    type Error = NonEmptyStringError;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const ALLOW_WHITESPACE: bool> TryFrom<&str> for NonEmptyStringBase<ALLOW_WHITESPACE> {
    type Error = NonEmptyStringError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const ALLOW_WHITESPACE: bool> std::ops::Deref for NonEmptyStringBase<ALLOW_WHITESPACE> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const ALLOW_WHITESPACE: bool> NonEmptyStringBase<ALLOW_WHITESPACE> {
    /// Creates a new [`NonEmptyStringBase`].
    ///
    /// # Errors
    /// - [`NonEmptyStringError::Empty`] when input is empty.
    /// - [`NonEmptyStringError::WhitespaceOnly`] when input is whitespace only and `ALLOW_WHITESPACE` is `false`.
    #[inline]
    pub fn new(value: impl Into<String>) -> Result<Self, NonEmptyStringError> {
        let value = value.into();
        if value.is_empty() {
            Err(NonEmptyStringError::Empty)
        } else if !ALLOW_WHITESPACE && value.trim().is_empty() {
            Err(NonEmptyStringError::WhitespaceOnly)
        } else {
            Ok(Self(value))
        }
    }

    /// Consumes this [`NonEmptyStringBase`] and returns the [`String`] value.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() -> Result<(), NonEmptyStringError> {
        // arrange
        let str = "Hello, World!";

        // act
        let result: NonEmptyString = NonEmptyString::new(str)?;

        // assert
        assert_eq!(result.as_ref(), str);
        Ok(())
    }

    #[test]
    fn test_new_empty() {
        // arrange
        let str = "";

        // act
        let result = NonEmptyString::new(str);

        // assert
        assert_eq!(result, Err(NonEmptyStringError::Empty));
    }

    #[test]
    fn test_new_whitespace() {
        // arrange
        let str = "\t \n";

        // act
        let result = NonEmptyString::new(str);

        // assert
        assert_eq!(result, Err(NonEmptyStringError::WhitespaceOnly));
    }

    #[test]
    fn test_new_allow_whitespace() -> Result<(), NonEmptyStringError> {
        // arrange
        let str = "Hello, World!";

        // act
        let result = NonEmptyStringAllowWhitespace::new(str)?;

        // assert
        assert_eq!(result.as_ref(), str);
        Ok(())
    }

    #[test]
    fn test_new_allow_whitespace_empty() {
        // arrange
        let str = "";

        // act
        let result = NonEmptyStringAllowWhitespace::new(str);

        // assert
        assert_eq!(result, Err(NonEmptyStringError::Empty));
    }

    #[test]
    fn test_new_allow_whitespace_whitespace() -> Result<(), NonEmptyStringError> {
        // arrange
        let str = "\t \n";

        // act
        let result = NonEmptyStringAllowWhitespace::new(str)?;

        // assert
        assert_eq!(result.as_ref(), str);
        Ok(())
    }

    #[test]
    fn test_into_inner() -> Result<(), NonEmptyStringError> {
        // arrange
        let str = "Hello, World!";
        let non_empty: NonEmptyString = NonEmptyString::new(str)?;

        // act
        let result = non_empty.into_inner();

        // assert
        assert_eq!(result, str);
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<(), NonEmptyStringError> {
        // arrange
        let str = "Hello, World!";

        // act
        let result: NonEmptyString = NonEmptyString::try_from(str)?;

        // assert
        assert_eq!(result.as_ref(), str);
        Ok(())
    }

    #[test]
    fn test_try_from_str_empty() {
        // arrange
        let str = "";

        // act
        let result = NonEmptyString::try_from(str);

        // assert
        assert_eq!(result, Err(NonEmptyStringError::Empty));
    }

    #[test]
    fn test_try_from_str_whitespace() {
        // arrange
        let str = "\t \n";

        // act
        let result = NonEmptyString::try_from(str);

        // assert
        assert_eq!(result, Err(NonEmptyStringError::WhitespaceOnly));
    }

    #[test]
    fn test_try_from_string() -> Result<(), NonEmptyStringError> {
        // arrange
        let str = "Hello, World!".to_owned();

        // act
        let result: NonEmptyString = NonEmptyString::try_from(str)?;

        // assert
        assert_eq!(result.as_ref(), "Hello, World!");
        Ok(())
    }

    #[test]
    fn test_try_from_string_empty() {
        // arrange
        let str = String::new();

        // act
        let result = NonEmptyString::try_from(str);

        // assert
        assert_eq!(result, Err(NonEmptyStringError::Empty));
    }

    #[test]
    fn test_try_from_string_whitespace() {
        // arrange
        let str = "\t \n".to_owned();

        // act
        let result = NonEmptyString::try_from(str);

        // assert
        assert_eq!(result, Err(NonEmptyStringError::WhitespaceOnly));
    }

    #[test]
    fn test_try_from_str_allow_whitespace() -> Result<(), NonEmptyStringError> {
        // arrange
        let str = "Hello, World!";

        // act
        let result = NonEmptyStringAllowWhitespace::try_from(str)?;

        // assert
        assert_eq!(result.as_ref(), str);
        Ok(())
    }

    #[test]
    fn test_try_from_str_allow_whitespace_empty() {
        // arrange
        let str = "";

        // act
        let result = NonEmptyStringAllowWhitespace::try_from(str);

        // assert
        assert_eq!(result, Err(NonEmptyStringError::Empty));
    }

    #[test]
    fn test_try_from_str_allow_whitespace_whitespace() -> Result<(), NonEmptyStringError> {
        // arrange
        let str = "\t \n";

        // act
        let result = NonEmptyStringAllowWhitespace::try_from(str)?;

        // assert
        assert_eq!(result.as_ref(), str);
        Ok(())
    }

    #[test]
    fn test_try_from_string_allow_whitespace() -> Result<(), NonEmptyStringError> {
        // arrange
        let str = "Hello, World!".to_owned();

        // act
        let result = NonEmptyStringAllowWhitespace::try_from(str)?;

        // assert
        assert_eq!(result.as_ref(), "Hello, World!");
        Ok(())
    }

    #[test]
    fn test_try_from_string_allow_whitespace_empty() {
        // arrange
        let str = String::new();

        // act
        let result = NonEmptyStringAllowWhitespace::try_from(str);

        // assert
        assert_eq!(result, Err(NonEmptyStringError::Empty));
    }

    #[test]
    fn test_try_from_string_allow_whitespace_whitespace() -> Result<(), NonEmptyStringError> {
        // arrange
        let str = "\t \n".to_owned();

        // act
        let result = NonEmptyStringAllowWhitespace::try_from(str)?;

        // assert
        assert_eq!(result.as_ref(), "\t \n");
        Ok(())
    }

    #[test]
    fn test_deref() -> Result<(), NonEmptyStringError> {
        // arrange
        let str = "Hello, World!";
        let non_empty = NonEmptyString::new(str)?;

        // act
        let result: &str = &non_empty;

        // assert
        assert_eq!(result, str);
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_serialize() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let str = "Hello, World!";
        let expected = r#""Hello, World!""#;
        let non_empty: NonEmptyString = NonEmptyString::new(str)?;

        // act
        let result = serde_json::to_string(&non_empty)?;

        // assert
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let json = r#""Hello, World!""#;
        let expected = "Hello, World!";

        // act
        let result: NonEmptyString = serde_json::from_str(json)?;

        // assert
        assert_eq!(result.as_ref(), expected);
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_deserialize_empty() {
        // arrange
        let json = r#""""#;

        // act
        let result = serde_json::from_str::<NonEmptyString>(json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_deserialize_whitespace() {
        // arrange
        let json = r#""\t \n""#;

        // act
        let result = serde_json::from_str::<NonEmptyString>(json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_serialize_allow_whitespace() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let str = "Hello, World!";
        let expected = r#""Hello, World!""#;
        let non_empty = NonEmptyStringAllowWhitespace::new(str)?;

        // act
        let result = serde_json::to_string(&non_empty)?;

        // assert
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_deserialize_allow_whitespace() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let json = r#""Hello, World!""#;
        let expected = "Hello, World!";

        // act
        let result = serde_json::from_str::<NonEmptyStringAllowWhitespace>(json)?;

        // assert
        assert_eq!(result.as_ref(), expected);
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_deserialize_allow_whitespace_empty() {
        // arrange
        let json = r#""""#;

        // act
        let result = serde_json::from_str::<NonEmptyStringAllowWhitespace>(json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_deserialize_allow_whitespace_whitespace() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let json = r#""\t \n""#;

        // act
        let result = serde_json::from_str::<NonEmptyStringAllowWhitespace>(json)?;

        // assert
        assert_eq!(result.as_ref(), "\t \n");
        Ok(())
    }
}
