#![doc = include_str!("../README.md")]

pub mod value {
    pub use stratosphere_core::value::*;
}

pub mod template {
    pub use stratosphere_core::template::*;
}

pub mod resource_specification {
    pub use stratosphere_core::resource_specification::*;
}

pub mod token {
    pub use stratosphere_core::token::*;
}

pub use crate::services::tag::Tag;
pub use crate::services::tag::Tag_;
pub use crate::template::Template;

pub mod lambda;
pub mod logs;
pub mod services;

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
            values: Box::new(vec![$($value.into()),*].into()),
        }
    };
}

#[macro_export]
macro_rules! fn_get_att {
    ($resource:expr, $attribute:expr) => {
        $crate::value::get_att($resource, $attribute)
    };
}

#[macro_export]
macro_rules! fn_get_att_arn {
    ($resource:expr) => {
        $crate::value::get_att_arn($resource)
    };
}

#[macro_export]
macro_rules! fn_import_value {
    ($export_name:expr) => {
        $crate::value::ExpString::ImportValue($export_name.into())
    };
}

#[macro_export]
macro_rules! fn_base64 {
    ($value:expr) => {
        $crate::value::ExpString::Base64(Box::new($value.into()))
    };
}

#[macro_export]
macro_rules! mk_name {
    ($suffix:expr) => {
        $crate::value::mk_name($suffix)
    };
}

#[macro_export]
macro_rules! fn_if {
    ($condition:expr, $true_value:expr, $false_value:expr) => {
        $crate::value::fn_if::<$crate::value::ExpString>($condition, $true_value, $false_value)
    };
}

#[macro_export]
macro_rules! fn_and {
    ($left:expr, $right:expr) => {
        $crate::value::ExpBool::And(Box::new($left.into()), Box::new($right.into()))
    };
}

#[macro_export]
macro_rules! fn_or {
    ([$($condition:expr),* $(,)?]) => {
        $crate::value::ExpBool::Or(vec![$($condition.into()),*])
    };
}

#[macro_export]
macro_rules! fn_not {
    ($value:expr) => {
        $crate::value::ExpBool::Not(Box::new($value.into()))
    };
}

#[macro_export]
macro_rules! fn_equals_string {
    ($left:expr, $right:expr) => {
        $crate::value::equals_string($left, $right)
    };
}

#[macro_export]
macro_rules! fn_equals_bool {
    ($left:expr, $right:expr) => {
        $crate::value::equals_bool($left, $right)
    };
}
