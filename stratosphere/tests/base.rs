use pretty_assertions::assert_eq;
use stratosphere::template::*;
use stratosphere::token::*;
use stratosphere::value::*;

const EXPECTED: &str = r#"{
  "Version": "2010-09-09",
  "Resources": {
    "SecurityGroupA": {
      "Type": "AWS::EC2::SecurityGroup",
      "Properties": {
        "GroupDescription": "Test Description A",
        "Tags": [
          {
            "Key": "Test Tag Key",
            "Value": "Test Tag Value"
          }
        ]
      }
    },
    "SecurityGroupB": {
      "Type": "AWS::EC2::SecurityGroup",
      "Properties": {
        "GroupDescription": "Test Description B",
        "SecurityGroupIngress": [
          {
            "CidrIp": "127.0.0.1",
            "IpProtocol": "tcp"
          }
        ]
      }
    }
  }
}"#;

#[test]
fn test_template_explicit() {
    a::foo!();

    let template = Template::new()
        .resource(
            "SecurityGroupA",
            cloudformation::aws::ec2::SecurityGroup {
                GroupDescription: "Test Description A".to_exp(),
                GroupName: None,
                SecurityGroupIngress: None,
                SecurityGroupEgress: None,
                VpcId: None,
                Tags: Some(
                    [cloudformation::Tag {
                        Key: "Test Tag Key".to_exp(),
                        Value: "Test Tag Value".to_exp(),
                    }]
                    .into(),
                ),
            },
        )
        .resource(
            "SecurityGroupB",
            cloudformation::aws::ec2::SecurityGroup {
                GroupDescription: "Test Description B".to_exp(),
                SecurityGroupIngress: vec![cloudformation::aws::ec2::securitygroup::Ingress {
                    IpProtocol: "tcp".to_exp(),
                    CidrIp: "127.0.0.1".to_exp().into(),
                    CidrIpv6: None,
                    Description: None,
                    FromPort: None,
                    SourcePrefixListId: None,
                    SourceSecurityGroupId: None,
                    SourceSecurityGroupName: None,
                    SourceSecurityGroupOwnerId: None,
                    ToPort: None,
                }]
                .into(),
                GroupName: None,
                SecurityGroupEgress: None,
                VpcId: None,
                Tags: None,
            },
        );

    assert_eq!(serde_json::to_string_pretty(&template).unwrap(), EXPECTED)
}

#[test]
fn test_template_macro() {
    let template = Template::new()
        .resource(
            "SecurityGroupA",
            cloudformation::aws::ec2::SecurityGroup {
                GroupDescription: "Test Description A".to_exp(),
                GroupName: None,
                SecurityGroupIngress: None,
                SecurityGroupEgress: None,
                VpcId: None,
                Tags: Some([cloudformation::Tag! { Key: "Test Tag Key", Value: "Test Tag Value"}].into()),
            },
        )
        .resource(
            "SecurityGroupB",
            cloudformation::aws::ec2::SecurityGroup {
                GroupDescription: "Test Description B".to_exp(),
                SecurityGroupIngress: vec![cloudformation::aws::ec2::securitygroup::Ingress! {
                    IpProtocol: "tcp",
                    CidrIp: "127.0.0.1"
                }]
                .into(),
                GroupName: None,
                SecurityGroupEgress: None,
                VpcId: None,
                Tags: None,
            },
        );

    assert_eq!(EXPECTED, serde_json::to_string_pretty(&template).unwrap())
}

#[test]
fn test_generation() {
    use indoc::indoc;
    use stratosphere_core::resource_specification::*;

    let stream = stratosphere_core::token::token_stream(Target::for_service(
        instance(),
        &ServiceIdentifier {
            vendor_name: VendorName("AWS"),
            service_name: ServiceName("CertificateManager"),
        },
    ));

    match syn::parse2(stream.clone()) {
        Ok(abstract_file) => {
            let code = prettyplease::unparse(&abstract_file);

            std::fs::write("ec2.rs", &code);

            assert_eq!(
                indoc! {r#"
                    pub mod cloudformation {
                        ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html
                        pub struct Tag {
                            pub Key: stratosphere::value::ExpString,
                            pub Value: stratosphere::value::ExpString,
                        }
                        #[macro_export]
                        macro_rules! Tag {
                            ($($field:ident : $value:expr),*) => {
                                stratosphere::generator::construct_property_type!("Tag" $($field $value)*)
                            };
                        }
                        impl stratosphere::value::ToValue for Tag {
                            fn to_value(&self) -> serde_json::Value {
                                let mut properties = serde_json::Map::new();
                                properties
                                    .insert(
                                        "Key".to_string(),
                                        stratosphere::value::ToValue::to_value(&self.Key),
                                    );
                                properties
                                    .insert(
                                        "Value".to_string(),
                                        stratosphere::value::ToValue::to_value(&self.Value),
                                    );
                                properties.into()
                            }
                        }
                        pub mod aws {
                            pub mod certificatemanager {
                                pub mod account {
                                    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-certificatemanager-account-expiryeventsconfiguration.html
                                    pub struct ExpiryEventsConfiguration {
                                        pub DaysBeforeExpiry: Option<i64>,
                                    }
                                    #[macro_export]
                                    macro_rules! Account_ExpiryEventsConfiguration {
                                        ($($field:ident : $value:expr),*) => {
                                            stratosphere::generator::construct_property_type!("AWS::CertificateManager::Account.ExpiryEventsConfiguration"
                                            $($field $value)*)
                                        };
                                    }
                                    impl stratosphere::value::ToValue for ExpiryEventsConfiguration {
                                        fn to_value(&self) -> serde_json::Value {
                                            let mut properties = serde_json::Map::new();
                                            if let Some(ref value) = self.DaysBeforeExpiry {
                                                properties
                                                    .insert(
                                                        "DaysBeforeExpiry".to_string(),
                                                        stratosphere::value::ToValue::to_value(value),
                                                    );
                                            }
                                            properties.into()
                                        }
                                    }
                                }
                                pub mod certificate {
                                    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-certificatemanager-certificate-domainvalidationoption.html
                                    pub struct DomainValidationOption {
                                        pub DomainName: stratosphere::value::ExpString,
                                        pub HostedZoneId: Option<stratosphere::value::ExpString>,
                                        pub ValidationDomain: Option<stratosphere::value::ExpString>,
                                    }
                                    #[macro_export]
                                    macro_rules! Certificate_DomainValidationOption {
                                        ($($field:ident : $value:expr),*) => {
                                            stratosphere::generator::construct_property_type!("AWS::CertificateManager::Certificate.DomainValidationOption"
                                            $($field $value)*)
                                        };
                                    }
                                    impl stratosphere::value::ToValue for DomainValidationOption {
                                        fn to_value(&self) -> serde_json::Value {
                                            let mut properties = serde_json::Map::new();
                                            properties
                                                .insert(
                                                    "DomainName".to_string(),
                                                    stratosphere::value::ToValue::to_value(&self.DomainName),
                                                );
                                            if let Some(ref value) = self.HostedZoneId {
                                                properties
                                                    .insert(
                                                        "HostedZoneId".to_string(),
                                                        stratosphere::value::ToValue::to_value(value),
                                                    );
                                            }
                                            if let Some(ref value) = self.ValidationDomain {
                                                properties
                                                    .insert(
                                                        "ValidationDomain".to_string(),
                                                        stratosphere::value::ToValue::to_value(value),
                                                    );
                                            }
                                            properties.into()
                                        }
                                    }
                                }
                                ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-account.html
                                pub struct Account {
                                    pub ExpiryEventsConfiguration: super::certificatemanager::account::ExpiryEventsConfiguration,
                                }
                                impl stratosphere::template::ToResource for Account {
                                    const RESOURCE_TYPE_NAME: stratosphere::resource_specification::ResourceTypeName<
                                        'static,
                                    > = stratosphere::resource_specification::ResourceTypeName {
                                        service: stratosphere::resource_specification::ServiceIdentifier {
                                            service_name: stratosphere::resource_specification::ServiceName(
                                                "CertificateManager",
                                            ),
                                            vendor_name: stratosphere::resource_specification::VendorName(
                                                "AWS",
                                            ),
                                        },
                                        resource_name: stratosphere::resource_specification::ResourceName(
                                            "Account",
                                        ),
                                    };
                                    fn to_resource_properties(
                                        &self,
                                    ) -> stratosphere::template::ResourceProperties {
                                        let mut properties = stratosphere::template::ResourceProperties::new();
                                        properties
                                            .insert(
                                                "ExpiryEventsConfiguration".to_string(),
                                                stratosphere::value::ToValue::to_value(
                                                    &self.ExpiryEventsConfiguration,
                                                ),
                                            );
                                        properties
                                    }
                                }
                                ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html
                                pub struct Certificate {
                                    pub CertificateAuthorityArn: Option<stratosphere::value::ExpString>,
                                    pub CertificateTransparencyLoggingPreference: Option<
                                        stratosphere::value::ExpString,
                                    >,
                                    pub DomainName: stratosphere::value::ExpString,
                                    pub DomainValidationOptions: Option<
                                        Vec<super::certificatemanager::certificate::DomainValidationOption>,
                                    >,
                                    pub KeyAlgorithm: Option<stratosphere::value::ExpString>,
                                    pub SubjectAlternativeNames: Option<Vec<stratosphere::value::ExpString>>,
                                    pub Tags: Option<Vec<crate::cloudformation::Tag>>,
                                    pub ValidationMethod: Option<stratosphere::value::ExpString>,
                                }
                                impl stratosphere::template::ToResource for Certificate {
                                    const RESOURCE_TYPE_NAME: stratosphere::resource_specification::ResourceTypeName<
                                        'static,
                                    > = stratosphere::resource_specification::ResourceTypeName {
                                        service: stratosphere::resource_specification::ServiceIdentifier {
                                            service_name: stratosphere::resource_specification::ServiceName(
                                                "CertificateManager",
                                            ),
                                            vendor_name: stratosphere::resource_specification::VendorName(
                                                "AWS",
                                            ),
                                        },
                                        resource_name: stratosphere::resource_specification::ResourceName(
                                            "Certificate",
                                        ),
                                    };
                                    fn to_resource_properties(
                                        &self,
                                    ) -> stratosphere::template::ResourceProperties {
                                        let mut properties = stratosphere::template::ResourceProperties::new();
                                        if let Some(ref value) = self.CertificateAuthorityArn {
                                            properties
                                                .insert(
                                                    "CertificateAuthorityArn".to_string(),
                                                    stratosphere::value::ToValue::to_value(value),
                                                );
                                        }
                                        if let Some(ref value) = self
                                            .CertificateTransparencyLoggingPreference
                                        {
                                            properties
                                                .insert(
                                                    "CertificateTransparencyLoggingPreference".to_string(),
                                                    stratosphere::value::ToValue::to_value(value),
                                                );
                                        }
                                        properties
                                            .insert(
                                                "DomainName".to_string(),
                                                stratosphere::value::ToValue::to_value(&self.DomainName),
                                            );
                                        if let Some(ref value) = self.DomainValidationOptions {
                                            properties
                                                .insert(
                                                    "DomainValidationOptions".to_string(),
                                                    stratosphere::value::ToValue::to_value(value),
                                                );
                                        }
                                        if let Some(ref value) = self.KeyAlgorithm {
                                            properties
                                                .insert(
                                                    "KeyAlgorithm".to_string(),
                                                    stratosphere::value::ToValue::to_value(value),
                                                );
                                        }
                                        if let Some(ref value) = self.SubjectAlternativeNames {
                                            properties
                                                .insert(
                                                    "SubjectAlternativeNames".to_string(),
                                                    stratosphere::value::ToValue::to_value(value),
                                                );
                                        }
                                        if let Some(ref value) = self.Tags {
                                            properties
                                                .insert(
                                                    "Tags".to_string(),
                                                    stratosphere::value::ToValue::to_value(value),
                                                );
                                        }
                                        if let Some(ref value) = self.ValidationMethod {
                                            properties
                                                .insert(
                                                    "ValidationMethod".to_string(),
                                                    stratosphere::value::ToValue::to_value(value),
                                                );
                                        }
                                        properties
                                    }
                                }
                            }
                        }
                    }
                "#},
                code
            )
        }
        Err(error) => {
            panic!("Code failed to parse with error: {error:#?}\nCode:\n{stream}");
        }
    }
}
