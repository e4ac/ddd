/// Errors that can occur when manipulating or constructing a [`Timeline`].
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum TimelineError {
    /// An operation attempted to set `updated_at` that predates `created_at`.
    #[error("updated_at cannot be before created_at")]
    InvalidSequence,
}

/// A tracker representing the lifecycle dates of a single entity.
/// Enforces the invariant that `updated_at >= created_at`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Timeline<T> {
    created_at: T,
    updated_at: T,
}

impl<T: Clone> From<T> for Timeline<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self {
            created_at: value.clone(),
            updated_at: value,
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Timeline<T>
where
    T: Ord + serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Temp<T> {
            created_at: T,
            updated_at: T,
        }

        let t = Temp::deserialize(deserializer)?;
        Self::new(t.created_at, t.updated_at).map_err(serde::de::Error::custom)
    }
}

impl<T: Ord> Timeline<T> {
    /// Creates a new `Timeline` container.
    ///
    /// # Errors
    /// [`TimelineError::InvalidSequence`] when `updated_at < created_at`.
    #[inline]
    pub fn new(created_at: T, updated_at: T) -> Result<Self, TimelineError> {
        if updated_at < created_at {
            Err(TimelineError::InvalidSequence)
        } else {
            Ok(Self { created_at, updated_at })
        }
    }

    /// Updates the `updated_at` to a new value.
    ///
    /// # Errors
    /// [`TimelineError::InvalidSequence`] when `time < created_at`.
    #[inline]
    pub fn update(&mut self, time: T) -> Result<(), TimelineError> {
        if time < self.created_at {
            Err(TimelineError::InvalidSequence)
        } else {
            self.updated_at = time;
            Ok(())
        }
    }

    /// Returns the `created_at` value.
    #[inline]
    #[must_use]
    pub const fn created_at(&self) -> &T {
        &self.created_at
    }

    /// Returns the `updated_at` value.
    #[inline]
    #[must_use]
    pub const fn updated_at(&self) -> &T {
        &self.updated_at
    }
}

impl<T: Clone> Timeline<T> {
    /// Resets `updated_at` to match `created_at`.
    #[inline]
    pub fn reset(&mut self) {
        self.updated_at = self.created_at.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        // arrange
        let expected = 1000;

        // assert
        let time = Timeline::from(expected);

        // assert
        assert_eq!(time.created_at(), &expected);
        assert_eq!(time.updated_at(), &expected);
    }

    #[test]
    fn test_new_valid() -> Result<(), TimelineError> {
        // arrange
        let expected = 1000;

        // act
        let time = Timeline::new(expected, expected)?;

        // assert
        assert_eq!(time.created_at(), &expected);
        assert_eq!(time.updated_at(), &expected);
        Ok(())
    }

    #[test]
    fn test_new_invalid() {
        // arrange
        let expected = 1000;

        // act
        let time = Timeline::new(expected, expected - 100);

        // assert
        assert_eq!(time, Err(TimelineError::InvalidSequence));
    }

    #[test]
    fn test_update_valid() -> Result<(), TimelineError> {
        // arrange
        let old_value = 1000;
        let new_value = old_value + 100;
        let mut time = Timeline::new(old_value, old_value)?;

        // act
        time.update(new_value)?;

        // assert
        assert_eq!(time.created_at(), &old_value);
        assert_eq!(time.updated_at(), &new_value);
        Ok(())
    }

    #[test]
    fn test_update_invalid() -> Result<(), TimelineError> {
        // arrange
        let old_value = 1000;
        let new_value = old_value - 100;
        let mut time = Timeline::new(old_value, old_value)?;

        // act
        let result = time.update(new_value);

        // assert
        assert_eq!(result, Err(TimelineError::InvalidSequence));
        Ok(())
    }

    #[test]
    fn test_deserialize_valid() -> Result<(), serde_json::Error> {
        // arrange
        let expected = 1000;
        let json = r#"{"created_at": 1000, "updated_at": 1000}"#;

        // act
        let result = serde_json::from_str::<Timeline<i32>>(json)?;

        // assert
        assert_eq!(result.created_at(), &expected);
        assert_eq!(result.updated_at(), &expected);
        Ok(())
    }

    #[test]
    fn test_deserialize_invalid() {
        // arrange
        let json = r#"{"created_at": 1000, "updated_at": 500}"#;

        // act
        let result = serde_json::from_str::<Timeline<i32>>(json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_reset() -> Result<(), TimelineError> {
        // arrange
        let created = 1000;
        let updated = 1500;
        let mut time = Timeline::new(created, updated)?;

        // act
        time.reset();

        // assert
        assert_eq!(time.created_at(), &created);
        assert_eq!(time.updated_at(), &created);
        Ok(())
    }
}
