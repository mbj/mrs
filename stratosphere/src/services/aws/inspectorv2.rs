pub mod cisscanconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-cisscanconfiguration-cistargets.html
    pub struct CisTargets_ {
        pub account_ids: Vec<crate::value::ExpString>,
        pub target_resource_tags: serde_json::Value,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CisScanConfiguration_CisTargets {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CisScanConfiguration.CisTargets"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CisScanConfiguration_CisTargets as CisTargets;
    impl crate::value::ToValue for CisTargets_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccountIds".to_string(),
                crate::value::ToValue::to_value(&self.account_ids),
            );
            properties.insert(
                "TargetResourceTags".to_string(),
                crate::value::ToValue::to_value(&self.target_resource_tags),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-cisscanconfiguration-dailyschedule.html
    pub struct DailySchedule_ {
        pub start_time: Box<Time_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CisScanConfiguration_DailySchedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CisScanConfiguration.DailySchedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CisScanConfiguration_DailySchedule as DailySchedule;
    impl crate::value::ToValue for DailySchedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(&self.start_time),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-cisscanconfiguration-monthlyschedule.html
    pub struct MonthlySchedule_ {
        pub day: crate::value::ExpString,
        pub start_time: Box<Time_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CisScanConfiguration_MonthlySchedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CisScanConfiguration.MonthlySchedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CisScanConfiguration_MonthlySchedule as MonthlySchedule;
    impl crate::value::ToValue for MonthlySchedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Day".to_string(),
                crate::value::ToValue::to_value(&self.day),
            );
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(&self.start_time),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-cisscanconfiguration-schedule.html
    pub struct Schedule_ {
        pub daily: Option<Box<DailySchedule_>>,
        pub monthly: Option<Box<MonthlySchedule_>>,
        pub one_time: Option<serde_json::Value>,
        pub weekly: Option<Box<WeeklySchedule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CisScanConfiguration_Schedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CisScanConfiguration.Schedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CisScanConfiguration_Schedule as Schedule;
    impl crate::value::ToValue for Schedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.daily {
                properties.insert("Daily".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.monthly {
                properties.insert(
                    "Monthly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.one_time {
                properties.insert(
                    "OneTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weekly {
                properties.insert("Weekly".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-cisscanconfiguration-time.html
    pub struct Time_ {
        pub time_of_day: crate::value::ExpString,
        pub time_zone: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CisScanConfiguration_Time {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CisScanConfiguration.Time"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CisScanConfiguration_Time as Time;
    impl crate::value::ToValue for Time_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TimeOfDay".to_string(),
                crate::value::ToValue::to_value(&self.time_of_day),
            );
            properties.insert(
                "TimeZone".to_string(),
                crate::value::ToValue::to_value(&self.time_zone),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-cisscanconfiguration-weeklyschedule.html
    pub struct WeeklySchedule_ {
        pub days: Vec<crate::value::ExpString>,
        pub start_time: Box<Time_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CisScanConfiguration_WeeklySchedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CisScanConfiguration.WeeklySchedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CisScanConfiguration_WeeklySchedule as WeeklySchedule;
    impl crate::value::ToValue for WeeklySchedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Days".to_string(),
                crate::value::ToValue::to_value(&self.days),
            );
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(&self.start_time),
            );
            properties.into()
        }
    }
}
pub mod codesecurityintegration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-codesecurityintegration-createdetails.html
    pub struct CreateDetails_ {
        pub gitlab_self_managed: Box<CreateGitLabSelfManagedIntegrationDetail_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CodeSecurityIntegration_CreateDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CodeSecurityIntegration.CreateDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CodeSecurityIntegration_CreateDetails as CreateDetails;
    impl crate::value::ToValue for CreateDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "gitlabSelfManaged".to_string(),
                crate::value::ToValue::to_value(&self.gitlab_self_managed),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-codesecurityintegration-creategitlabselfmanagedintegrationdetail.html
    pub struct CreateGitLabSelfManagedIntegrationDetail_ {
        pub access_token: crate::value::ExpString,
        pub instance_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CodeSecurityIntegration_CreateGitLabSelfManagedIntegrationDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CodeSecurityIntegration.CreateGitLabSelfManagedIntegrationDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CodeSecurityIntegration_CreateGitLabSelfManagedIntegrationDetail as CreateGitLabSelfManagedIntegrationDetail;
    impl crate::value::ToValue for CreateGitLabSelfManagedIntegrationDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "accessToken".to_string(),
                crate::value::ToValue::to_value(&self.access_token),
            );
            properties.insert(
                "instanceUrl".to_string(),
                crate::value::ToValue::to_value(&self.instance_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-codesecurityintegration-updatedetails.html
    pub struct UpdateDetails_ {
        pub github: Option<Box<UpdateGitHubIntegrationDetail_>>,
        pub gitlab_self_managed: Option<Box<UpdateGitLabSelfManagedIntegrationDetail_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CodeSecurityIntegration_UpdateDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CodeSecurityIntegration.UpdateDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CodeSecurityIntegration_UpdateDetails as UpdateDetails;
    impl crate::value::ToValue for UpdateDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.github {
                properties.insert("github".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.gitlab_self_managed {
                properties.insert(
                    "gitlabSelfManaged".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-codesecurityintegration-updategithubintegrationdetail.html
    pub struct UpdateGitHubIntegrationDetail_ {
        pub code: crate::value::ExpString,
        pub installation_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CodeSecurityIntegration_UpdateGitHubIntegrationDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CodeSecurityIntegration.UpdateGitHubIntegrationDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CodeSecurityIntegration_UpdateGitHubIntegrationDetail as UpdateGitHubIntegrationDetail;
    impl crate::value::ToValue for UpdateGitHubIntegrationDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "code".to_string(),
                crate::value::ToValue::to_value(&self.code),
            );
            properties.insert(
                "installationId".to_string(),
                crate::value::ToValue::to_value(&self.installation_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-codesecurityintegration-updategitlabselfmanagedintegrationdetail.html
    pub struct UpdateGitLabSelfManagedIntegrationDetail_ {
        pub auth_code: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CodeSecurityIntegration_UpdateGitLabSelfManagedIntegrationDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CodeSecurityIntegration.UpdateGitLabSelfManagedIntegrationDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CodeSecurityIntegration_UpdateGitLabSelfManagedIntegrationDetail as UpdateGitLabSelfManagedIntegrationDetail;
    impl crate::value::ToValue for UpdateGitLabSelfManagedIntegrationDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "authCode".to_string(),
                crate::value::ToValue::to_value(&self.auth_code),
            );
            properties.into()
        }
    }
}
pub mod codesecurityscanconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-codesecurityscanconfiguration-codesecurityscanconfiguration.html
    pub struct CodeSecurityScanConfiguration_ {
        pub continuous_integration_scan_configuration:
            Option<Box<ContinuousIntegrationScanConfiguration_>>,
        pub periodic_scan_configuration: Option<Box<PeriodicScanConfiguration_>>,
        pub rule_set_categories: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CodeSecurityScanConfiguration_CodeSecurityScanConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CodeSecurityScanConfiguration.CodeSecurityScanConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CodeSecurityScanConfiguration_CodeSecurityScanConfiguration as CodeSecurityScanConfiguration;
    impl crate::value::ToValue for CodeSecurityScanConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.continuous_integration_scan_configuration {
                properties.insert(
                    "continuousIntegrationScanConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.periodic_scan_configuration {
                properties.insert(
                    "periodicScanConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ruleSetCategories".to_string(),
                crate::value::ToValue::to_value(&self.rule_set_categories),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-codesecurityscanconfiguration-continuousintegrationscanconfiguration.html
    pub struct ContinuousIntegrationScanConfiguration_ {
        pub supported_events: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CodeSecurityScanConfiguration_ContinuousIntegrationScanConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CodeSecurityScanConfiguration.ContinuousIntegrationScanConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CodeSecurityScanConfiguration_ContinuousIntegrationScanConfiguration as ContinuousIntegrationScanConfiguration;
    impl crate::value::ToValue for ContinuousIntegrationScanConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "supportedEvents".to_string(),
                crate::value::ToValue::to_value(&self.supported_events),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-codesecurityscanconfiguration-periodicscanconfiguration.html
    pub struct PeriodicScanConfiguration_ {
        pub frequency: Option<crate::value::ExpString>,
        pub frequency_expression: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CodeSecurityScanConfiguration_PeriodicScanConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CodeSecurityScanConfiguration.PeriodicScanConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CodeSecurityScanConfiguration_PeriodicScanConfiguration as PeriodicScanConfiguration;
    impl crate::value::ToValue for PeriodicScanConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.frequency {
                properties.insert(
                    "frequency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.frequency_expression {
                properties.insert(
                    "frequencyExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-codesecurityscanconfiguration-scopesettings.html
    pub struct ScopeSettings_ {
        pub project_selection_scope: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_CodeSecurityScanConfiguration_ScopeSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::CodeSecurityScanConfiguration.ScopeSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_CodeSecurityScanConfiguration_ScopeSettings as ScopeSettings;
    impl crate::value::ToValue for ScopeSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.project_selection_scope {
                properties.insert(
                    "projectSelectionScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod filter {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-datefilter.html
    pub struct DateFilter_ {
        pub end_inclusive: Option<i32>,
        pub start_inclusive: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_Filter_DateFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::Filter.DateFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_Filter_DateFilter as DateFilter;
    impl crate::value::ToValue for DateFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.end_inclusive {
                properties.insert(
                    "EndInclusive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_inclusive {
                properties.insert(
                    "StartInclusive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html
    pub struct FilterCriteria_ {
        pub aws_account_id: Option<Vec<StringFilter_>>,
        pub code_vulnerability_detector_name: Option<Vec<StringFilter_>>,
        pub code_vulnerability_detector_tags: Option<Vec<StringFilter_>>,
        pub code_vulnerability_file_path: Option<Vec<StringFilter_>>,
        pub component_id: Option<Vec<StringFilter_>>,
        pub component_type: Option<Vec<StringFilter_>>,
        pub ec2_instance_image_id: Option<Vec<StringFilter_>>,
        pub ec2_instance_subnet_id: Option<Vec<StringFilter_>>,
        pub ec2_instance_vpc_id: Option<Vec<StringFilter_>>,
        pub ecr_image_architecture: Option<Vec<StringFilter_>>,
        pub ecr_image_hash: Option<Vec<StringFilter_>>,
        pub ecr_image_pushed_at: Option<Vec<DateFilter_>>,
        pub ecr_image_registry: Option<Vec<StringFilter_>>,
        pub ecr_image_repository_name: Option<Vec<StringFilter_>>,
        pub ecr_image_tags: Option<Vec<StringFilter_>>,
        pub epss_score: Option<Vec<NumberFilter_>>,
        pub exploit_available: Option<Vec<StringFilter_>>,
        pub finding_arn: Option<Vec<StringFilter_>>,
        pub finding_status: Option<Vec<StringFilter_>>,
        pub finding_type: Option<Vec<StringFilter_>>,
        pub first_observed_at: Option<Vec<DateFilter_>>,
        pub fix_available: Option<Vec<StringFilter_>>,
        pub inspector_score: Option<Vec<NumberFilter_>>,
        pub lambda_function_execution_role_arn: Option<Vec<StringFilter_>>,
        pub lambda_function_last_modified_at: Option<Vec<DateFilter_>>,
        pub lambda_function_layers: Option<Vec<StringFilter_>>,
        pub lambda_function_name: Option<Vec<StringFilter_>>,
        pub lambda_function_runtime: Option<Vec<StringFilter_>>,
        pub last_observed_at: Option<Vec<DateFilter_>>,
        pub network_protocol: Option<Vec<StringFilter_>>,
        pub port_range: Option<Vec<PortRangeFilter_>>,
        pub related_vulnerabilities: Option<Vec<StringFilter_>>,
        pub resource_id: Option<Vec<StringFilter_>>,
        pub resource_tags: Option<Vec<MapFilter_>>,
        pub resource_type: Option<Vec<StringFilter_>>,
        pub severity: Option<Vec<StringFilter_>>,
        pub title: Option<Vec<StringFilter_>>,
        pub updated_at: Option<Vec<DateFilter_>>,
        pub vendor_severity: Option<Vec<StringFilter_>>,
        pub vulnerability_id: Option<Vec<StringFilter_>>,
        pub vulnerability_source: Option<Vec<StringFilter_>>,
        pub vulnerable_packages: Option<Vec<PackageFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_Filter_FilterCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::Filter.FilterCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_Filter_FilterCriteria as FilterCriteria;
    impl crate::value::ToValue for FilterCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_account_id {
                properties.insert(
                    "AwsAccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.code_vulnerability_detector_name {
                properties.insert(
                    "CodeVulnerabilityDetectorName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.code_vulnerability_detector_tags {
                properties.insert(
                    "CodeVulnerabilityDetectorTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.code_vulnerability_file_path {
                properties.insert(
                    "CodeVulnerabilityFilePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component_id {
                properties.insert(
                    "ComponentId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component_type {
                properties.insert(
                    "ComponentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ec2_instance_image_id {
                properties.insert(
                    "Ec2InstanceImageId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ec2_instance_subnet_id {
                properties.insert(
                    "Ec2InstanceSubnetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ec2_instance_vpc_id {
                properties.insert(
                    "Ec2InstanceVpcId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecr_image_architecture {
                properties.insert(
                    "EcrImageArchitecture".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecr_image_hash {
                properties.insert(
                    "EcrImageHash".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecr_image_pushed_at {
                properties.insert(
                    "EcrImagePushedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecr_image_registry {
                properties.insert(
                    "EcrImageRegistry".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecr_image_repository_name {
                properties.insert(
                    "EcrImageRepositoryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecr_image_tags {
                properties.insert(
                    "EcrImageTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.epss_score {
                properties.insert(
                    "EpssScore".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exploit_available {
                properties.insert(
                    "ExploitAvailable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.finding_arn {
                properties.insert(
                    "FindingArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.finding_status {
                properties.insert(
                    "FindingStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.finding_type {
                properties.insert(
                    "FindingType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.first_observed_at {
                properties.insert(
                    "FirstObservedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fix_available {
                properties.insert(
                    "FixAvailable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inspector_score {
                properties.insert(
                    "InspectorScore".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_function_execution_role_arn {
                properties.insert(
                    "LambdaFunctionExecutionRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_function_last_modified_at {
                properties.insert(
                    "LambdaFunctionLastModifiedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_function_layers {
                properties.insert(
                    "LambdaFunctionLayers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_function_name {
                properties.insert(
                    "LambdaFunctionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_function_runtime {
                properties.insert(
                    "LambdaFunctionRuntime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.last_observed_at {
                properties.insert(
                    "LastObservedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_protocol {
                properties.insert(
                    "NetworkProtocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port_range {
                properties.insert(
                    "PortRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.related_vulnerabilities {
                properties.insert(
                    "RelatedVulnerabilities".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_id {
                properties.insert(
                    "ResourceId".to_string(),
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
            if let Some(ref value) = self.severity {
                properties.insert(
                    "Severity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.title {
                properties.insert("Title".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.updated_at {
                properties.insert(
                    "UpdatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vendor_severity {
                properties.insert(
                    "VendorSeverity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vulnerability_id {
                properties.insert(
                    "VulnerabilityId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vulnerability_source {
                properties.insert(
                    "VulnerabilitySource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vulnerable_packages {
                properties.insert(
                    "VulnerablePackages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-mapfilter.html
    pub struct MapFilter_ {
        pub comparison: crate::value::ExpString,
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_Filter_MapFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::Filter.MapFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_Filter_MapFilter as MapFilter;
    impl crate::value::ToValue for MapFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Comparison".to_string(),
                crate::value::ToValue::to_value(&self.comparison),
            );
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-numberfilter.html
    pub struct NumberFilter_ {
        pub lower_inclusive: Option<f64>,
        pub upper_inclusive: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_Filter_NumberFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::Filter.NumberFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_Filter_NumberFilter as NumberFilter;
    impl crate::value::ToValue for NumberFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.lower_inclusive {
                properties.insert(
                    "LowerInclusive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.upper_inclusive {
                properties.insert(
                    "UpperInclusive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-packagefilter.html
    pub struct PackageFilter_ {
        pub architecture: Option<Box<StringFilter_>>,
        pub epoch: Option<Box<NumberFilter_>>,
        pub file_path: Option<Box<StringFilter_>>,
        pub name: Option<Box<StringFilter_>>,
        pub release: Option<Box<StringFilter_>>,
        pub source_lambda_layer_arn: Option<Box<StringFilter_>>,
        pub source_layer_hash: Option<Box<StringFilter_>>,
        pub version: Option<Box<StringFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_Filter_PackageFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::Filter.PackageFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_Filter_PackageFilter as PackageFilter;
    impl crate::value::ToValue for PackageFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.architecture {
                properties.insert(
                    "Architecture".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.epoch {
                properties.insert("Epoch".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.file_path {
                properties.insert(
                    "FilePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.release {
                properties.insert(
                    "Release".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_lambda_layer_arn {
                properties.insert(
                    "SourceLambdaLayerArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_layer_hash {
                properties.insert(
                    "SourceLayerHash".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-portrangefilter.html
    pub struct PortRangeFilter_ {
        pub begin_inclusive: Option<i32>,
        pub end_inclusive: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_Filter_PortRangeFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::Filter.PortRangeFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_Filter_PortRangeFilter as PortRangeFilter;
    impl crate::value::ToValue for PortRangeFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.begin_inclusive {
                properties.insert(
                    "BeginInclusive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.end_inclusive {
                properties.insert(
                    "EndInclusive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-stringfilter.html
    pub struct StringFilter_ {
        pub comparison: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_inspectorv2_Filter_StringFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::InspectorV2::Filter.StringFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_inspectorv2_Filter_StringFilter as StringFilter;
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspectorv2-cisscanconfiguration.html
pub struct CisScanConfiguration_ {
    pub scan_name: crate::value::ExpString,
    pub schedule: super::inspectorv2::cisscanconfiguration::Schedule_,
    pub security_level: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub targets: super::inspectorv2::cisscanconfiguration::CisTargets_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_inspectorv2_CisScanConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::InspectorV2::CisScanConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_inspectorv2_CisScanConfiguration as CisScanConfiguration;
impl crate::template::ToResource for CisScanConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("InspectorV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CisScanConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ScanName".to_string(),
            crate::value::ToValue::to_value(&self.scan_name),
        );
        properties.insert(
            "Schedule".to_string(),
            crate::value::ToValue::to_value(&self.schedule),
        );
        properties.insert(
            "SecurityLevel".to_string(),
            crate::value::ToValue::to_value(&self.security_level),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Targets".to_string(),
            crate::value::ToValue::to_value(&self.targets),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspectorv2-codesecurityintegration.html
pub struct CodeSecurityIntegration_ {
    pub create_integration_details:
        Option<super::inspectorv2::codesecurityintegration::CreateDetails_>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub r#type: Option<crate::value::ExpString>,
    pub update_integration_details:
        Option<super::inspectorv2::codesecurityintegration::UpdateDetails_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_inspectorv2_CodeSecurityIntegration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::InspectorV2::CodeSecurityIntegration"
        $($field $value)*)
    };
}
pub use crate::__aws_inspectorv2_CodeSecurityIntegration as CodeSecurityIntegration;
impl crate::template::ToResource for CodeSecurityIntegration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("InspectorV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CodeSecurityIntegration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.create_integration_details {
            properties.insert(
                "CreateIntegrationDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.update_integration_details {
            properties.insert(
                "UpdateIntegrationDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspectorv2-codesecurityscanconfiguration.html
pub struct CodeSecurityScanConfiguration_ {
    pub configuration:
        Option<super::inspectorv2::codesecurityscanconfiguration::CodeSecurityScanConfiguration_>,
    pub level: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub scope_settings: Option<super::inspectorv2::codesecurityscanconfiguration::ScopeSettings_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_inspectorv2_CodeSecurityScanConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::InspectorV2::CodeSecurityScanConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_inspectorv2_CodeSecurityScanConfiguration as CodeSecurityScanConfiguration;
impl crate::template::ToResource for CodeSecurityScanConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("InspectorV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "CodeSecurityScanConfiguration",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.configuration {
            properties.insert(
                "Configuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.level {
            properties.insert("Level".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.scope_settings {
            properties.insert(
                "ScopeSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspectorv2-filter.html
pub struct Filter_ {
    pub description: Option<crate::value::ExpString>,
    pub filter_action: crate::value::ExpString,
    pub filter_criteria: super::inspectorv2::filter::FilterCriteria_,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_inspectorv2_Filter {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::InspectorV2::Filter"
        $($field $value)*)
    };
}
pub use crate::__aws_inspectorv2_Filter as Filter;
impl crate::template::ToResource for Filter_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("InspectorV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Filter"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FilterAction".to_string(),
            crate::value::ToValue::to_value(&self.filter_action),
        );
        properties.insert(
            "FilterCriteria".to_string(),
            crate::value::ToValue::to_value(&self.filter_criteria),
        );
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
