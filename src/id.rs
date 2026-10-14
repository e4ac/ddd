use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[cfg(feature = "uuid")]
use uuid::Uuid;

/// A strongly typed identifier wrapper bound to a specific entity type.
/// Use this to prevent accidental mixing of IDs between different entity types (e.g. mixing `UserId` and `ProductId`).
pub struct Id<T, K> {
    value: K,
    _marker: PhantomData<fn() -> T>,
}

impl<T, K: Debug> Debug for Id<T, K> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

impl<T, K: Display> Display for Id<T, K> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl<T, K: Default> Default for Id<T, K> {
    #[inline]
    fn default() -> Self {
        Self::new(K::default())
    }
}

impl<T, K: Copy> Copy for Id<T, K> {}
impl<T, K: Clone> Clone for Id<T, K> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T, K: Eq> Eq for Id<T, K> {}
impl<T, K: PartialEq> PartialEq for Id<T, K> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<T, K: PartialOrd> PartialOrd for Id<T, K> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T, K: Ord> Ord for Id<T, K> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T, K: Hash> Hash for Id<T, K> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T, K> From<K> for Id<T, K> {
    #[inline]
    fn from(value: K) -> Self {
        Self::new(value)
    }
}

impl<T, K> AsRef<K> for Id<T, K> {
    #[inline]
    fn as_ref(&self) -> &K {
        &self.value
    }
}

#[cfg(feature = "serde")]
impl<T, K: serde::Serialize> serde::Serialize for Id<T, K> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T, K: serde::Deserialize<'de>> serde::Deserialize<'de> for Id<T, K> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        K::deserialize(deserializer).map(Self::new)
    }
}

impl<T, K> Id<T, K> {
    /// Creates a new strongly typed identifier.
    #[inline]
    #[must_use]
    pub const fn new(value: K) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    /// Consumes the identifier, returning its underlying value.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> K {
        self.value
    }
}

#[cfg(feature = "uuid")]
impl<T> Id<T, Uuid> {
    /// Creates a new identifier with a random version 4 UUID.
    #[inline]
    #[must_use]
    pub fn new_v4() -> Self {
        Self::new(Uuid::new_v4())
    }

    /// Creates a new identifier with a random version 7 UUID.
    #[inline]
    #[must_use]
    pub fn now_v7() -> Self {
        Self::new(Uuid::now_v7())
    }

    /// Creates a new identifier with a nil UUID.
    #[inline]
    #[must_use]
    pub const fn nil() -> Self {
        Self::new(Uuid::nil())
    }

    /// Creates a new identifier from a string representation.
    #[inline]
    pub fn parse_str(value: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(value).map(Self::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        // arrange
        let id = Id::<(), String>::new(String::from("id"));

        // act
        let clone = id.clone();

        // assert
        assert_eq!(id, clone);
    }

    #[test]
    fn test_eq() {
        // arrange
        let id1 = Id::<(), String>::new(String::from("id"));
        let id2 = Id::<(), String>::new(String::from("id"));
        let id3 = Id::<(), String>::new(String::from("different_id"));

        // assert
        assert_eq!(id1, id2);
        assert_eq!(id1, id1);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_cmp() {
        // arrange
        let id1 = Id::<(), i32>::new(1);
        let id2 = Id::<(), i32>::new(2);
        let id3 = Id::<(), i32>::new(3);
        let id4 = Id::<(), i32>::new(3);

        // assert
        assert!(id1 < id2);
        assert!(id2 >= id1);
        assert!(id2 < id3);
        assert!(id3 >= id2);
        assert!(id3 <= id4);
        assert!(id3 >= id4);
    }

    #[test]
    fn test_partial_cmp() {
        // arrange
        let id1 = Id::<(), i32>::new(1);
        let id2 = Id::<(), i32>::new(2);
        let id3 = Id::<(), i32>::new(1);

        // assert
        assert_eq!(id1.partial_cmp(&id2), Some(Ordering::Less));
        assert_eq!(id2.partial_cmp(&id1), Some(Ordering::Greater));
        assert_eq!(id1.partial_cmp(&id3), Some(Ordering::Equal));
    }

    #[test]
    fn test_partial_cmp_nan() {
        // arrange
        let id_num = Id::<(), f32>::new(1.0f32);
        let id_nan = Id::<(), f32>::new(f32::NAN);

        // assert
        assert_eq!(id_num.partial_cmp(&id_nan), None);
        assert_eq!(id_nan.partial_cmp(&id_num), None);
        assert_eq!(id_nan.partial_cmp(&id_nan), None);
    }

    #[test]
    fn test_from() {
        // arrange
        let expected = "id_123";

        // act
        let id: Id<(), String> = Id::from(String::from(expected));

        // assert
        assert_eq!(id.as_ref(), expected);
    }

    #[test]
    fn test_into_inner() {
        // arrange
        let expected = String::from("id_123");
        let id = Id::<(), String>::new(expected.clone());

        // act
        let result = id.into_inner();

        // assert
        assert_eq!(result, expected);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialize() -> Result<(), serde_json::Error> {
        // arrange
        let num = 42;
        let id = Id::<(), i64>::new(num);

        // act
        let json = serde_json::to_string(&id)?;

        // assert
        assert_eq!(json, num.to_string());
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_deserialize() -> Result<(), serde_json::Error> {
        // arrange
        let num = 42;

        // act
        let result = serde_json::from_str::<Id<(), i32>>(&num.to_string())?;

        // assert
        assert_eq!(result.into_inner(), num);
        Ok(())
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_new_v4() {
        // act
        let id = Id::<(), Uuid>::new_v4();

        // assert
        assert_eq!(id.into_inner().get_version(), Some(uuid::Version::Random));
        assert_ne!(id.into_inner(), Uuid::nil());
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_now_v7() {
        // act
        let id = Id::<(), Uuid>::now_v7();

        // assert
        assert_eq!(id.into_inner().get_version(), Some(uuid::Version::SortRand));
        assert_ne!(id.into_inner(), Uuid::nil());
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_nil() {
        // act
        let id = Id::<(), Uuid>::nil();

        // assert
        assert_eq!(id.into_inner(), Uuid::nil());
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_parse_str_valid() -> Result<(), uuid::Error> {
        // arrange
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let expected = Uuid::parse_str(uuid_str)?;

        // act
        let id = Id::<(), Uuid>::parse_str(uuid_str)?;

        // assert
        assert_eq!(id.into_inner(), expected);
        Ok(())
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_parse_str_nil() -> Result<(), uuid::Error> {
        // arrange
        let uuid_str = "00000000-0000-0000-0000-000000000000";

        // act
        let id = Id::<(), Uuid>::parse_str(uuid_str)?;

        // assert
        assert_eq!(id.into_inner(), Uuid::nil());
        Ok(())
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_parse_str_uppercase() -> Result<(), uuid::Error> {
        // arrange
        let uuid_str = "550E8400-E29B-41D4-A716-446655440000";
        let expected = Uuid::parse_str(uuid_str)?;

        // act
        let id = Id::<(), Uuid>::parse_str(uuid_str)?;

        // assert
        assert_eq!(id.into_inner(), expected);
        Ok(())
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_parse_str_invalid_format() {
        // arrange
        let invalid_str = "not-a-uuid";

        // act
        let result = Id::<(), Uuid>::parse_str(invalid_str);

        // assert
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_parse_str_empty() {
        // arrange
        let empty_str = "";

        // act
        let result = Id::<(), Uuid>::parse_str(empty_str);

        // assert
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_parse_str_wrong_length() {
        // arrange
        let short_str = "550e8400-e29b-41d4-a716";

        // act
        let result = Id::<(), Uuid>::parse_str(short_str);

        // assert
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_parse_str_missing_hyphens() -> Result<(), uuid::Error> {
        // arrange
        let no_hyphens = "550e8400e29b41d4a716446655440000";
        let expected = Uuid::parse_str(no_hyphens)?;

        // act
        let id = Id::<(), Uuid>::parse_str(no_hyphens)?;

        // assert
        assert_eq!(id.into_inner(), expected);
        Ok(())
    }
}
