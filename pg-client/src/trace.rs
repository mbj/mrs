#[cfg(feature = "tracing")]
pub(crate) use tracing::{Instrument, Span, debug_span, field};

#[cfg(not(feature = "tracing"))]
mod disabled {
    #[derive(Clone, Copy, Debug)]
    pub(crate) struct Span;

    impl Span {
        pub(crate) const fn none() -> Self {
            Self
        }

        pub(crate) const fn current() -> Self {
            Self
        }

        pub(crate) fn record<V>(&self, _field: &str, _value: V) -> &Self {
            self
        }

        pub(crate) const fn is_disabled(&self) -> bool {
            true
        }
    }

    pub(crate) trait Instrument: Sized {
        fn instrument(self, _span: Span) -> Self {
            self
        }
    }

    impl<T> Instrument for T {}

    macro_rules! debug_span {
        (parent: $parent:expr, $($ignored:tt)*) => {{
            let _: &$crate::trace::Span = $parent;
            $crate::trace::Span
        }};
        ($($ignored:tt)*) => {
            $crate::trace::Span
        };
    }

    pub(crate) use debug_span;
}

#[cfg(not(feature = "tracing"))]
pub(crate) use disabled::{Instrument, Span, debug_span};
