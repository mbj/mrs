pub mod contact {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-channeltargetinfo.html>
    pub struct ChannelTargetInfo_ {
        pub channel_id: crate::value::ExpString,
        pub retry_interval_in_minutes: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmcontacts_Contact_ChannelTargetInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMContacts::Contact.ChannelTargetInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmcontacts_Contact_ChannelTargetInfo as ChannelTargetInfo;
    impl crate::value::ToValue for ChannelTargetInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ChannelId".to_string(),
                crate::value::ToValue::to_value(&self.channel_id),
            );
            properties.insert(
                "RetryIntervalInMinutes".to_string(),
                crate::value::ToValue::to_value(&self.retry_interval_in_minutes),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-contacttargetinfo.html>
    pub struct ContactTargetInfo_ {
        pub contact_id: crate::value::ExpString,
        pub is_essential: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmcontacts_Contact_ContactTargetInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMContacts::Contact.ContactTargetInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmcontacts_Contact_ContactTargetInfo as ContactTargetInfo;
    impl crate::value::ToValue for ContactTargetInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContactId".to_string(),
                crate::value::ToValue::to_value(&self.contact_id),
            );
            properties.insert(
                "IsEssential".to_string(),
                crate::value::ToValue::to_value(&self.is_essential),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-stage.html>
    pub struct Stage_ {
        pub duration_in_minutes: Option<i32>,
        pub rotation_ids: Option<Vec<crate::value::ExpString>>,
        pub targets: Option<Vec<Targets_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmcontacts_Contact_Stage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMContacts::Contact.Stage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmcontacts_Contact_Stage as Stage;
    impl crate::value::ToValue for Stage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration_in_minutes {
                properties.insert(
                    "DurationInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rotation_ids {
                properties.insert(
                    "RotationIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.targets {
                properties.insert(
                    "Targets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-targets.html>
    pub struct Targets_ {
        pub channel_target_info: Option<Box<ChannelTargetInfo_>>,
        pub contact_target_info: Option<Box<ContactTargetInfo_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmcontacts_Contact_Targets {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMContacts::Contact.Targets"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmcontacts_Contact_Targets as Targets;
    impl crate::value::ToValue for Targets_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.channel_target_info {
                properties.insert(
                    "ChannelTargetInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.contact_target_info {
                properties.insert(
                    "ContactTargetInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod plan {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-channeltargetinfo.html>
    pub struct ChannelTargetInfo_ {
        pub channel_id: crate::value::ExpString,
        pub retry_interval_in_minutes: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmcontacts_Plan_ChannelTargetInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMContacts::Plan.ChannelTargetInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmcontacts_Plan_ChannelTargetInfo as ChannelTargetInfo;
    impl crate::value::ToValue for ChannelTargetInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ChannelId".to_string(),
                crate::value::ToValue::to_value(&self.channel_id),
            );
            properties.insert(
                "RetryIntervalInMinutes".to_string(),
                crate::value::ToValue::to_value(&self.retry_interval_in_minutes),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-contacttargetinfo.html>
    pub struct ContactTargetInfo_ {
        pub contact_id: crate::value::ExpString,
        pub is_essential: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmcontacts_Plan_ContactTargetInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMContacts::Plan.ContactTargetInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmcontacts_Plan_ContactTargetInfo as ContactTargetInfo;
    impl crate::value::ToValue for ContactTargetInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContactId".to_string(),
                crate::value::ToValue::to_value(&self.contact_id),
            );
            properties.insert(
                "IsEssential".to_string(),
                crate::value::ToValue::to_value(&self.is_essential),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-stage.html>
    pub struct Stage_ {
        pub duration_in_minutes: i32,
        pub targets: Option<Vec<Targets_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmcontacts_Plan_Stage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMContacts::Plan.Stage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmcontacts_Plan_Stage as Stage;
    impl crate::value::ToValue for Stage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DurationInMinutes".to_string(),
                crate::value::ToValue::to_value(&self.duration_in_minutes),
            );
            if let Some(ref value) = self.targets {
                properties.insert(
                    "Targets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-targets.html>
    pub struct Targets_ {
        pub channel_target_info: Option<Box<ChannelTargetInfo_>>,
        pub contact_target_info: Option<Box<ContactTargetInfo_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmcontacts_Plan_Targets {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMContacts::Plan.Targets"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmcontacts_Plan_Targets as Targets;
    impl crate::value::ToValue for Targets_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.channel_target_info {
                properties.insert(
                    "ChannelTargetInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.contact_target_info {
                properties.insert(
                    "ContactTargetInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod rotation {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-coveragetime.html>
    pub struct CoverageTime_ {
        pub end_time: crate::value::ExpString,
        pub start_time: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmcontacts_Rotation_CoverageTime {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMContacts::Rotation.CoverageTime"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmcontacts_Rotation_CoverageTime as CoverageTime;
    impl crate::value::ToValue for CoverageTime_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EndTime".to_string(),
                crate::value::ToValue::to_value(&self.end_time),
            );
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(&self.start_time),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-monthlysetting.html>
    pub struct MonthlySetting_ {
        pub day_of_month: i32,
        pub hand_off_time: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmcontacts_Rotation_MonthlySetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMContacts::Rotation.MonthlySetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmcontacts_Rotation_MonthlySetting as MonthlySetting;
    impl crate::value::ToValue for MonthlySetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DayOfMonth".to_string(),
                crate::value::ToValue::to_value(&self.day_of_month),
            );
            properties.insert(
                "HandOffTime".to_string(),
                crate::value::ToValue::to_value(&self.hand_off_time),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-recurrencesettings.html>
    pub struct RecurrenceSettings_ {
        pub daily_settings: Option<Vec<crate::value::ExpString>>,
        pub monthly_settings: Option<Vec<MonthlySetting_>>,
        pub number_of_on_calls: i32,
        pub recurrence_multiplier: i32,
        pub shift_coverages: Option<Vec<ShiftCoverage_>>,
        pub weekly_settings: Option<Vec<WeeklySetting_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmcontacts_Rotation_RecurrenceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMContacts::Rotation.RecurrenceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmcontacts_Rotation_RecurrenceSettings as RecurrenceSettings;
    impl crate::value::ToValue for RecurrenceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.daily_settings {
                properties.insert(
                    "DailySettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.monthly_settings {
                properties.insert(
                    "MonthlySettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "NumberOfOnCalls".to_string(),
                crate::value::ToValue::to_value(&self.number_of_on_calls),
            );
            properties.insert(
                "RecurrenceMultiplier".to_string(),
                crate::value::ToValue::to_value(&self.recurrence_multiplier),
            );
            if let Some(ref value) = self.shift_coverages {
                properties.insert(
                    "ShiftCoverages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weekly_settings {
                properties.insert(
                    "WeeklySettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-shiftcoverage.html>
    pub struct ShiftCoverage_ {
        pub coverage_times: Vec<CoverageTime_>,
        pub day_of_week: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmcontacts_Rotation_ShiftCoverage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMContacts::Rotation.ShiftCoverage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmcontacts_Rotation_ShiftCoverage as ShiftCoverage;
    impl crate::value::ToValue for ShiftCoverage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CoverageTimes".to_string(),
                crate::value::ToValue::to_value(&self.coverage_times),
            );
            properties.insert(
                "DayOfWeek".to_string(),
                crate::value::ToValue::to_value(&self.day_of_week),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-weeklysetting.html>
    pub struct WeeklySetting_ {
        pub day_of_week: crate::value::ExpString,
        pub hand_off_time: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmcontacts_Rotation_WeeklySetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMContacts::Rotation.WeeklySetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmcontacts_Rotation_WeeklySetting as WeeklySetting;
    impl crate::value::ToValue for WeeklySetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DayOfWeek".to_string(),
                crate::value::ToValue::to_value(&self.day_of_week),
            );
            properties.insert(
                "HandOffTime".to_string(),
                crate::value::ToValue::to_value(&self.hand_off_time),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-contact.html>
pub struct Contact_ {
    pub alias: crate::value::ExpString,
    pub display_name: crate::value::ExpString,
    pub plan: Option<Vec<super::ssmcontacts::contact::Stage_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssmcontacts_Contact {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSMContacts::Contact"
        $($field $value)*)
    };
}
pub use crate::__aws_ssmcontacts_Contact as Contact;
impl crate::template::ToResource for Contact_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSMContacts"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Contact"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Alias".to_string(),
            crate::value::ToValue::to_value(&self.alias),
        );
        properties.insert(
            "DisplayName".to_string(),
            crate::value::ToValue::to_value(&self.display_name),
        );
        if let Some(ref value) = self.plan {
            properties.insert("Plan".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-contactchannel.html>
pub struct ContactChannel_ {
    pub channel_address: crate::value::ExpString,
    pub channel_name: crate::value::ExpString,
    pub channel_type: crate::value::ExpString,
    pub contact_id: crate::value::ExpString,
    pub defer_activation: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssmcontacts_ContactChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSMContacts::ContactChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_ssmcontacts_ContactChannel as ContactChannel;
impl crate::template::ToResource for ContactChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSMContacts"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ContactChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ChannelAddress".to_string(),
            crate::value::ToValue::to_value(&self.channel_address),
        );
        properties.insert(
            "ChannelName".to_string(),
            crate::value::ToValue::to_value(&self.channel_name),
        );
        properties.insert(
            "ChannelType".to_string(),
            crate::value::ToValue::to_value(&self.channel_type),
        );
        properties.insert(
            "ContactId".to_string(),
            crate::value::ToValue::to_value(&self.contact_id),
        );
        if let Some(ref value) = self.defer_activation {
            properties.insert(
                "DeferActivation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-plan.html>
pub struct Plan_ {
    pub contact_id: crate::value::ExpString,
    pub rotation_ids: Option<Vec<crate::value::ExpString>>,
    pub stages: Option<Vec<super::ssmcontacts::plan::Stage_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssmcontacts_Plan {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSMContacts::Plan"
        $($field $value)*)
    };
}
pub use crate::__aws_ssmcontacts_Plan as Plan;
impl crate::template::ToResource for Plan_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSMContacts"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Plan"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ContactId".to_string(),
            crate::value::ToValue::to_value(&self.contact_id),
        );
        if let Some(ref value) = self.rotation_ids {
            properties.insert(
                "RotationIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.stages {
            properties.insert("Stages".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-rotation.html>
pub struct Rotation_ {
    pub contact_ids: Vec<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub recurrence: super::ssmcontacts::rotation::RecurrenceSettings_,
    pub start_time: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub time_zone_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssmcontacts_Rotation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSMContacts::Rotation"
        $($field $value)*)
    };
}
pub use crate::__aws_ssmcontacts_Rotation as Rotation;
impl crate::template::ToResource for Rotation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSMContacts"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Rotation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ContactIds".to_string(),
            crate::value::ToValue::to_value(&self.contact_ids),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Recurrence".to_string(),
            crate::value::ToValue::to_value(&self.recurrence),
        );
        properties.insert(
            "StartTime".to_string(),
            crate::value::ToValue::to_value(&self.start_time),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TimeZoneId".to_string(),
            crate::value::ToValue::to_value(&self.time_zone_id),
        );
        properties
    }
}
