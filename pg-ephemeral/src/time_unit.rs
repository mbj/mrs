//! Values in PostgreSQL's time-unit parameter syntax.
//!
//! [`Value`] accepts exactly what PostgreSQL's `parse_int` accepts for an
//! integer parameter with a time unit, such as `statement_timeout`, and
//! [`Value::to_milliseconds`] converts it exactly as `convert_to_base_unit` and
//! `parse_int` do for such a parameter with base unit milliseconds.
//!
//! The number is read the way C's `strtol` reads it with base 0, falling back
//! to `strtod` when that stops at `.`, `e` or `E` or overflows. So leading
//! whitespace, a sign, octal (`010` is 8), hexadecimal (`0x1e`), decimal
//! fractions and exponents (`1.5`, `1e3`) and hexadecimal fractions (`0x1.8`)
//! are all accepted. Whitespace may follow the number and the unit. The unit is
//! one of `us`, `ms`, `s`, `min`, `h` or `d`, case-sensitive, or absent for the
//! parameter's base unit.
//!
//! `strtol` and `strtod` behave as glibc's do on a 64-bit platform, as in the
//! official PostgreSQL images.
//!
//! PostgreSQL's parser is `parse_int` and `convert_to_base_unit` in
//! <https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/utils/misc/guc.c>.

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag_no_case, take_while, take_while_m_n, take_while1},
    character::complete::{char, digit0, digit1, one_of},
    combinator::{eof, opt, recognize, verify},
    sequence::{pair, preceded},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
}

impl Unit {
    const ALL: [Self; 6] = [
        Self::Microsecond,
        Self::Millisecond,
        Self::Second,
        Self::Minute,
        Self::Hour,
        Self::Day,
    ];

    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Microsecond => "us",
            Self::Millisecond => "ms",
            Self::Second => "s",
            Self::Minute => "min",
            Self::Hour => "h",
            Self::Day => "d",
        }
    }

    /// The multiplier converting this unit to milliseconds, as in
    /// `time_unit_conversion_table`.
    fn milliseconds(self) -> f64 {
        match self {
            Self::Microsecond => 1.0 / 1000.0,
            Self::Millisecond => 1.0,
            Self::Second => 1000.0,
            Self::Minute => 60_000.0,
            Self::Hour => 3_600_000.0,
            Self::Day => 86_400_000.0,
        }
    }

    /// The unit a fraction of this unit is rounded to, the next entry of
    /// `time_unit_conversion_table` for base unit milliseconds.
    fn next_smaller(self) -> Option<Self> {
        match self {
            Self::Microsecond => None,
            Self::Millisecond => Some(Self::Microsecond),
            Self::Second => Some(Self::Millisecond),
            Self::Minute => Some(Self::Second),
            Self::Hour => Some(Self::Minute),
            Self::Day => Some(Self::Hour),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Value {
    pub number: f64,
    /// `None` for the base unit of the parameter.
    pub unit: Option<Unit>,
}

impl std::fmt::Display for Value {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.number)?;

        match self.unit {
            Some(unit) => write!(formatter, "{}", unit.as_str()),
            None => Ok(()),
        }
    }
}

impl Value {
    /// Whole milliseconds, as PostgreSQL computes them for an integer
    /// parameter with base unit milliseconds: a fraction is rounded half to
    /// even to the next smaller unit, then the result half to even to whole
    /// milliseconds. `None` when that is outside the range of a 32-bit integer.
    #[must_use]
    pub fn to_milliseconds(&self) -> Option<i32> {
        let mut milliseconds = self.number;

        if let Some(unit) = self.unit {
            milliseconds *= unit.milliseconds();

            if let Some(next) = unit.next_smaller() {
                milliseconds =
                    (milliseconds / next.milliseconds()).round_ties_even() * next.milliseconds();
            }
        }

        let milliseconds = milliseconds.round_ties_even();

        if milliseconds > f64::from(i32::MAX) || milliseconds < f64::from(i32::MIN) {
            None
        } else {
            Some(milliseconds as i32)
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error(
        "invalid time value {input:?}: expected a number, optionally followed by one of us, ms, s, min, h, d"
    )]
    Syntax { input: String },
    #[error("time value {input:?} is out of floating point range")]
    Range { input: String },
}

impl std::str::FromStr for Value {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let syntax_error = || Error::Syntax {
            input: input.to_string(),
        };

        let integer = strtol(input);

        let integer_end = integer.as_ref().map_or(input, |(_, rest)| rest);

        let reparse = matches!(integer, Some((Strtol::Overflow, _)))
            || integer_end.starts_with(['.', 'e', 'E']);

        let (number, rest) = if reparse {
            match strtod(input) {
                Some((Strtod::Value(number), rest)) => (number, rest),
                Some((Strtod::Range, _)) => {
                    return Err(Error::Range {
                        input: input.to_string(),
                    });
                }
                None => return Err(syntax_error()),
            }
        } else {
            match integer {
                Some((Strtol::Value(number), rest)) => (number as f64, rest),
                _ => return Err(syntax_error()),
            }
        };

        if number.is_nan() {
            return Err(syntax_error());
        }

        let (rest, _) = c_space(rest).map_err(|_| syntax_error())?;

        if rest.is_empty() {
            return Ok(Self { number, unit: None });
        }

        let (_, (text, _, _)) = (
            take_while_m_n(0, 3, |character| !is_c_space(character)),
            c_space,
            eof,
        )
            .parse(rest)
            .map_err(|_: nom::Err<nom::error::Error<&str>>| syntax_error())?;

        let unit = Unit::ALL
            .into_iter()
            .find(|unit| unit.as_str() == text)
            .ok_or_else(syntax_error)?;

        Ok(Self {
            number,
            unit: Some(unit),
        })
    }
}

impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::de::Deserialize<'de>>::deserialize(deserializer)
            .and_then(|value| value.parse().map_err(serde::de::Error::custom))
    }
}

/// C's `isspace` in the C locale.
fn is_c_space(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\n' | '\u{b}' | '\u{c}' | '\r')
}

fn c_space(input: &str) -> IResult<&str, &str> {
    take_while(is_c_space).parse(input)
}

fn negative(input: &str) -> IResult<&str, bool> {
    opt(one_of("+-")).map(|sign| sign == Some('-')).parse(input)
}

enum Strtol {
    Value(i64),
    Overflow,
}

/// What `strtol(input, &end, 0)` reads, with the input after it; `None` when
/// it reads no number.
fn strtol(input: &str) -> Option<(Strtol, &str)> {
    let (rest, (_, negative, (radix, digits))) = (
        c_space,
        negative,
        alt((
            preceded(
                tag_no_case("0x"),
                take_while1(|character: char| character.is_ascii_hexdigit()),
            )
            .map(|digits| (16, digits)),
            recognize(pair(
                char('0'),
                take_while(|character: char| character.is_digit(8)),
            ))
            .map(|digits| (8, digits)),
            take_while1(|character: char| character.is_ascii_digit()).map(|digits| (10, digits)),
        )),
    )
        .parse(input)
        .ok()?;

    let value = u64::from_str_radix(digits, radix)
        .ok()
        .and_then(|magnitude| {
            if negative {
                0i64.checked_sub_unsigned(magnitude)
            } else {
                i64::try_from(magnitude).ok()
            }
        });

    Some((value.map_or(Strtol::Overflow, Strtol::Value), rest))
}

enum Strtod {
    Value(f64),
    Range,
}

/// What `strtod(input, &end)` reads, with the input after it; `None` when it
/// reads no number.
fn strtod(input: &str) -> Option<(Strtod, &str)> {
    fn hexadecimal(input: &str) -> IResult<&str, (&str, Option<&str>, Option<&str>)> {
        let hex_digits = || take_while(|character: char| character.is_ascii_hexdigit());

        preceded(
            tag_no_case("0x"),
            verify(
                (
                    hex_digits(),
                    opt(preceded(char('.'), hex_digits())),
                    opt(preceded(
                        one_of("pP"),
                        recognize(pair(opt(one_of("+-")), digit1)),
                    )),
                ),
                |(integer, fraction, _): &(&str, Option<&str>, Option<&str>)| {
                    !integer.is_empty() || fraction.is_some_and(|fraction| !fraction.is_empty())
                },
            ),
        )
        .parse(input)
    }

    fn decimal(input: &str) -> IResult<&str, (&str, Option<&str>)> {
        (
            verify(
                recognize(pair(digit0, opt(pair(char('.'), digit0)))),
                |mantissa: &str| mantissa.bytes().any(|byte| byte.is_ascii_digit()),
            ),
            opt(preceded(
                one_of("eE"),
                recognize(pair(opt(one_of("+-")), digit1)),
            )),
        )
            .parse(input)
    }

    let (input, (_, negative)) = (c_space, negative).parse(input).ok()?;

    if let Ok((rest, (integer, fraction, exponent))) = hexadecimal(input) {
        let digits = integer.chars().chain(fraction.unwrap_or("").chars());

        let exponent = exponent
            .map_or(0, parse_exponent)
            .saturating_sub(4 * i64::try_from(fraction.map_or(0, str::len)).unwrap_or(i64::MAX));

        return Some((hexadecimal_value(digits, exponent, negative), rest));
    }

    let (rest, (mantissa, exponent)) = decimal(input).ok()?;

    let sign = if negative { "-" } else { "" };

    let value: f64 = format!("{sign}{mantissa}e{}", exponent.unwrap_or("0"))
        .parse()
        .ok()?;

    let underflow = value.is_subnormal()
        || (value == 0.0 && mantissa.bytes().any(|byte| matches!(byte, b'1'..=b'9')));

    if value.is_infinite() || underflow {
        Some((Strtod::Range, rest))
    } else {
        Some((Strtod::Value(value), rest))
    }
}

/// A decimal exponent, saturated far beyond any finite `f64`.
fn parse_exponent(exponent: &str) -> i64 {
    const SATURATED: i64 = 1 << 40;

    exponent.parse::<i64>().map_or_else(
        |_| {
            if exponent.starts_with('-') {
                -SATURATED
            } else {
                SATURATED
            }
        },
        |exponent| exponent.clamp(-SATURATED, SATURATED),
    )
}

/// The hexadecimal digits times two to the power of `exponent`, correctly
/// rounded half to even, as glibc's `strtod` reads a hexadecimal number.
fn hexadecimal_value(digits: impl Iterator<Item = char>, exponent: i64, negative: bool) -> Strtod {
    let mut mantissa: u128 = 0;
    let mut exponent = exponent;
    let mut sticky = false;

    for digit in digits.filter_map(|character| character.to_digit(16)) {
        if mantissa >> 124 == 0 {
            mantissa = mantissa << 4 | u128::from(digit);
        } else {
            exponent = exponent.saturating_add(4);
            sticky |= digit != 0;
        }
    }

    if mantissa == 0 {
        return Strtod::Value(if negative { -0.0 } else { 0.0 });
    }

    let bits = i64::from(128 - mantissa.leading_zeros());
    let dropped = bits - 53;

    let mut significand = if dropped <= 0 {
        mantissa << -dropped
    } else {
        let kept = mantissa >> dropped;
        let remainder = mantissa & ((1 << dropped) - 1);
        let half = 1 << (dropped - 1);
        let round_up = remainder > half || (remainder == half && (sticky || kept & 1 == 1));

        kept + u128::from(round_up)
    };

    let mut leading_exponent = exponent.saturating_add(bits - 1);

    if significand == 1 << 53 {
        significand >>= 1;
        leading_exponent = leading_exponent.saturating_add(1);
    }

    if !(-1022..=1023).contains(&leading_exponent) {
        return Strtod::Range;
    }

    let biased_exponent = u64::try_from(leading_exponent + 1023).unwrap();
    let fraction = u64::try_from(significand).unwrap() & ((1 << 52) - 1);
    let value = f64::from_bits(biased_exponent << 52 | fraction);

    Strtod::Value(if negative { -value } else { value })
}
