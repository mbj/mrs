pub mod deviceprofile {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html
    pub struct LoRaWANDeviceProfile_ {
        pub class_b_timeout: Option<i32>,
        pub class_c_timeout: Option<i32>,
        pub factory_preset_freqs_list: Option<Vec<i32>>,
        pub mac_version: Option<crate::value::ExpString>,
        pub max_duty_cycle: Option<i32>,
        pub max_eirp: Option<i32>,
        pub ping_slot_dr: Option<i32>,
        pub ping_slot_freq: Option<i32>,
        pub ping_slot_period: Option<i32>,
        pub reg_params_revision: Option<crate::value::ExpString>,
        pub rf_region: Option<crate::value::ExpString>,
        pub rx_data_rate2: Option<i32>,
        pub rx_delay1: Option<i32>,
        pub rx_dr_offset1: Option<i32>,
        pub rx_freq2: Option<i32>,
        pub supports32_bit_f_cnt: Option<crate::value::ExpBool>,
        pub supports_class_b: Option<crate::value::ExpBool>,
        pub supports_class_c: Option<crate::value::ExpBool>,
        pub supports_join: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_DeviceProfile_LoRaWANDeviceProfile {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::DeviceProfile.LoRaWANDeviceProfile"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_DeviceProfile_LoRaWANDeviceProfile as LoRaWANDeviceProfile;
    impl crate::value::ToValue for LoRaWANDeviceProfile_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.class_b_timeout {
                properties.insert(
                    "ClassBTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.class_c_timeout {
                properties.insert(
                    "ClassCTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.factory_preset_freqs_list {
                properties.insert(
                    "FactoryPresetFreqsList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mac_version {
                properties.insert(
                    "MacVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_duty_cycle {
                properties.insert(
                    "MaxDutyCycle".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_eirp {
                properties.insert(
                    "MaxEirp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ping_slot_dr {
                properties.insert(
                    "PingSlotDr".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ping_slot_freq {
                properties.insert(
                    "PingSlotFreq".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ping_slot_period {
                properties.insert(
                    "PingSlotPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.reg_params_revision {
                properties.insert(
                    "RegParamsRevision".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rf_region {
                properties.insert(
                    "RfRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rx_data_rate2 {
                properties.insert(
                    "RxDataRate2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rx_delay1 {
                properties.insert(
                    "RxDelay1".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rx_dr_offset1 {
                properties.insert(
                    "RxDrOffset1".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rx_freq2 {
                properties.insert(
                    "RxFreq2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.supports32_bit_f_cnt {
                properties.insert(
                    "Supports32BitFCnt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.supports_class_b {
                properties.insert(
                    "SupportsClassB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.supports_class_c {
                properties.insert(
                    "SupportsClassC".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.supports_join {
                properties.insert(
                    "SupportsJoin".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod fuotatask {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-fuotatask-lorawan.html
    pub struct LoRaWAN_ {
        pub rf_region: crate::value::ExpString,
        pub start_time: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_FuotaTask_LoRaWAN {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::FuotaTask.LoRaWAN"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_FuotaTask_LoRaWAN as LoRaWAN;
    impl crate::value::ToValue for LoRaWAN_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RfRegion".to_string(),
                crate::value::ToValue::to_value(&self.rf_region),
            );
            if let Some(ref value) = self.start_time {
                properties.insert(
                    "StartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod multicastgroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-multicastgroup-lorawan.html
    pub struct LoRaWAN_ {
        pub dl_class: crate::value::ExpString,
        pub number_of_devices_in_group: Option<i32>,
        pub number_of_devices_requested: Option<i32>,
        pub rf_region: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_MulticastGroup_LoRaWAN {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::MulticastGroup.LoRaWAN"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_MulticastGroup_LoRaWAN as LoRaWAN;
    impl crate::value::ToValue for LoRaWAN_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DlClass".to_string(),
                crate::value::ToValue::to_value(&self.dl_class),
            );
            if let Some(ref value) = self.number_of_devices_in_group {
                properties.insert(
                    "NumberOfDevicesInGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_devices_requested {
                properties.insert(
                    "NumberOfDevicesRequested".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RfRegion".to_string(),
                crate::value::ToValue::to_value(&self.rf_region),
            );
            properties.into()
        }
    }
}
pub mod networkanalyzerconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-networkanalyzerconfiguration-tracecontent.html
    pub struct TraceContent_ {
        pub log_level: Option<crate::value::ExpString>,
        pub wireless_device_frame_info: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_NetworkAnalyzerConfiguration_TraceContent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::NetworkAnalyzerConfiguration.TraceContent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_NetworkAnalyzerConfiguration_TraceContent as TraceContent;
    impl crate::value::ToValue for TraceContent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_level {
                properties.insert(
                    "LogLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.wireless_device_frame_info {
                properties.insert(
                    "WirelessDeviceFrameInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod partneraccount {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-partneraccount-sidewalkaccountinfo.html
    pub struct SidewalkAccountInfo_ {
        pub app_server_private_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_PartnerAccount_SidewalkAccountInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::PartnerAccount.SidewalkAccountInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_PartnerAccount_SidewalkAccountInfo as SidewalkAccountInfo;
    impl crate::value::ToValue for SidewalkAccountInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppServerPrivateKey".to_string(),
                crate::value::ToValue::to_value(&self.app_server_private_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-partneraccount-sidewalkaccountinfowithfingerprint.html
    pub struct SidewalkAccountInfoWithFingerprint_ {
        pub amazon_id: Option<crate::value::ExpString>,
        pub arn: Option<crate::value::ExpString>,
        pub fingerprint: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_PartnerAccount_SidewalkAccountInfoWithFingerprint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::PartnerAccount.SidewalkAccountInfoWithFingerprint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_PartnerAccount_SidewalkAccountInfoWithFingerprint as SidewalkAccountInfoWithFingerprint;
    impl crate::value::ToValue for SidewalkAccountInfoWithFingerprint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.amazon_id {
                properties.insert(
                    "AmazonId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.fingerprint {
                properties.insert(
                    "Fingerprint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-partneraccount-sidewalkupdateaccount.html
    pub struct SidewalkUpdateAccount_ {
        pub app_server_private_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_PartnerAccount_SidewalkUpdateAccount {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::PartnerAccount.SidewalkUpdateAccount"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_PartnerAccount_SidewalkUpdateAccount as SidewalkUpdateAccount;
    impl crate::value::ToValue for SidewalkUpdateAccount_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_server_private_key {
                properties.insert(
                    "AppServerPrivateKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod serviceprofile {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html
    pub struct LoRaWANServiceProfile_ {
        pub add_gw_metadata: Option<crate::value::ExpBool>,
        pub channel_mask: Option<crate::value::ExpString>,
        pub dev_status_req_freq: Option<i32>,
        pub dl_bucket_size: Option<i32>,
        pub dl_rate: Option<i32>,
        pub dl_rate_policy: Option<crate::value::ExpString>,
        pub dr_max: Option<i32>,
        pub dr_min: Option<i32>,
        pub hr_allowed: Option<crate::value::ExpBool>,
        pub min_gw_diversity: Option<i32>,
        pub nwk_geo_loc: Option<crate::value::ExpBool>,
        pub pr_allowed: Option<crate::value::ExpBool>,
        pub ra_allowed: Option<crate::value::ExpBool>,
        pub report_dev_status_battery: Option<crate::value::ExpBool>,
        pub report_dev_status_margin: Option<crate::value::ExpBool>,
        pub target_per: Option<i32>,
        pub ul_bucket_size: Option<i32>,
        pub ul_rate: Option<i32>,
        pub ul_rate_policy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_ServiceProfile_LoRaWANServiceProfile {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::ServiceProfile.LoRaWANServiceProfile"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_ServiceProfile_LoRaWANServiceProfile as LoRaWANServiceProfile;
    impl crate::value::ToValue for LoRaWANServiceProfile_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.add_gw_metadata {
                properties.insert(
                    "AddGwMetadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.channel_mask {
                properties.insert(
                    "ChannelMask".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dev_status_req_freq {
                properties.insert(
                    "DevStatusReqFreq".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dl_bucket_size {
                properties.insert(
                    "DlBucketSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dl_rate {
                properties.insert("DlRate".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.dl_rate_policy {
                properties.insert(
                    "DlRatePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dr_max {
                properties.insert("DrMax".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.dr_min {
                properties.insert("DrMin".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.hr_allowed {
                properties.insert(
                    "HrAllowed".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_gw_diversity {
                properties.insert(
                    "MinGwDiversity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nwk_geo_loc {
                properties.insert(
                    "NwkGeoLoc".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pr_allowed {
                properties.insert(
                    "PrAllowed".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ra_allowed {
                properties.insert(
                    "RaAllowed".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.report_dev_status_battery {
                properties.insert(
                    "ReportDevStatusBattery".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.report_dev_status_margin {
                properties.insert(
                    "ReportDevStatusMargin".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_per {
                properties.insert(
                    "TargetPer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ul_bucket_size {
                properties.insert(
                    "UlBucketSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ul_rate {
                properties.insert("UlRate".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ul_rate_policy {
                properties.insert(
                    "UlRatePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod taskdefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawangatewayversion.html
    pub struct LoRaWANGatewayVersion_ {
        pub model: Option<crate::value::ExpString>,
        pub package_version: Option<crate::value::ExpString>,
        pub station: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_TaskDefinition_LoRaWANGatewayVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::TaskDefinition.LoRaWANGatewayVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_TaskDefinition_LoRaWANGatewayVersion as LoRaWANGatewayVersion;
    impl crate::value::ToValue for LoRaWANGatewayVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.model {
                properties.insert("Model".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.package_version {
                properties.insert(
                    "PackageVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.station {
                properties.insert(
                    "Station".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawanupdategatewaytaskcreate.html
    pub struct LoRaWANUpdateGatewayTaskCreate_ {
        pub current_version: Option<Box<LoRaWANGatewayVersion_>>,
        pub sig_key_crc: Option<i32>,
        pub update_signature: Option<crate::value::ExpString>,
        pub update_version: Option<Box<LoRaWANGatewayVersion_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_TaskDefinition_LoRaWANUpdateGatewayTaskCreate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::TaskDefinition.LoRaWANUpdateGatewayTaskCreate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_TaskDefinition_LoRaWANUpdateGatewayTaskCreate as LoRaWANUpdateGatewayTaskCreate;
    impl crate::value::ToValue for LoRaWANUpdateGatewayTaskCreate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.current_version {
                properties.insert(
                    "CurrentVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sig_key_crc {
                properties.insert(
                    "SigKeyCrc".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_signature {
                properties.insert(
                    "UpdateSignature".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_version {
                properties.insert(
                    "UpdateVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawanupdategatewaytaskentry.html
    pub struct LoRaWANUpdateGatewayTaskEntry_ {
        pub current_version: Option<Box<LoRaWANGatewayVersion_>>,
        pub update_version: Option<Box<LoRaWANGatewayVersion_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_TaskDefinition_LoRaWANUpdateGatewayTaskEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::TaskDefinition.LoRaWANUpdateGatewayTaskEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_TaskDefinition_LoRaWANUpdateGatewayTaskEntry as LoRaWANUpdateGatewayTaskEntry;
    impl crate::value::ToValue for LoRaWANUpdateGatewayTaskEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.current_version {
                properties.insert(
                    "CurrentVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_version {
                properties.insert(
                    "UpdateVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-updatewirelessgatewaytaskcreate.html
    pub struct UpdateWirelessGatewayTaskCreate_ {
        pub lo_ra_wan: Option<Box<LoRaWANUpdateGatewayTaskCreate_>>,
        pub update_data_role: Option<crate::value::ExpString>,
        pub update_data_source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_TaskDefinition_UpdateWirelessGatewayTaskCreate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::TaskDefinition.UpdateWirelessGatewayTaskCreate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_TaskDefinition_UpdateWirelessGatewayTaskCreate as UpdateWirelessGatewayTaskCreate;
    impl crate::value::ToValue for UpdateWirelessGatewayTaskCreate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.lo_ra_wan {
                properties.insert(
                    "LoRaWAN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_data_role {
                properties.insert(
                    "UpdateDataRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_data_source {
                properties.insert(
                    "UpdateDataSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod wirelessdevice {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-abpv10x.html
    pub struct AbpV10x_ {
        pub dev_addr: crate::value::ExpString,
        pub session_keys: Box<SessionKeysAbpV10x_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_WirelessDevice_AbpV10x {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::WirelessDevice.AbpV10x"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_WirelessDevice_AbpV10x as AbpV10x;
    impl crate::value::ToValue for AbpV10x_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DevAddr".to_string(),
                crate::value::ToValue::to_value(&self.dev_addr),
            );
            properties.insert(
                "SessionKeys".to_string(),
                crate::value::ToValue::to_value(&self.session_keys),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-abpv11.html
    pub struct AbpV11_ {
        pub dev_addr: crate::value::ExpString,
        pub session_keys: Box<SessionKeysAbpV11_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_WirelessDevice_AbpV11 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::WirelessDevice.AbpV11"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_WirelessDevice_AbpV11 as AbpV11;
    impl crate::value::ToValue for AbpV11_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DevAddr".to_string(),
                crate::value::ToValue::to_value(&self.dev_addr),
            );
            properties.insert(
                "SessionKeys".to_string(),
                crate::value::ToValue::to_value(&self.session_keys),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-application.html
    pub struct Application_ {
        pub destination_name: Option<crate::value::ExpString>,
        pub f_port: Option<i32>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_WirelessDevice_Application {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::WirelessDevice.Application"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_WirelessDevice_Application as Application;
    impl crate::value::ToValue for Application_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_name {
                properties.insert(
                    "DestinationName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.f_port {
                properties.insert("FPort".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-fports.html
    pub struct FPorts_ {
        pub applications: Option<Vec<Application_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_WirelessDevice_FPorts {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::WirelessDevice.FPorts"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_WirelessDevice_FPorts as FPorts;
    impl crate::value::ToValue for FPorts_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.applications {
                properties.insert(
                    "Applications".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-lorawandevice.html
    pub struct LoRaWANDevice_ {
        pub abp_v10x: Option<Box<AbpV10x_>>,
        pub abp_v11: Option<Box<AbpV11_>>,
        pub dev_eui: Option<crate::value::ExpString>,
        pub device_profile_id: Option<crate::value::ExpString>,
        pub f_ports: Option<Box<FPorts_>>,
        pub otaa_v10x: Option<Box<OtaaV10x_>>,
        pub otaa_v11: Option<Box<OtaaV11_>>,
        pub service_profile_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_WirelessDevice_LoRaWANDevice {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::WirelessDevice.LoRaWANDevice"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_WirelessDevice_LoRaWANDevice as LoRaWANDevice;
    impl crate::value::ToValue for LoRaWANDevice_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.abp_v10x {
                properties.insert(
                    "AbpV10x".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.abp_v11 {
                properties.insert("AbpV11".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.dev_eui {
                properties.insert("DevEui".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.device_profile_id {
                properties.insert(
                    "DeviceProfileId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.f_ports {
                properties.insert("FPorts".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.otaa_v10x {
                properties.insert(
                    "OtaaV10x".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.otaa_v11 {
                properties.insert(
                    "OtaaV11".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_profile_id {
                properties.insert(
                    "ServiceProfileId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-otaav10x.html
    pub struct OtaaV10x_ {
        pub app_eui: crate::value::ExpString,
        pub app_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_WirelessDevice_OtaaV10x {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::WirelessDevice.OtaaV10x"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_WirelessDevice_OtaaV10x as OtaaV10x;
    impl crate::value::ToValue for OtaaV10x_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppEui".to_string(),
                crate::value::ToValue::to_value(&self.app_eui),
            );
            properties.insert(
                "AppKey".to_string(),
                crate::value::ToValue::to_value(&self.app_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-otaav11.html
    pub struct OtaaV11_ {
        pub app_key: crate::value::ExpString,
        pub join_eui: crate::value::ExpString,
        pub nwk_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_WirelessDevice_OtaaV11 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::WirelessDevice.OtaaV11"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_WirelessDevice_OtaaV11 as OtaaV11;
    impl crate::value::ToValue for OtaaV11_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppKey".to_string(),
                crate::value::ToValue::to_value(&self.app_key),
            );
            properties.insert(
                "JoinEui".to_string(),
                crate::value::ToValue::to_value(&self.join_eui),
            );
            properties.insert(
                "NwkKey".to_string(),
                crate::value::ToValue::to_value(&self.nwk_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-sessionkeysabpv10x.html
    pub struct SessionKeysAbpV10x_ {
        pub app_s_key: crate::value::ExpString,
        pub nwk_s_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_WirelessDevice_SessionKeysAbpV10x {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::WirelessDevice.SessionKeysAbpV10x"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_WirelessDevice_SessionKeysAbpV10x as SessionKeysAbpV10x;
    impl crate::value::ToValue for SessionKeysAbpV10x_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppSKey".to_string(),
                crate::value::ToValue::to_value(&self.app_s_key),
            );
            properties.insert(
                "NwkSKey".to_string(),
                crate::value::ToValue::to_value(&self.nwk_s_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-sessionkeysabpv11.html
    pub struct SessionKeysAbpV11_ {
        pub app_s_key: crate::value::ExpString,
        pub f_nwk_s_int_key: crate::value::ExpString,
        pub nwk_s_enc_key: crate::value::ExpString,
        pub s_nwk_s_int_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_WirelessDevice_SessionKeysAbpV11 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::WirelessDevice.SessionKeysAbpV11"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_WirelessDevice_SessionKeysAbpV11 as SessionKeysAbpV11;
    impl crate::value::ToValue for SessionKeysAbpV11_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppSKey".to_string(),
                crate::value::ToValue::to_value(&self.app_s_key),
            );
            properties.insert(
                "FNwkSIntKey".to_string(),
                crate::value::ToValue::to_value(&self.f_nwk_s_int_key),
            );
            properties.insert(
                "NwkSEncKey".to_string(),
                crate::value::ToValue::to_value(&self.nwk_s_enc_key),
            );
            properties.insert(
                "SNwkSIntKey".to_string(),
                crate::value::ToValue::to_value(&self.s_nwk_s_int_key),
            );
            properties.into()
        }
    }
}
pub mod wirelessdeviceimporttask {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdeviceimporttask-sidewalk.html
    pub struct Sidewalk_ {
        pub device_creation_file: Option<crate::value::ExpString>,
        pub device_creation_file_list: Option<Vec<crate::value::ExpString>>,
        pub role: Option<crate::value::ExpString>,
        pub sidewalk_manufacturing_sn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_WirelessDeviceImportTask_Sidewalk {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::WirelessDeviceImportTask.Sidewalk"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_WirelessDeviceImportTask_Sidewalk as Sidewalk;
    impl crate::value::ToValue for Sidewalk_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.device_creation_file {
                properties.insert(
                    "DeviceCreationFile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_creation_file_list {
                properties.insert(
                    "DeviceCreationFileList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role {
                properties.insert("Role".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sidewalk_manufacturing_sn {
                properties.insert(
                    "SidewalkManufacturingSn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod wirelessgateway {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessgateway-lorawangateway.html
    pub struct LoRaWANGateway_ {
        pub gateway_eui: crate::value::ExpString,
        pub rf_region: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotwireless_WirelessGateway_LoRaWANGateway {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTWireless::WirelessGateway.LoRaWANGateway"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotwireless_WirelessGateway_LoRaWANGateway as LoRaWANGateway;
    impl crate::value::ToValue for LoRaWANGateway_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GatewayEui".to_string(),
                crate::value::ToValue::to_value(&self.gateway_eui),
            );
            properties.insert(
                "RfRegion".to_string(),
                crate::value::ToValue::to_value(&self.rf_region),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-destination.html
pub struct Destination_ {
    pub description: Option<crate::value::ExpString>,
    pub expression: crate::value::ExpString,
    pub expression_type: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub role_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotwireless_Destination {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTWireless::Destination"
        $($field $value)*)
    };
}
pub use crate::__aws_iotwireless_Destination as Destination;
impl crate::template::ToResource for Destination_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTWireless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Destination"),
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
            "Expression".to_string(),
            crate::value::ToValue::to_value(&self.expression),
        );
        properties.insert(
            "ExpressionType".to_string(),
            crate::value::ToValue::to_value(&self.expression_type),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-deviceprofile.html
pub struct DeviceProfile_ {
    pub lo_ra_wan: Option<super::iotwireless::deviceprofile::LoRaWANDeviceProfile_>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotwireless_DeviceProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTWireless::DeviceProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_iotwireless_DeviceProfile as DeviceProfile;
impl crate::template::ToResource for DeviceProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTWireless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DeviceProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.lo_ra_wan {
            properties.insert(
                "LoRaWAN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-fuotatask.html
pub struct FuotaTask_ {
    pub associate_multicast_group: Option<crate::value::ExpString>,
    pub associate_wireless_device: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub disassociate_multicast_group: Option<crate::value::ExpString>,
    pub disassociate_wireless_device: Option<crate::value::ExpString>,
    pub firmware_update_image: crate::value::ExpString,
    pub firmware_update_role: crate::value::ExpString,
    pub lo_ra_wan: super::iotwireless::fuotatask::LoRaWAN_,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotwireless_FuotaTask {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTWireless::FuotaTask"
        $($field $value)*)
    };
}
pub use crate::__aws_iotwireless_FuotaTask as FuotaTask;
impl crate::template::ToResource for FuotaTask_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTWireless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FuotaTask"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.associate_multicast_group {
            properties.insert(
                "AssociateMulticastGroup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.associate_wireless_device {
            properties.insert(
                "AssociateWirelessDevice".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.disassociate_multicast_group {
            properties.insert(
                "DisassociateMulticastGroup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.disassociate_wireless_device {
            properties.insert(
                "DisassociateWirelessDevice".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FirmwareUpdateImage".to_string(),
            crate::value::ToValue::to_value(&self.firmware_update_image),
        );
        properties.insert(
            "FirmwareUpdateRole".to_string(),
            crate::value::ToValue::to_value(&self.firmware_update_role),
        );
        properties.insert(
            "LoRaWAN".to_string(),
            crate::value::ToValue::to_value(&self.lo_ra_wan),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-multicastgroup.html
pub struct MulticastGroup_ {
    pub associate_wireless_device: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub disassociate_wireless_device: Option<crate::value::ExpString>,
    pub lo_ra_wan: super::iotwireless::multicastgroup::LoRaWAN_,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotwireless_MulticastGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTWireless::MulticastGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_iotwireless_MulticastGroup as MulticastGroup;
impl crate::template::ToResource for MulticastGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTWireless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MulticastGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.associate_wireless_device {
            properties.insert(
                "AssociateWirelessDevice".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.disassociate_wireless_device {
            properties.insert(
                "DisassociateWirelessDevice".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LoRaWAN".to_string(),
            crate::value::ToValue::to_value(&self.lo_ra_wan),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-networkanalyzerconfiguration.html
pub struct NetworkAnalyzerConfiguration_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub trace_content: Option<super::iotwireless::networkanalyzerconfiguration::TraceContent_>,
    pub wireless_devices: Option<Vec<crate::value::ExpString>>,
    pub wireless_gateways: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotwireless_NetworkAnalyzerConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTWireless::NetworkAnalyzerConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_iotwireless_NetworkAnalyzerConfiguration as NetworkAnalyzerConfiguration;
impl crate::template::ToResource for NetworkAnalyzerConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTWireless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "NetworkAnalyzerConfiguration",
            ),
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
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.trace_content {
            properties.insert(
                "TraceContent".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.wireless_devices {
            properties.insert(
                "WirelessDevices".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.wireless_gateways {
            properties.insert(
                "WirelessGateways".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-partneraccount.html
pub struct PartnerAccount_ {
    pub account_linked: Option<crate::value::ExpBool>,
    pub partner_account_id: Option<crate::value::ExpString>,
    pub partner_type: Option<crate::value::ExpString>,
    pub sidewalk: Option<super::iotwireless::partneraccount::SidewalkAccountInfo_>,
    pub sidewalk_response:
        Option<super::iotwireless::partneraccount::SidewalkAccountInfoWithFingerprint_>,
    pub sidewalk_update: Option<super::iotwireless::partneraccount::SidewalkUpdateAccount_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotwireless_PartnerAccount {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTWireless::PartnerAccount"
        $($field $value)*)
    };
}
pub use crate::__aws_iotwireless_PartnerAccount as PartnerAccount;
impl crate::template::ToResource for PartnerAccount_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTWireless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PartnerAccount"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.account_linked {
            properties.insert(
                "AccountLinked".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.partner_account_id {
            properties.insert(
                "PartnerAccountId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.partner_type {
            properties.insert(
                "PartnerType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sidewalk {
            properties.insert(
                "Sidewalk".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sidewalk_response {
            properties.insert(
                "SidewalkResponse".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sidewalk_update {
            properties.insert(
                "SidewalkUpdate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-serviceprofile.html
pub struct ServiceProfile_ {
    pub lo_ra_wan: Option<super::iotwireless::serviceprofile::LoRaWANServiceProfile_>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotwireless_ServiceProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTWireless::ServiceProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_iotwireless_ServiceProfile as ServiceProfile;
impl crate::template::ToResource for ServiceProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTWireless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ServiceProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.lo_ra_wan {
            properties.insert(
                "LoRaWAN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-taskdefinition.html
pub struct TaskDefinition_ {
    pub auto_create_tasks: crate::value::ExpBool,
    pub lo_ra_wan_update_gateway_task_entry:
        Option<super::iotwireless::taskdefinition::LoRaWANUpdateGatewayTaskEntry_>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub task_definition_type: Option<crate::value::ExpString>,
    pub update: Option<super::iotwireless::taskdefinition::UpdateWirelessGatewayTaskCreate_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotwireless_TaskDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTWireless::TaskDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_iotwireless_TaskDefinition as TaskDefinition;
impl crate::template::ToResource for TaskDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTWireless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TaskDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AutoCreateTasks".to_string(),
            crate::value::ToValue::to_value(&self.auto_create_tasks),
        );
        if let Some(ref value) = self.lo_ra_wan_update_gateway_task_entry {
            properties.insert(
                "LoRaWANUpdateGatewayTaskEntry".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.task_definition_type {
            properties.insert(
                "TaskDefinitionType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.update {
            properties.insert("Update".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessdevice.html
pub struct WirelessDevice_ {
    pub description: Option<crate::value::ExpString>,
    pub destination_name: crate::value::ExpString,
    pub last_uplink_received_at: Option<crate::value::ExpString>,
    pub lo_ra_wan: Option<super::iotwireless::wirelessdevice::LoRaWANDevice_>,
    pub name: Option<crate::value::ExpString>,
    pub positioning: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub thing_arn: Option<crate::value::ExpString>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotwireless_WirelessDevice {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTWireless::WirelessDevice"
        $($field $value)*)
    };
}
pub use crate::__aws_iotwireless_WirelessDevice as WirelessDevice;
impl crate::template::ToResource for WirelessDevice_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTWireless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WirelessDevice"),
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
            "DestinationName".to_string(),
            crate::value::ToValue::to_value(&self.destination_name),
        );
        if let Some(ref value) = self.last_uplink_received_at {
            properties.insert(
                "LastUplinkReceivedAt".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lo_ra_wan {
            properties.insert(
                "LoRaWAN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.positioning {
            properties.insert(
                "Positioning".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.thing_arn {
            properties.insert(
                "ThingArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessdeviceimporttask.html
pub struct WirelessDeviceImportTask_ {
    pub destination_name: crate::value::ExpString,
    pub sidewalk: super::iotwireless::wirelessdeviceimporttask::Sidewalk_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotwireless_WirelessDeviceImportTask {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTWireless::WirelessDeviceImportTask"
        $($field $value)*)
    };
}
pub use crate::__aws_iotwireless_WirelessDeviceImportTask as WirelessDeviceImportTask;
impl crate::template::ToResource for WirelessDeviceImportTask_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTWireless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WirelessDeviceImportTask"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DestinationName".to_string(),
            crate::value::ToValue::to_value(&self.destination_name),
        );
        properties.insert(
            "Sidewalk".to_string(),
            crate::value::ToValue::to_value(&self.sidewalk),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessgateway.html
pub struct WirelessGateway_ {
    pub description: Option<crate::value::ExpString>,
    pub last_uplink_received_at: Option<crate::value::ExpString>,
    pub lo_ra_wan: super::iotwireless::wirelessgateway::LoRaWANGateway_,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub thing_arn: Option<crate::value::ExpString>,
    pub thing_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotwireless_WirelessGateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTWireless::WirelessGateway"
        $($field $value)*)
    };
}
pub use crate::__aws_iotwireless_WirelessGateway as WirelessGateway;
impl crate::template::ToResource for WirelessGateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTWireless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WirelessGateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.last_uplink_received_at {
            properties.insert(
                "LastUplinkReceivedAt".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LoRaWAN".to_string(),
            crate::value::ToValue::to_value(&self.lo_ra_wan),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.thing_arn {
            properties.insert(
                "ThingArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.thing_name {
            properties.insert(
                "ThingName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
