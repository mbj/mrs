use crate::template::LogicalResourceName;
use serde;
use serde_json;
use serde_json::json;

#[derive(Debug, serde::Serialize)]
pub struct AttributeName(String);
#[derive(Debug, serde::Serialize)]
pub struct ConditionName(String);

pub fn equals_bool<T: ToExp<Output = ExpBool>>(left: T, right: T) -> ExpBool {
    ExpBool::Equals(ExpPair::Bool {
        left: Box::new(left.to_exp()),
        right: Box::new(right.to_exp()),
    })
}

pub fn equals_string<A: ToExp<Output = ExpString>, B: ToExp<Output = ExpString>>(
    left: A,
    right: B,
) -> ExpBool {
    ExpBool::Equals(ExpPair::String {
        left: Box::new(left.to_exp()),
        right: Box::new(right.to_exp()),
    })
}

pub trait ToValue {
    fn to_value(&self) -> serde_json::Value;
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

pub trait ToConditionName {
    fn to_condition_name(&self) -> ConditionName;
}

impl ToConditionName for str {
    fn to_condition_name(&self) -> ConditionName {
        ConditionName(String::from(self))
    }
}

#[derive(Debug, serde::Serialize)]
pub struct OutputExportName(String);

impl<T: ToValue> ToValue for Box<T> {
    fn to_value(&self) -> serde_json::Value {
        self.as_ref().to_value()
    }
}

pub trait ToAttributeName {
    fn to_attribute_name(&self) -> AttributeName;
}

impl ToAttributeName for str {
    fn to_attribute_name(&self) -> AttributeName {
        AttributeName(String::from(self))
    }
}

pub trait ToLogicalResourceName {
    fn to_logical_name(&self) -> LogicalResourceName;
}

impl ToLogicalResourceName for &str {
    fn to_logical_name(&self) -> LogicalResourceName {
        LogicalResourceName(self.to_string())
    }
}

pub trait ToRef {
    type Exp;

    fn to_ref(self) -> Self::Exp;
}

impl ToRef for LogicalResourceName {
    type Exp = ExpString;

    fn to_ref(self) -> Self::Exp {
        ExpString::Ref(self)
    }
}

#[derive(Debug)]
pub enum ExpString {
    Base64(Box<ExpString>),
    GetAtt {
        logical_name: LogicalResourceName,
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
}

impl ExpString {
    pub fn base64(self) -> ExpString {
        ExpString::Base64(Box::new(self))
    }
}

pub trait ToExp {
    type Output;

    fn to_exp(self) -> Self::Output;
}

impl ToExp for &str {
    type Output = ExpString;

    fn to_exp(self) -> Self::Output {
        ExpString::Literal(String::from(self))
    }
}

impl ToExp for ExpString {
    type Output = ExpString;

    fn to_exp(self) -> Self::Output {
        self
    }
}

impl ToExp for bool {
    type Output = ExpBool;

    fn to_exp(self) -> Self::Output {
        ExpBool::Literal(self)
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
    /// # use stratosphere::value::*;
    /// # use serde_json::json;
    /// assert_eq!(
    ///   json!({"Fn::Base64":"some-literal"}),
    ///   "some-literal".to_exp().base64().to_value()
    /// )
    /// ```
    ///
    /// [Fn::If](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-conditions.html#intrinsic-function-reference-conditions-if)
    ///
    /// ```
    /// # use stratosphere::value::*;
    /// # use serde_json::json;
    /// assert_eq!(
    ///   json!({"Fn::If":["condition-name",{"Ref":"resource-a"},{"Ref":"resource-b"}]}),
    ///   ExpString::If{
    ///     condition_name: "condition-name".to_condition_name(),
    ///     true_branch: Box::new("resource-a".to_logical_name().to_ref()),
    ///     else_branch: Box::new("resource-b".to_logical_name().to_ref()),
    ///   }.to_value()
    /// )
    /// ```
    ///
    /// [Ref](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-ref.html)
    ///
    /// ```
    /// # use stratosphere::value::*;
    /// # use serde_json::json;
    /// assert_eq!(
    ///   json!({"Ref":"some-logical-name"}),
    ///   "some-logical-name".to_logical_name().to_ref().to_value()
    /// )
    /// ```
    ///
    /// [Fn::GetAtt](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-getatt.html)
    ///
    /// ```
    /// # use stratosphere::value::*;
    /// # use serde_json::json;
    /// assert_eq!(
    ///   json!({"Fn::GetAtt":["some-logical-name", "some-attribute-name"]}),
    ///   ExpString::GetAtt{
    ///     logical_name: "some-logical-name".to_logical_name(),
    ///     attribute_name: "some-attribute-name".to_attribute_name()
    ///   }.to_value()
    /// )
    /// ```
    ///
    /// String Literal
    ///
    /// ```
    /// # use stratosphere::value::*;
    /// # use serde_json::json;
    /// assert_eq!(
    ///   json!{"some-literal"},
    ///   "some-literal".to_exp().to_value()
    /// )
    /// ```
    ///
    /// [Fn::Join](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-join.html)
    ///
    /// ```
    /// # use stratosphere::value::*;
    /// # use serde_json::json;
    /// assert_eq!(
    ///   json!({"Fn::Join":[',', [{"Ref": "some-logical-name"}, "some-literal"]]}),
    ///   ExpString::Join{
    ///     delimiter: String::from(","),
    ///     values: vec![
    ///       "some-logical-name".to_logical_name().to_ref(),
    ///       "some-literal".to_exp()
    ///     ]
    ///   }.to_value()
    /// )
    /// ```
    fn to_value(&self) -> serde_json::Value {
        match self {
            ExpString::Base64(value) => mk_func("Fn::Base64", value.to_value()),
            ExpString::GetAtt {
                logical_name,
                attribute_name,
            } => mk_func(
                "Fn::GetAtt",
                &[
                    serde_json::to_value(logical_name).unwrap(),
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
            _ => todo!(),
        }
    }
}

impl<A: ToValue> ToValue for Vec<A> {
    fn to_value(&self) -> serde_json::Value {
        self.iter().map(ToValue::to_value).collect()
    }
}

impl ToValue for () {
    fn to_value(&self) -> serde_json::Value {
        todo!()
    }
}

impl ToValue for i64 {
    fn to_value(&self) -> serde_json::Value {
        todo!()
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

fn mk_func<T: serde::Serialize>(name: &str, value: T) -> serde_json::Value {
    json!({name:value})
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ExpBool {
    And(Box<ExpBool>, Box<ExpBool>),
    Equals(ExpPair),
    Literal(bool),
    Not(Box<ExpBool>),
    Or(Vec<ExpBool>),
}

impl ToValue for ExpBool {
    /// Literal
    ///
    /// ```
    /// # use stratosphere::value::*;
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
    /// # use stratosphere::value::*;
    /// # use serde_json::json;
    ///
    /// assert_eq!(
    ///   json!({"Fn::Equals":[{"Ref":"resource-a"},"some-literal"]}),
    ///   equals_string(
    ///     "resource-a".to_logical_name().to_ref(),
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
