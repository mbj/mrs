#[derive(Clone, Debug, PartialEq)]
pub struct StackName(pub String);
pub struct OutputKey(pub String);

pub trait StackIdentifier: std::fmt::Debug {
    fn as_str(&self) -> &str;
}

impl std::str::FromStr for StackName {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<StackName, Self::Err> {
        Ok(Self(String::from(input)))
    }
}

impl From<&StackName> for String {
    fn from(value: &StackName) -> Self {
        value.0.clone()
    }
}

impl StackIdentifier for StackName {
    fn as_str(&self) -> &str {
        &self.0
    }
}

impl StackIdentifier for &StackName {
    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct StackId(pub String);

impl From<&StackId> for String {
    fn from(value: &StackId) -> Self {
        value.0.clone()
    }
}

impl StackIdentifier for &StackId {
    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct ClientRequestToken(pub String);

impl ClientRequestToken {
    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl From<&ClientRequestToken> for String {
    fn from(value: &ClientRequestToken) -> Self {
        value.0.clone()
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ParameterKey(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterValue(pub String);

pub type ParameterKeys = std::collections::BTreeSet<ParameterKey>;

#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    pub key: ParameterKey,
    pub value: ParameterValue,
}

impl std::str::FromStr for Parameter {
    type Err = &'static str;

    /// Parse parameter from string
    ///
    /// ### Examples
    ///
    /// Success, both present
    ///
    /// ```
    /// # use stack_deploy::types::*;
    /// assert_eq!(
    ///     Ok(Parameter {
    ///         key: ParameterKey(String::from("some-key")),
    ///         value: ParameterValue(String::from("some-value")),
    ///     }),
    ///     std::str::FromStr::from_str("some-key:some-value")
    /// )
    /// ```
    ///
    /// Success, empty value
    ///
    /// ```
    /// # use stack_deploy::types::*;
    /// assert_eq!(
    ///     Ok(Parameter {
    ///         key: ParameterKey(String::from("some-key")),
    ///         value: ParameterValue(String::from("")),
    ///     }),
    ///     std::str::FromStr::from_str("some-key:")
    /// )
    /// ```
    ///
    /// Missing delimiter
    ///
    /// ```
    /// # use stack_deploy::types::*;
    /// assert_eq!(
    ///     Err("missing ':' in input"),
    ///     <Parameter as std::str::FromStr>::from_str("some-key")
    /// )
    /// ```
    ///
    /// Empty key
    ///
    /// ```
    /// # use stack_deploy::types::*;
    /// assert_eq!(
    ///     Err("parameter key cannot be empty"),
    ///     <Parameter as std::str::FromStr>::from_str(":value")
    /// )
    /// ```
    ///
    /// Overlong key
    ///
    /// ```
    /// # use stack_deploy::types::*;
    /// assert_eq!(
    ///     Err("parameter key cannot be longer than 255 bytes"),
    ///     <Parameter as std::str::FromStr>::from_str(&("a".repeat(256) + ":"))
    /// )
    /// ```
    ///
    /// Overlong value
    ///
    /// ```
    /// # use stack_deploy::types::*;
    /// assert_eq!(
    ///     Err("parameter value cannot be longer than 4096 bytes"),
    ///     <Parameter as std::str::FromStr>::from_str(&("a:".to_owned() + &"b".repeat(4097)))
    /// )
    /// ```
    fn from_str(input: &str) -> Result<Parameter, Self::Err> {
        match input.split_once(':') {
            None => Err("missing ':' in input"),
            Some((key, value)) => match (key.len(), value.len()) {
                (0, _) => Err("parameter key cannot be empty"),
                (key_len, _) if key_len > 255 => {
                    Err("parameter key cannot be longer than 255 bytes")
                }
                (_, value_len) if value_len > 4096 => {
                    Err("parameter value cannot be longer than 4096 bytes")
                }
                _other => Ok(Parameter {
                    key: ParameterKey(String::from(key)),
                    value: ParameterValue(String::from(value)),
                }),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParameterMap(pub std::collections::BTreeMap<ParameterKey, ParameterValue>);

impl Default for ParameterMap {
    fn default() -> Self {
        Self::new()
    }
}

impl ParameterMap {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    /// Parse a parameter vector into a parameter map
    ///
    /// ### Examples
    ///
    /// No duplicates
    ///
    /// ```
    /// # use stack_deploy::types::*;
    ///
    /// let parameters = vec![
    ///     Parameter {
    ///         key: ParameterKey("key-a".to_string()),
    ///         value: ParameterValue("value-a".to_string()),
    ///     },
    ///     Parameter {
    ///         key: ParameterKey("key-b".to_string()),
    ///         value: ParameterValue("value-b".to_string()),
    ///     },
    /// ];
    ///
    /// assert_eq!(
    ///     Ok(ParameterMap(std::collections::BTreeMap::from([
    ///         (
    ///             ParameterKey(String::from("key-a")),
    ///             ParameterValue(String::from("value-a"))
    ///         ),
    ///         (
    ///             ParameterKey(String::from("key-b")),
    ///             ParameterValue(String::from("value-b"))
    ///         ),
    ///     ]))),
    ///     ParameterMap::parse(&parameters)
    /// );
    /// ```
    ///
    /// Present duplicate, errors on first duplicate
    ///
    /// ```
    /// # use stack_deploy::types::*;
    ///
    /// let parameters = vec![
    ///     Parameter {
    ///         key: ParameterKey("key-a".to_string()),
    ///         value: ParameterValue("value-a".to_string()),
    ///     },
    ///     Parameter {
    ///         key: ParameterKey("key-a".to_string()),
    ///         value: ParameterValue("value-a".to_string()),
    ///     },
    ///     Parameter {
    ///         key: ParameterKey("key-b".to_string()),
    ///         value: ParameterValue("value-b".to_string()),
    ///     },
    ///     Parameter {
    ///         key: ParameterKey("key-b".to_string()),
    ///         value: ParameterValue("value-b".to_string()),
    ///     },
    /// ];
    ///
    /// assert_eq!(
    ///     Err(String::from(
    ///         "Parameter key: key-a is present multiple times"
    ///     )),
    ///     ParameterMap::parse(&parameters)
    /// );
    /// ```
    pub fn parse<'a>(
        values: impl IntoIterator<Item = &'a Parameter>,
    ) -> Result<ParameterMap, String> {
        let mut map = std::collections::BTreeMap::new();

        for parameter in values {
            if map.contains_key(&parameter.key) {
                return Err(format!(
                    "Parameter key: {} is present multiple times",
                    parameter.key.0
                ));
            } else {
                map.insert(parameter.key.clone(), parameter.value.clone());
            }
        }

        Ok(ParameterMap(map))
    }

    /// Merge parameter map into receiver
    ///
    /// ### Examples
    ///
    /// ```
    /// # use stack_deploy::types::*;
    /// let original = ParameterMap(std::collections::BTreeMap::from([
    ///     (
    ///         ParameterKey(String::from("key-a")),
    ///         ParameterValue(String::from("value-a")),
    ///     ),
    ///     (
    ///         ParameterKey(String::from("key-b")),
    ///         ParameterValue(String::from("value-b1")),
    ///     ),
    /// ]));
    ///
    /// let extra = ParameterMap(std::collections::BTreeMap::from([
    ///     (
    ///         ParameterKey(String::from("key-b")),
    ///         ParameterValue(String::from("value-b2")),
    ///     ),
    ///     (
    ///         ParameterKey(String::from("key-c")),
    ///         ParameterValue(String::from("value-c")),
    ///     ),
    /// ]));
    ///
    /// assert_eq!(
    ///     ParameterMap(std::collections::BTreeMap::from([
    ///         (
    ///             ParameterKey(String::from("key-a")),
    ///             ParameterValue(String::from("value-a"))
    ///         ),
    ///         (
    ///             ParameterKey(String::from("key-b")),
    ///             ParameterValue(String::from("value-b2"))
    ///         ),
    ///         (
    ///             ParameterKey(String::from("key-c")),
    ///             ParameterValue(String::from("value-c"))
    ///         ),
    ///     ])),
    ///     original.merge(&extra)
    /// )
    /// ```
    pub fn merge(&self, other: &Self) -> Self {
        let mut self_new = self.0.clone();

        self_new.append(&mut other.0.clone());

        ParameterMap(self_new)
    }

    /// To create parameters
    ///
    /// ### Examples
    ///
    /// ```
    /// # use stack_deploy::types::*;
    /// assert_eq!(
    ///     vec![
    ///         aws_sdk_cloudformation::types::Parameter::builder()
    ///             .parameter_key("key-a")
    ///             .parameter_value("value-a")
    ///             .build(),
    ///         aws_sdk_cloudformation::types::Parameter::builder()
    ///             .parameter_key("key-b")
    ///             .parameter_value("value-b")
    ///             .build(),
    ///     ],
    ///     ParameterMap(std::collections::BTreeMap::from([
    ///         (
    ///             ParameterKey(String::from("key-a")),
    ///             ParameterValue(String::from("value-a"))
    ///         ),
    ///         (
    ///             ParameterKey(String::from("key-b")),
    ///             ParameterValue(String::from("value-b"))
    ///         ),
    ///     ]))
    ///     .to_create_parameters(),
    /// )
    /// ```
    pub fn to_create_parameters(&self) -> Vec<aws_sdk_cloudformation::types::Parameter> {
        self.0
            .iter()
            .map(|(key, value)| {
                aws_sdk_cloudformation::types::Parameter::builder()
                    .parameter_key(&key.0)
                    .parameter_value(&value.0)
                    .build()
            })
            .collect()
    }

    /// To parameter update parameters
    ///
    /// ### Examples
    ///
    /// Some updates
    ///
    /// ```
    /// # use stack_deploy::types::*;
    ///
    /// let existing_stack = aws_sdk_cloudformation::types::Stack::builder()
    ///     .set_parameters(Some(vec![
    ///         aws_sdk_cloudformation::types::Parameter::builder()
    ///             .parameter_key("key-a")
    ///             .parameter_value("value-a1")
    ///             .build(),
    ///         aws_sdk_cloudformation::types::Parameter::builder()
    ///             .parameter_key("key-b")
    ///             .parameter_value("value-b1")
    ///             .build(),
    ///     ]))
    ///     .build();
    ///
    /// assert_eq!(
    ///     vec![
    ///         aws_sdk_cloudformation::types::Parameter::builder()
    ///             .parameter_key("key-a")
    ///             .use_previous_value(true)
    ///             .build(),
    ///         aws_sdk_cloudformation::types::Parameter::builder()
    ///             .parameter_key("key-b")
    ///             .parameter_value("value-b2")
    ///             .build(),
    ///     ],
    ///     ParameterMap(std::collections::BTreeMap::from([(
    ///         ParameterKey(String::from("key-b")),
    ///         ParameterValue(String::from("value-b2"))
    ///     ),]))
    ///     .to_update_parameters(&existing_stack),
    /// )
    /// ```
    pub fn to_update_parameters(
        &self,
        existing_stack: &aws_sdk_cloudformation::types::Stack,
    ) -> Vec<aws_sdk_cloudformation::types::Parameter> {
        match existing_stack.parameters.as_ref() {
            None => vec![],
            Some(parameters) => parameters
                .iter()
                .map(|parameter| {
                    let existing_parameter_key: &str = parameter.parameter_key().unwrap();

                    let builder = aws_sdk_cloudformation::types::Parameter::builder()
                        .parameter_key(existing_parameter_key);

                    match self
                        .0
                        .get(&ParameterKey(String::from(existing_parameter_key)))
                    {
                        Some(value) => builder.parameter_value(value.0.clone()),
                        None => builder.use_previous_value(true),
                    }
                    .build()
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
pub enum Template {
    Plain {
        body: String,
        parameter_keys: ParameterKeys,
    },
}

impl Template {
    pub fn parameter_keys(&self) -> &ParameterKeys {
        match self {
            Self::Plain { parameter_keys, .. } => parameter_keys,
        }
    }
}
