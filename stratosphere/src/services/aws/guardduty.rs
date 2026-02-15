pub mod detector {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfndatasourceconfigurations.html
    pub struct CFNDataSourceConfigurations_ {
        pub kubernetes: Option<Box<CFNKubernetesConfiguration_>>,
        pub malware_protection: Option<Box<CFNMalwareProtectionConfiguration_>>,
        pub s3_logs: Option<Box<CFNS3LogsConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_Detector_CFNDataSourceConfigurations {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::Detector.CFNDataSourceConfigurations"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_Detector_CFNDataSourceConfigurations as CFNDataSourceConfigurations;
    impl crate::value::ToValue for CFNDataSourceConfigurations_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kubernetes {
                properties.insert(
                    "Kubernetes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.malware_protection {
                properties.insert(
                    "MalwareProtection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_logs {
                properties.insert("S3Logs".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfnfeatureadditionalconfiguration.html
    pub struct CFNFeatureAdditionalConfiguration_ {
        pub name: Option<crate::value::ExpString>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_Detector_CFNFeatureAdditionalConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::Detector.CFNFeatureAdditionalConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_Detector_CFNFeatureAdditionalConfiguration as CFNFeatureAdditionalConfiguration;
    impl crate::value::ToValue for CFNFeatureAdditionalConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfnfeatureconfiguration.html
    pub struct CFNFeatureConfiguration_ {
        pub additional_configuration: Option<Vec<CFNFeatureAdditionalConfiguration_>>,
        pub name: crate::value::ExpString,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_Detector_CFNFeatureConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::Detector.CFNFeatureConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_Detector_CFNFeatureConfiguration as CFNFeatureConfiguration;
    impl crate::value::ToValue for CFNFeatureConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_configuration {
                properties.insert(
                    "AdditionalConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfnkubernetesauditlogsconfiguration.html
    pub struct CFNKubernetesAuditLogsConfiguration_ {
        pub enable: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_Detector_CFNKubernetesAuditLogsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::Detector.CFNKubernetesAuditLogsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_Detector_CFNKubernetesAuditLogsConfiguration as CFNKubernetesAuditLogsConfiguration;
    impl crate::value::ToValue for CFNKubernetesAuditLogsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enable".to_string(),
                crate::value::ToValue::to_value(&self.enable),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfnkubernetesconfiguration.html
    pub struct CFNKubernetesConfiguration_ {
        pub audit_logs: Box<CFNKubernetesAuditLogsConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_Detector_CFNKubernetesConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::Detector.CFNKubernetesConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_Detector_CFNKubernetesConfiguration as CFNKubernetesConfiguration;
    impl crate::value::ToValue for CFNKubernetesConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuditLogs".to_string(),
                crate::value::ToValue::to_value(&self.audit_logs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfnmalwareprotectionconfiguration.html
    pub struct CFNMalwareProtectionConfiguration_ {
        pub scan_ec2_instance_with_findings:
            Option<Box<CFNScanEc2InstanceWithFindingsConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_Detector_CFNMalwareProtectionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::Detector.CFNMalwareProtectionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_Detector_CFNMalwareProtectionConfiguration as CFNMalwareProtectionConfiguration;
    impl crate::value::ToValue for CFNMalwareProtectionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.scan_ec2_instance_with_findings {
                properties.insert(
                    "ScanEc2InstanceWithFindings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfns3logsconfiguration.html
    pub struct CFNS3LogsConfiguration_ {
        pub enable: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_Detector_CFNS3LogsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::Detector.CFNS3LogsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_Detector_CFNS3LogsConfiguration as CFNS3LogsConfiguration;
    impl crate::value::ToValue for CFNS3LogsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enable".to_string(),
                crate::value::ToValue::to_value(&self.enable),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-cfnscanec2instancewithfindingsconfiguration.html
    pub struct CFNScanEc2InstanceWithFindingsConfiguration_ {
        pub ebs_volumes: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_Detector_CFNScanEc2InstanceWithFindingsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::Detector.CFNScanEc2InstanceWithFindingsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_Detector_CFNScanEc2InstanceWithFindingsConfiguration as CFNScanEc2InstanceWithFindingsConfiguration;
    impl crate::value::ToValue for CFNScanEc2InstanceWithFindingsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ebs_volumes {
                properties.insert(
                    "EbsVolumes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-detector-tagitem.html
    pub struct TagItem_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_Detector_TagItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::Detector.TagItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_Detector_TagItem as TagItem;
    impl crate::value::ToValue for TagItem_ {
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
}
pub mod filter {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-filter-condition.html
    pub struct Condition_ {
        pub eq: Option<Vec<crate::value::ExpString>>,
        pub equals: Option<Vec<crate::value::ExpString>>,
        pub greater_than: Option<i64>,
        pub greater_than_or_equal: Option<i64>,
        pub gt: Option<i32>,
        pub gte: Option<i32>,
        pub less_than: Option<i64>,
        pub less_than_or_equal: Option<i64>,
        pub lt: Option<i32>,
        pub lte: Option<i32>,
        pub neq: Option<Vec<crate::value::ExpString>>,
        pub not_equals: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_Filter_Condition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::Filter.Condition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_Filter_Condition as Condition;
    impl crate::value::ToValue for Condition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.eq {
                properties.insert("Eq".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.equals {
                properties.insert("Equals".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.greater_than {
                properties.insert(
                    "GreaterThan".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.greater_than_or_equal {
                properties.insert(
                    "GreaterThanOrEqual".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gt {
                properties.insert("Gt".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.gte {
                properties.insert("Gte".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.less_than {
                properties.insert(
                    "LessThan".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.less_than_or_equal {
                properties.insert(
                    "LessThanOrEqual".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lt {
                properties.insert("Lt".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.lte {
                properties.insert("Lte".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.neq {
                properties.insert("Neq".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.not_equals {
                properties.insert(
                    "NotEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-filter-findingcriteria.html
    pub struct FindingCriteria_ {
        pub criterion: Option<std::collections::BTreeMap<String, Condition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_Filter_FindingCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::Filter.FindingCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_Filter_FindingCriteria as FindingCriteria;
    impl crate::value::ToValue for FindingCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.criterion {
                properties.insert(
                    "Criterion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-filter-tagitem.html
    pub struct TagItem_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_Filter_TagItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::Filter.TagItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_Filter_TagItem as TagItem;
    impl crate::value::ToValue for TagItem_ {
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
}
pub mod ipset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-ipset-tagitem.html
    pub struct TagItem_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_IPSet_TagItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::IPSet.TagItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_IPSet_TagItem as TagItem;
    impl crate::value::ToValue for TagItem_ {
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
}
pub mod malwareprotectionplan {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-malwareprotectionplan-cfnactions.html
    pub struct CFNActions_ {
        pub tagging: Option<Box<CFNTagging_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_MalwareProtectionPlan_CFNActions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::MalwareProtectionPlan.CFNActions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_MalwareProtectionPlan_CFNActions as CFNActions;
    impl crate::value::ToValue for CFNActions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.tagging {
                properties.insert(
                    "Tagging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-malwareprotectionplan-cfnprotectedresource.html
    pub struct CFNProtectedResource_ {
        pub s3_bucket: Box<S3Bucket_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_MalwareProtectionPlan_CFNProtectedResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::MalwareProtectionPlan.CFNProtectedResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_MalwareProtectionPlan_CFNProtectedResource as CFNProtectedResource;
    impl crate::value::ToValue for CFNProtectedResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-malwareprotectionplan-cfnstatusreasons.html
    pub struct CFNStatusReasons_ {
        pub code: Option<crate::value::ExpString>,
        pub message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_MalwareProtectionPlan_CFNStatusReasons {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::MalwareProtectionPlan.CFNStatusReasons"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_MalwareProtectionPlan_CFNStatusReasons as CFNStatusReasons;
    impl crate::value::ToValue for CFNStatusReasons_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code {
                properties.insert("Code".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.message {
                properties.insert(
                    "Message".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-malwareprotectionplan-cfntagging.html
    pub struct CFNTagging_ {
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_MalwareProtectionPlan_CFNTagging {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::MalwareProtectionPlan.CFNTagging"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_MalwareProtectionPlan_CFNTagging as CFNTagging;
    impl crate::value::ToValue for CFNTagging_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-malwareprotectionplan-s3bucket.html
    pub struct S3Bucket_ {
        pub bucket_name: Option<crate::value::ExpString>,
        pub object_prefixes: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_MalwareProtectionPlan_S3Bucket {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::MalwareProtectionPlan.S3Bucket"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_MalwareProtectionPlan_S3Bucket as S3Bucket;
    impl crate::value::ToValue for S3Bucket_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_name {
                properties.insert(
                    "BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.object_prefixes {
                properties.insert(
                    "ObjectPrefixes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-malwareprotectionplan-tagitem.html
    pub struct TagItem_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_MalwareProtectionPlan_TagItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::MalwareProtectionPlan.TagItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_MalwareProtectionPlan_TagItem as TagItem;
    impl crate::value::ToValue for TagItem_ {
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
}
pub mod publishingdestination {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-publishingdestination-cfndestinationproperties.html
    pub struct CFNDestinationProperties_ {
        pub destination_arn: Option<crate::value::ExpString>,
        pub kms_key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_PublishingDestination_CFNDestinationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::PublishingDestination.CFNDestinationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_PublishingDestination_CFNDestinationProperties as CFNDestinationProperties;
    impl crate::value::ToValue for CFNDestinationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_arn {
                properties.insert(
                    "DestinationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-publishingdestination-tagitem.html
    pub struct TagItem_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_PublishingDestination_TagItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::PublishingDestination.TagItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_PublishingDestination_TagItem as TagItem;
    impl crate::value::ToValue for TagItem_ {
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
}
pub mod threatentityset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-threatentityset-tagitem.html
    pub struct TagItem_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_ThreatEntitySet_TagItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::ThreatEntitySet.TagItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_ThreatEntitySet_TagItem as TagItem;
    impl crate::value::ToValue for TagItem_ {
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
}
pub mod threatintelset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-threatintelset-tagitem.html
    pub struct TagItem_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_ThreatIntelSet_TagItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::ThreatIntelSet.TagItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_ThreatIntelSet_TagItem as TagItem;
    impl crate::value::ToValue for TagItem_ {
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
}
pub mod trustedentityset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-guardduty-trustedentityset-tagitem.html
    pub struct TagItem_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_guardduty_TrustedEntitySet_TagItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GuardDuty::TrustedEntitySet.TagItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_guardduty_TrustedEntitySet_TagItem as TagItem;
    impl crate::value::ToValue for TagItem_ {
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
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-detector.html
pub struct Detector_ {
    pub data_sources: Option<super::guardduty::detector::CFNDataSourceConfigurations_>,
    pub enable: crate::value::ExpBool,
    pub features: Option<Vec<super::guardduty::detector::CFNFeatureConfiguration_>>,
    pub finding_publishing_frequency: Option<crate::value::ExpString>,
    pub tags: Option<Vec<super::guardduty::detector::TagItem_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_guardduty_Detector {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GuardDuty::Detector"
        $($field $value)*)
    };
}
pub use crate::__aws_guardduty_Detector as Detector;
impl crate::template::ToResource for Detector_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GuardDuty"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Detector"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_sources {
            properties.insert(
                "DataSources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Enable".to_string(),
            crate::value::ToValue::to_value(&self.enable),
        );
        if let Some(ref value) = self.features {
            properties.insert(
                "Features".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.finding_publishing_frequency {
            properties.insert(
                "FindingPublishingFrequency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-filter.html
pub struct Filter_ {
    pub action: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub detector_id: crate::value::ExpString,
    pub finding_criteria: super::guardduty::filter::FindingCriteria_,
    pub name: crate::value::ExpString,
    pub rank: Option<i32>,
    pub tags: Option<Vec<super::guardduty::filter::TagItem_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_guardduty_Filter {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GuardDuty::Filter"
        $($field $value)*)
    };
}
pub use crate::__aws_guardduty_Filter as Filter;
impl crate::template::ToResource for Filter_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GuardDuty"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Filter"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.action {
            properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DetectorId".to_string(),
            crate::value::ToValue::to_value(&self.detector_id),
        );
        properties.insert(
            "FindingCriteria".to_string(),
            crate::value::ToValue::to_value(&self.finding_criteria),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.rank {
            properties.insert("Rank".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-ipset.html
pub struct IPSet_ {
    pub activate: Option<crate::value::ExpBool>,
    pub detector_id: Option<crate::value::ExpString>,
    pub expected_bucket_owner: Option<crate::value::ExpString>,
    pub format: crate::value::ExpString,
    pub location: crate::value::ExpString,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<super::guardduty::ipset::TagItem_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_guardduty_IPSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GuardDuty::IPSet"
        $($field $value)*)
    };
}
pub use crate::__aws_guardduty_IPSet as IPSet;
impl crate::template::ToResource for IPSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GuardDuty"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IPSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.activate {
            properties.insert(
                "Activate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.detector_id {
            properties.insert(
                "DetectorId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.expected_bucket_owner {
            properties.insert(
                "ExpectedBucketOwner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Format".to_string(),
            crate::value::ToValue::to_value(&self.format),
        );
        properties.insert(
            "Location".to_string(),
            crate::value::ToValue::to_value(&self.location),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-malwareprotectionplan.html
pub struct MalwareProtectionPlan_ {
    pub actions: Option<super::guardduty::malwareprotectionplan::CFNActions_>,
    pub protected_resource: super::guardduty::malwareprotectionplan::CFNProtectedResource_,
    pub role: crate::value::ExpString,
    pub tags: Option<Vec<super::guardduty::malwareprotectionplan::TagItem_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_guardduty_MalwareProtectionPlan {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GuardDuty::MalwareProtectionPlan"
        $($field $value)*)
    };
}
pub use crate::__aws_guardduty_MalwareProtectionPlan as MalwareProtectionPlan;
impl crate::template::ToResource for MalwareProtectionPlan_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GuardDuty"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MalwareProtectionPlan"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.actions {
            properties.insert(
                "Actions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ProtectedResource".to_string(),
            crate::value::ToValue::to_value(&self.protected_resource),
        );
        properties.insert(
            "Role".to_string(),
            crate::value::ToValue::to_value(&self.role),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-master.html
pub struct Master_ {
    pub detector_id: crate::value::ExpString,
    pub invitation_id: Option<crate::value::ExpString>,
    pub master_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_guardduty_Master {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GuardDuty::Master"
        $($field $value)*)
    };
}
pub use crate::__aws_guardduty_Master as Master;
impl crate::template::ToResource for Master_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GuardDuty"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Master"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DetectorId".to_string(),
            crate::value::ToValue::to_value(&self.detector_id),
        );
        if let Some(ref value) = self.invitation_id {
            properties.insert(
                "InvitationId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MasterId".to_string(),
            crate::value::ToValue::to_value(&self.master_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-member.html
pub struct Member_ {
    pub detector_id: Option<crate::value::ExpString>,
    pub disable_email_notification: Option<crate::value::ExpBool>,
    pub email: crate::value::ExpString,
    pub member_id: Option<crate::value::ExpString>,
    pub message: Option<crate::value::ExpString>,
    pub status: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_guardduty_Member {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GuardDuty::Member"
        $($field $value)*)
    };
}
pub use crate::__aws_guardduty_Member as Member;
impl crate::template::ToResource for Member_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GuardDuty"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Member"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.detector_id {
            properties.insert(
                "DetectorId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.disable_email_notification {
            properties.insert(
                "DisableEmailNotification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Email".to_string(),
            crate::value::ToValue::to_value(&self.email),
        );
        if let Some(ref value) = self.member_id {
            properties.insert(
                "MemberId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.message {
            properties.insert(
                "Message".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-publishingdestination.html
pub struct PublishingDestination_ {
    pub destination_properties: super::guardduty::publishingdestination::CFNDestinationProperties_,
    pub destination_type: crate::value::ExpString,
    pub detector_id: crate::value::ExpString,
    pub tags: Option<Vec<super::guardduty::publishingdestination::TagItem_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_guardduty_PublishingDestination {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GuardDuty::PublishingDestination"
        $($field $value)*)
    };
}
pub use crate::__aws_guardduty_PublishingDestination as PublishingDestination;
impl crate::template::ToResource for PublishingDestination_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GuardDuty"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PublishingDestination"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DestinationProperties".to_string(),
            crate::value::ToValue::to_value(&self.destination_properties),
        );
        properties.insert(
            "DestinationType".to_string(),
            crate::value::ToValue::to_value(&self.destination_type),
        );
        properties.insert(
            "DetectorId".to_string(),
            crate::value::ToValue::to_value(&self.detector_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-threatentityset.html
pub struct ThreatEntitySet_ {
    pub activate: Option<crate::value::ExpBool>,
    pub detector_id: Option<crate::value::ExpString>,
    pub expected_bucket_owner: Option<crate::value::ExpString>,
    pub format: crate::value::ExpString,
    pub location: crate::value::ExpString,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<super::guardduty::threatentityset::TagItem_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_guardduty_ThreatEntitySet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GuardDuty::ThreatEntitySet"
        $($field $value)*)
    };
}
pub use crate::__aws_guardduty_ThreatEntitySet as ThreatEntitySet;
impl crate::template::ToResource for ThreatEntitySet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GuardDuty"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ThreatEntitySet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.activate {
            properties.insert(
                "Activate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.detector_id {
            properties.insert(
                "DetectorId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.expected_bucket_owner {
            properties.insert(
                "ExpectedBucketOwner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Format".to_string(),
            crate::value::ToValue::to_value(&self.format),
        );
        properties.insert(
            "Location".to_string(),
            crate::value::ToValue::to_value(&self.location),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-threatintelset.html
pub struct ThreatIntelSet_ {
    pub activate: Option<crate::value::ExpBool>,
    pub detector_id: Option<crate::value::ExpString>,
    pub expected_bucket_owner: Option<crate::value::ExpString>,
    pub format: crate::value::ExpString,
    pub location: crate::value::ExpString,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<super::guardduty::threatintelset::TagItem_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_guardduty_ThreatIntelSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GuardDuty::ThreatIntelSet"
        $($field $value)*)
    };
}
pub use crate::__aws_guardduty_ThreatIntelSet as ThreatIntelSet;
impl crate::template::ToResource for ThreatIntelSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GuardDuty"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ThreatIntelSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.activate {
            properties.insert(
                "Activate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.detector_id {
            properties.insert(
                "DetectorId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.expected_bucket_owner {
            properties.insert(
                "ExpectedBucketOwner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Format".to_string(),
            crate::value::ToValue::to_value(&self.format),
        );
        properties.insert(
            "Location".to_string(),
            crate::value::ToValue::to_value(&self.location),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-guardduty-trustedentityset.html
pub struct TrustedEntitySet_ {
    pub activate: Option<crate::value::ExpBool>,
    pub detector_id: Option<crate::value::ExpString>,
    pub expected_bucket_owner: Option<crate::value::ExpString>,
    pub format: crate::value::ExpString,
    pub location: crate::value::ExpString,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<super::guardduty::trustedentityset::TagItem_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_guardduty_TrustedEntitySet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GuardDuty::TrustedEntitySet"
        $($field $value)*)
    };
}
pub use crate::__aws_guardduty_TrustedEntitySet as TrustedEntitySet;
impl crate::template::ToResource for TrustedEntitySet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GuardDuty"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TrustedEntitySet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.activate {
            properties.insert(
                "Activate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.detector_id {
            properties.insert(
                "DetectorId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.expected_bucket_owner {
            properties.insert(
                "ExpectedBucketOwner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Format".to_string(),
            crate::value::ToValue::to_value(&self.format),
        );
        properties.insert(
            "Location".to_string(),
            crate::value::ToValue::to_value(&self.location),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
