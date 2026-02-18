pub mod zonalautoshiftconfiguration {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arczonalshift-zonalautoshiftconfiguration-controlcondition.html>
    pub struct ControlCondition_ {
        pub alarm_identifier: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arczonalshift_ZonalAutoshiftConfiguration_ControlCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCZonalShift::ZonalAutoshiftConfiguration.ControlCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arczonalshift_ZonalAutoshiftConfiguration_ControlCondition as ControlCondition;
    impl crate::value::ToValue for ControlCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlarmIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.alarm_identifier),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arczonalshift-zonalautoshiftconfiguration-practicerunconfiguration.html>
    pub struct PracticeRunConfiguration_ {
        pub blocked_dates: Option<Vec<crate::value::ExpString>>,
        pub blocked_windows: Option<Vec<crate::value::ExpString>>,
        pub blocking_alarms: Option<Vec<ControlCondition_>>,
        pub outcome_alarms: Vec<ControlCondition_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arczonalshift_ZonalAutoshiftConfiguration_PracticeRunConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCZonalShift::ZonalAutoshiftConfiguration.PracticeRunConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arczonalshift_ZonalAutoshiftConfiguration_PracticeRunConfiguration as PracticeRunConfiguration;
    impl crate::value::ToValue for PracticeRunConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.blocked_dates {
                properties.insert(
                    "BlockedDates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.blocked_windows {
                properties.insert(
                    "BlockedWindows".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.blocking_alarms {
                properties.insert(
                    "BlockingAlarms".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OutcomeAlarms".to_string(),
                crate::value::ToValue::to_value(&self.outcome_alarms),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-arczonalshift-autoshiftobservernotificationstatus.html>
pub struct AutoshiftObserverNotificationStatus_ {
    pub status: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_arczonalshift_AutoshiftObserverNotificationStatus {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ARCZonalShift::AutoshiftObserverNotificationStatus"
        $($field $value)*)
    };
}
pub use crate::__aws_arczonalshift_AutoshiftObserverNotificationStatus as AutoshiftObserverNotificationStatus;
impl crate::template::ToResource for AutoshiftObserverNotificationStatus_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ARCZonalShift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "AutoshiftObserverNotificationStatus",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Status".to_string(),
            crate::value::ToValue::to_value(&self.status),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-arczonalshift-zonalautoshiftconfiguration.html>
pub struct ZonalAutoshiftConfiguration_ {
    pub practice_run_configuration:
        Option<super::arczonalshift::zonalautoshiftconfiguration::PracticeRunConfiguration_>,
    pub resource_identifier: crate::value::ExpString,
    pub zonal_autoshift_status: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_arczonalshift_ZonalAutoshiftConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ARCZonalShift::ZonalAutoshiftConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_arczonalshift_ZonalAutoshiftConfiguration as ZonalAutoshiftConfiguration;
impl crate::template::ToResource for ZonalAutoshiftConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ARCZonalShift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ZonalAutoshiftConfiguration",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.practice_run_configuration {
            properties.insert(
                "PracticeRunConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.resource_identifier),
        );
        if let Some(ref value) = self.zonal_autoshift_status {
            properties.insert(
                "ZonalAutoshiftStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
