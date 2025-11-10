use crate::template::LogicalResourceName;
use serde;
use serde_json;
use serde_json::json;

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct AttributeName(String);

impl From<&str> for AttributeName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct ConditionName(String);

impl From<&str> for ConditionName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

pub fn equals_bool<A: Into<ExpBool>, B: Into<ExpBool>>(left: A, right: B) -> ExpBool {
    ExpBool::Equals(ExpPair::Bool {
        left: Box::new(left.into()),
        right: Box::new(right.into()),
    })
}

pub fn equals_string<A: Into<ExpString>, B: Into<ExpString>>(left: A, right: B) -> ExpBool {
    ExpBool::Equals(ExpPair::String {
        left: Box::new(left.into()),
        right: Box::new(right.into()),
    })
}

/// Trait for expression types that support conditional (`Fn::If`) expressions
///
/// This trait enables generic helper functions for constructing conditional
/// expressions across different expression types (ExpString, ExpBool, etc.)
pub trait FnIf: Sized {
    /// Creates a conditional expression using `Fn::If`
    ///
    /// # Arguments
    ///
    /// * `condition_name` - Name of the condition to evaluate
    /// * `true_branch` - Value returned when condition is true
    /// * `else_branch` - Value returned when condition is false
    fn fn_if(
        condition_name: impl Into<ConditionName>,
        true_branch: impl Into<Self>,
        else_branch: impl Into<Self>,
    ) -> Self;
}

/// Generic helper function to create conditional (`Fn::If`) expressions
///
/// This function works with any type implementing [`FnIf`],
/// allowing for type-safe conditional expressions across different expression types.
///
/// # Arguments
///
/// * `condition_name` - Name of the condition to evaluate
/// * `true_branch` - Value returned when condition is true
/// * `else_branch` - Value returned when condition is false
///
/// # Examples
///
/// ```
/// # use stratosphere_core::value::*;
/// let expr: ExpString = fn_if("MyCondition", "value-if-true", "value-if-false");
/// ```
pub fn fn_if<T: FnIf>(
    condition_name: impl Into<ConditionName>,
    true_branch: impl Into<T>,
    else_branch: impl Into<T>,
) -> T {
    T::fn_if(condition_name, true_branch, else_branch)
}

/// Type-specific helper for boolean conditional expressions that doesn't require turbofish syntax.
/// Use this when working with ExpBool values.
pub fn fn_if_bool(
    condition_name: impl Into<ConditionName>,
    true_branch: impl Into<ExpBool>,
    false_branch: impl Into<ExpBool>,
) -> ExpBool {
    ExpBool::fn_if(condition_name, true_branch, false_branch)
}

/// Trait for expression types that support select (`Fn::Select`) expressions
///
/// This trait enables generic helper functions for selecting an item from a list
/// across different expression types (ExpString, ExpBool, etc.)
pub trait FnSelect: Sized {
    /// The type representing a list of values that can be selected from
    type ValueList;

    /// Creates a select expression using `Fn::Select`
    ///
    /// # Arguments
    ///
    /// * `index` - Zero-based index of the item to select
    /// * `values` - List of values to select from
    fn fn_select(index: u8, values: Self::ValueList) -> Self;
}

/// Generic helper function to create select (`Fn::Select`) expressions
///
/// This function works with any type implementing [`FnSelect`],
/// allowing for type-safe selection from lists across different expression types.
///
/// # Arguments
///
/// * `index` - Zero-based index of the item to select
/// * `values` - List of values to select from
///
/// # Examples
///
/// ```
/// # use stratosphere_core::value::*;
/// let expr: ExpString = fn_select(0, vec!["first".into(), "second".into()].into());
/// ```
pub fn fn_select<T: FnSelect>(index: u8, values: T::ValueList) -> T {
    T::fn_select(index, values)
}

/// Type-specific helper for boolean select expressions that doesn't require turbofish syntax.
/// Use this when working with ExpBool values.
pub fn fn_select_bool(index: u8, values: Vec<ExpBool>) -> ExpBool {
    ExpBool::fn_select(index, values)
}

/// Type-specific helper for string select expressions that doesn't require turbofish syntax.
/// Use this when working with ExpString values.
pub fn fn_select_string(index: u8, values: impl Into<ExpStringList>) -> ExpString {
    ExpString::fn_select(index, values.into())
}

/// Helper function to create a split expression
///
/// Returns an ExpString::Split expression representing a Fn::Split operation.
/// Typically used with fn_select_string to extract specific elements.
///
/// # Arguments
///
/// * `delimiter` - The delimiter to split on
/// * `source` - The source string to split
///
/// # Examples
///
/// ```
/// # use stratosphere_core::value::*;
/// // Select first element from split result
/// let first = fn_select_string(0, vec![fn_split(",", "a,b,c")]);
/// ```
pub fn fn_split(delimiter: impl Into<String>, source: impl Into<ExpString>) -> ExpString {
    ExpString::Split {
        delimiter: delimiter.into(),
        source: Box::new(source.into()),
    }
}

/// Trait for expression types that support find in map (`Fn::FindInMap`) expressions
///
/// This trait enables generic helper functions for retrieving values from mappings
/// across different expression types (ExpString, ExpBool, etc.)
pub trait FnFindInMap: Sized {
    /// Creates a find in map expression using `Fn::FindInMap`
    ///
    /// # Arguments
    ///
    /// * `map_name` - MapName referencing the mapping to look up
    /// * `top_level_key` - First-level key in the mapping
    /// * `second_level_key` - Second-level key in the mapping
    fn fn_find_in_map(
        map_name: impl Into<crate::template::MapName>,
        top_level_key: impl Into<ExpString>,
        second_level_key: impl Into<String>,
    ) -> Self;
}

/// Generic helper function to create find in map (`Fn::FindInMap`) expressions
///
/// This function works with any type implementing [`FnFindInMap`],
/// allowing for type-safe mapping lookups across different expression types.
///
/// # Arguments
///
/// * `map_name` - MapName referencing the mapping to look up
/// * `top_level_key` - First-level key in the mapping
/// * `second_level_key` - Second-level key in the mapping
///
/// # Examples
///
/// ```
/// # use stratosphere_core::value::*;
/// # use stratosphere_core::template::MapName;
/// let region_map: MapName = "RegionMap".into();
/// let expr: ExpString = fn_find_in_map(&region_map, "us-east-1", "AMI");
/// ```
pub fn fn_find_in_map<T: FnFindInMap>(
    map_name: impl Into<crate::template::MapName>,
    top_level_key: impl Into<ExpString>,
    second_level_key: impl Into<String>,
) -> T {
    T::fn_find_in_map(map_name, top_level_key, second_level_key)
}

/// Type-specific helper for string find in map expressions that doesn't require turbofish syntax.
/// Use this when working with ExpString values.
pub fn fn_find_in_map_string(
    map_name: impl Into<crate::template::MapName>,
    top_level_key: impl Into<ExpString>,
    second_level_key: impl Into<String>,
) -> ExpString {
    ExpString::fn_find_in_map(map_name, top_level_key, second_level_key)
}

/// Type-specific helper for boolean find in map expressions that doesn't require turbofish syntax.
/// Use this when working with ExpBool values.
pub fn fn_find_in_map_bool(
    map_name: impl Into<crate::template::MapName>,
    top_level_key: impl Into<ExpString>,
    second_level_key: impl Into<String>,
) -> ExpBool {
    ExpBool::fn_find_in_map(map_name, top_level_key, second_level_key)
}

/// Helper function to create a Fn::GetAZs expression
///
/// Returns an ExpStringList representing the list of Availability Zones for a region.
/// Pass an empty string to get AZs for the current region.
///
/// # Arguments
///
/// * `region` - The region to get AZs for, or empty string for current region
///
/// # Examples
///
/// ```
/// # use stratosphere_core::value::*;
/// // Get AZs for current region
/// let azs = fn_get_azs("");
///
/// // Get AZs for specific region
/// let azs = fn_get_azs("us-west-2");
///
/// // Use with Fn::Select to pick first AZ
/// let first_az = fn_select_string(0, fn_get_azs(""));
/// ```
pub fn fn_get_azs(region: impl Into<ExpString>) -> ExpStringList {
    ExpStringList::GetAZs {
        region: Box::new(region.into()),
    }
}

pub trait ToValue {
    fn to_value(&self) -> serde_json::Value;
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct OutputExportName(String);

impl From<&str> for OutputExportName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl<T: ToValue> ToValue for Box<T> {
    fn to_value(&self) -> serde_json::Value {
        self.as_ref().to_value()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpString {
    Base64(Box<ExpString>),
    FindInMap {
        map_name: crate::template::MapName,
        top_level_key: Box<ExpString>,
        second_level_key: String,
    },
    GetAtt {
        logical_resource_name: LogicalResourceName,
        attribute_name: AttributeName,
    },
    If {
        condition_name: ConditionName,
        true_branch: Box<ExpString>,
        else_branch: Box<ExpString>,
    },
    ImportValue(OutputExportName),
    Join {
        delimiter: String,
        values: Vec<ExpString>,
    },
    Literal(String),
    Ref(LogicalResourceName),
    Select {
        index: u8,
        values: Box<ExpStringList>,
    },
    Split {
        delimiter: String,
        source: Box<ExpString>,
    },
    Sub {
        pattern: String,
    },
}

impl ExpString {
    pub fn base64(self) -> ExpString {
        ExpString::Base64(Box::new(self))
    }
}

impl FnIf for ExpString {
    fn fn_if(
        condition_name: impl Into<ConditionName>,
        true_branch: impl Into<Self>,
        else_branch: impl Into<Self>,
    ) -> Self {
        ExpString::If {
            condition_name: condition_name.into(),
            true_branch: Box::new(true_branch.into()),
            else_branch: Box::new(else_branch.into()),
        }
    }
}

impl FnSelect for ExpString {
    type ValueList = ExpStringList;

    fn fn_select(index: u8, values: Self::ValueList) -> Self {
        ExpString::Select {
            index,
            values: Box::new(values),
        }
    }
}

impl FnFindInMap for ExpString {
    fn fn_find_in_map(
        map_name: impl Into<crate::template::MapName>,
        top_level_key: impl Into<ExpString>,
        second_level_key: impl Into<String>,
    ) -> Self {
        ExpString::FindInMap {
            map_name: map_name.into(),
            top_level_key: Box::new(top_level_key.into()),
            second_level_key: second_level_key.into(),
        }
    }
}

impl serde::Serialize for ExpString {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_value().serialize(serializer)
    }
}

impl From<&str> for ExpString {
    fn from(value: &str) -> Self {
        Self::Literal(value.to_string())
    }
}

impl From<LogicalResourceName> for ExpString {
    fn from(value: LogicalResourceName) -> Self {
        Self::Ref(value)
    }
}

impl From<&LogicalResourceName> for ExpString {
    fn from(value: &LogicalResourceName) -> Self {
        Self::Ref(value.clone())
    }
}

pub fn join(delimiter: &str, values: impl IntoIterator<Item = ExpString>) -> ExpString {
    ExpString::Join {
        delimiter: delimiter.to_string(),
        values: values.into_iter().collect(),
    }
}

pub fn get_att(
    logical_resource_name: impl Into<LogicalResourceName>,
    attribute_name: impl Into<AttributeName>,
) -> ExpString {
    ExpString::GetAtt {
        logical_resource_name: logical_resource_name.into(),
        attribute_name: attribute_name.into(),
    }
}

pub fn get_att_arn(logical_resource_name: impl Into<LogicalResourceName>) -> ExpString {
    get_att(logical_resource_name, "Arn")
}

pub fn mk_name(suffix: impl Into<ExpString>) -> ExpString {
    join("-", [AWS_STACK_NAME.into(), suffix.into()])
}

impl ToValue for &String {
    fn to_value(&self) -> serde_json::Value {
        ExpString::Literal(self.to_string()).to_value()
    }
}

impl ToValue for &LogicalResourceName {
    fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.0).unwrap()
    }
}

impl ToValue for ExpString {
    /// Render expression to CF template value^
    ///
    /// # Panics
    ///
    /// On internal errors/bugs, there is no public API that
    /// allows to construct values that panic on this call.
    ///
    /// # Examples
    ///
    /// [Fn::Base64](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-base64.html)
    ///
    /// ```
    /// # use stratosphere_core::value::*;
    /// # use serde_json::json;
    ///
    /// assert_eq!(
    ///   json!({"Fn::Base64":"some-literal"}),
    ///   ExpString::from("some-literal").base64().to_value()
    /// )
    /// ```
    ///
    /// [Fn::If](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-conditions.html#intrinsic-function-reference-conditions-if)
    ///
    /// ```
    /// # use stratosphere_core::template::*;
    /// # use stratosphere_core::value::*;
    /// # use serde_json::json;
    /// assert_eq!(
    ///   json!({"Fn::If":["condition-name",{"Ref":"resource-a"},{"Ref":"resource-b"}]}),
    ///   ExpString::If{
    ///     condition_name: "condition-name".into(),
    ///     true_branch: Box::new(LogicalResourceName::from("resource-a").into()),
    ///     else_branch: Box::new(LogicalResourceName::from("resource-b").into()),
    ///   }.to_value()
    /// )
    /// ```
    ///
    /// [Ref](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-ref.html)
    ///
    /// ```
    /// # use stratosphere_core::template::LogicalResourceName;
    /// # use stratosphere_core::value::*;
    /// # use serde_json::json;
    ///
    /// let logical_resource_name = LogicalResourceName::from("some-logical-resource-name");
    /// let reference : ExpString = logical_resource_name.into();
    ///
    /// assert_eq!(
    ///   json!({"Ref":"some-logical-resource-name"}),
    ///   reference.to_value()
    /// )
    /// ```
    ///
    /// [Fn::GetAtt](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-getatt.html)
    ///
    /// ```
    /// # use stratosphere_core::value::*;
    /// # use serde_json::json;
    /// assert_eq!(
    ///   json!({"Fn::GetAtt":["some-logical-resource-name", "some-attribute-name"]}),
    ///   ExpString::GetAtt{
    ///     logical_resource_name: "some-logical-resource-name".into(),
    ///     attribute_name: "some-attribute-name".into()
    ///   }.to_value()
    /// )
    /// ```
    ///
    /// String Literal
    ///
    /// ```
    /// # use stratosphere_core::value::*;
    /// # use serde_json::json;
    ///
    /// let exp : ExpString = "some-literal".into();
    ///
    /// assert_eq!(json!{"some-literal"}, exp.to_value())
    /// ```
    ///
    /// [Fn::Join](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-join.html)
    ///
    /// ```
    /// # use stratosphere_core::template::*;
    /// # use stratosphere_core::value::*;
    /// # use serde_json::json;
    /// assert_eq!(
    ///   json!({"Fn::Join":[',', [{"Ref": "some-logical-resource-name"}, "some-literal"]]}),
    ///   ExpString::Join{
    ///     delimiter: String::from(","),
    ///     values: vec![
    ///       LogicalResourceName::from("some-logical-resource-name").into(),
    ///       "some-literal".into()
    ///     ]
    ///   }.to_value()
    /// )
    /// ```
    fn to_value(&self) -> serde_json::Value {
        match self {
            ExpString::Base64(value) => mk_func("Fn::Base64", value.to_value()),
            ExpString::FindInMap {
                map_name,
                top_level_key,
                second_level_key,
            } => mk_func(
                "Fn::FindInMap",
                vec![
                    serde_json::to_value(map_name).unwrap(),
                    top_level_key.to_value(),
                    serde_json::to_value(second_level_key).unwrap(),
                ],
            ),
            ExpString::GetAtt {
                logical_resource_name,
                attribute_name,
            } => mk_func(
                "Fn::GetAtt",
                &[
                    serde_json::to_value(logical_resource_name).unwrap(),
                    serde_json::to_value(attribute_name).unwrap(),
                ],
            ),
            ExpString::If {
                condition_name,
                true_branch,
                else_branch,
            } => mk_func(
                "Fn::If",
                &[
                    serde_json::to_value(condition_name).unwrap(),
                    true_branch.to_value(),
                    else_branch.to_value(),
                ],
            ),
            ExpString::Join { delimiter, values } => mk_func(
                "Fn::Join",
                vec![
                    delimiter.to_value(),
                    serde_json::to_value(
                        values
                            .iter()
                            .map(|item| item.to_value())
                            .collect::<Vec<_>>(),
                    )
                    .unwrap(),
                ],
            ),
            ExpString::Literal(value) => serde_json::to_value(value).unwrap(),
            ExpString::Ref(value) => mk_func("Ref", value),
            ExpString::ImportValue(value) => mk_func("Fn::ImportValue", value),
            ExpString::Sub { pattern } => mk_func("Fn::Sub", pattern),
            ExpString::Select { index, values } => mk_func(
                "Fn::Select",
                vec![serde_json::to_value(index).unwrap(), values.to_value()],
            ),
            ExpString::Split { delimiter, source } => mk_func(
                "Fn::Split",
                vec![serde_json::to_value(delimiter).unwrap(), source.to_value()],
            ),
        }
    }
}

impl<A: ToValue> ToValue for Vec<A> {
    fn to_value(&self) -> serde_json::Value {
        self.iter().map(ToValue::to_value).collect()
    }
}

impl ToValue for i64 {
    fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl ToValue for f64 {
    fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl ToValue for bool {
    fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl ToValue for serde_json::Value {
    fn to_value(&self) -> serde_json::Value {
        self.clone()
    }
}

impl ToValue for std::collections::BTreeMap<String, ExpString> {
    fn to_value(&self) -> serde_json::Value {
        serde_json::Value::Object(serde_json::Map::from_iter(
            self.iter().map(|(key, exp)| (key.clone(), exp.to_value())),
        ))
    }
}

impl ToValue for chrono::DateTime<chrono::Utc> {
    /// Converts a timestamp to a JSON string value in RFC3339 format
    ///
    /// # Examples
    ///
    /// ```
    /// # use stratosphere_core::value::ToValue;
    /// # use chrono::{DateTime, Utc, TimeZone};
    /// # use serde_json::json;
    ///
    /// let timestamp = Utc.with_ymd_and_hms(2024, 10, 13, 19, 0, 0).unwrap();
    /// let value = timestamp.to_value();
    ///
    /// assert_eq!(value, json!("2024-10-13T19:00:00+00:00"));
    /// ```
    fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self.to_rfc3339()).unwrap()
    }
}

fn mk_func<T: serde::Serialize>(name: &str, value: T) -> serde_json::Value {
    json!({name:value})
}

#[derive(Clone, Debug)]
pub enum ExpPair {
    Bool {
        left: Box<ExpBool>,
        right: Box<ExpBool>,
    },
    String {
        left: Box<ExpString>,
        right: Box<ExpString>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpStringList {
    GetAZs { region: Box<ExpString> },
    Literal(Vec<ExpString>),
}

impl From<Vec<ExpString>> for ExpStringList {
    fn from(values: Vec<ExpString>) -> Self {
        ExpStringList::Literal(values)
    }
}

impl ToValue for ExpStringList {
    fn to_value(&self) -> serde_json::Value {
        match self {
            ExpStringList::GetAZs { region } => mk_func("Fn::GetAZs", region.to_value()),
            ExpStringList::Literal(values) => serde_json::to_value(
                values
                    .iter()
                    .map(|item| item.to_value())
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ExpBool {
    And(Box<ExpBool>, Box<ExpBool>),
    Equals(ExpPair),
    FindInMap {
        map_name: crate::template::MapName,
        top_level_key: Box<ExpString>,
        second_level_key: String,
    },
    If {
        condition_name: ConditionName,
        true_branch: Box<ExpBool>,
        else_branch: Box<ExpBool>,
    },
    Literal(bool),
    Not(Box<ExpBool>),
    Or(Vec<ExpBool>),
    Select {
        index: u8,
        values: Vec<ExpBool>,
    },
}

impl From<bool> for ExpBool {
    fn from(value: bool) -> Self {
        Self::Literal(value)
    }
}

impl FnIf for ExpBool {
    fn fn_if(
        condition_name: impl Into<ConditionName>,
        true_branch: impl Into<Self>,
        else_branch: impl Into<Self>,
    ) -> Self {
        Self::If {
            condition_name: condition_name.into(),
            true_branch: Box::new(true_branch.into()),
            else_branch: Box::new(else_branch.into()),
        }
    }
}

impl FnSelect for ExpBool {
    type ValueList = Vec<ExpBool>;

    fn fn_select(index: u8, values: Self::ValueList) -> Self {
        ExpBool::Select { index, values }
    }
}

impl FnFindInMap for ExpBool {
    fn fn_find_in_map(
        map_name: impl Into<crate::template::MapName>,
        top_level_key: impl Into<ExpString>,
        second_level_key: impl Into<String>,
    ) -> Self {
        ExpBool::FindInMap {
            map_name: map_name.into(),
            top_level_key: Box::new(top_level_key.into()),
            second_level_key: second_level_key.into(),
        }
    }
}

impl ToValue for ExpBool {
    /// Literal
    ///
    /// ```
    /// # use stratosphere_core::value::*;
    /// # use serde_json::json;
    /// assert_eq!(
    ///   serde_json::Value::Bool(true),
    ///   ExpBool::Literal(true).to_value()
    /// )
    /// ```
    ///
    /// [Fn::Equals](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-conditions.html#intrinsic-function-reference-conditions-equals)
    ///
    /// ```
    /// # use stratosphere_core::template::*;
    /// # use stratosphere_core::value::*;
    /// # use serde_json::json;
    ///
    /// assert_eq!(
    ///   json!({"Fn::Equals":[{"Ref":"resource-a"},"some-literal"]}),
    ///   equals_string(
    ///     LogicalResourceName::from("resource-a"),
    ///     "some-literal"
    ///   ).to_value()
    /// );
    ///
    /// assert_eq!(
    ///   json!({"Fn::Equals":[true,false]}),
    ///   equals_bool(
    ///       true,
    ///       false
    ///   ).to_value()
    /// )
    /// ```
    ///
    fn to_value(&self) -> serde_json::Value {
        match self {
            ExpBool::And(left, right) => mk_func("Fn::And", [left.to_value(), right.to_value()]),
            ExpBool::Equals(pair) => match pair {
                ExpPair::Bool { left, right } => {
                    mk_func("Fn::Equals", [left.to_value(), right.to_value()])
                }
                ExpPair::String { left, right } => {
                    mk_func("Fn::Equals", [left.to_value(), right.to_value()])
                }
            },
            ExpBool::FindInMap {
                map_name,
                top_level_key,
                second_level_key,
            } => mk_func(
                "Fn::FindInMap",
                vec![
                    serde_json::to_value(map_name).unwrap(),
                    top_level_key.to_value(),
                    serde_json::to_value(second_level_key).unwrap(),
                ],
            ),
            ExpBool::If {
                condition_name,
                true_branch,
                else_branch,
            } => mk_func(
                "Fn::If",
                [
                    serde_json::to_value(condition_name).unwrap(),
                    true_branch.to_value(),
                    else_branch.to_value(),
                ],
            ),
            ExpBool::Literal(value) => serde_json::Value::Bool(*value),
            ExpBool::Not(value) => mk_func("Fn::Not", [value.to_value()]),
            ExpBool::Or(conditions) => mk_func(
                "Fn::Or",
                conditions
                    .iter()
                    .map(|condition| condition.to_value())
                    .collect::<Vec<_>>(),
            ),
            ExpBool::Select { index, values } => mk_func(
                "Fn::Select",
                [
                    serde_json::Value::Number((*index).into()),
                    serde_json::Value::Array(
                        values.iter().map(|v| v.to_value()).collect::<Vec<_>>(),
                    ),
                ],
            ),
        }
    }
}

pub struct Pseudo(&'static str);

pub const AWS_ACCOUNT_ID: Pseudo = Pseudo("AWS::AccountId");
pub const AWS_REGION: Pseudo = Pseudo("AWS::Region");
pub const AWS_STACK_NAME: Pseudo = Pseudo("AWS::StackName");

impl From<Pseudo> for ExpString {
    fn from(value: Pseudo) -> Self {
        LogicalResourceName::from(value.0).into()
    }
}
