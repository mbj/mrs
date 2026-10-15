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
        }
    }
}

impl frontend::Parameter for Value {
    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        match self {
            Self::Bool(value) => value.write_to(writer),
            Self::Bytea(value) => value.write_to(writer),
            Self::Char(value) => value.write_to(writer),
            Self::Int2(value) => value.write_to(writer),
            Self::Int4(value) => value.write_to(writer),
            Self::Int8(value) => value.write_to(writer),
            Self::Float4(value) => value.write_to(writer),
            Self::Float8(value) => value.write_to(writer),
            Self::Text(value) | Self::Varchar(value) | Self::Bpchar(value) => {
                value.write_to(writer);
            }
            Self::Oid(value) => value.write_to(writer),

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
        }
    }
}

pub trait Encode {
    const BUILTIN: catalog::Builtin;

    fn write_to(&self, writer: &mut frontend::Writer<'_>);

    fn is_null(&self) -> bool {
        false
    }
}

impl<T: Encode + ?Sized> Encode for &T {
    const BUILTIN: catalog::Builtin = T::BUILTIN;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        T::write_to(self, writer);
    }

    fn is_null(&self) -> bool {
        T::is_null(self)
    }
}

impl<T: Encode> Encode for Option<T> {
    const BUILTIN: catalog::Builtin = T::BUILTIN;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        if let Some(value) = self {
            value.write_to(writer);
        }
    }

    fn is_null(&self) -> bool {
        self.as_ref().is_none_or(Encode::is_null)
    }
}

impl Encode for bool {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Bool;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.byte(u8::from(*self));
    }
}

impl Encode for i8 {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Char;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.int8(*self);
    }
}

impl Encode for i16 {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Int2;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.int16(*self);
    }
}

impl Encode for i32 {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Int4;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.int32(*self);
    }
}

impl Encode for i64 {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Int8;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.int64(*self);
    }
}

impl Encode for f32 {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Float4;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.uint32(self.to_bits());
    }
}

impl Encode for f64 {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Float8;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.uint64(self.to_bits());
    }
}

impl Encode for str {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Text;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.bytes(self.as_bytes());
    }
}

impl Encode for String {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Text;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.bytes(self.as_bytes());
    }
}

impl Encode for [u8] {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Bytea;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.bytes(self);
    }
}

impl Encode for Vec<u8> {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Bytea;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.bytes(self);
    }
}

impl Encode for Oid {
    const BUILTIN: catalog::Builtin = catalog::Builtin::Oid;

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        writer.uint32(self.get());
    }
}

impl<T: Encode> Encode for Array<T> {
    const BUILTIN: catalog::Builtin = match T::BUILTIN.array() {
        Some(array) => array,
        None => panic!("the element type has no array type"),
    };

    fn write_to(&self, writer: &mut frontend::Writer<'_>) {
        write_array(writer, self, T::BUILTIN);
    }
}

fn write_array<T: Encode>(
    writer: &mut frontend::Writer<'_>,
    array: &Array<T>,
    element: catalog::Builtin,
) {
    writer.int32(
        i32::try_from(array.dimensions.len())
            .expect("dimension count exceeds the Int32 field, though Array::new caps it at 6"),
    );
    writer.int32(i32::from(array.elements.iter().any(Option::is_none)));
    writer.uint32(element.oid().get());

    for dimension in &array.dimensions {
        writer.int32(
            i32::try_from(dimension.length).expect("dimension length exceeds the Int32 field"),
        );
        writer.int32(dimension.lower_bound);
    }

    for element in &array.elements {
        match element {
            None => writer.int32(-1),
            Some(value) => writer.length_prefixed(|writer| value.write_to(writer)),
        }
    }
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
    #[error("value is not a valid name")]
    InvalidName(#[source] crate::name::ParseError),
    #[cfg(feature = "serde")]
    #[error("value is not the expected JSON")]
    Json(#[source] serde_json::Error),
}

pub trait Decode<'a>: Sized {
    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool;

    fn decode(value: &'a [u8]) -> Result<Self, DecodeError>;

    fn decode_null() -> Result<Self, DecodeError> {
        Err(DecodeError::Null)
    }
}

impl Decode<'_> for bool {
    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        column.data_type == catalog::Builtin::Bool.oid()
            && column.format == crate::protocol::Format::Binary
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        match value {
            [byte] => Ok(*byte != 0),
            _ => Err(DecodeError::Length {
                expected: 1,
                actual: value.len(),
            }),
        }
    }
}

impl Decode<'_> for i32 {
    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        column.data_type == catalog::Builtin::Int4.oid()
            && column.format == crate::protocol::Format::Binary
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        <[u8; 4]>::try_from(value)
            .map(Self::from_be_bytes)
            .map_err(|_| DecodeError::Length {
                expected: 4,
                actual: value.len(),
            })
    }
}

impl Decode<'_> for i64 {
    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        column.data_type == catalog::Builtin::Int8.oid()
            && column.format == crate::protocol::Format::Binary
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        <[u8; 8]>::try_from(value)
            .map(Self::from_be_bytes)
            .map_err(|_| DecodeError::Length {
                expected: 8,
                actual: value.len(),
            })
    }
}

pub(crate) fn accepts_text(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
    [
        catalog::Builtin::Text,
        catalog::Builtin::Varchar,
        catalog::Builtin::Bpchar,
        catalog::Builtin::Name,
    ]
    .iter()
    .any(|builtin| column.data_type == builtin.oid())
        && column.format == crate::protocol::Format::Binary
}

impl Decode<'_> for String {
    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        accepts_text(column)
    }

    fn decode(value: &[u8]) -> Result<Self, DecodeError> {
        core::str::from_utf8(value)
            .map(str::to_owned)
            .map_err(|_| DecodeError::NotUtf8)
    }
}

#[cfg(feature = "serde")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Json<T>(pub T);

#[cfg(feature = "serde")]
impl<'a, T: serde::Deserialize<'a>> Decode<'a> for Json<T> {
    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        column.data_type == catalog::Builtin::Json.oid()
            && column.format == crate::protocol::Format::Binary
    }

    fn decode(value: &'a [u8]) -> Result<Self, DecodeError> {
        serde_json::from_slice(value)
            .map(Self)
            .map_err(DecodeError::Json)
    }
}

#[cfg(feature = "serde")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Jsonb<T>(pub T);

#[cfg(feature = "serde")]
impl<'a, T: serde::Deserialize<'a>> Decode<'a> for Jsonb<T> {
    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        column.data_type == catalog::Builtin::Jsonb.oid()
            && column.format == crate::protocol::Format::Binary
    }

    fn decode(value: &'a [u8]) -> Result<Self, DecodeError> {
        match value {
            [1, text @ ..] => serde_json::from_slice(text)
                .map(Self)
                .map_err(DecodeError::Json),
            [version, ..] => Err(DecodeError::JsonbVersion { version: *version }),
            [] => Err(DecodeError::Length {
                expected: 1,
                actual: 0,
            }),
        }
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Option<T> {
    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        T::accepts(column)
    }

    fn decode(value: &'a [u8]) -> Result<Self, DecodeError> {
        T::decode(value).map(Some)
    }

    fn decode_null() -> Result<Self, DecodeError> {
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
