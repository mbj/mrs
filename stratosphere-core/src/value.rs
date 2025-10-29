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
        values: Vec<ExpString>,
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
                vec![
                    serde_json::to_value(index).unwrap(),
                    serde_json::to_value(
                        values
                            .iter()
                            .map(|item| item.to_value())
                            .collect::<Vec<_>>(),
                    )
                    .unwrap(),
                ],
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
        todo!()
    }
}

impl ToValue for bool {
    fn to_value(&self) -> serde_json::Value {
        todo!()
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

#[derive(Clone, Debug)]
pub enum ExpBool {
    And(Box<ExpBool>, Box<ExpBool>),
    Equals(ExpPair),
    Literal(bool),
    Not(Box<ExpBool>),
    Or(Vec<ExpBool>),
}

impl From<bool> for ExpBool {
    fn from(value: bool) -> Self {
        Self::Literal(value)
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
            ExpBool::Equals(pair) => match pair {
                ExpPair::Bool { left, right } => {
                    mk_func("Fn::Equals", [left.to_value(), right.to_value()])
                }
                ExpPair::String { left, right } => {
                    mk_func("Fn::Equals", [left.to_value(), right.to_value()])
                }
            },
            ExpBool::Literal(value) => serde_json::Value::Bool(*value),
            other => todo!("{other:#?}"),
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
