//! Parsing infrastructure for validated types.

use nom::IResult;
use nom_language::error::VerboseError;

/// A type that can be parsed from a string using nom.
pub trait Parse: Sized {
    /// Parses the type from a string.
    fn parse(remaining: &str) -> IResult<&str, Self, VerboseError<&str>>;
}

/// Implements `FromStr` for types that implement `Parse`.
#[macro_export]
macro_rules! impl_from_str {
    ($($type:ty),* $(,)?) => {
        $(
            impl std::str::FromStr for $type {
                type Err = String;

                fn from_str(string: &str) -> Result<Self, Self::Err> {
                    match nom::combinator::all_consuming(<Self as $crate::parse::Parse>::parse).parse(string) {
                        Ok((_remaining, value)) => Ok(value),
                        Err(error) => Err(format!("parse error: {error}")),
                    }
                }
            }
        )*
    };
}
