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
macro_rules! Output {
    (
        description: $description:expr,
        value: $value:expr,
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
        export: $export:expr,
    ) => {
        $crate::template::Output {
            description: $description.into(),
            value: $value.into(),
            export: Some($export.into()),
        }
    };
}
