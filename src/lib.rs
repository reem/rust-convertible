#![license = "MIT"]
#![deny(missing_docs)]
#![deny(warnings)]

//! Defines the `Convertible` trait, for safely attempting to convert
//! types from a raw representation at runtime.

/// Mixin trait for all types that defines the `to` method,
/// which allows checked, runtime conversion to another type that
/// is marked as Convertible from that type.
pub trait Raw {
    /// Checked, runtime conversion of a type to a type that
    /// is convertible from `Self`.
    fn to<C: Convertible<Self>>(&self) -> Option<C>;

    /// Checked, runtime coercion which consumes the passed-in
    /// type. Uses the Coercible trait to do the conversion.
    ///
    /// self is returned inside the Err variant if the coercion fails.
    fn coerce<C: Coercible<Self>>(self) -> Result<C, Self>;
}

impl<T> Raw for T {
    fn to<C: Convertible<T>>(&self) -> Option<C> {
        Convertible::convert(self)
    }

    fn coerce<C: Coercible<T>>(self) -> Result<C, T> {
        Coercible::coerce(self)
    }
}

/// Defines the convert method, which allows checked construction
/// of a type from a type it is convertible from.
///
///
pub trait Convertible<From> {
    /// Try to create an instance of `Self` from `From`.
    fn convert(&From) -> Option<Self>;
}

/// Defines the coerce method, which allows checked conversion
/// of a type from a type that it is coercible from.
///
/// This is like the Convertible trait, except the method
/// takes From by-value.
pub trait Coercible<From> {
    /// Try to convert an instance of `From` to `Self`
    ///
    /// This should return Err(from) if the coercion fails.
    fn coerce(From) -> Result<Self, From>;
}

// Mostly just usability tests.
#[cfg(test)]
mod test {
    use super::{Convertible, Raw};

    struct Raw {
        name: &'static str
    }

    #[deriving(PartialEq, Show)]
    enum NotRaw {
        First, Second, Third
    }

    impl Convertible<Raw> for NotRaw {
        fn convert(raw: &Raw) -> Option<NotRaw> {
            match raw.name {
                "first" => Some(First),
                "second" => Some(Second),
                "third" => Some(Third),
                _ => None
            }
        }
    }

    #[test] fn test_conversion() {
        let first = Raw { name: "first" };
        let second = Raw { name: "second" };
        let malformed = Raw { name: "malformed" };

        assert_eq!(first.to::<NotRaw>(), Some(First));
        assert_eq!(second.to::<NotRaw>(), Some(Second));
        assert_eq!(malformed.to::<NotRaw>(), None);
    }
}

