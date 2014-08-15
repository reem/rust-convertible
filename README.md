# Convertible

> Checked, typesafe, ergonomic runtime conversion of types.

## Overview

Defines the `Convertible<From>` trait, which defines a `convert`
method from `&From` to `Option<Self>`.

Additionally, it defines the universal mixin trait `Raw`, which
adds the `to` method to all types to allow for ergonomic usage
of these conversions.

This can be used to implement extensible enums based on a raw
representation that variants can be created from

## Example

```rust
use convertible::{Convertible, Raw};

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
```

