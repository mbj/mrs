use chrono::Datelike as _;

use crate::catalog;
use crate::protocol::{Oid, frontend};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Bytea(Vec<u8>),
    Char(i8),
    Int2(i16),
    Int4(i32),
    Int8(i64),
    Float4(f32),
    Float8(f64),
    Text(String),
    Varchar(String),
    Bpchar(String),
    Oid(Oid),
    Uuid(uuid::Uuid),
    Timestamp(Timestamp),
    TimestampTz(TimestampTz),
    Date(Date),
    Numeric(Numeric),

    BoolArray(Array<bool>),
    ByteaArray(Array<Vec<u8>>),
    CharArray(Array<i8>),
    Int2Array(Array<i16>),
    Int4Array(Array<i32>),
    Int8Array(Array<i64>),
    Float4Array(Array<f32>),
    Float8Array(Array<f64>),
    TextArray(Array<String>),
    VarcharArray(Array<String>),
    BpcharArray(Array<String>),
    OidArray(Array<Oid>),
    UuidArray(Array<uuid::Uuid>),
    TimestampArray(Array<Timestamp>),
    TimestampTzArray(Array<TimestampTz>),
    DateArray(Array<Date>),
    NumericArray(Array<Numeric>),
}

impl Value {
    #[must_use]
    pub const fn builtin(&self) -> catalog::Builtin {
        match self {
            Self::Bool(_) => catalog::Builtin::Bool,
            Self::Bytea(_) => catalog::Builtin::Bytea,
            Self::Char(_) => catalog::Builtin::Char,
            Self::Int2(_) => catalog::Builtin::Int2,
            Self::Int4(_) => catalog::Builtin::Int4,
            Self::Int8(_) => catalog::Builtin::Int8,
            Self::Float4(_) => catalog::Builtin::Float4,
            Self::Float8(_) => catalog::Builtin::Float8,
            Self::Text(_) => catalog::Builtin::Text,
            Self::Varchar(_) => catalog::Builtin::Varchar,
            Self::Bpchar(_) => catalog::Builtin::Bpchar,
            Self::Oid(_) => catalog::Builtin::Oid,
            Self::Uuid(_) => catalog::Builtin::Uuid,
            Self::Timestamp(_) => catalog::Builtin::Timestamp,
            Self::TimestampTz(_) => catalog::Builtin::Timestamptz,
            Self::Date(_) => catalog::Builtin::Date,
            Self::Numeric(_) => catalog::Builtin::Numeric,

            Self::BoolArray(_) => catalog::Builtin::BoolArray,
            Self::ByteaArray(_) => catalog::Builtin::ByteaArray,
            Self::CharArray(_) => catalog::Builtin::CharArray,
            Self::Int2Array(_) => catalog::Builtin::Int2Array,
            Self::Int4Array(_) => catalog::Builtin::Int4Array,
            Self::Int8Array(_) => catalog::Builtin::Int8Array,
            Self::Float4Array(_) => catalog::Builtin::Float4Array,
            Self::Float8Array(_) => catalog::Builtin::Float8Array,
            Self::TextArray(_) => catalog::Builtin::TextArray,
            Self::VarcharArray(_) => catalog::Builtin::VarcharArray,
            Self::BpcharArray(_) => catalog::Builtin::BpcharArray,
            Self::OidArray(_) => catalog::Builtin::OidArray,
            Self::UuidArray(_) => catalog::Builtin::UuidArray,
            Self::TimestampArray(_) => catalog::Builtin::TimestampArray,
            Self::TimestampTzArray(_) => catalog::Builtin::TimestamptzArray,
            Self::DateArray(_) => catalog::Builtin::DateArray,
            Self::NumericArray(_) => catalog::Builtin::NumericArray,
        }
    }
}

fn encode<T: Encode + ?Sized>(
    value: &T,
    writer: &mut frontend::Writer<'_>,
) -> Result<(), EncodeError> {
    value.write_to(writer).into_result()
}

impl Value {
    pub fn write_to(&self, writer: &mut frontend::Writer<'_>) -> Result<(), EncodeError> {
        match self {
            Self::Bool(value) => encode(value, writer),
            Self::Bytea(value) => value.write_to(writer),
            Self::Char(value) => encode(value, writer),
            Self::Int2(value) => encode(value, writer),
            Self::Int4(value) => encode(value, writer),
            Self::Int8(value) => encode(value, writer),
            Self::Float4(value) => encode(value, writer),
            Self::Float8(value) => encode(value, writer),
            Self::Text(value) | Self::Varchar(value) | Self::Bpchar(value) => {
                value.write_to(writer)
            }
            Self::Oid(value) => encode(value, writer),
            Self::Uuid(value) => encode(value, writer),
            Self::Timestamp(value) => encode(value, writer),
            Self::TimestampTz(value) => encode(value, writer),
            Self::Date(value) => encode(value, writer),
            Self::Numeric(value) => encode(value, writer),

            Self::BoolArray(array) => write_array(writer, array, catalog::Builtin::Bool),
            Self::ByteaArray(array) => write_array(writer, array, catalog::Builtin::Bytea),
            Self::CharArray(array) => write_array(writer, array, catalog::Builtin::Char),
            Self::Int2Array(array) => write_array(writer, array, catalog::Builtin::Int2),
            Self::Int4Array(array) => write_array(writer, array, catalog::Builtin::Int4),
            Self::Int8Array(array) => write_array(writer, array, catalog::Builtin::Int8),
            Self::Float4Array(array) => write_array(writer, array, catalog::Builtin::Float4),
            Self::Float8Array(array) => write_array(writer, array, catalog::Builtin::Float8),
            Self::TextArray(array) => write_array(writer, array, catalog::Builtin::Text),
            Self::VarcharArray(array) => write_array(writer, array, catalog::Builtin::Varchar),
            Self::BpcharArray(array) => write_array(writer, array, catalog::Builtin::Bpchar),
            Self::OidArray(array) => write_array(writer, array, catalog::Builtin::Oid),
            Self::UuidArray(array) => write_array(writer, array, catalog::Builtin::Uuid),
            Self::TimestampArray(array) => write_array(writer, array, catalog::Builtin::Timestamp),
            Self::TimestampTzArray(array) => {
                write_array(writer, array, catalog::Builtin::Timestamptz)
            }
            Self::DateArray(array) => write_array(writer, array, catalog::Builtin::Date),
            Self::NumericArray(array) => write_array(writer, array, catalog::Builtin::Numeric),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EncodeError {
    #[error(transparent)]
    TooLong(#[from] frontend::TooLong),
    #[error("array axis of {length} elements exceeds the Int32 length field")]
    ArrayAxis { length: u32 },
    #[cfg(feature = "serde")]
    #[error("value cannot be serialized as JSON")]
    Json(#[source] serde_json::Error),
    #[error("timestamp has sub-microsecond digits, which PostgreSQL cannot store")]
    TimestampPrecision,
    #[error(transparent)]
    TimestampRange(#[from] TimestampRangeError),
    #[error("custom encoder failed")]
    Custom(#[source] Box<dyn std::error::Error + Send + Sync>),
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for () {}

    impl Sealed for Result<(), super::EncodeError> {}
}

pub trait Outcome: sealed::Sealed {
    type Checked<B, E>;

    fn done() -> Self;

    fn into_result(self) -> Result<(), EncodeError>;

    fn check<B, E>(
        self,
        builder: B,
        fixed: impl FnOnce(B) -> B,
        fallible: impl FnOnce(Result<(), EncodeError>, B) -> Result<B, E>,
    ) -> Self::Checked<B, E>;

    fn into_checked_result<B, E>(checked: Self::Checked<B, E>) -> Result<B, E>;
}

impl Outcome for () {
    type Checked<B, E> = B;

    fn done() -> Self {}

    fn into_result(self) -> Result<(), EncodeError> {
        Ok(())
    }

    fn check<B, E>(
        self,
        builder: B,
        fixed: impl FnOnce(B) -> B,
        _: impl FnOnce(Result<(), EncodeError>, B) -> Result<B, E>,
    ) -> B {
        fixed(builder)
    }

    fn into_checked_result<B, E>(checked: B) -> Result<B, E> {
        Ok(checked)
    }
}

impl Outcome for Result<(), EncodeError> {
    type Checked<B, E> = Result<B, E>;

    fn done() -> Self {
        Ok(())
    }

    fn into_result(self) -> Result<(), EncodeError> {
        self
    }

    fn check<B, E>(
        self,
        builder: B,
        _: impl FnOnce(B) -> B,
        fallible: impl FnOnce(Result<(), EncodeError>, B) -> Result<B, E>,
    ) -> Result<B, E> {
        fallible(self, builder)
    }

    fn into_checked_result<B, E>(checked: Result<B, E>) -> Result<B, E> {
        checked
    }
}

pub trait Encode {
    const BUILTIN: catalog::Builtin;

    type Outcome: Outcome;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) -> Self::Outcome;

    fn is_null(&self) -> bool {
        false
    }
}

impl<T: Encode + ?Sized> Encode for &T {
    const BUILTIN: catalog::Builtin = T::BUILTIN;

    type Outcome = T::Outcome;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) -> T::Outcome {
        T::write_to(self, writer)
    }

    fn is_null(&self) -> bool {
        T::is_null(self)
    }
}

impl<T: Encode> Encode for Option<T> {
    const BUILTIN: catalog::Builtin = T::BUILTIN;

    type Outcome = T::Outcome;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) -> T::Outcome {
        match self {
            Some(value) => value.write_to(writer),
            None => T::Outcome::done(),
        }
    }

    fn is_null(&self) -> bool {
        self.as_ref().is_none_or(Encode::is_null)
    }
}

impl Encode for bool {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Bool;

    type Outcome = ();

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.byte(u8::from(*self));
    }
}

impl Encode for i8 {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Char;

    type Outcome = ();

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.int8(*self);
    }
}

impl Encode for i16 {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Int2;

    type Outcome = ();

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.int16(*self);
    }
}

impl Encode for i32 {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Int4;

    type Outcome = ();

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.int32(*self);
    }
}

impl Encode for i64 {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Int8;

    type Outcome = ();

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.int64(*self);
    }
}

impl Encode for f32 {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Float4;

    type Outcome = ();

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.uint32(self.to_bits());
    }
}

impl Encode for f64 {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Float8;

    type Outcome = ();

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.uint64(self.to_bits());
    }
}

impl Encode for str {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Text;

    type Outcome = Result<(), EncodeError>;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) -> Result<(), EncodeError> {
        writer.bytes(self.as_bytes());

        Ok(())
    }
}

impl Encode for String {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Text;

    type Outcome = Result<(), EncodeError>;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) -> Result<(), EncodeError> {
        writer.bytes(self.as_bytes());

        Ok(())
    }
}

impl Encode for [u8] {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Bytea;

    type Outcome = Result<(), EncodeError>;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) -> Result<(), EncodeError> {
        writer.bytes(self);

        Ok(())
    }
}

impl Encode for Vec<u8> {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Bytea;

    type Outcome = Result<(), EncodeError>;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) -> Result<(), EncodeError> {
        writer.bytes(self);

        Ok(())
    }
}

impl Encode for Oid {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Oid;

    type Outcome = ();

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.uint32(self.get());
    }
}

impl Encode for uuid::Uuid {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Uuid;

    type Outcome = ();

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.bytes(self.as_bytes());
    }
}

const POSTGRES_EPOCH_UNIX_MICROSECONDS: i64 = 946_684_800_000_000;

// https://github.com/postgres/postgres/blob/master/src/include/datatype/timestamp.h
const MIN_TIMESTAMP: i64 = -211_813_488_000_000_000;
const END_TIMESTAMP: i64 = 9_223_371_331_200_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("timestamp is outside the range PostgreSQL can store")]
pub struct TimestampRangeError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimestampTz {
    NegativeInfinity,
    Finite(FiniteTimestampTz),
    Infinity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FiniteTimestampTz(chrono::DateTime<chrono::Utc>);

impl FiniteTimestampTz {
    /// PostgreSQL stores microseconds, so finer digits are dropped. That
    /// rounds towards the past, also before 1970.
    ///
    /// Refuses instants before 4714-11-24 BC, the earliest PostgreSQL stores.
    pub fn truncate_precision(
        value: chrono::DateTime<chrono::Utc>,
    ) -> Result<Self, TimestampRangeError> {
        value
            .timestamp_micros()
            .checked_sub(POSTGRES_EPOCH_UNIX_MICROSECONDS)
            .and_then(|microseconds| Self::from_microseconds(microseconds).ok())
            .ok_or(TimestampRangeError)
    }

    #[must_use]
    pub const fn get(self) -> chrono::DateTime<chrono::Utc> {
        self.0
    }

    fn from_microseconds(microseconds: i64) -> Result<Self, DecodeError> {
        if !(MIN_TIMESTAMP..END_TIMESTAMP).contains(&microseconds) {
            return Err(DecodeError::TimestampRange { microseconds });
        }

        microseconds
            .checked_add(POSTGRES_EPOCH_UNIX_MICROSECONDS)
            .and_then(chrono::DateTime::from_timestamp_micros)
            .map(Self)
            .ok_or(DecodeError::TimestampUnrepresentable { microseconds })
    }

    fn microseconds(self) -> i64 {
        self.0.timestamp_micros() - POSTGRES_EPOCH_UNIX_MICROSECONDS
    }
}

impl Encode for TimestampTz {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Timestamptz;

    type Outcome = ();

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.int64(match self {
            Self::NegativeInfinity => i64::MIN,
            Self::Finite(finite) => finite.microseconds(),
            Self::Infinity => i64::MAX,
        });
    }
}

impl Encode for chrono::DateTime<chrono::Utc> {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Timestamptz;

    type Outcome = Result<(), EncodeError>;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) -> Result<(), EncodeError> {
        if !self.timestamp_subsec_nanos().is_multiple_of(1_000) {
            return Err(EncodeError::TimestampPrecision);
        }

        writer.int64(FiniteTimestampTz::truncate_precision(*self)?.microseconds());

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Timestamp {
    NegativeInfinity,
    Finite(FiniteTimestamp),
    Infinity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FiniteTimestamp(chrono::NaiveDateTime);

impl FiniteTimestamp {
    /// PostgreSQL stores microseconds, so finer digits are dropped. That
    /// rounds towards the past, also before 1970.
    ///
    /// Refuses date-times before 4714-11-24 BC, the earliest PostgreSQL stores.
    pub fn truncate_precision(value: chrono::NaiveDateTime) -> Result<Self, TimestampRangeError> {
        value
            .and_utc()
            .timestamp_micros()
            .checked_sub(POSTGRES_EPOCH_UNIX_MICROSECONDS)
            .and_then(|microseconds| Self::from_microseconds(microseconds).ok())
            .ok_or(TimestampRangeError)
    }

    #[must_use]
    pub const fn get(self) -> chrono::NaiveDateTime {
        self.0
    }

    fn from_microseconds(microseconds: i64) -> Result<Self, DecodeError> {
        FiniteTimestampTz::from_microseconds(microseconds).map(|finite| Self(finite.0.naive_utc()))
    }

    fn microseconds(self) -> i64 {
        self.0.and_utc().timestamp_micros() - POSTGRES_EPOCH_UNIX_MICROSECONDS
    }
}

impl Encode for Timestamp {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Timestamp;

    type Outcome = ();

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.int64(match self {
            Self::NegativeInfinity => i64::MIN,
            Self::Finite(finite) => finite.microseconds(),
            Self::Infinity => i64::MAX,
        });
    }
}

const POSTGRES_EPOCH_DAYS_FROM_CE: i32 = 730_120;

// https://github.com/postgres/postgres/blob/master/src/include/datatype/timestamp.h
const MIN_DATE: i32 = -2_451_545;
const END_DATE: i32 = 2_145_031_949;

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("date is outside the range PostgreSQL can store")]
pub struct DateRangeError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Date {
    NegativeInfinity,
    Finite(FiniteDate),
    Infinity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FiniteDate(chrono::NaiveDate);

impl FiniteDate {
    pub fn new(value: chrono::NaiveDate) -> Result<Self, DateRangeError> {
        value
            .num_days_from_ce()
            .checked_sub(POSTGRES_EPOCH_DAYS_FROM_CE)
            .and_then(|days| Self::from_days(days).ok())
            .ok_or(DateRangeError)
    }

    #[must_use]
    pub const fn get(self) -> chrono::NaiveDate {
        self.0
    }

    fn from_days(days: i32) -> Result<Self, DecodeError> {
        if !(MIN_DATE..END_DATE).contains(&days) {
            return Err(DecodeError::DateRange { days });
        }

        days.checked_add(POSTGRES_EPOCH_DAYS_FROM_CE)
            .and_then(chrono::NaiveDate::from_num_days_from_ce_opt)
            .map(Self)
            .ok_or(DecodeError::DateUnrepresentable { days })
    }

    fn days(self) -> i32 {
        self.0.num_days_from_ce() - POSTGRES_EPOCH_DAYS_FROM_CE
    }
}

impl Encode for Date {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Date;

    type Outcome = ();

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.int32(match self {
            Self::NegativeInfinity => i32::MIN,
            Self::Finite(finite) => finite.days(),
            Self::Infinity => i32::MAX,
        });
    }
}

// https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/numeric.c
const NUMERIC_POSITIVE: u16 = 0x0000;
const NUMERIC_NEGATIVE: u16 = 0x4000;
const NUMERIC_NAN: u16 = 0xC000;
const NUMERIC_POSITIVE_INFINITY: u16 = 0xD000;
const NUMERIC_NEGATIVE_INFINITY: u16 = 0xF000;
const NUMERIC_DSCALE_MAX: u16 = 0x3FFF;
const NBASE: u16 = 10_000;
const NBASE_DECIMAL_DIGITS: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("numeric is outside the range PostgreSQL can store")]
pub struct NumericRangeError;

/// Ordered as PostgreSQL orders `numeric`: `NaN` is equal to itself and
/// greater than every other value, `Infinity` included.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Numeric {
    NegativeInfinity,
    Finite(FiniteNumeric),
    Infinity,
    NaN,
}

#[derive(Clone, Debug)]
pub struct FiniteNumeric {
    value: bigdecimal::BigDecimal,
    layout: NumericLayout,
}

#[derive(Clone, Debug)]
struct NumericLayout {
    ndigits: i16,
    weight: i16,
    negative: bool,
    dscale: u16,
    digits: smallvec::SmallVec<[u16; 8]>,
}

impl NumericLayout {
    fn of(value: &bigdecimal::BigDecimal) -> Option<Self> {
        let (integer, scale) = value.as_bigint_and_scale();
        let dscale = u16::try_from(scale)
            .ok()
            .filter(|dscale| *dscale <= NUMERIC_DSCALE_MAX)?;
        let (sign, magnitude) = integer.into_owned().into_parts();

        let fractional_digits = usize::from(dscale).next_multiple_of(NBASE_DECIMAL_DIGITS);
        let padding = fractional_digits - usize::from(dscale);
        let fractional_groups = fractional_digits / NBASE_DECIMAL_DIGITS;

        let mut decimal = vec![0; padding];
        decimal.extend(magnitude.to_radix_le(10));

        let groups: Vec<u16> = decimal
            .chunks(NBASE_DECIMAL_DIGITS)
            .map(|chunk| {
                chunk
                    .iter()
                    .rev()
                    .fold(0, |group, digit| group * 10 + u16::from(*digit))
            })
            .collect();

        let Some(low) = groups.iter().position(|group| *group != 0) else {
            return Some(Self {
                ndigits: 0,
                weight: 0,
                negative: false,
                dscale,
                digits: smallvec::SmallVec::new(),
            });
        };
        let high = groups.iter().rposition(|group| *group != 0)?;

        let digits: smallvec::SmallVec<[u16; 8]> =
            groups[low..=high].iter().rev().copied().collect();

        Some(Self {
            ndigits: i16::try_from(digits.len()).ok()?,
            weight: i16::try_from(
                i64::try_from(high).ok()? - i64::try_from(fractional_groups).ok()?,
            )
            .ok()?,
            negative: sign == bigdecimal::num_bigint::Sign::Minus,
            dscale,
            digits,
        })
    }

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.int16(self.ndigits);
        writer.int16(self.weight);
        writer.uint16(if self.negative {
            NUMERIC_NEGATIVE
        } else {
            NUMERIC_POSITIVE
        });
        writer.uint16(self.dscale);

        for digit in &self.digits {
            writer.bytes(&digit.to_be_bytes());
        }
    }
}

impl FiniteNumeric {
    /// Keeps the scale, which PostgreSQL stores and prints: `1.50` stays
    /// `1.50`, though it equals `1.5`. A negative scale, as in `1E+3`, becomes
    /// 0, since PostgreSQL has none.
    ///
    /// Refuses more than 131072 digits before the point or 16383 after it.
    pub fn new(value: bigdecimal::BigDecimal) -> Result<Self, NumericRangeError> {
        let value = if value.fractional_digit_count() < 0 {
            value.with_scale(0)
        } else {
            value
        };

        let layout = NumericLayout::of(&value).ok_or(NumericRangeError)?;

        Ok(Self { value, layout })
    }

    #[must_use]
    pub const fn get(&self) -> &bigdecimal::BigDecimal {
        &self.value
    }

    #[must_use]
    pub fn into_inner(self) -> bigdecimal::BigDecimal {
        self.value
    }
}

impl PartialEq for FiniteNumeric {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for FiniteNumeric {}

impl PartialOrd for FiniteNumeric {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FiniteNumeric {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl Encode for Numeric {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Numeric;

    type Outcome = ();

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        let special = match self {
            Self::Finite(finite) => return finite.layout.write_to(writer),
            Self::NegativeInfinity => NUMERIC_NEGATIVE_INFINITY,
            Self::Infinity => NUMERIC_POSITIVE_INFINITY,
            Self::NaN => NUMERIC_NAN,
        };

        writer.int16(0);
        writer.int16(0);
        writer.uint16(special);
        writer.uint16(0);
    }
}

impl<T: Encode> Encode for Array<T> {
    const BUILTIN: catalog::Builtin = match T::BUILTIN.array() {
        Some(array) => array,
        None => panic!("the element type has no array type"),
    };

    type Outcome = Result<(), EncodeError>;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) -> Result<(), EncodeError> {
        write_array(writer, self, T::BUILTIN)
    }
}

fn write_array<T: Encode>(
    writer: &mut frontend::Writer<'_>,
    array: &Array<T>,
    element: catalog::Builtin,
) -> Result<(), EncodeError> {
    writer.int32(
        i32::try_from(array.dimensions.len())
            .expect("dimension count exceeds the Int32 field, though Array::new caps it at 6"),
    );
    writer.int32(i32::from(array.elements.iter().any(Option::is_none)));
    writer.uint32(element.oid().get());

    for dimension in &array.dimensions {
        writer.int32(
            i32::try_from(dimension.length).map_err(|_| EncodeError::ArrayAxis {
                length: dimension.length,
            })?,
        );
        writer.int32(dimension.lower_bound);
    }

    for element in &array.elements {
        match element {
            None => writer.int32(-1),
            Some(value) => writer.length_prefixed(|writer| value.write_to(writer).into_result())?,
        }
    }

    Ok(())
}

/// `int[]` and `int[][]` are both `_int4`.
#[derive(Clone, Debug, PartialEq)]
pub struct Array<T> {
    dimensions: Vec<Dimension>,
    elements: Vec<Option<T>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dimension {
    pub length: u32,

    /// Ordinarily 1, but a literal may set its own: `'[0:2]={a,b,c}'`.
    pub lower_bound: i32,
}

/// PostgreSQL's `MAXDIM`: an array with more axes cannot exist server side.
pub const MAX_DIMENSIONS: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ArrayError {
    #[error("array has {count} dimensions, more than the {MAX_DIMENSIONS} allowed")]
    Dimensions { count: usize },

    #[error("dimensions describe {expected} elements, but {actual} were given")]
    ElementCount { expected: usize, actual: usize },

    #[error("dimensions describe more elements than can be counted")]
    Unbounded,
}

impl<T> Array<T> {
    pub fn new(dimensions: Vec<Dimension>, elements: Vec<Option<T>>) -> Result<Self, ArrayError> {
        if dimensions.len() > MAX_DIMENSIONS {
            return Err(ArrayError::Dimensions {
                count: dimensions.len(),
            });
        }

        let mut expected: usize = 1;

        for dimension in &dimensions {
            expected = usize::try_from(dimension.length)
                .ok()
                .and_then(|length| expected.checked_mul(length))
                .ok_or(ArrayError::Unbounded)?;
        }

        if dimensions.is_empty() {
            expected = 0;
        }

        if expected != elements.len() {
            return Err(ArrayError::ElementCount {
                expected,
                actual: elements.len(),
            });
        }

        Ok(Self {
            dimensions,
            elements,
        })
    }

    #[must_use]
    pub fn dimensions(&self) -> &[Dimension] {
        &self.dimensions
    }

    #[must_use]
    pub fn elements(&self) -> &[Option<T>] {
        &self.elements
    }

    #[must_use]
    pub fn get(&self, subscripts: &[i32]) -> Option<Option<&T>> {
        if subscripts.len() != self.dimensions.len() {
            return None;
        }

        let mut offset: usize = 0;

        for (subscript, dimension) in subscripts.iter().zip(&self.dimensions) {
            let length = usize::try_from(dimension.length).ok()?;
            let index = usize::try_from(subscript.checked_sub(dimension.lower_bound)?).ok()?;

            if index >= length {
                return None;
            }

            offset = offset.checked_mul(length)?.checked_add(index)?;
        }

        self.elements.get(offset).map(Option::as_ref)
    }
}

impl<T> From<Vec<Option<T>>> for Array<T> {
    fn from(elements: Vec<Option<T>>) -> Self {
        let dimensions = if elements.is_empty() {
            Vec::new()
        } else {
            vec![Dimension {
                length: u32::try_from(elements.len()).expect("array length fits in u32"),
                lower_bound: 1,
            }]
        };

        Self {
            dimensions,
            elements,
        }
    }
}

impl<T> From<Vec<T>> for Array<T> {
    fn from(elements: Vec<T>) -> Self {
        elements.into_iter().map(Some).collect::<Vec<_>>().into()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("value is NULL")]
    Null,
    #[error("value is {actual} bytes, expected {expected}")]
    Length { expected: usize, actual: usize },
    #[error("value is not UTF-8")]
    NotUtf8,
    #[error("value contains a NUL at byte {index}")]
    ContainsNul { index: usize },
    #[error("jsonb version {version} is not 1")]
    JsonbVersion { version: u8 },
    #[error("timestamp {microseconds} is outside the range PostgreSQL can store")]
    TimestampRange { microseconds: i64 },
    /// PostgreSQL stores timestamps up to 294276 AD, chrono only up to
    /// 262143 AD.
    #[error("timestamp {microseconds} is beyond the range chrono can represent")]
    TimestampUnrepresentable { microseconds: i64 },
    #[error("timestamp is infinite")]
    InfiniteTimestamp,
    #[error("date {days} is outside the range PostgreSQL can store")]
    DateRange { days: i32 },
    /// PostgreSQL stores dates up to 5874897 AD, chrono only up to 262143 AD.
    #[error("date {days} is beyond the range chrono can represent")]
    DateUnrepresentable { days: i32 },
    #[error("date is infinite")]
    InfiniteDate,
    #[error("numeric has a negative digit count {ndigits}")]
    NumericDigitCount { ndigits: i16 },
    #[error("numeric sign {sign:#06x} is not one PostgreSQL sends")]
    NumericSign { sign: u16 },
    #[error("numeric display scale {dscale} exceeds the maximum PostgreSQL stores")]
    NumericScale { dscale: u16 },
    #[error("numeric digit {digit} is outside base 10000")]
    NumericDigit { digit: i16 },
    #[error("numeric has digits beyond its display scale")]
    NumericDigitsBeyondScale,
    #[error("numeric is NaN")]
    NumericNaN,
    #[error("numeric is infinite")]
    InfiniteNumeric,
    #[error("value is not a valid name")]
    InvalidName(#[source] crate::name::ParseError),
    #[cfg(feature = "serde")]
    #[error("value is not the expected JSON")]
    Json(#[source] serde_json::Error),
    #[error("array has {count} dimensions, PostgreSQL allows 0 to {MAX_DIMENSIONS}")]
    ArrayDimensions { count: i32 },
    #[error("array axis has negative length {length}")]
    ArrayAxis { length: i32 },
    #[error("array shape does not match its elements")]
    ArrayShape(#[source] ArrayError),
    #[error("array element {index} could not be decoded")]
    ArrayElement {
        index: usize,
        #[source]
        source: Box<DecodeError>,
    },
    #[error("custom decoder failed")]
    Custom(#[source] Box<dyn std::error::Error + Send + Sync>),
}

pub trait Decode {
    type Output<'r>;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool;

    fn decode(value: &[u8]) -> Result<Self::Output<'_>, DecodeError>;

    fn decode_null<'r>() -> Result<Self::Output<'r>, DecodeError> {
        Err(DecodeError::Null)
    }
}

pub(crate) fn accepts_binary(
    column: &crate::protocol::backend::ColumnDescription<'_>,
    builtin: catalog::Builtin,
) -> bool {
    column.data_type == builtin.oid() && column.format == crate::protocol::Format::Binary
}

fn decode_fixed<const N: usize, T>(
    value: &[u8],
    convert: impl FnOnce([u8; N]) -> T,
) -> Result<T, DecodeError> {
    <[u8; N]>::try_from(value)
        .map(convert)
        .map_err(|_| DecodeError::Length {
            expected: N,
            actual: value.len(),
        })
}

impl Decode for bool {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Bool)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        decode_fixed(value, |[byte]| byte != 0)
    }
}

impl Decode for i8 {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Char)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        decode_fixed(value, Self::from_be_bytes)
    }
}

impl Decode for i16 {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Int2)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        decode_fixed(value, Self::from_be_bytes)
    }
}

impl Decode for i32 {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Int4)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        decode_fixed(value, Self::from_be_bytes)
    }
}

impl Decode for i64 {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Int8)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        decode_fixed(value, Self::from_be_bytes)
    }
}

impl Decode for f32 {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Float4)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        decode_fixed(value, Self::from_be_bytes)
    }
}

impl Decode for f64 {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Float8)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        decode_fixed(value, Self::from_be_bytes)
    }
}

impl Decode for Oid {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Oid)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        decode_fixed(value, |bytes| Self::new(u32::from_be_bytes(bytes)))
    }
}

impl Decode for &[u8] {
    type Output<'r> = &'r [u8];

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Bytea)
    }

    fn decode(value: &[u8]) -> Result<&[u8], DecodeError> {
        Ok(value)
    }
}

impl Decode for Vec<u8> {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        <&[u8]>::accepts(column)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        <&[u8]>::decode(value).map(<[u8]>::to_vec)
    }
}

impl Decode for uuid::Uuid {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Uuid)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        decode_fixed(value, Self::from_bytes)
    }
}

impl Decode for TimestampTz {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Timestamptz)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        match decode_fixed(value, i64::from_be_bytes)? {
            i64::MIN => Ok(Self::NegativeInfinity),
            i64::MAX => Ok(Self::Infinity),
            microseconds => FiniteTimestampTz::from_microseconds(microseconds).map(Self::Finite),
        }
    }
}

impl Decode for Timestamp {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Timestamp)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        match decode_fixed(value, i64::from_be_bytes)? {
            i64::MIN => Ok(Self::NegativeInfinity),
            i64::MAX => Ok(Self::Infinity),
            microseconds => FiniteTimestamp::from_microseconds(microseconds).map(Self::Finite),
        }
    }
}

impl Decode for Date {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Date)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        match decode_fixed(value, i32::from_be_bytes)? {
            i32::MIN => Ok(Self::NegativeInfinity),
            i32::MAX => Ok(Self::Infinity),
            days => FiniteDate::from_days(days).map(Self::Finite),
        }
    }
}

impl Decode for chrono::NaiveDate {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        Date::accepts(column)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        match Date::decode(value)? {
            Date::Finite(finite) => Ok(finite.get()),
            Date::NegativeInfinity | Date::Infinity => Err(DecodeError::InfiniteDate),
        }
    }
}

impl Decode for Numeric {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Numeric)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        let (header, body) = value.split_first_chunk::<8>().ok_or(DecodeError::Length {
            expected: 8,
            actual: value.len(),
        })?;

        let ndigits = i16::from_be_bytes([header[0], header[1]]);
        let weight = i16::from_be_bytes([header[2], header[3]]);
        let sign = u16::from_be_bytes([header[4], header[5]]);
        let dscale = u16::from_be_bytes([header[6], header[7]]);

        let count =
            usize::try_from(ndigits).map_err(|_| DecodeError::NumericDigitCount { ndigits })?;

        if body.len() != count * 2 {
            return Err(DecodeError::Length {
                expected: 8 + count * 2,
                actual: value.len(),
            });
        }

        let negative = match sign {
            NUMERIC_POSITIVE => false,
            NUMERIC_NEGATIVE => true,
            NUMERIC_NAN => return Ok(Self::NaN),
            NUMERIC_POSITIVE_INFINITY => return Ok(Self::Infinity),
            NUMERIC_NEGATIVE_INFINITY => return Ok(Self::NegativeInfinity),
            sign => return Err(DecodeError::NumericSign { sign }),
        };

        if dscale > NUMERIC_DSCALE_MAX {
            return Err(DecodeError::NumericScale { dscale });
        }

        let digits = body
            .as_chunks::<2>()
            .0
            .iter()
            .map(|pair| {
                let digit = i16::from_be_bytes(*pair);

                u16::try_from(digit)
                    .ok()
                    .filter(|digit| *digit < NBASE)
                    .ok_or(DecodeError::NumericDigit { digit })
            })
            .collect::<Result<smallvec::SmallVec<[u16; 8]>, DecodeError>>()?;

        let magnitude = digits
            .iter()
            .fold(bigdecimal::num_bigint::BigUint::ZERO, |magnitude, digit| {
                magnitude * u32::from(NBASE) + u32::from(*digit)
            });

        let integer = bigdecimal::num_bigint::BigInt::from_biguint(
            if negative {
                bigdecimal::num_bigint::Sign::Minus
            } else {
                bigdecimal::num_bigint::Sign::Plus
            },
            magnitude,
        );

        let groups_after_point = i64::from(ndigits) - 1 - i64::from(weight);
        let exact = bigdecimal::BigDecimal::new(integer, groups_after_point * 4);
        let scaled = exact.with_scale(i64::from(dscale));

        if scaled != exact {
            return Err(DecodeError::NumericDigitsBeyondScale);
        }

        Ok(Self::Finite(FiniteNumeric {
            value: scaled,
            layout: NumericLayout {
                ndigits,
                weight,
                negative,
                dscale,
                digits,
            },
        }))
    }
}

impl Decode for bigdecimal::BigDecimal {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        Numeric::accepts(column)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        match Numeric::decode(value)? {
            Numeric::Finite(finite) => Ok(finite.into_inner()),
            Numeric::NaN => Err(DecodeError::NumericNaN),
            Numeric::NegativeInfinity | Numeric::Infinity => Err(DecodeError::InfiniteNumeric),
        }
    }
}

impl Decode for chrono::DateTime<chrono::Utc> {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        TimestampTz::accepts(column)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        match TimestampTz::decode(value)? {
            TimestampTz::Finite(finite) => Ok(finite.get()),
            TimestampTz::NegativeInfinity | TimestampTz::Infinity => {
                Err(DecodeError::InfiniteTimestamp)
            }
        }
    }
}

pub(crate) fn accepts_text(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
    [
        catalog::Builtin::Text,
        catalog::Builtin::Varchar,
        catalog::Builtin::Bpchar,
        catalog::Builtin::Name,
    ]
    .into_iter()
    .any(|builtin| accepts_binary(column, builtin))
}

impl Decode for &str {
    type Output<'r> = &'r str;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_text(column)
    }

    fn decode(value: &[u8]) -> Result<&str, DecodeError> {
        core::str::from_utf8(value).map_err(|_| DecodeError::NotUtf8)
    }
}

impl Decode for String {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        <&str>::accepts(column)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        <&str>::decode(value).map(str::to_owned)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JsonText<'r>(&'r str);

impl<'r> JsonText<'r> {
    #[must_use]
    pub const fn new(text: &'r str) -> Self {
        Self(text)
    }

    #[must_use]
    pub const fn as_str(&self) -> &'r str {
        self.0
    }
}

impl Encode for JsonText<'_> {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Json;

    type Outcome = Result<(), EncodeError>;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) -> Result<(), EncodeError> {
        writer.bytes(self.0.as_bytes());

        Ok(())
    }
}

impl Decode for JsonText<'_> {
    type Output<'r> = JsonText<'r>;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Json)
    }

    fn decode(value: &[u8]) -> Result<JsonText<'_>, DecodeError> {
        <&str>::decode(value).map(JsonText)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JsonbText<'r>(&'r str);

impl<'r> JsonbText<'r> {
    #[must_use]
    pub const fn new(text: &'r str) -> Self {
        Self(text)
    }

    #[must_use]
    pub const fn as_str(&self) -> &'r str {
        self.0
    }
}

const JSONB_VERSION: u8 = 1;

impl Encode for JsonbText<'_> {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Jsonb;

    type Outcome = Result<(), EncodeError>;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) -> Result<(), EncodeError> {
        writer.byte(JSONB_VERSION);
        writer.bytes(self.0.as_bytes());

        Ok(())
    }
}

impl Decode for JsonbText<'_> {
    type Output<'r> = JsonbText<'r>;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Jsonb)
    }

    fn decode(value: &[u8]) -> Result<JsonbText<'_>, DecodeError> {
        match value {
            [JSONB_VERSION, text @ ..] => <&str>::decode(text).map(JsonbText),
            [version, ..] => Err(DecodeError::JsonbVersion { version: *version }),
            [] => Err(DecodeError::Length {
                expected: 1,
                actual: 0,
            }),
        }
    }
}

#[cfg(feature = "serde")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Json<T>(pub T);

#[cfg(feature = "serde")]
impl<T: serde::de::DeserializeOwned> Decode for Json<T> {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Json)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        serde_json::from_str(JsonText::decode(value)?.as_str())
            .map(Self)
            .map_err(DecodeError::Json)
    }
}

#[cfg(feature = "serde")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Jsonb<T>(pub T);

#[cfg(feature = "serde")]
impl<T: serde::de::DeserializeOwned> Decode for Jsonb<T> {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_binary(column, catalog::Builtin::Jsonb)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        serde_json::from_str(JsonbText::decode(value)?.as_str())
            .map(Self)
            .map_err(DecodeError::Json)
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> Encode for Json<T> {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Json;

    type Outcome = Result<(), EncodeError>;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) -> Result<(), EncodeError> {
        writer.bytes(&serde_json::to_vec(&self.0).map_err(EncodeError::Json)?);

        Ok(())
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> Encode for Jsonb<T> {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Jsonb;

    type Outcome = Result<(), EncodeError>;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) -> Result<(), EncodeError> {
        let text = serde_json::to_vec(&self.0).map_err(EncodeError::Json)?;

        writer.byte(JSONB_VERSION);
        writer.bytes(&text);

        Ok(())
    }
}

fn read_int32(value: &[u8], offset: &mut usize) -> Result<i32, DecodeError> {
    let end = *offset + 4;

    let bytes = value
        .get(*offset..end)
        .and_then(|bytes| <[u8; 4]>::try_from(bytes).ok())
        .ok_or(DecodeError::Length {
            expected: end,
            actual: value.len(),
        })?;

    *offset = end;

    Ok(i32::from_be_bytes(bytes))
}

impl<T: Decode> Decode for Array<T> {
    type Output<'r> = Array<T::Output<'r>>;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        catalog::Builtin::from_oid(column.data_type)
            .and_then(catalog::Builtin::element)
            .is_some_and(|element| {
                T::accepts(&crate::protocol::backend::ColumnDescription {
                    data_type: element.oid(),
                    ..column.clone()
                })
            })
    }

    fn decode(value: &[u8]) -> Result<Array<T::Output<'_>>, DecodeError> {
        let mut offset = 0;

        let ndim = read_int32(value, &mut offset)?;
        let axes = usize::try_from(ndim)
            .ok()
            .filter(|axes| *axes <= MAX_DIMENSIONS)
            .ok_or(DecodeError::ArrayDimensions { count: ndim })?;

        // The null flag and the element type: the column's type already
        // settled what the elements are.
        read_int32(value, &mut offset)?;
        read_int32(value, &mut offset)?;

        let mut dimensions = Vec::with_capacity(axes);

        for _ in 0..axes {
            let length = read_int32(value, &mut offset)?;
            let lower_bound = read_int32(value, &mut offset)?;

            dimensions.push(Dimension {
                length: u32::try_from(length).map_err(|_| DecodeError::ArrayAxis { length })?,
                lower_bound,
            });
        }

        let count = if dimensions.is_empty() {
            0
        } else {
            dimensions
                .iter()
                .try_fold(1usize, |count, dimension| {
                    usize::try_from(dimension.length)
                        .ok()
                        .and_then(|length| count.checked_mul(length))
                })
                .ok_or(DecodeError::ArrayShape(ArrayError::Unbounded))?
        };

        // Grown as elements arrive: the declared count alone could be huge.
        let mut elements = Vec::new();

        for index in 0..count {
            let length = read_int32(value, &mut offset)?;

            let Ok(length) = usize::try_from(length) else {
                elements.push(None);
                continue;
            };

            let end = offset + length;

            let bytes = value.get(offset..end).ok_or(DecodeError::Length {
                expected: end,
                actual: value.len(),
            })?;

            offset = end;

            elements.push(Some(T::decode(bytes).map_err(|source| {
                DecodeError::ArrayElement {
                    index,
                    source: Box::new(source),
                }
            })?));
        }

        if offset != value.len() {
            return Err(DecodeError::Length {
                expected: offset,
                actual: value.len(),
            });
        }

        Array::new(dimensions, elements).map_err(DecodeError::ArrayShape)
    }
}

impl<T: Decode> Decode for Option<T> {
    type Output<'r> = Option<T::Output<'r>>;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        T::accepts(column)
    }

    fn decode(value: &[u8]) -> Result<Self::Output<'_>, DecodeError> {
        T::decode(value).map(Some)
    }

    fn decode_null<'r>() -> Result<Self::Output<'r>, DecodeError> {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    fn dimension(length: u32) -> Dimension {
        Dimension {
            length,
            lower_bound: 1,
        }
    }

    #[test]
    fn says_which_type_it_is() {
        assert_eq!(Value::Int4(1).builtin(), catalog::Builtin::Int4);
        assert_eq!(
            Value::Text(String::from("a")).builtin(),
            catalog::Builtin::Text
        );
    }

    fn utc(year: i32, month: u32, day: u32, nanosecond: u32) -> chrono::DateTime<chrono::Utc> {
        chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_nano_opt(0, 0, 0, nanosecond)
            .unwrap()
            .and_utc()
    }

    #[test]
    fn encodes_a_date_time_like_the_timestamptz_it_names() {
        let date_time = utc(2024, 2, 29, 123_456_000);
        let (mut direct, mut wrapped) = (Vec::new(), Vec::new());

        date_time
            .write_to(&mut frontend::Writer::new(&mut direct))
            .unwrap();
        TimestampTz::Finite(FiniteTimestampTz::truncate_precision(date_time).unwrap())
            .write_to(&mut frontend::Writer::new(&mut wrapped));

        assert_eq!(direct, wrapped);
    }

    #[test]
    fn refuses_to_encode_a_date_time_with_sub_microsecond_digits() {
        let error = utc(2024, 2, 29, 123_456_789)
            .write_to(&mut frontend::Writer::new(&mut Vec::new()))
            .unwrap_err();

        assert!(
            matches!(error, EncodeError::TimestampPrecision),
            "{error:?}"
        );
    }

    #[test]
    fn refuses_to_encode_a_date_time_postgresql_cannot_store() {
        let error = (utc(-4713, 11, 24, 0) - chrono::TimeDelta::microseconds(1))
            .write_to(&mut frontend::Writer::new(&mut Vec::new()))
            .unwrap_err();

        assert!(matches!(error, EncodeError::TimestampRange(_)), "{error:?}");
    }

    #[test]
    fn truncates_a_timestamp_to_microseconds() {
        assert_eq!(
            FiniteTimestampTz::truncate_precision(utc(2024, 2, 29, 123_456_789))
                .unwrap()
                .get(),
            utc(2024, 2, 29, 123_456_000)
        );
    }

    #[test]
    fn truncates_a_timestamp_before_1970_towards_the_past() {
        assert_eq!(
            FiniteTimestampTz::truncate_precision(utc(1969, 12, 31, 999_999_999))
                .unwrap()
                .get(),
            utc(1969, 12, 31, 999_999_000)
        );
    }

    #[test]
    fn accepts_the_earliest_timestamp_postgresql_stores() {
        let earliest = utc(-4713, 11, 24, 0);

        assert_eq!(
            FiniteTimestampTz::truncate_precision(earliest)
                .unwrap()
                .get(),
            earliest
        );
        assert_eq!(
            FiniteTimestampTz::truncate_precision(earliest - chrono::TimeDelta::microseconds(1)),
            Err(TimestampRangeError)
        );
    }

    #[test]
    fn accepts_the_latest_timestamp_chrono_represents() {
        assert_eq!(
            FiniteTimestampTz::truncate_precision(chrono::DateTime::<chrono::Utc>::MAX_UTC)
                .unwrap()
                .get(),
            chrono::DateTime::<chrono::Utc>::MAX_UTC - chrono::TimeDelta::nanoseconds(999)
        );
    }

    #[test]
    fn refuses_to_decode_a_timestamp_chrono_cannot_represent() {
        let microseconds = END_TIMESTAMP - 1;

        let error = TimestampTz::decode(&microseconds.to_be_bytes()).unwrap_err();

        assert!(
            matches!(error, DecodeError::TimestampUnrepresentable { microseconds: actual } if actual == microseconds),
            "{error:?}"
        );
    }

    #[test]
    fn decodes_the_infinities() {
        assert_eq!(
            TimestampTz::decode(&i64::MIN.to_be_bytes()).unwrap(),
            TimestampTz::NegativeInfinity
        );
        assert_eq!(
            TimestampTz::decode(&i64::MAX.to_be_bytes()).unwrap(),
            TimestampTz::Infinity
        );
        assert_eq!(
            Timestamp::decode(&i64::MIN.to_be_bytes()).unwrap(),
            Timestamp::NegativeInfinity
        );
        assert_eq!(
            Timestamp::decode(&i64::MAX.to_be_bytes()).unwrap(),
            Timestamp::Infinity
        );
    }

    #[test]
    fn refuses_to_decode_an_infinity_into_a_date_time() {
        let error =
            <chrono::DateTime<chrono::Utc> as Decode>::decode(&i64::MAX.to_be_bytes()).unwrap_err();

        assert!(matches!(error, DecodeError::InfiniteTimestamp), "{error:?}");
    }

    #[test]
    fn refuses_to_decode_a_timestamp_outside_the_range() {
        for microseconds in [MIN_TIMESTAMP - 1, END_TIMESTAMP] {
            let error = TimestampTz::decode(&microseconds.to_be_bytes()).unwrap_err();

            assert!(
                matches!(error, DecodeError::TimestampRange { microseconds: actual } if actual == microseconds),
                "{error:?}"
            );
        }
    }

    #[test]
    fn orders_the_infinities_around_every_finite_timestamp() {
        let finite = TimestampTz::Finite(
            FiniteTimestampTz::truncate_precision(utc(2024, 2, 29, 0)).unwrap(),
        );

        assert!(TimestampTz::NegativeInfinity < finite);
        assert!(finite < TimestampTz::Infinity);
    }

    #[test]
    fn accepts_the_earliest_date_postgresql_stores() {
        let earliest = chrono::NaiveDate::from_ymd_opt(-4713, 11, 24).unwrap();

        assert_eq!(FiniteDate::new(earliest).unwrap().get(), earliest);
        assert_eq!(
            FiniteDate::new(earliest.pred_opt().unwrap()),
            Err(DateRangeError)
        );
    }

    #[test]
    fn accepts_the_latest_date_chrono_represents() {
        assert_eq!(
            FiniteDate::new(chrono::NaiveDate::MAX).unwrap().get(),
            chrono::NaiveDate::MAX
        );
    }

    #[test]
    fn decodes_the_date_infinities() {
        assert_eq!(
            Date::decode(&i32::MIN.to_be_bytes()).unwrap(),
            Date::NegativeInfinity
        );
        assert_eq!(
            Date::decode(&i32::MAX.to_be_bytes()).unwrap(),
            Date::Infinity
        );
    }

    #[test]
    fn refuses_to_decode_an_infinity_into_a_naive_date() {
        let error = <chrono::NaiveDate as Decode>::decode(&i32::MIN.to_be_bytes()).unwrap_err();

        assert!(matches!(error, DecodeError::InfiniteDate), "{error:?}");
    }

    #[test]
    fn refuses_to_decode_a_date_outside_the_range() {
        for days in [MIN_DATE - 1, END_DATE] {
            let error = Date::decode(&days.to_be_bytes()).unwrap_err();

            assert!(
                matches!(error, DecodeError::DateRange { days: actual } if actual == days),
                "{error:?}"
            );
        }
    }

    #[test]
    fn refuses_to_decode_a_date_chrono_cannot_represent() {
        let days = END_DATE - 1;

        let error = Date::decode(&days.to_be_bytes()).unwrap_err();

        assert!(
            matches!(error, DecodeError::DateUnrepresentable { days: actual } if actual == days),
            "{error:?}"
        );
    }

    fn numeric_bytes(ndigits: i16, weight: i16, sign: u16, dscale: u16, digits: &[i16]) -> Vec<u8> {
        [
            ndigits.to_be_bytes(),
            weight.to_be_bytes(),
            sign.to_be_bytes(),
            dscale.to_be_bytes(),
        ]
        .concat()
        .into_iter()
        .chain(digits.iter().flat_map(|digit| digit.to_be_bytes()))
        .collect()
    }

    fn numeric(text: &str) -> Numeric {
        Numeric::Finite(FiniteNumeric::new(text.parse().unwrap()).unwrap())
    }

    #[test]
    fn decodes_a_numeric_from_its_base_10000_digits() {
        assert_eq!(
            Numeric::decode(&numeric_bytes(3, 1, NUMERIC_NEGATIVE, 3, &[12, 3456, 7800])).unwrap(),
            numeric("-123456.780")
        );
    }

    #[test]
    fn refuses_a_numeric_sign_postgresql_does_not_send() {
        let error = Numeric::decode(&numeric_bytes(0, 0, 0x8000, 0, &[])).unwrap_err();

        assert!(
            matches!(error, DecodeError::NumericSign { sign: 0x8000 }),
            "{error:?}"
        );
    }

    #[test]
    fn refuses_a_numeric_digit_outside_base_10000() {
        let error =
            Numeric::decode(&numeric_bytes(1, 0, NUMERIC_POSITIVE, 0, &[10_000])).unwrap_err();

        assert!(
            matches!(error, DecodeError::NumericDigit { digit: 10_000 }),
            "{error:?}"
        );
    }

    #[test]
    fn refuses_a_negative_numeric_digit_count() {
        let error = Numeric::decode(&numeric_bytes(-1, 0, NUMERIC_POSITIVE, 0, &[])).unwrap_err();

        assert!(
            matches!(error, DecodeError::NumericDigitCount { ndigits: -1 }),
            "{error:?}"
        );
    }

    #[test]
    fn refuses_numeric_digits_the_length_does_not_cover() {
        let error = Numeric::decode(&numeric_bytes(2, 0, NUMERIC_POSITIVE, 0, &[1])).unwrap_err();

        assert!(
            matches!(
                error,
                DecodeError::Length {
                    expected: 12,
                    actual: 10
                }
            ),
            "{error:?}"
        );
    }

    #[test]
    fn refuses_a_numeric_display_scale_above_the_maximum() {
        let error =
            Numeric::decode(&numeric_bytes(0, 0, NUMERIC_POSITIVE, 0x4000, &[])).unwrap_err();

        assert!(
            matches!(error, DecodeError::NumericScale { dscale: 0x4000 }),
            "{error:?}"
        );
    }

    #[test]
    fn refuses_numeric_digits_beyond_the_display_scale() {
        let error =
            Numeric::decode(&numeric_bytes(1, -1, NUMERIC_POSITIVE, 2, &[1234])).unwrap_err();

        assert!(
            matches!(error, DecodeError::NumericDigitsBeyondScale),
            "{error:?}"
        );
    }

    #[test]
    fn refuses_a_numeric_scale_above_the_maximum() {
        assert_eq!(
            FiniteNumeric::new(bigdecimal::BigDecimal::new(1.into(), 16_384)),
            Err(NumericRangeError)
        );
        assert!(FiniteNumeric::new(bigdecimal::BigDecimal::new(1.into(), 16_383)).is_ok());
    }

    #[test]
    fn refuses_more_numeric_digits_before_the_point_than_postgresql_stores() {
        let largest = bigdecimal::BigDecimal::new(1.into(), -131_071);

        assert!(FiniteNumeric::new(largest.clone()).is_ok());
        assert_eq!(
            FiniteNumeric::new(largest * bigdecimal::BigDecimal::from(10)),
            Err(NumericRangeError)
        );
    }

    #[test]
    fn compares_numerics_by_value_not_scale() {
        assert_eq!(numeric("1.5"), numeric("1.50"));
    }

    #[test]
    fn orders_nan_above_every_other_numeric() {
        assert!(Numeric::NegativeInfinity < numeric("-1"));
        assert!(numeric("1") < Numeric::Infinity);
        assert!(Numeric::Infinity < Numeric::NaN);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn refuses_to_encode_what_json_cannot_express() {
        let mut bytes = Vec::new();

        let error = Json(std::collections::BTreeMap::from([(vec![1u8], 1)]))
            .write_to(&mut frontend::Writer::new(&mut bytes))
            .unwrap_err();

        assert!(matches!(error, EncodeError::Json(_)), "{error:?}");
    }

    fn array_bytes(ndim: i32, axes: &[(i32, i32)], elements: &[&[u8]]) -> Vec<u8> {
        let mut bytes = [ndim.to_be_bytes(), 0i32.to_be_bytes(), 23u32.to_be_bytes()].concat();

        for (length, lower_bound) in axes {
            bytes.extend(length.to_be_bytes());
            bytes.extend(lower_bound.to_be_bytes());
        }

        for element in elements {
            bytes.extend(i32::try_from(element.len()).unwrap().to_be_bytes());
            bytes.extend(*element);
        }

        bytes
    }

    #[test]
    fn refuses_an_array_with_more_dimensions_than_postgresql_allows() {
        let error = <Array<i32> as Decode>::decode(&array_bytes(7, &[], &[])).unwrap_err();

        assert!(
            matches!(error, DecodeError::ArrayDimensions { count: 7 }),
            "{error:?}"
        );
    }

    #[test]
    fn refuses_an_array_axis_with_a_negative_length() {
        let error = <Array<i32> as Decode>::decode(&array_bytes(1, &[(-1, 1)], &[])).unwrap_err();

        assert!(
            matches!(error, DecodeError::ArrayAxis { length: -1 }),
            "{error:?}"
        );
    }

    #[test]
    fn refuses_an_array_with_bytes_after_its_elements() {
        let mut bytes = array_bytes(1, &[(1, 1)], &[&7i32.to_be_bytes()]);
        bytes.push(0);

        let error = <Array<i32> as Decode>::decode(&bytes).unwrap_err();

        assert!(matches!(error, DecodeError::Length { .. }), "{error:?}");
    }

    #[test]
    fn names_the_array_element_that_does_not_decode() {
        let bytes = array_bytes(1, &[(2, 1)], &[&7i32.to_be_bytes(), &[0, 0, 7]]);

        let error = <Array<i32> as Decode>::decode(&bytes).unwrap_err();

        assert!(
            matches!(error, DecodeError::ArrayElement { index: 1, .. }),
            "{error:?}"
        );
    }

    #[test]
    fn refuses_a_uuid_that_is_not_16_bytes() {
        let error = <uuid::Uuid as Decode>::decode(&[0xab; 15]).unwrap_err();

        assert!(
            matches!(
                error,
                DecodeError::Length {
                    expected: 16,
                    actual: 15
                }
            ),
            "{error:?}"
        );
    }

    #[test]
    fn keeps_types_apart_that_share_a_representation() {
        let text = String::from("a");

        assert_ne!(
            Value::Text(text.clone()).builtin(),
            Value::Varchar(text.clone()).builtin()
        );
        assert_ne!(
            Value::Varchar(text.clone()).builtin(),
            Value::Bpchar(text).builtin()
        );
    }

    #[test]
    fn accepts_a_shape_that_matches_its_elements() {
        let array = Array::new(
            vec![dimension(2), dimension(3)],
            (1..=6).map(Some).collect::<Vec<Option<i32>>>(),
        )
        .unwrap();

        assert_eq!(array.dimensions(), [dimension(2), dimension(3)]);
        assert_eq!(array.elements().len(), 6);
    }

    #[test]
    fn refuses_a_shape_that_does_not_match_its_elements() {
        assert_eq!(
            Array::new(vec![dimension(2), dimension(3)], vec![Some(1), Some(2)]),
            Err(ArrayError::ElementCount {
                expected: 6,
                actual: 2
            })
        );
    }

    #[test]
    fn accepts_the_empty_array() {
        let array = Array::new(Vec::new(), Vec::<Option<i32>>::new()).unwrap();

        assert_eq!(array.dimensions(), []);
        assert_eq!(array.elements(), []);
    }

    #[test]
    fn builds_one_axis_from_a_vec() {
        let array = Array::from(vec![Some(1), None, Some(3)]);

        assert_eq!(array.dimensions(), [dimension(3)]);
        assert_eq!(array.elements(), [Some(1), None, Some(3)]);
    }

    #[test]
    fn builds_the_empty_array_from_an_empty_vec() {
        let array = Array::from(Vec::<i32>::new());

        assert_eq!(array, Array::new(Vec::new(), Vec::new()).unwrap());
    }

    #[test]
    fn refuses_more_axes_than_postgresql_admits() {
        assert_eq!(
            Array::new(vec![dimension(1); 7], vec![Some(1)]),
            Err(ArrayError::Dimensions { count: 7 })
        );
    }

    #[test]
    fn refuses_a_shape_too_large_to_count() {
        assert_eq!(
            Array::new(vec![dimension(u32::MAX); 5], vec![Some(1)]),
            Err(ArrayError::Unbounded)
        );
    }

    #[test]
    fn addresses_an_element_by_its_subscripts() {
        let array = Array::new(
            vec![dimension(2), dimension(3)],
            (1..=6).map(Some).collect::<Vec<Option<i32>>>(),
        )
        .unwrap();

        assert_eq!(array.get(&[1, 1]), Some(Some(&1)));
        assert_eq!(array.get(&[1, 3]), Some(Some(&3)));
        assert_eq!(array.get(&[2, 1]), Some(Some(&4)));
        assert_eq!(array.get(&[2, 3]), Some(Some(&6)));
    }

    #[test]
    fn addresses_from_the_lower_bound_the_axis_declares() {
        let array = Array::new(
            vec![Dimension {
                length: 3,
                lower_bound: 0,
            }],
            (1..=3).map(Some).collect::<Vec<Option<i32>>>(),
        )
        .unwrap();

        assert_eq!(array.get(&[0]), Some(Some(&1)));
        assert_eq!(array.get(&[2]), Some(Some(&3)));
        assert_eq!(array.get(&[3]), None);
    }

    #[test]
    fn tells_a_null_element_from_one_that_is_not_there() {
        let array = Array::new(vec![dimension(2)], vec![Some(1), None]).unwrap();

        assert_eq!(array.get(&[2]), Some(None));
        assert_eq!(array.get(&[3]), None);
    }

    #[test]
    fn refuses_subscripts_that_do_not_match_the_axes() {
        let array = Array::new(
            vec![dimension(2), dimension(3)],
            (1..=6).map(Some).collect::<Vec<Option<i32>>>(),
        )
        .unwrap();

        assert_eq!(array.get(&[1]), None);
        assert_eq!(array.get(&[1, 1, 1]), None);
    }
}
