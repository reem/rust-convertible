#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

//! Defines the `Convertible` trait, for safely attempting to convert
//! types from a raw representation.

/// Mixin trait for all types that defines the `to` method,
/// which allows checked conversion to another type that
/// is marked as Convertible from that type.
pub trait Raw {
    /// Checked, runtime conversion of a type to a type that
    /// is convertible from `Self`.
    fn to<C: Convertible<Self>>(&self) -> Option<C>;
}

impl<T> Raw for T {
    fn to<C: Convertible<T>>(&self) -> Option<C> {
        Convertible::convert(self)
    }
}

/// Defines the convert method, which allows checked construction
/// of a type from a type it is convertible from.
pub trait Convertible<From> {
    /// Try to create an instance of `Self` from `From`.
    fn convert(&From) -> Option<Self>;
}

