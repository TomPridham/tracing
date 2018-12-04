//! Spans represent periods of time in the execution of a program.

/// Identifies a span within the context of a process.
///
/// Span IDs are used primarily to determine of two handles refer to the same
/// span, without requiring the comparison of the span's fields.
///
/// They are generated by [`Subscriber`](::Subscriber)s for each span as it is
/// created, through the [`new_id`](::Subscriber::new_span_id) trait
/// method. See the documentation for that method for more information on span
/// ID generation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Span(u64);

// ===== impl Id =====

impl Span {
    /// Constructs a new span ID from the given `u64`.
    pub fn from_u64(u: u64) -> Self {
        Span(u)
    }
}

#[cfg(any(test, feature = "test-support"))]
pub use self::test_support::*;

#[cfg(any(test, feature = "test-support"))]
mod test_support {
    #![allow(missing_docs)]

    use std::fmt;

    /// A mock span.
    ///
    /// This is intended for use with the mock subscriber API in the
    /// `subscriber` module.
    #[derive(Debug, Default, Eq, PartialEq)]
    pub struct MockSpan {
        pub name: Option<&'static str>,
        // TODO: more
    }

    pub fn mock() -> MockSpan {
        MockSpan {
            ..Default::default()
        }
    }

    impl MockSpan {
        pub fn named(mut self, name: &'static str) -> Self {
            self.name = Some(name);
            self
        }

        // TODO: fields, etc
    }

    impl fmt::Display for MockSpan {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.name {
                Some(name) => write!(f, "a span named {:?}", name),
                None => write!(f, "any span"),
            }
        }
    }
}