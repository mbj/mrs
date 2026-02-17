///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html>
pub struct Tag_ {
    pub key: crate::value::ExpString,
    pub value: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __Tag {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_property_type!("Tag" $($field $value)*)
    };
}
pub use crate::__Tag as Tag;
impl crate::value::ToValue for Tag_ {
    fn to_value(&self) -> serde_json::Value {
        let mut properties = serde_json::Map::new();
        properties.insert(
            "Key".to_string(),
            crate::value::ToValue::to_value(&self.key),
        );
        properties.insert(
            "Value".to_string(),
            crate::value::ToValue::to_value(&self.value),
        );
        properties.into()
    }
}
