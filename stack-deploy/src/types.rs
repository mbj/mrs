pub use stratosphere::template::OutputKey;

const INLINE_TEMPLATE_LIMIT_BYTES: usize = 51200;

#[derive(Clone, Debug, PartialEq)]
pub struct StackName(pub String);

impl From<&str> for StackName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TemplateName(pub String);

impl From<&str> for TemplateName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl TemplateName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub trait StackIdentifier: std::fmt::Debug + std::fmt::Display {
    fn as_str(&self) -> &str;
}

impl std::str::FromStr for StackName {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<StackName, Self::Err> {
        Ok(Self(String::from(input)))
    }
}

impl AsRef<str> for StackName {
    fn as_ref(&self) -> &str {
        &self.0
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

impl std::fmt::Display for StackName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
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

impl std::fmt::Display for StackId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize)]
pub struct ParameterValue(pub String);

pub use stratosphere::template::{ParameterKey, ParameterKeys};

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
    ///     .to_parameter_update_parameters(&existing_stack),
    /// )
    /// ```
    pub fn to_parameter_update_parameters(
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

    /// To template update parameters
    ///
    /// ### Examples
    ///
    /// One parameter unchanged `key-a`, one parameter changed `key-b`, one parameter
    /// is probably new in the template `key-c`.
    ///
    /// ```
    /// # use stack_deploy::types::*;
    ///
    /// let template_parameter_keys = ParameterKeys::from([
    ///     ParameterKey(String::from("key-a")),
    ///     ParameterKey(String::from("key-b")),
    ///     ParameterKey(String::from("key-c")),
    /// ]);
    ///
    /// let existing_stack_parameter_keys = ParameterKeys::from([ParameterKey(String::from("key-a"))]);
    ///
    /// assert_eq!(
    ///     vec![
    ///         aws_sdk_cloudformation::types::Parameter::builder()
    ///             .parameter_key("key-a")
    ///             .use_previous_value(true)
    ///             .build(),
    ///         aws_sdk_cloudformation::types::Parameter::builder()
    ///             .parameter_key("key-b")
    ///             .parameter_value("value-b")
    ///             .build(),
    ///     ],
    ///     ParameterMap(std::collections::BTreeMap::from([(
    ///         ParameterKey(String::from("key-b")),
    ///         ParameterValue(String::from("value-b"))
    ///     ),]))
    ///     .to_template_update_parameters(&template_parameter_keys, &existing_stack_parameter_keys),
    /// )
    /// ```
    pub fn to_template_update_parameters(
        &self,
        template_parameter_keys: &ParameterKeys,
        existing_stack_parameter_keys: &ParameterKeys,
    ) -> Vec<aws_sdk_cloudformation::types::Parameter> {
        let mut parameters = vec![];

        for key in self.to_parameter_keys().union(template_parameter_keys) {
            let builder = aws_sdk_cloudformation::types::Parameter::builder().parameter_key(&key.0);

            parameters.push(
                match self.0.get(key) {
                    Some(value) => builder.parameter_value(value.0.clone()),
                    None => {
                        if existing_stack_parameter_keys.contains(key) {
                            builder.use_previous_value(true)
                        } else {
                            continue;
                        }
                    }
                }
                .build(),
            )
        }

        parameters
    }

    /// To parameter keys
    ///
    /// ### Examples
    ///
    /// ```
    /// # use stack_deploy::types::*;
    ///
    /// let template = Template::Plain {
    ///     name: "example".into(),
    ///     rendered: TemplateRendered {
    ///         body: "ununsed".into(),
    ///         format: TemplateFormat::JSON,
    ///     },
    ///     parameter_keys: ParameterKeys::from([ParameterKey(String::from("key-a"))]),
    /// };
    ///
    /// assert_eq!(
    ///     ParameterKeys::from([
    ///         ParameterKey(String::from("key-a")),
    ///         ParameterKey(String::from("key-b")),
    ///     ]),
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
    ///     .to_parameter_keys()
    /// )
    /// ```
    pub fn to_parameter_keys(&self) -> ParameterKeys {
        ParameterKeys::from_iter(self.0.keys().cloned())
    }
}

#[derive(Debug, PartialEq)]
pub enum Template {
    Plain {
        name: TemplateName,
        parameter_keys: ParameterKeys,
        rendered: TemplateRendered,
    },
    Stratosphere {
        name: TemplateName,
        template: stratosphere::Template<'static>,
    },
}

impl Template {
    pub fn parameter_keys(&self) -> ParameterKeys {
        match self {
            Self::Stratosphere { template, .. } => template.parameter_keys(),
            Self::Plain { parameter_keys, .. } => parameter_keys.clone(),
        }
    }

    pub fn name(&self) -> &TemplateName {
        match self {
            Self::Stratosphere { name, .. } => name,
            Self::Plain { name, .. } => name,
        }
    }

    pub fn rendered(&self) -> TemplateRendered {
        match self {
            Self::Plain { rendered, .. } => rendered.clone(),
            Self::Stratosphere { template, .. } => TemplateRendered {
                body: template.render_json().into(),
                format: TemplateFormat::JSON,
            },
        }
    }

    pub fn rendered_pretty(&self) -> TemplateRendered {
        match self {
            Self::Plain { rendered, .. } => rendered.clone(),
            Self::Stratosphere { template, .. } => TemplateRendered {
                body: template.render_json_pretty().into(),
                format: TemplateFormat::JSON,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateBody(String);

impl AsRef<[u8]> for TemplateBody {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl From<String> for TemplateBody {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<TemplateBody> for String {
    fn from(value: TemplateBody) -> Self {
        value.0
    }
}

impl From<&TemplateBody> for String {
    fn from(value: &TemplateBody) -> Self {
        value.0.clone()
    }
}

impl From<&str> for TemplateBody {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<&TemplateBody> for aws_sdk_s3::primitives::ByteStream {
    fn from(value: &TemplateBody) -> Self {
        value.0.as_bytes().to_vec().into()
    }
}

impl TemplateBody {
    pub fn needs_upload(&self) -> bool {
        self.0.len() > INLINE_TEMPLATE_LIMIT_BYTES
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TemplateFormat {
    JSON,
    YAML,
}

impl TemplateFormat {
    pub fn file_ext(&self) -> &str {
        match self {
            Self::JSON => "json",
            Self::YAML => "yaml",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateRendered {
    pub format: TemplateFormat,
    pub body: TemplateBody,
}

#[derive(Debug)]
pub struct TemplateUrl(String);

impl From<&str> for TemplateUrl {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for TemplateUrl {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<TemplateUrl> for String {
    fn from(value: TemplateUrl) -> Self {
        value.0
    }
}

#[derive(Clone, Debug)]
pub struct ChangeSetName(String);

impl std::str::FromStr for ChangeSetName {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(input.to_string()))
    }
}

impl From<&ChangeSetName> for String {
    fn from(value: &ChangeSetName) -> String {
        value.0.to_string()
    }
}

impl From<String> for ChangeSetName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for ChangeSetName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct ChangeSetArn(pub String);

impl ChangeSetArn {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ChangeSetArn {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

impl From<ChangeSetArn> for String {
    fn from(value: ChangeSetArn) -> Self {
        (&value.0).into()
    }
}

impl From<&ChangeSetArn> for String {
    fn from(value: &ChangeSetArn) -> Self {
        value.0.to_string()
    }
}
