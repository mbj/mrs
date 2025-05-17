#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Index(u32);

impl Index {
    /// Return successor of index
    ///
    /// # Example
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let a: Index = 0_u32.into();
    /// let b: Index = 1_u32.into();
    ///
    /// assert_eq!(a.succ(), b);
    /// ```
    pub fn succ(&self) -> Index {
        Self(self.0.checked_add(1).unwrap())
    }

    /// Test if index is initial one
    ///
    /// # Example
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let a: Index = 0_u32.into();
    /// let b: Index = 1_u32.into();
    ///
    /// assert_eq!(a.is_initial(), true);
    /// assert_eq!(b.is_initial(), false);
    /// ```
    pub fn is_initial(&self) -> bool {
        self == &Self::initial()
    }

    /// Return initial index
    ///
    /// # Example
    /// ```
    /// # use mmigration::*;
    ///
    /// let index : Index = 0.into();
    ///
    /// assert_eq!(index, Index::initial());
    /// ```
    pub fn initial() -> Index {
        Self(0)
    }

    /// Test if other is successor of self
    ///
    /// # Example
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let a: Index = 0_u32.into();
    /// let b: Index = 1_u32.into();
    /// let c: Index = 2_u32.into();
    ///
    /// assert_eq!(a.is_succ_of(a), false);
    /// assert_eq!(a.is_succ_of(b), false);
    /// assert_eq!(b.is_succ_of(a), true);
    /// assert_eq!(c.is_succ_of(a), false);
    /// assert_eq!(c.is_succ_of(b), true);
    /// ```
    pub fn is_succ_of(&self, other: Self) -> bool {
        *self == other.succ()
    }
}

impl std::fmt::Display for Index {
    /// Format index
    ///
    /// ```
    /// # use mmigration::*;
    /// assert_eq!("0", format!("{}", Index::initial()));
    /// ```
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

impl From<u32> for Index {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for Index {
    type Err = std::num::ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        <u32 as std::str::FromStr>::from_str(value).map(Self)
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for Index {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'_>,
    ) -> Result<Index, Box<dyn std::error::Error + 'static + Send + Sync>> {
        <i64 as sqlx::Decode<sqlx::Postgres>>::decode(value).and_then(|value| {
            match value.try_into() {
                Ok(valid) => Ok(Self(valid)),
                Err(_) => Err("out of u32 range".into()),
            }
        })
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for Index {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <i64 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0.into(), buf)
    }
}

impl sqlx::Type<sqlx::Postgres> for Index {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <i64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl sqlx::postgres::PgHasArrayType for Index {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <i64 as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawSql(String);

impl AsRef<[u8]> for RawSql {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<str> for RawSql {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<String> for RawSql {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for RawSql {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct PendingMigration {
    pub(crate) index: Index,
    pub(crate) sql: std::rc::Rc<RawSql>,
}

impl PendingMigration {
    pub fn new(index: Index, raw_sql: RawSql) -> Self {
        Self {
            index,
            sql: std::rc::Rc::new(raw_sql),
        }
    }

    pub fn digest(&self) -> [u8; 32] {
        <sha2::Sha256 as sha2::Digest>::digest(self.raw_sql()).into()
    }

    pub fn index(&self) -> Index {
        self.index
    }

    pub fn raw_sql(&self) -> &RawSql {
        std::rc::Rc::<RawSql>::as_ref(&self.sql)
    }
}

pub struct SchemaDump(String);

impl AsRef<[u8]> for SchemaDump {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<String> for SchemaDump {
    fn from(value: String) -> Self {
        Self(value)
    }
}
