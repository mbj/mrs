pub mod automationrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesaction.html
    pub struct AutomationRulesAction_ {
        pub finding_fields_update: Box<AutomationRulesFindingFieldsUpdate_>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRule_AutomationRulesAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRule.AutomationRulesAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRule_AutomationRulesAction as AutomationRulesAction;
    impl crate::value::ToValue for AutomationRulesAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FindingFieldsUpdate".to_string(),
                crate::value::ToValue::to_value(&self.finding_fields_update),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfieldsupdate.html
    pub struct AutomationRulesFindingFieldsUpdate_ {
        pub confidence: Option<i32>,
        pub criticality: Option<i32>,
        pub note: Option<Box<NoteUpdate_>>,
        pub related_findings: Option<Vec<RelatedFinding_>>,
        pub severity: Option<Box<SeverityUpdate_>>,
        pub types: Option<Vec<crate::value::ExpString>>,
        pub user_defined_fields:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub verification_state: Option<crate::value::ExpString>,
        pub workflow: Option<Box<WorkflowUpdate_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRule_AutomationRulesFindingFieldsUpdate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRule.AutomationRulesFindingFieldsUpdate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRule_AutomationRulesFindingFieldsUpdate as AutomationRulesFindingFieldsUpdate;
    impl crate::value::ToValue for AutomationRulesFindingFieldsUpdate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.confidence {
                properties.insert(
                    "Confidence".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.criticality {
                properties.insert(
                    "Criticality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.note {
                properties.insert("Note".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.related_findings {
                properties.insert(
                    "RelatedFindings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.severity {
                properties.insert(
                    "Severity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.types {
                properties.insert("Types".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.user_defined_fields {
                properties.insert(
                    "UserDefinedFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.verification_state {
                properties.insert(
                    "VerificationState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.workflow {
                properties.insert(
                    "Workflow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html
    pub struct AutomationRulesFindingFilters_ {
        pub aws_account_id: Option<Vec<StringFilter_>>,
        pub company_name: Option<Vec<StringFilter_>>,
        pub compliance_associated_standards_id: Option<Vec<StringFilter_>>,
        pub compliance_security_control_id: Option<Vec<StringFilter_>>,
        pub compliance_status: Option<Vec<StringFilter_>>,
        pub confidence: Option<Vec<NumberFilter_>>,
        pub created_at: Option<Vec<DateFilter_>>,
        pub criticality: Option<Vec<NumberFilter_>>,
        pub description: Option<Vec<StringFilter_>>,
        pub first_observed_at: Option<Vec<DateFilter_>>,
        pub generator_id: Option<Vec<StringFilter_>>,
        pub id: Option<Vec<StringFilter_>>,
        pub last_observed_at: Option<Vec<DateFilter_>>,
        pub note_text: Option<Vec<StringFilter_>>,
        pub note_updated_at: Option<Vec<DateFilter_>>,
        pub note_updated_by: Option<Vec<StringFilter_>>,
        pub product_arn: Option<Vec<StringFilter_>>,
        pub product_name: Option<Vec<StringFilter_>>,
        pub record_state: Option<Vec<StringFilter_>>,
        pub related_findings_id: Option<Vec<StringFilter_>>,
        pub related_findings_product_arn: Option<Vec<StringFilter_>>,
        pub resource_details_other: Option<Vec<MapFilter_>>,
        pub resource_id: Option<Vec<StringFilter_>>,
        pub resource_partition: Option<Vec<StringFilter_>>,
        pub resource_region: Option<Vec<StringFilter_>>,
        pub resource_tags: Option<Vec<MapFilter_>>,
        pub resource_type: Option<Vec<StringFilter_>>,
        pub severity_label: Option<Vec<StringFilter_>>,
        pub source_url: Option<Vec<StringFilter_>>,
        pub title: Option<Vec<StringFilter_>>,
        pub r#type: Option<Vec<StringFilter_>>,
        pub updated_at: Option<Vec<DateFilter_>>,
        pub user_defined_fields: Option<Vec<MapFilter_>>,
        pub verification_state: Option<Vec<StringFilter_>>,
        pub workflow_status: Option<Vec<StringFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRule_AutomationRulesFindingFilters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRule.AutomationRulesFindingFilters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRule_AutomationRulesFindingFilters as AutomationRulesFindingFilters;
    impl crate::value::ToValue for AutomationRulesFindingFilters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_account_id {
                properties.insert(
                    "AwsAccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.company_name {
                properties.insert(
                    "CompanyName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compliance_associated_standards_id {
                properties.insert(
                    "ComplianceAssociatedStandardsId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compliance_security_control_id {
                properties.insert(
                    "ComplianceSecurityControlId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compliance_status {
                properties.insert(
                    "ComplianceStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.confidence {
                properties.insert(
                    "Confidence".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.created_at {
                properties.insert(
                    "CreatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.criticality {
                properties.insert(
                    "Criticality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.first_observed_at {
                properties.insert(
                    "FirstObservedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.generator_id {
                properties.insert(
                    "GeneratorId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.last_observed_at {
                properties.insert(
                    "LastObservedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.note_text {
                properties.insert(
                    "NoteText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.note_updated_at {
                properties.insert(
                    "NoteUpdatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.note_updated_by {
                properties.insert(
                    "NoteUpdatedBy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.product_arn {
                properties.insert(
                    "ProductArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.product_name {
                properties.insert(
                    "ProductName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.record_state {
                properties.insert(
                    "RecordState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.related_findings_id {
                properties.insert(
                    "RelatedFindingsId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.related_findings_product_arn {
                properties.insert(
                    "RelatedFindingsProductArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_details_other {
                properties.insert(
                    "ResourceDetailsOther".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_id {
                properties.insert(
                    "ResourceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_partition {
                properties.insert(
                    "ResourcePartition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_region {
                properties.insert(
                    "ResourceRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_tags {
                properties.insert(
                    "ResourceTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_type {
                properties.insert(
                    "ResourceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.severity_label {
                properties.insert(
                    "SeverityLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_url {
                properties.insert(
                    "SourceUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.title {
                properties.insert("Title".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.updated_at {
                properties.insert(
                    "UpdatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_defined_fields {
                properties.insert(
                    "UserDefinedFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.verification_state {
                properties.insert(
                    "VerificationState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.workflow_status {
                properties.insert(
                    "WorkflowStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-datefilter.html
    pub struct DateFilter_ {
        pub date_range: Option<Box<DateRange_>>,
        pub end: Option<crate::value::ExpString>,
        pub start: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRule_DateFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRule.DateFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRule_DateFilter as DateFilter;
    impl crate::value::ToValue for DateFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.date_range {
                properties.insert(
                    "DateRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.end {
                properties.insert("End".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.start {
                properties.insert("Start".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-daterange.html
    pub struct DateRange_ {
        pub unit: crate::value::ExpString,
        pub value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRule_DateRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRule.DateRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRule_DateRange as DateRange;
    impl crate::value::ToValue for DateRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-mapfilter.html
    pub struct MapFilter_ {
        pub comparison: crate::value::ExpString,
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRule_MapFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRule.MapFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRule_MapFilter as MapFilter;
    impl crate::value::ToValue for MapFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Comparison".to_string(),
                crate::value::ToValue::to_value(&self.comparison),
            );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-noteupdate.html
    pub struct NoteUpdate_ {
        pub text: crate::value::ExpString,
        pub updated_by: serde_json::Value,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRule_NoteUpdate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRule.NoteUpdate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRule_NoteUpdate as NoteUpdate;
    impl crate::value::ToValue for NoteUpdate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.insert(
                "UpdatedBy".to_string(),
                crate::value::ToValue::to_value(&self.updated_by),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-numberfilter.html
    pub struct NumberFilter_ {
        pub eq: Option<f64>,
        pub gte: Option<f64>,
        pub lte: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRule_NumberFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRule.NumberFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRule_NumberFilter as NumberFilter;
    impl crate::value::ToValue for NumberFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.eq {
                properties.insert("Eq".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.gte {
                properties.insert("Gte".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.lte {
                properties.insert("Lte".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-relatedfinding.html
    pub struct RelatedFinding_ {
        pub id: serde_json::Value,
        pub product_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRule_RelatedFinding {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRule.RelatedFinding"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRule_RelatedFinding as RelatedFinding;
    impl crate::value::ToValue for RelatedFinding_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "ProductArn".to_string(),
                crate::value::ToValue::to_value(&self.product_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-severityupdate.html
    pub struct SeverityUpdate_ {
        pub label: Option<crate::value::ExpString>,
        pub normalized: Option<i32>,
        pub product: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRule_SeverityUpdate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRule.SeverityUpdate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRule_SeverityUpdate as SeverityUpdate;
    impl crate::value::ToValue for SeverityUpdate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.label {
                properties.insert("Label".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.normalized {
                properties.insert(
                    "Normalized".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.product {
                properties.insert(
                    "Product".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-stringfilter.html
    pub struct StringFilter_ {
        pub comparison: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRule_StringFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRule.StringFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRule_StringFilter as StringFilter;
    impl crate::value::ToValue for StringFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Comparison".to_string(),
                crate::value::ToValue::to_value(&self.comparison),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-workflowupdate.html
    pub struct WorkflowUpdate_ {
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRule_WorkflowUpdate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRule.WorkflowUpdate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRule_WorkflowUpdate as WorkflowUpdate;
    impl crate::value::ToValue for WorkflowUpdate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
}
pub mod automationrulev2 {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-automationrulesactionv2.html
    pub struct AutomationRulesActionV2_ {
        pub external_integration_configuration: Option<Box<ExternalIntegrationConfiguration_>>,
        pub finding_fields_update: Option<Box<AutomationRulesFindingFieldsUpdateV2_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_AutomationRulesActionV2 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.AutomationRulesActionV2"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_AutomationRulesActionV2 as AutomationRulesActionV2;
    impl crate::value::ToValue for AutomationRulesActionV2_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.external_integration_configuration {
                properties.insert(
                    "ExternalIntegrationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.finding_fields_update {
                properties.insert(
                    "FindingFieldsUpdate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-automationrulesfindingfieldsupdatev2.html
    pub struct AutomationRulesFindingFieldsUpdateV2_ {
        pub comment: Option<crate::value::ExpString>,
        pub severity_id: Option<i32>,
        pub status_id: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_AutomationRulesFindingFieldsUpdateV2 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.AutomationRulesFindingFieldsUpdateV2"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_AutomationRulesFindingFieldsUpdateV2 as AutomationRulesFindingFieldsUpdateV2;
    impl crate::value::ToValue for AutomationRulesFindingFieldsUpdateV2_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comment {
                properties.insert(
                    "Comment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.severity_id {
                properties.insert(
                    "SeverityId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status_id {
                properties.insert(
                    "StatusId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-booleanfilter.html
    pub struct BooleanFilter_ {
        pub value: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_BooleanFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.BooleanFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_BooleanFilter as BooleanFilter;
    impl crate::value::ToValue for BooleanFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-compositefilter.html
    pub struct CompositeFilter_ {
        pub boolean_filters: Option<Vec<OcsfBooleanFilter_>>,
        pub date_filters: Option<Vec<OcsfDateFilter_>>,
        pub map_filters: Option<Vec<OcsfMapFilter_>>,
        pub number_filters: Option<Vec<OcsfNumberFilter_>>,
        pub operator: Option<crate::value::ExpString>,
        pub string_filters: Option<Vec<OcsfStringFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_CompositeFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.CompositeFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_CompositeFilter as CompositeFilter;
    impl crate::value::ToValue for CompositeFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.boolean_filters {
                properties.insert(
                    "BooleanFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.date_filters {
                properties.insert(
                    "DateFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.map_filters {
                properties.insert(
                    "MapFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_filters {
                properties.insert(
                    "NumberFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.operator {
                properties.insert(
                    "Operator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_filters {
                properties.insert(
                    "StringFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-criteria.html
    pub struct Criteria_ {
        pub ocsf_finding_criteria: Option<Box<OcsfFindingFilters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_Criteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.Criteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_Criteria as Criteria;
    impl crate::value::ToValue for Criteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ocsf_finding_criteria {
                properties.insert(
                    "OcsfFindingCriteria".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-datefilter.html
    pub struct DateFilter_ {
        pub date_range: Option<Box<DateRange_>>,
        pub end: Option<crate::value::ExpString>,
        pub start: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_DateFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.DateFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_DateFilter as DateFilter;
    impl crate::value::ToValue for DateFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.date_range {
                properties.insert(
                    "DateRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.end {
                properties.insert("End".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.start {
                properties.insert("Start".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-daterange.html
    pub struct DateRange_ {
        pub unit: crate::value::ExpString,
        pub value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_DateRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.DateRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_DateRange as DateRange;
    impl crate::value::ToValue for DateRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-externalintegrationconfiguration.html
    pub struct ExternalIntegrationConfiguration_ {
        pub connector_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_ExternalIntegrationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.ExternalIntegrationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_ExternalIntegrationConfiguration as ExternalIntegrationConfiguration;
    impl crate::value::ToValue for ExternalIntegrationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connector_arn {
                properties.insert(
                    "ConnectorArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-mapfilter.html
    pub struct MapFilter_ {
        pub comparison: crate::value::ExpString,
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_MapFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.MapFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_MapFilter as MapFilter;
    impl crate::value::ToValue for MapFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Comparison".to_string(),
                crate::value::ToValue::to_value(&self.comparison),
            );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-numberfilter.html
    pub struct NumberFilter_ {
        pub eq: Option<f64>,
        pub gte: Option<f64>,
        pub lte: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_NumberFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.NumberFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_NumberFilter as NumberFilter;
    impl crate::value::ToValue for NumberFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.eq {
                properties.insert("Eq".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.gte {
                properties.insert("Gte".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.lte {
                properties.insert("Lte".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-ocsfbooleanfilter.html
    pub struct OcsfBooleanFilter_ {
        pub field_name: crate::value::ExpString,
        pub filter: Box<BooleanFilter_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_OcsfBooleanFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.OcsfBooleanFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_OcsfBooleanFilter as OcsfBooleanFilter;
    impl crate::value::ToValue for OcsfBooleanFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldName".to_string(),
                crate::value::ToValue::to_value(&self.field_name),
            );
            properties.insert(
                "Filter".to_string(),
                crate::value::ToValue::to_value(&self.filter),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-ocsfdatefilter.html
    pub struct OcsfDateFilter_ {
        pub field_name: crate::value::ExpString,
        pub filter: Box<DateFilter_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_OcsfDateFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.OcsfDateFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_OcsfDateFilter as OcsfDateFilter;
    impl crate::value::ToValue for OcsfDateFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldName".to_string(),
                crate::value::ToValue::to_value(&self.field_name),
            );
            properties.insert(
                "Filter".to_string(),
                crate::value::ToValue::to_value(&self.filter),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-ocsffindingfilters.html
    pub struct OcsfFindingFilters_ {
        pub composite_filters: Option<Vec<CompositeFilter_>>,
        pub composite_operator: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_OcsfFindingFilters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.OcsfFindingFilters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_OcsfFindingFilters as OcsfFindingFilters;
    impl crate::value::ToValue for OcsfFindingFilters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.composite_filters {
                properties.insert(
                    "CompositeFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.composite_operator {
                properties.insert(
                    "CompositeOperator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-ocsfmapfilter.html
    pub struct OcsfMapFilter_ {
        pub field_name: crate::value::ExpString,
        pub filter: Box<MapFilter_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_OcsfMapFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.OcsfMapFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_OcsfMapFilter as OcsfMapFilter;
    impl crate::value::ToValue for OcsfMapFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldName".to_string(),
                crate::value::ToValue::to_value(&self.field_name),
            );
            properties.insert(
                "Filter".to_string(),
                crate::value::ToValue::to_value(&self.filter),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-ocsfnumberfilter.html
    pub struct OcsfNumberFilter_ {
        pub field_name: crate::value::ExpString,
        pub filter: Box<NumberFilter_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_OcsfNumberFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.OcsfNumberFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_OcsfNumberFilter as OcsfNumberFilter;
    impl crate::value::ToValue for OcsfNumberFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldName".to_string(),
                crate::value::ToValue::to_value(&self.field_name),
            );
            properties.insert(
                "Filter".to_string(),
                crate::value::ToValue::to_value(&self.filter),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-ocsfstringfilter.html
    pub struct OcsfStringFilter_ {
        pub field_name: crate::value::ExpString,
        pub filter: Box<StringFilter_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_OcsfStringFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.OcsfStringFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_OcsfStringFilter as OcsfStringFilter;
    impl crate::value::ToValue for OcsfStringFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldName".to_string(),
                crate::value::ToValue::to_value(&self.field_name),
            );
            properties.insert(
                "Filter".to_string(),
                crate::value::ToValue::to_value(&self.filter),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrulev2-stringfilter.html
    pub struct StringFilter_ {
        pub comparison: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_AutomationRuleV2_StringFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::AutomationRuleV2.StringFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_AutomationRuleV2_StringFilter as StringFilter;
    impl crate::value::ToValue for StringFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Comparison".to_string(),
                crate::value::ToValue::to_value(&self.comparison),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
}
pub mod configurationpolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-configurationpolicy-parameterconfiguration.html
    pub struct ParameterConfiguration_ {
        pub value: Option<Box<ParameterValue_>>,
        pub value_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_ConfigurationPolicy_ParameterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::ConfigurationPolicy.ParameterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_ConfigurationPolicy_ParameterConfiguration as ParameterConfiguration;
    impl crate::value::ToValue for ParameterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "ValueType".to_string(),
                crate::value::ToValue::to_value(&self.value_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-configurationpolicy-parametervalue.html
    pub struct ParameterValue_ {
        pub boolean: Option<crate::value::ExpBool>,
        pub double: Option<f64>,
        pub r#enum: Option<crate::value::ExpString>,
        pub enum_list: Option<Vec<crate::value::ExpString>>,
        pub integer: Option<i32>,
        pub integer_list: Option<Vec<i32>>,
        pub string: Option<crate::value::ExpString>,
        pub string_list: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_ConfigurationPolicy_ParameterValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::ConfigurationPolicy.ParameterValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_ConfigurationPolicy_ParameterValue as ParameterValue;
    impl crate::value::ToValue for ParameterValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.boolean {
                properties.insert(
                    "Boolean".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.double {
                properties.insert("Double".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#enum {
                properties.insert("Enum".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.enum_list {
                properties.insert(
                    "EnumList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.integer {
                properties.insert(
                    "Integer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.integer_list {
                properties.insert(
                    "IntegerList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string {
                properties.insert("String".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.string_list {
                properties.insert(
                    "StringList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-configurationpolicy-policy.html
    pub struct Policy_ {
        pub security_hub: Option<Box<SecurityHubPolicy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_ConfigurationPolicy_Policy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::ConfigurationPolicy.Policy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_ConfigurationPolicy_Policy as Policy;
    impl crate::value::ToValue for Policy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.security_hub {
                properties.insert(
                    "SecurityHub".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-configurationpolicy-securitycontrolcustomparameter.html
    pub struct SecurityControlCustomParameter_ {
        pub parameters: Option<std::collections::BTreeMap<String, ParameterConfiguration_>>,
        pub security_control_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_ConfigurationPolicy_SecurityControlCustomParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::ConfigurationPolicy.SecurityControlCustomParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_ConfigurationPolicy_SecurityControlCustomParameter as SecurityControlCustomParameter;
    impl crate::value::ToValue for SecurityControlCustomParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_control_id {
                properties.insert(
                    "SecurityControlId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-configurationpolicy-securitycontrolsconfiguration.html
    pub struct SecurityControlsConfiguration_ {
        pub disabled_security_control_identifiers: Option<Vec<crate::value::ExpString>>,
        pub enabled_security_control_identifiers: Option<Vec<crate::value::ExpString>>,
        pub security_control_custom_parameters: Option<Vec<SecurityControlCustomParameter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_ConfigurationPolicy_SecurityControlsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::ConfigurationPolicy.SecurityControlsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_ConfigurationPolicy_SecurityControlsConfiguration as SecurityControlsConfiguration;
    impl crate::value::ToValue for SecurityControlsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.disabled_security_control_identifiers {
                properties.insert(
                    "DisabledSecurityControlIdentifiers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled_security_control_identifiers {
                properties.insert(
                    "EnabledSecurityControlIdentifiers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_control_custom_parameters {
                properties.insert(
                    "SecurityControlCustomParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-configurationpolicy-securityhubpolicy.html
    pub struct SecurityHubPolicy_ {
        pub enabled_standard_identifiers: Option<Vec<crate::value::ExpString>>,
        pub security_controls_configuration: Option<Box<SecurityControlsConfiguration_>>,
        pub service_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_ConfigurationPolicy_SecurityHubPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::ConfigurationPolicy.SecurityHubPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_ConfigurationPolicy_SecurityHubPolicy as SecurityHubPolicy;
    impl crate::value::ToValue for SecurityHubPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled_standard_identifiers {
                properties.insert(
                    "EnabledStandardIdentifiers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_controls_configuration {
                properties.insert(
                    "SecurityControlsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_enabled {
                properties.insert(
                    "ServiceEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod insight {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-insight-awssecurityfindingfilters.html
    pub struct AwsSecurityFindingFilters_ {
        pub aws_account_id: Option<Vec<StringFilter_>>,
        pub aws_account_name: Option<Vec<StringFilter_>>,
        pub company_name: Option<Vec<StringFilter_>>,
        pub compliance_associated_standards_id: Option<Vec<StringFilter_>>,
        pub compliance_security_control_id: Option<Vec<StringFilter_>>,
        pub compliance_security_control_parameters_name: Option<Vec<StringFilter_>>,
        pub compliance_security_control_parameters_value: Option<Vec<StringFilter_>>,
        pub compliance_status: Option<Vec<StringFilter_>>,
        pub confidence: Option<Vec<NumberFilter_>>,
        pub created_at: Option<Vec<DateFilter_>>,
        pub criticality: Option<Vec<NumberFilter_>>,
        pub description: Option<Vec<StringFilter_>>,
        pub finding_provider_fields_confidence: Option<Vec<NumberFilter_>>,
        pub finding_provider_fields_criticality: Option<Vec<NumberFilter_>>,
        pub finding_provider_fields_related_findings_id: Option<Vec<StringFilter_>>,
        pub finding_provider_fields_related_findings_product_arn: Option<Vec<StringFilter_>>,
        pub finding_provider_fields_severity_label: Option<Vec<StringFilter_>>,
        pub finding_provider_fields_severity_original: Option<Vec<StringFilter_>>,
        pub finding_provider_fields_types: Option<Vec<StringFilter_>>,
        pub first_observed_at: Option<Vec<DateFilter_>>,
        pub generator_id: Option<Vec<StringFilter_>>,
        pub id: Option<Vec<StringFilter_>>,
        pub last_observed_at: Option<Vec<DateFilter_>>,
        pub malware_name: Option<Vec<StringFilter_>>,
        pub malware_path: Option<Vec<StringFilter_>>,
        pub malware_state: Option<Vec<StringFilter_>>,
        pub malware_type: Option<Vec<StringFilter_>>,
        pub network_destination_domain: Option<Vec<StringFilter_>>,
        pub network_destination_ip_v4: Option<Vec<IpFilter_>>,
        pub network_destination_ip_v6: Option<Vec<IpFilter_>>,
        pub network_destination_port: Option<Vec<NumberFilter_>>,
        pub network_direction: Option<Vec<StringFilter_>>,
        pub network_protocol: Option<Vec<StringFilter_>>,
        pub network_source_domain: Option<Vec<StringFilter_>>,
        pub network_source_ip_v4: Option<Vec<IpFilter_>>,
        pub network_source_ip_v6: Option<Vec<IpFilter_>>,
        pub network_source_mac: Option<Vec<StringFilter_>>,
        pub network_source_port: Option<Vec<NumberFilter_>>,
        pub note_text: Option<Vec<StringFilter_>>,
        pub note_updated_at: Option<Vec<DateFilter_>>,
        pub note_updated_by: Option<Vec<StringFilter_>>,
        pub process_launched_at: Option<Vec<DateFilter_>>,
        pub process_name: Option<Vec<StringFilter_>>,
        pub process_parent_pid: Option<Vec<NumberFilter_>>,
        pub process_path: Option<Vec<StringFilter_>>,
        pub process_pid: Option<Vec<NumberFilter_>>,
        pub process_terminated_at: Option<Vec<DateFilter_>>,
        pub product_arn: Option<Vec<StringFilter_>>,
        pub product_fields: Option<Vec<MapFilter_>>,
        pub product_name: Option<Vec<StringFilter_>>,
        pub recommendation_text: Option<Vec<StringFilter_>>,
        pub record_state: Option<Vec<StringFilter_>>,
        pub region: Option<Vec<StringFilter_>>,
        pub related_findings_id: Option<Vec<StringFilter_>>,
        pub related_findings_product_arn: Option<Vec<StringFilter_>>,
        pub resource_application_arn: Option<Vec<StringFilter_>>,
        pub resource_application_name: Option<Vec<StringFilter_>>,
        pub resource_aws_ec2_instance_iam_instance_profile_arn: Option<Vec<StringFilter_>>,
        pub resource_aws_ec2_instance_image_id: Option<Vec<StringFilter_>>,
        pub resource_aws_ec2_instance_ip_v4_addresses: Option<Vec<IpFilter_>>,
        pub resource_aws_ec2_instance_ip_v6_addresses: Option<Vec<IpFilter_>>,
        pub resource_aws_ec2_instance_key_name: Option<Vec<StringFilter_>>,
        pub resource_aws_ec2_instance_launched_at: Option<Vec<DateFilter_>>,
        pub resource_aws_ec2_instance_subnet_id: Option<Vec<StringFilter_>>,
        pub resource_aws_ec2_instance_type: Option<Vec<StringFilter_>>,
        pub resource_aws_ec2_instance_vpc_id: Option<Vec<StringFilter_>>,
        pub resource_aws_iam_access_key_created_at: Option<Vec<DateFilter_>>,
        pub resource_aws_iam_access_key_principal_name: Option<Vec<StringFilter_>>,
        pub resource_aws_iam_access_key_status: Option<Vec<StringFilter_>>,
        pub resource_aws_iam_user_user_name: Option<Vec<StringFilter_>>,
        pub resource_aws_s3_bucket_owner_id: Option<Vec<StringFilter_>>,
        pub resource_aws_s3_bucket_owner_name: Option<Vec<StringFilter_>>,
        pub resource_container_image_id: Option<Vec<StringFilter_>>,
        pub resource_container_image_name: Option<Vec<StringFilter_>>,
        pub resource_container_launched_at: Option<Vec<DateFilter_>>,
        pub resource_container_name: Option<Vec<StringFilter_>>,
        pub resource_details_other: Option<Vec<MapFilter_>>,
        pub resource_id: Option<Vec<StringFilter_>>,
        pub resource_partition: Option<Vec<StringFilter_>>,
        pub resource_region: Option<Vec<StringFilter_>>,
        pub resource_tags: Option<Vec<MapFilter_>>,
        pub resource_type: Option<Vec<StringFilter_>>,
        pub sample: Option<Vec<BooleanFilter_>>,
        pub severity_label: Option<Vec<StringFilter_>>,
        pub source_url: Option<Vec<StringFilter_>>,
        pub threat_intel_indicator_category: Option<Vec<StringFilter_>>,
        pub threat_intel_indicator_last_observed_at: Option<Vec<DateFilter_>>,
        pub threat_intel_indicator_source: Option<Vec<StringFilter_>>,
        pub threat_intel_indicator_source_url: Option<Vec<StringFilter_>>,
        pub threat_intel_indicator_type: Option<Vec<StringFilter_>>,
        pub threat_intel_indicator_value: Option<Vec<StringFilter_>>,
        pub title: Option<Vec<StringFilter_>>,
        pub r#type: Option<Vec<StringFilter_>>,
        pub updated_at: Option<Vec<DateFilter_>>,
        pub user_defined_fields: Option<Vec<MapFilter_>>,
        pub verification_state: Option<Vec<StringFilter_>>,
        pub vulnerabilities_exploit_available: Option<Vec<StringFilter_>>,
        pub vulnerabilities_fix_available: Option<Vec<StringFilter_>>,
        pub workflow_state: Option<Vec<StringFilter_>>,
        pub workflow_status: Option<Vec<StringFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_Insight_AwsSecurityFindingFilters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::Insight.AwsSecurityFindingFilters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_Insight_AwsSecurityFindingFilters as AwsSecurityFindingFilters;
    impl crate::value::ToValue for AwsSecurityFindingFilters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_account_id {
                properties.insert(
                    "AwsAccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.aws_account_name {
                properties.insert(
                    "AwsAccountName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.company_name {
                properties.insert(
                    "CompanyName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compliance_associated_standards_id {
                properties.insert(
                    "ComplianceAssociatedStandardsId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compliance_security_control_id {
                properties.insert(
                    "ComplianceSecurityControlId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compliance_security_control_parameters_name {
                properties.insert(
                    "ComplianceSecurityControlParametersName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compliance_security_control_parameters_value {
                properties.insert(
                    "ComplianceSecurityControlParametersValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compliance_status {
                properties.insert(
                    "ComplianceStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.confidence {
                properties.insert(
                    "Confidence".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.created_at {
                properties.insert(
                    "CreatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.criticality {
                properties.insert(
                    "Criticality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.finding_provider_fields_confidence {
                properties.insert(
                    "FindingProviderFieldsConfidence".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.finding_provider_fields_criticality {
                properties.insert(
                    "FindingProviderFieldsCriticality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.finding_provider_fields_related_findings_id {
                properties.insert(
                    "FindingProviderFieldsRelatedFindingsId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.finding_provider_fields_related_findings_product_arn {
                properties.insert(
                    "FindingProviderFieldsRelatedFindingsProductArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.finding_provider_fields_severity_label {
                properties.insert(
                    "FindingProviderFieldsSeverityLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.finding_provider_fields_severity_original {
                properties.insert(
                    "FindingProviderFieldsSeverityOriginal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.finding_provider_fields_types {
                properties.insert(
                    "FindingProviderFieldsTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.first_observed_at {
                properties.insert(
                    "FirstObservedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.generator_id {
                properties.insert(
                    "GeneratorId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.last_observed_at {
                properties.insert(
                    "LastObservedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.malware_name {
                properties.insert(
                    "MalwareName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.malware_path {
                properties.insert(
                    "MalwarePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.malware_state {
                properties.insert(
                    "MalwareState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.malware_type {
                properties.insert(
                    "MalwareType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_destination_domain {
                properties.insert(
                    "NetworkDestinationDomain".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_destination_ip_v4 {
                properties.insert(
                    "NetworkDestinationIpV4".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_destination_ip_v6 {
                properties.insert(
                    "NetworkDestinationIpV6".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_destination_port {
                properties.insert(
                    "NetworkDestinationPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_direction {
                properties.insert(
                    "NetworkDirection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_protocol {
                properties.insert(
                    "NetworkProtocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_source_domain {
                properties.insert(
                    "NetworkSourceDomain".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_source_ip_v4 {
                properties.insert(
                    "NetworkSourceIpV4".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_source_ip_v6 {
                properties.insert(
                    "NetworkSourceIpV6".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_source_mac {
                properties.insert(
                    "NetworkSourceMac".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_source_port {
                properties.insert(
                    "NetworkSourcePort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.note_text {
                properties.insert(
                    "NoteText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.note_updated_at {
                properties.insert(
                    "NoteUpdatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.note_updated_by {
                properties.insert(
                    "NoteUpdatedBy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.process_launched_at {
                properties.insert(
                    "ProcessLaunchedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.process_name {
                properties.insert(
                    "ProcessName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.process_parent_pid {
                properties.insert(
                    "ProcessParentPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.process_path {
                properties.insert(
                    "ProcessPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.process_pid {
                properties.insert(
                    "ProcessPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.process_terminated_at {
                properties.insert(
                    "ProcessTerminatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.product_arn {
                properties.insert(
                    "ProductArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.product_fields {
                properties.insert(
                    "ProductFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.product_name {
                properties.insert(
                    "ProductName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.recommendation_text {
                properties.insert(
                    "RecommendationText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.record_state {
                properties.insert(
                    "RecordState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.related_findings_id {
                properties.insert(
                    "RelatedFindingsId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.related_findings_product_arn {
                properties.insert(
                    "RelatedFindingsProductArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_application_arn {
                properties.insert(
                    "ResourceApplicationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_application_name {
                properties.insert(
                    "ResourceApplicationName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_ec2_instance_iam_instance_profile_arn {
                properties.insert(
                    "ResourceAwsEc2InstanceIamInstanceProfileArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_ec2_instance_image_id {
                properties.insert(
                    "ResourceAwsEc2InstanceImageId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_ec2_instance_ip_v4_addresses {
                properties.insert(
                    "ResourceAwsEc2InstanceIpV4Addresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_ec2_instance_ip_v6_addresses {
                properties.insert(
                    "ResourceAwsEc2InstanceIpV6Addresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_ec2_instance_key_name {
                properties.insert(
                    "ResourceAwsEc2InstanceKeyName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_ec2_instance_launched_at {
                properties.insert(
                    "ResourceAwsEc2InstanceLaunchedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_ec2_instance_subnet_id {
                properties.insert(
                    "ResourceAwsEc2InstanceSubnetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_ec2_instance_type {
                properties.insert(
                    "ResourceAwsEc2InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_ec2_instance_vpc_id {
                properties.insert(
                    "ResourceAwsEc2InstanceVpcId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_iam_access_key_created_at {
                properties.insert(
                    "ResourceAwsIamAccessKeyCreatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_iam_access_key_principal_name {
                properties.insert(
                    "ResourceAwsIamAccessKeyPrincipalName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_iam_access_key_status {
                properties.insert(
                    "ResourceAwsIamAccessKeyStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_iam_user_user_name {
                properties.insert(
                    "ResourceAwsIamUserUserName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_s3_bucket_owner_id {
                properties.insert(
                    "ResourceAwsS3BucketOwnerId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_aws_s3_bucket_owner_name {
                properties.insert(
                    "ResourceAwsS3BucketOwnerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_container_image_id {
                properties.insert(
                    "ResourceContainerImageId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_container_image_name {
                properties.insert(
                    "ResourceContainerImageName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_container_launched_at {
                properties.insert(
                    "ResourceContainerLaunchedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_container_name {
                properties.insert(
                    "ResourceContainerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_details_other {
                properties.insert(
                    "ResourceDetailsOther".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_id {
                properties.insert(
                    "ResourceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_partition {
                properties.insert(
                    "ResourcePartition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_region {
                properties.insert(
                    "ResourceRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_tags {
                properties.insert(
                    "ResourceTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_type {
                properties.insert(
                    "ResourceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sample {
                properties.insert("Sample".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.severity_label {
                properties.insert(
                    "SeverityLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_url {
                properties.insert(
                    "SourceUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.threat_intel_indicator_category {
                properties.insert(
                    "ThreatIntelIndicatorCategory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.threat_intel_indicator_last_observed_at {
                properties.insert(
                    "ThreatIntelIndicatorLastObservedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.threat_intel_indicator_source {
                properties.insert(
                    "ThreatIntelIndicatorSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.threat_intel_indicator_source_url {
                properties.insert(
                    "ThreatIntelIndicatorSourceUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.threat_intel_indicator_type {
                properties.insert(
                    "ThreatIntelIndicatorType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.threat_intel_indicator_value {
                properties.insert(
                    "ThreatIntelIndicatorValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.title {
                properties.insert("Title".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.updated_at {
                properties.insert(
                    "UpdatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_defined_fields {
                properties.insert(
                    "UserDefinedFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.verification_state {
                properties.insert(
                    "VerificationState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vulnerabilities_exploit_available {
                properties.insert(
                    "VulnerabilitiesExploitAvailable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vulnerabilities_fix_available {
                properties.insert(
                    "VulnerabilitiesFixAvailable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.workflow_state {
                properties.insert(
                    "WorkflowState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.workflow_status {
                properties.insert(
                    "WorkflowStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-insight-booleanfilter.html
    pub struct BooleanFilter_ {
        pub value: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_Insight_BooleanFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::Insight.BooleanFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_Insight_BooleanFilter as BooleanFilter;
    impl crate::value::ToValue for BooleanFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-insight-datefilter.html
    pub struct DateFilter_ {
        pub date_range: Option<Box<DateRange_>>,
        pub end: Option<crate::value::ExpString>,
        pub start: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_Insight_DateFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::Insight.DateFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_Insight_DateFilter as DateFilter;
    impl crate::value::ToValue for DateFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.date_range {
                properties.insert(
                    "DateRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.end {
                properties.insert("End".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.start {
                properties.insert("Start".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-insight-daterange.html
    pub struct DateRange_ {
        pub unit: crate::value::ExpString,
        pub value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_Insight_DateRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::Insight.DateRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_Insight_DateRange as DateRange;
    impl crate::value::ToValue for DateRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-insight-ipfilter.html
    pub struct IpFilter_ {
        pub cidr: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_Insight_IpFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::Insight.IpFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_Insight_IpFilter as IpFilter;
    impl crate::value::ToValue for IpFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Cidr".to_string(),
                crate::value::ToValue::to_value(&self.cidr),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-insight-mapfilter.html
    pub struct MapFilter_ {
        pub comparison: crate::value::ExpString,
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_Insight_MapFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::Insight.MapFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_Insight_MapFilter as MapFilter;
    impl crate::value::ToValue for MapFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Comparison".to_string(),
                crate::value::ToValue::to_value(&self.comparison),
            );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-insight-numberfilter.html
    pub struct NumberFilter_ {
        pub eq: Option<f64>,
        pub gte: Option<f64>,
        pub lte: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_Insight_NumberFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::Insight.NumberFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_Insight_NumberFilter as NumberFilter;
    impl crate::value::ToValue for NumberFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.eq {
                properties.insert("Eq".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.gte {
                properties.insert("Gte".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.lte {
                properties.insert("Lte".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-insight-stringfilter.html
    pub struct StringFilter_ {
        pub comparison: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_Insight_StringFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::Insight.StringFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_Insight_StringFilter as StringFilter;
    impl crate::value::ToValue for StringFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Comparison".to_string(),
                crate::value::ToValue::to_value(&self.comparison),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
}
pub mod securitycontrol {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-securitycontrol-parameterconfiguration.html
    pub struct ParameterConfiguration_ {
        pub value: Option<Box<ParameterValue_>>,
        pub value_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_SecurityControl_ParameterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::SecurityControl.ParameterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_SecurityControl_ParameterConfiguration as ParameterConfiguration;
    impl crate::value::ToValue for ParameterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "ValueType".to_string(),
                crate::value::ToValue::to_value(&self.value_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-securitycontrol-parametervalue.html
    pub struct ParameterValue_ {
        pub boolean: Option<crate::value::ExpBool>,
        pub double: Option<f64>,
        pub r#enum: Option<crate::value::ExpString>,
        pub enum_list: Option<Vec<crate::value::ExpString>>,
        pub integer: Option<i32>,
        pub integer_list: Option<Vec<i32>>,
        pub string: Option<crate::value::ExpString>,
        pub string_list: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_SecurityControl_ParameterValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::SecurityControl.ParameterValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_SecurityControl_ParameterValue as ParameterValue;
    impl crate::value::ToValue for ParameterValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.boolean {
                properties.insert(
                    "Boolean".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.double {
                properties.insert("Double".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#enum {
                properties.insert("Enum".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.enum_list {
                properties.insert(
                    "EnumList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.integer {
                properties.insert(
                    "Integer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.integer_list {
                properties.insert(
                    "IntegerList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string {
                properties.insert("String".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.string_list {
                properties.insert(
                    "StringList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod standard {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-standard-standardscontrol.html
    pub struct StandardsControl_ {
        pub reason: Option<crate::value::ExpString>,
        pub standards_control_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securityhub_Standard_StandardsControl {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SecurityHub::Standard.StandardsControl"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securityhub_Standard_StandardsControl as StandardsControl;
    impl crate::value::ToValue for StandardsControl_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.reason {
                properties.insert("Reason".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "StandardsControlArn".to_string(),
                crate::value::ToValue::to_value(&self.standards_control_arn),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-aggregatorv2.html
pub struct AggregatorV2_ {
    pub linked_regions: Vec<crate::value::ExpString>,
    pub region_linking_mode: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_AggregatorV2 {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::AggregatorV2"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_AggregatorV2 as AggregatorV2;
impl crate::template::ToResource for AggregatorV2_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AggregatorV2"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "LinkedRegions".to_string(),
            crate::value::ToValue::to_value(&self.linked_regions),
        );
        properties.insert(
            "RegionLinkingMode".to_string(),
            crate::value::ToValue::to_value(&self.region_linking_mode),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-automationrule.html
pub struct AutomationRule_ {
    pub actions: Vec<super::securityhub::automationrule::AutomationRulesAction_>,
    pub criteria: super::securityhub::automationrule::AutomationRulesFindingFilters_,
    pub description: crate::value::ExpString,
    pub is_terminal: Option<crate::value::ExpBool>,
    pub rule_name: crate::value::ExpString,
    pub rule_order: i32,
    pub rule_status: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_AutomationRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::AutomationRule"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_AutomationRule as AutomationRule;
impl crate::template::ToResource for AutomationRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AutomationRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Actions".to_string(),
            crate::value::ToValue::to_value(&self.actions),
        );
        properties.insert(
            "Criteria".to_string(),
            crate::value::ToValue::to_value(&self.criteria),
        );
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        if let Some(ref value) = self.is_terminal {
            properties.insert(
                "IsTerminal".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RuleName".to_string(),
            crate::value::ToValue::to_value(&self.rule_name),
        );
        properties.insert(
            "RuleOrder".to_string(),
            crate::value::ToValue::to_value(&self.rule_order),
        );
        if let Some(ref value) = self.rule_status {
            properties.insert(
                "RuleStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-automationrulev2.html
pub struct AutomationRuleV2_ {
    pub actions: Vec<super::securityhub::automationrulev2::AutomationRulesActionV2_>,
    pub criteria: super::securityhub::automationrulev2::Criteria_,
    pub description: crate::value::ExpString,
    pub rule_name: crate::value::ExpString,
    pub rule_order: f64,
    pub rule_status: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_AutomationRuleV2 {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::AutomationRuleV2"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_AutomationRuleV2 as AutomationRuleV2;
impl crate::template::ToResource for AutomationRuleV2_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AutomationRuleV2"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Actions".to_string(),
            crate::value::ToValue::to_value(&self.actions),
        );
        properties.insert(
            "Criteria".to_string(),
            crate::value::ToValue::to_value(&self.criteria),
        );
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        properties.insert(
            "RuleName".to_string(),
            crate::value::ToValue::to_value(&self.rule_name),
        );
        properties.insert(
            "RuleOrder".to_string(),
            crate::value::ToValue::to_value(&self.rule_order),
        );
        if let Some(ref value) = self.rule_status {
            properties.insert(
                "RuleStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-configurationpolicy.html
pub struct ConfigurationPolicy_ {
    pub configuration_policy: super::securityhub::configurationpolicy::Policy_,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_ConfigurationPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::ConfigurationPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_ConfigurationPolicy as ConfigurationPolicy;
impl crate::template::ToResource for ConfigurationPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConfigurationPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConfigurationPolicy".to_string(),
            crate::value::ToValue::to_value(&self.configuration_policy),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-delegatedadmin.html
pub struct DelegatedAdmin_ {
    pub admin_account_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_DelegatedAdmin {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::DelegatedAdmin"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_DelegatedAdmin as DelegatedAdmin;
impl crate::template::ToResource for DelegatedAdmin_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DelegatedAdmin"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AdminAccountId".to_string(),
            crate::value::ToValue::to_value(&self.admin_account_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-findingaggregator.html
pub struct FindingAggregator_ {
    pub region_linking_mode: crate::value::ExpString,
    pub regions: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_FindingAggregator {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::FindingAggregator"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_FindingAggregator as FindingAggregator;
impl crate::template::ToResource for FindingAggregator_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FindingAggregator"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "RegionLinkingMode".to_string(),
            crate::value::ToValue::to_value(&self.region_linking_mode),
        );
        if let Some(ref value) = self.regions {
            properties.insert(
                "Regions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-hub.html
pub struct Hub_ {
    pub auto_enable_controls: Option<crate::value::ExpBool>,
    pub control_finding_generator: Option<crate::value::ExpString>,
    pub enable_default_standards: Option<crate::value::ExpBool>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_Hub {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::Hub"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_Hub as Hub;
impl crate::template::ToResource for Hub_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Hub"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_enable_controls {
            properties.insert(
                "AutoEnableControls".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.control_finding_generator {
            properties.insert(
                "ControlFindingGenerator".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_default_standards {
            properties.insert(
                "EnableDefaultStandards".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-hubv2.html
pub struct HubV2_ {
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_HubV2 {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::HubV2"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_HubV2 as HubV2;
impl crate::template::ToResource for HubV2_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("HubV2"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-insight.html
pub struct Insight_ {
    pub filters: super::securityhub::insight::AwsSecurityFindingFilters_,
    pub group_by_attribute: crate::value::ExpString,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_Insight {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::Insight"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_Insight as Insight;
impl crate::template::ToResource for Insight_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Insight"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Filters".to_string(),
            crate::value::ToValue::to_value(&self.filters),
        );
        properties.insert(
            "GroupByAttribute".to_string(),
            crate::value::ToValue::to_value(&self.group_by_attribute),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-organizationconfiguration.html
pub struct OrganizationConfiguration_ {
    pub auto_enable: crate::value::ExpBool,
    pub auto_enable_standards: Option<crate::value::ExpString>,
    pub configuration_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_OrganizationConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::OrganizationConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_OrganizationConfiguration as OrganizationConfiguration;
impl crate::template::ToResource for OrganizationConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OrganizationConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AutoEnable".to_string(),
            crate::value::ToValue::to_value(&self.auto_enable),
        );
        if let Some(ref value) = self.auto_enable_standards {
            properties.insert(
                "AutoEnableStandards".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.configuration_type {
            properties.insert(
                "ConfigurationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-policyassociation.html
pub struct PolicyAssociation_ {
    pub configuration_policy_id: crate::value::ExpString,
    pub target_id: crate::value::ExpString,
    pub target_type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_PolicyAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::PolicyAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_PolicyAssociation as PolicyAssociation;
impl crate::template::ToResource for PolicyAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PolicyAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConfigurationPolicyId".to_string(),
            crate::value::ToValue::to_value(&self.configuration_policy_id),
        );
        properties.insert(
            "TargetId".to_string(),
            crate::value::ToValue::to_value(&self.target_id),
        );
        properties.insert(
            "TargetType".to_string(),
            crate::value::ToValue::to_value(&self.target_type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-productsubscription.html
pub struct ProductSubscription_ {
    pub product_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_ProductSubscription {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::ProductSubscription"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_ProductSubscription as ProductSubscription;
impl crate::template::ToResource for ProductSubscription_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ProductSubscription"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ProductArn".to_string(),
            crate::value::ToValue::to_value(&self.product_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-securitycontrol.html
pub struct SecurityControl_ {
    pub last_update_reason: Option<crate::value::ExpString>,
    pub parameters: std::collections::BTreeMap<
        String,
        super::securityhub::securitycontrol::ParameterConfiguration_,
    >,
    pub security_control_arn: Option<crate::value::ExpString>,
    pub security_control_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_SecurityControl {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::SecurityControl"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_SecurityControl as SecurityControl;
impl crate::template::ToResource for SecurityControl_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityControl"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.last_update_reason {
            properties.insert(
                "LastUpdateReason".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Parameters".to_string(),
            crate::value::ToValue::to_value(&self.parameters),
        );
        if let Some(ref value) = self.security_control_arn {
            properties.insert(
                "SecurityControlArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_control_id {
            properties.insert(
                "SecurityControlId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-standard.html
pub struct Standard_ {
    pub disabled_standards_controls: Option<Vec<super::securityhub::standard::StandardsControl_>>,
    pub standards_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securityhub_Standard {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SecurityHub::Standard"
        $($field $value)*)
    };
}
pub use crate::__aws_securityhub_Standard as Standard;
impl crate::template::ToResource for Standard_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Standard"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.disabled_standards_controls {
            properties.insert(
                "DisabledStandardsControls".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "StandardsArn".to_string(),
            crate::value::ToValue::to_value(&self.standards_arn),
        );
        properties
    }
}
