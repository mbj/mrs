#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sql<'a>(crate::protocol::String<'a>);

impl Sql<'static> {
    pub const EMPTY: Self = Self(crate::protocol::String::EMPTY);

    #[must_use]
    pub const fn from_static_or_panic(sql: &'static str) -> Self {
        Self(crate::protocol::String::from_static_or_panic(sql))
    }
}

impl<'a> Sql<'a> {
    pub fn sql_injection_is_on_you(
        sql: impl Into<std::borrow::Cow<'a, str>>,
    ) -> Result<Self, crate::protocol::StringError> {
        crate::protocol::String::new(sql).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[must_use]
    pub const fn as_protocol_string(&self) -> &crate::protocol::String<'a> {
        &self.0
    }
}

fn decode(value: &[u8]) -> Result<Sql<'_>, crate::value::DecodeError> {
    let text = <&str as crate::value::Decode>::decode(value)?;

    Sql::sql_injection_is_on_you(text).map_err(
        |crate::protocol::StringError::ContainsNul { index }| {
            crate::value::DecodeError::ContainsNul { index }
        },
    )
}

impl crate::value::Decode for &Sql<'_> {
    type Output<'r> = Sql<'r>;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        crate::value::accepts_binary(column, crate::catalog::Builtin::Text)
    }

    fn decode(value: &[u8]) -> Result<Sql<'_>, crate::value::DecodeError> {
        decode(value)
    }
}

impl crate::value::Decode for Sql<'static> {
    type Output<'r> = Self;

    fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
        <&Sql<'_>>::accepts(column)
    }

    fn decode(value: &[u8]) -> Result<Self, crate::value::DecodeError> {
        decode(value).map(|sql| Sql(sql.0.into_owned()))
    }
}

#[macro_export]
macro_rules! sql {
    (@reject format ! $($rest:tt)*) => {
        compile_error!("sql! refuses format!: bind values as parameters instead")
    };
    (@reject format_args ! $($rest:tt)*) => {
        compile_error!("sql! refuses format_args!: bind values as parameters instead")
    };
    (@reject ( $($inner:tt)* ) $($rest:tt)*) => {
        $crate::sql!(@reject $($inner)*);
        $crate::sql!(@reject $($rest)*);
    };
    (@reject [ $($inner:tt)* ] $($rest:tt)*) => {
        $crate::sql!(@reject $($inner)*);
        $crate::sql!(@reject $($rest)*);
    };
    (@reject { $($inner:tt)* } $($rest:tt)*) => {
        $crate::sql!(@reject $($inner)*);
        $crate::sql!(@reject $($rest)*);
    };
    (@reject $head:tt $($rest:tt)*) => {
        $crate::sql!(@reject $($rest)*)
    };
    (@reject) => {};
    ($($sql:tt)+) => {{
        $crate::sql!(@reject $($sql)+);
        $crate::Sql::sql_injection_is_on_you($($sql)+)
    }};
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    #[test]
    fn builds_sql_from_text_the_caller_vouches_for() {
        assert_eq!(
            crate::sql!(String::from("SELECT 1")).unwrap().as_str(),
            "SELECT 1"
        );
        assert_eq!(
            crate::sql!(String::from("SELECT\0 1")),
            Err(crate::protocol::StringError::ContainsNul { index: 6 })
        );
    }
}
