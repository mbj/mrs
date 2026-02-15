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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize)]
pub struct ConditionName(String);

impl std::fmt::Display for ConditionName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

impl From<&str> for ConditionName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<&ConditionName> for ConditionName {
    fn from(value: &ConditionName) -> Self {
        value.clone()
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
#[must_use]
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

/// Helper function to create a Fn::Cidr expression
///
/// Returns an ExpStringList representing a list of CIDR address blocks.
/// Used to generate subnet CIDR blocks from a larger VPC CIDR block.
///
/// # Arguments
///
/// * `ip_block` - The CIDR block to divide (e.g., "10.0.0.0/16")
/// * `count` - The number of CIDRs to generate (1-256)
/// * `cidr_bits` - The number of subnet bits for the CIDR (typically 8 for /24 subnets)
///
/// # Examples
///
/// ```
/// # use stratosphere_core::value::*;
/// // Generate 6 /24 subnets from a /16 VPC
/// let subnets = fn_cidr("10.0.0.0/16", 6, 8);
///
/// // Use with Fn::Select to pick specific subnet
/// let first_subnet = fn_select_string(0, fn_cidr("10.0.0.0/16", 6, 8));
/// ```
pub fn fn_cidr(ip_block: impl Into<ExpString>, count: u8, cidr_bits: u8) -> ExpStringList {
    ExpStringList::Cidr {
        ip_block: Box::new(ip_block.into()),
        count,
        cidr_bits,
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
    // Pseudo parameters that return strings
    AwsAccountId,
    AwsPartition,
    AwsRegion,
    AwsStackId,
    AwsStackName,
    AwsUrlSuffix,
}

impl ExpString {
    #[must_use]
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
    join("-", [AWS_STACK_NAME, suffix.into()])
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
            ExpString::Ref(value) => mk_ref(value),
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
            // Pseudo parameters serialize as Refs
            ExpString::AwsAccountId => mk_ref("AWS::AccountId"),
            ExpString::AwsPartition => mk_ref("AWS::Partition"),
            ExpString::AwsRegion => mk_ref("AWS::Region"),
            ExpString::AwsStackId => mk_ref("AWS::StackId"),
            ExpString::AwsStackName => mk_ref("AWS::StackName"),
            ExpString::AwsUrlSuffix => mk_ref("AWS::URLSuffix"),
        }
    }
}

impl<A: ToValue> ToValue for Vec<A> {
    fn to_value(&self) -> serde_json::Value {
        self.iter().map(ToValue::to_value).collect()
    }
}

impl ToValue for i32 {
    fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
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

impl<T: ToValue> ToValue for std::collections::BTreeMap<String, T> {
    fn to_value(&self) -> serde_json::Value {
        serde_json::Value::Object(serde_json::Map::from_iter(
            self.iter()
                .map(|(key, value)| (key.clone(), value.to_value())),
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

fn mk_ref<T: serde::Serialize>(value: T) -> serde_json::Value {
    mk_func("Ref", value)
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
    Cidr {
        ip_block: Box<ExpString>,
        count: u8,
        cidr_bits: u8,
    },
    GetAZs {
        region: Box<ExpString>,
    },
    Literal(Vec<ExpString>),
    // Pseudo parameter that returns a list of strings
    AwsNotificationArns,
}

impl From<Vec<ExpString>> for ExpStringList {
    fn from(values: Vec<ExpString>) -> Self {
        ExpStringList::Literal(values)
    }
}

impl ToValue for ExpStringList {
    fn to_value(&self) -> serde_json::Value {
        match self {
            ExpStringList::Cidr {
                ip_block,
                count,
                cidr_bits,
            } => mk_func(
                "Fn::Cidr",
                vec![
                    ip_block.to_value(),
                    serde_json::to_value(count).unwrap(),
                    serde_json::to_value(cidr_bits).unwrap(),
                ],
            ),
            ExpStringList::GetAZs { region } => mk_func("Fn::GetAZs", region.to_value()),
            ExpStringList::Literal(values) => serde_json::to_value(
                values
                    .iter()
                    .map(|item| item.to_value())
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
            // Pseudo parameter that returns a list serializes as a Ref
            ExpStringList::AwsNotificationArns => mk_ref("AWS::NotificationARNs"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpBool {
    And(Box<ExpBool>, Box<ExpBool>),
    Condition(ConditionName),
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

impl From<ConditionName> for ExpBool {
    fn from(value: ConditionName) -> Self {
        Self::Condition(value)
    }
}

impl From<&ConditionName> for ExpBool {
    fn from(value: &ConditionName) -> Self {
        Self::Condition(value.clone())
    }
}

impl serde::Serialize for ExpBool {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_value().serialize(serializer)
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
            ExpBool::Condition(name) => mk_func("Condition", name),
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

// Pseudo parameter constants

/// AWS account ID (e.g., "123456789012")
///
/// This pseudo parameter resolves to the AWS account ID of the account
/// in which the CloudFormation stack is being created.
///
/// # Examples
///
/// ```
/// # use stratosphere_core::value::*;
/// # use serde_json::json;
/// assert_eq!(AWS_ACCOUNT_ID.to_value(), json!({"Ref": "AWS::AccountId"}));
/// ```
pub const AWS_ACCOUNT_ID: ExpString = ExpString::AwsAccountId;

/// AWS partition (e.g., "aws", "aws-cn", "aws-us-gov")
///
/// This pseudo parameter resolves to the partition that the resource is in.
/// For standard AWS Regions, the partition is "aws". For resources in other
/// partitions, the partition is "aws-partitionname".
///
/// # Examples
///
/// ```
/// # use stratosphere_core::value::*;
/// # use serde_json::json;
/// assert_eq!(AWS_PARTITION.to_value(), json!({"Ref": "AWS::Partition"}));
/// ```
pub const AWS_PARTITION: ExpString = ExpString::AwsPartition;

/// AWS region (e.g., "us-east-1")
///
/// This pseudo parameter resolves to the AWS Region in which the
/// CloudFormation stack is being created.
///
/// # Examples
///
/// ```
/// # use stratosphere_core::value::*;
/// # use serde_json::json;
/// assert_eq!(AWS_REGION.to_value(), json!({"Ref": "AWS::Region"}));
/// ```
pub const AWS_REGION: ExpString = ExpString::AwsRegion;

/// Stack ID/ARN
///
/// This pseudo parameter resolves to the ID (ARN) of the stack.
///
/// # Examples
///
/// ```
/// # use stratosphere_core::value::*;
/// # use serde_json::json;
/// assert_eq!(AWS_STACK_ID.to_value(), json!({"Ref": "AWS::StackId"}));
/// ```
pub const AWS_STACK_ID: ExpString = ExpString::AwsStackId;

/// Stack name
///
/// This pseudo parameter resolves to the name of the CloudFormation stack.
///
/// # Examples
///
/// ```
/// # use stratosphere_core::value::*;
/// # use serde_json::json;
/// assert_eq!(AWS_STACK_NAME.to_value(), json!({"Ref": "AWS::StackName"}));
/// ```
pub const AWS_STACK_NAME: ExpString = ExpString::AwsStackName;

/// AWS domain suffix (e.g., "amazonaws.com")
///
/// This pseudo parameter resolves to the suffix for a domain. The suffix is
/// typically "amazonaws.com", but might differ by Region. For example, in
/// China (Beijing), the suffix is "amazonaws.com.cn".
///
/// # Examples
///
/// ```
/// # use stratosphere_core::value::*;
/// # use serde_json::json;
/// assert_eq!(AWS_URL_SUFFIX.to_value(), json!({"Ref": "AWS::URLSuffix"}));
/// ```
pub const AWS_URL_SUFFIX: ExpString = ExpString::AwsUrlSuffix;

/// List of SNS topic ARNs for stack notifications
///
/// This pseudo parameter resolves to the list of notification Amazon SNS
/// topic ARNs for the current stack.
///
/// # Examples
///
/// ```
/// # use stratosphere_core::value::*;
/// # use serde_json::json;
/// assert_eq!(AWS_NOTIFICATION_ARNS.to_value(), json!({"Ref": "AWS::NotificationARNs"}));
/// ```
pub const AWS_NOTIFICATION_ARNS: ExpStringList = ExpStringList::AwsNotificationArns;
