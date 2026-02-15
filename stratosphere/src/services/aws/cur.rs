///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cur-reportdefinition.html
pub struct ReportDefinition_ {
    pub additional_artifacts: Option<Vec<crate::value::ExpString>>,
    pub additional_schema_elements: Option<Vec<crate::value::ExpString>>,
    pub billing_view_arn: Option<crate::value::ExpString>,
    pub compression: crate::value::ExpString,
    pub format: crate::value::ExpString,
    pub refresh_closed_reports: crate::value::ExpBool,
    pub report_name: crate::value::ExpString,
    pub report_versioning: crate::value::ExpString,
    pub s3_bucket: crate::value::ExpString,
    pub s3_prefix: crate::value::ExpString,
    pub s3_region: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub time_unit: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cur_ReportDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CUR::ReportDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_cur_ReportDefinition as ReportDefinition;
impl crate::template::ToResource for ReportDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CUR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReportDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_artifacts {
            properties.insert(
                "AdditionalArtifacts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.additional_schema_elements {
            properties.insert(
                "AdditionalSchemaElements".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.billing_view_arn {
            properties.insert(
                "BillingViewArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Compression".to_string(),
            crate::value::ToValue::to_value(&self.compression),
        );
        properties.insert(
            "Format".to_string(),
            crate::value::ToValue::to_value(&self.format),
        );
        properties.insert(
            "RefreshClosedReports".to_string(),
            crate::value::ToValue::to_value(&self.refresh_closed_reports),
        );
        properties.insert(
            "ReportName".to_string(),
            crate::value::ToValue::to_value(&self.report_name),
        );
        properties.insert(
            "ReportVersioning".to_string(),
            crate::value::ToValue::to_value(&self.report_versioning),
        );
        properties.insert(
            "S3Bucket".to_string(),
            crate::value::ToValue::to_value(&self.s3_bucket),
        );
        properties.insert(
            "S3Prefix".to_string(),
            crate::value::ToValue::to_value(&self.s3_prefix),
        );
        properties.insert(
            "S3Region".to_string(),
            crate::value::ToValue::to_value(&self.s3_region),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TimeUnit".to_string(),
            crate::value::ToValue::to_value(&self.time_unit),
        );
        properties
    }
}
