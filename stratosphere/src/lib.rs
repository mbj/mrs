pub mod value {
    pub use stratosphere_core::value::*;
}

pub mod template {
    pub use stratosphere_core::template::*;
}

pub mod resource_specification {
    pub use stratosphere_core::resource_specification::*;
}

pub mod generator {
    pub use stratosphere_generator::*;
}

pub mod token {
    pub use stratosphere_core::token::*;
}

pub use crate::template::Template;

#[macro_export]
macro_rules! Parameter {
    (
        r#type: $type:expr
        $(,)?
    ) => {
        $crate::template::Parameter {
            description: None,
            r#type: $type,
            allowed_pattern: None,
        }
    };
    (
        description: $description:expr,
        r#type: $type:expr
        $(,)?
    ) => {
        $crate::template::Parameter {
            description: Some($description.into()),
            r#type: $type,
            allowed_pattern: None,
        }
    };
    (
        description: $description:expr,
        r#type: $type:expr,
        allowed_pattern: $allowed_pattern:expr
        $(,)?
    ) => {
        $crate::template::Parameter {
            description: Some($description.into()),
            r#type: $type,
            allowed_pattern: Some($allowed_pattern.into()),
        }
    };
}

#[macro_export]
macro_rules! Output {
    (
        description: $description:expr,
        value: $value:expr
        $(,)?
    ) => {
        $crate::template::Output {
            description: $description.into(),
            value: $value.into(),
            export: None,
        }
    };
    (
        description: $description:expr,
        value: $value:expr,
        export: $export:expr
        $(,)?
    ) => {
        $crate::template::Output {
            description: $description.into(),
            value: $value.into(),
            export: Some($export.into()),
        }
    };
}

#[macro_export]
macro_rules! fn_join {
    (
        $delimiter:expr,
        [$($value:expr),* $(,)?]
    ) => {
        $crate::value::join(
            $delimiter,
            vec![$($value.into()),*]
        )
    };
}

#[macro_export]
macro_rules! fn_sub {
    ($pattern:expr) => {
        $crate::value::ExpString::Sub {
            pattern: $pattern.into(),
        }
    };
}

#[macro_export]
macro_rules! fn_select {
    (
        $index:expr,
        [$($value:expr),* $(,)?]
    ) => {
        $crate::value::ExpString::Select {
            index: $index,
            values: vec![$($value.into()),*],
        }
    };
}

#[macro_export]
macro_rules! fn_get_att {
    ($resource:expr, $attribute:expr) => {
        $crate::value::get_att($resource, $attribute)
    };
}
