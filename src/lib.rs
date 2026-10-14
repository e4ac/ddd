#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]

pub mod id;
pub mod timeline;

pub use id::Id;
pub use timeline::{Timeline, TimelineError};

/// A value object in Domain-Driven Design.
///
/// Value objects are immutable objects that are defined by their **attributes** rather than a unique identity.
/// Two value objects with identical attributes are considered equal, regardless of whether they are the same instance in memory.
pub trait ValueObject: Eq + Clone {}
impl<T: Eq + Clone> ValueObject for T {}

/// An entity with a unique identifier.
///
/// In Domain-Driven Design, an **Entity** is defined by its identity rather than its attributes.
/// Even if other fields change over time, it remains the same object as long as its unique ID stays the same.
pub trait Entity {
    /// The underlying type used for identity.
    type Id;

    /// Returns a reference to the entity's strongly typed identifier.
    #[must_use]
    fn id(&self) -> &Self::Id;
}

/// An aggregate root is an **Entity** that serves as the entry point and consistency boundary for an aggregate.
///
/// # When to Use
/// - Entities that control other entities
/// - Entities that maintain business invariants
pub trait AggregateRoot: Entity {}

/// Domain events represent facts about state changes in aggregates.
/// They are immutable records of what occurred, enabling event-driven architectures.
///
/// # Type Parameters
/// - `I`: Event identifier type (e.g. `Uuid`, `String`)
/// - `T`: Event timestamp type (e.g. `i64`, `DateTime`)
pub trait DomainEvent<I, T>: Clone {
    /// Returns a reference to this event's identifier.
    #[must_use]
    fn event_id(&self) -> &I;

    /// Returns a reference to when this event occurred.
    #[must_use]
    fn event_time(&self) -> &T;
}
