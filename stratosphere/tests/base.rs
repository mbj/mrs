use pretty_assertions::assert_eq;
use stratosphere::template::*;
use stratosphere::token::*;

stratosphere::generator::services!("AWS::EC2", "AWS::SecretsManager");

const EXPECTED: &str = r#"{
  "AWSTemplateFormatVersion": "2010-09-09",
  "Outputs": {
    "SecurityGroupIdA": {
      "Description": "Id of the security group A",
      "Value": {
        "Ref": "SecurityGroupA"
      }
    }
  },
  "Parameters": {
    "VpcCidr": {
      "Description": "CIDR block for the VPC",
      "Type": "String",
      "AllowedPattern": "^(\\d{1,3}\\.){3}\\d{1,3}/\\d{1,2}$"
    }
  },
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
        ],
        "VpcId": {
          "Ref": "Vpc"
        }
      }
    },
    "SecurityGroupB": {
      "Type": "AWS::EC2::SecurityGroup",
      "Properties": {
        "GroupDescription": "Test Description B",
        "SecurityGroupIngress": [
          {
            "CidrIp": "127.0.0.1",
            "IpProtocol": "tcp",
            "SourceSecurityGroupId": {
              "Ref": "SecurityGroupA"
            }
          }
        ],
        "VpcId": {
          "Ref": "Vpc"
        }
      }
    },
    "Vpc": {
      "Type": "AWS::EC2::VPC",
      "Properties": {
        "CidrBlock": {
          "Ref": "VpcCidr"
        }
      }
    }
  }
}"#;

#[test]
fn test_template_explicit() {
    let template = Template::new()
        .parameter_(
            "VpcCidr",
            stratosphere::template::Parameter {
                description: Some("CIDR block for the VPC".into()),
                r#type: stratosphere::template::ParameterType::String,
                allowed_pattern: Some(r"^(\d{1,3}\.){3}\d{1,3}/\d{1,2}$".into()),
            },
        )
        .resource_(
            "Vpc",
            cloudformation::aws::ec2::VPC_ {
                cidr_block: Some(stratosphere::value::ExpString::Ref("VpcCidr".into())),
                enable_dns_hostnames: None,
                enable_dns_support: None,
                instance_tenancy: None,
                ipv4_ipam_pool_id: None,
                ipv4_netmask_length: None,
                tags: None,
            },
        )
        .resource_(
            "SecurityGroupA",
            cloudformation::aws::ec2::SecurityGroup_ {
                group_description: "Test Description A".into(),
                group_name: None,
                security_group_ingress: None,
                security_group_egress: None,
                vpc_id: Some(stratosphere::value::ExpString::Ref("Vpc".into())),
                tags: Some(
                    [cloudformation::Tag_ {
                        key: "Test Tag Key".into(),
                        value: "Test Tag Value".into(),
                    }]
                    .into(),
                ),
            },
        )
        .resource_(
            "SecurityGroupB",
            cloudformation::aws::ec2::SecurityGroup_ {
                group_description: "Test Description B".into(),
                security_group_ingress: vec![cloudformation::aws::ec2::securitygroup::Ingress_ {
                    ip_protocol: "tcp".into(),
                    cidr_ip: Some("127.0.0.1".into()),
                    cidr_ipv6: None,
                    description: None,
                    from_port: None,
                    source_prefix_list_id: None,
                    source_security_group_id: Some(stratosphere::value::ExpString::Ref(
                        "SecurityGroupA".into(),
                    )),
                    source_security_group_name: None,
                    source_security_group_owner_id: None,
                    to_port: None,
                }]
                .into(),
                group_name: None,
                security_group_egress: None,
                vpc_id: Some(stratosphere::value::ExpString::Ref("Vpc".into())),
                tags: None,
            },
        )
        .output_(
            "SecurityGroupIdA",
            stratosphere::template::Output {
                description: "Id of the security group A".into(),
                export: None,
                value: stratosphere::value::ExpString::Ref("SecurityGroupA".into()),
            },
        );

    assert_eq!(serde_json::to_string_pretty(&template).unwrap(), EXPECTED)
}

#[test]
fn test_template_builder() {
    use cloudformation::Tag;
    use cloudformation::aws::ec2;
    use stratosphere::template::ParameterType;

    let template = Template::build(|template| {
        let vpc_cidr = &template.parameter(
            "VpcCidr",
            stratosphere::Parameter! {
                description: "CIDR block for the VPC",
                r#type: ParameterType::String,
                allowed_pattern: r"^(\d{1,3}\.){3}\d{1,3}/\d{1,2}$"
            },
        );

        let vpc = &template.resource(
            "Vpc",
            ec2::VPC! {
                cidr_block: vpc_cidr
            },
        );

        let security_group_a = &template.resource(
            "SecurityGroupA",
            ec2::SecurityGroup! {
                group_description: "Test Description A",
                tags: [Tag! { key: "Test Tag Key", value: "Test Tag Value"}],
                vpc_id: vpc
            },
        );

        let _security_group_b = &template.resource(
            "SecurityGroupB",
            ec2::SecurityGroup! {
                group_description: "Test Description B",
                security_group_ingress: vec![ec2::securitygroup::Ingress! {
                    ip_protocol: "tcp",
                    cidr_ip: "127.0.0.1",
                    source_security_group_id: security_group_a
                }],
                vpc_id: vpc
            },
        );

        template.output(
            "SecurityGroupIdA",
            stratosphere::Output! {
                description: "Id of the security group A",
                value: security_group_a,
            },
        );

        // Test fn_join! macro - create a combined output with delimiter
        template.output(
            "VpcInfo",
            stratosphere::Output! {
                description: "VPC information with CIDR",
                value: stratosphere::fn_join![
                    " - ",
                    [
                        "VPC:",
                        vpc,
                        "CIDR:",
                        vpc_cidr,
                    ]
                ],
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Outputs": {
            "SecurityGroupIdA": {
                "Description": "Id of the security group A",
                "Value": {
                    "Ref": "SecurityGroupA"
                }
            },
            "VpcInfo": {
                "Description": "VPC information with CIDR",
                "Value": {
                    "Fn::Join": [
                        " - ",
                        [
                            "VPC:",
                            {"Ref": "Vpc"},
                            "CIDR:",
                            {"Ref": "VpcCidr"}
                        ]
                    ]
                }
            }
        },
        "Parameters": {
            "VpcCidr": {
                "Description": "CIDR block for the VPC",
                "Type": "String",
                "AllowedPattern": r"^(\d{1,3}\.){3}\d{1,3}/\d{1,2}$"
            }
        },
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
                    ],
                    "VpcId": {
                        "Ref": "Vpc"
                    }
                }
            },
            "SecurityGroupB": {
                "Type": "AWS::EC2::SecurityGroup",
                "Properties": {
                    "GroupDescription": "Test Description B",
                    "SecurityGroupIngress": [
                        {
                            "CidrIp": "127.0.0.1",
                            "IpProtocol": "tcp",
                            "SourceSecurityGroupId": {
                                "Ref": "SecurityGroupA"
                            }
                        }
                    ],
                    "VpcId": {
                        "Ref": "Vpc"
                    }
                }
            },
            "Vpc": {
                "Type": "AWS::EC2::VPC",
                "Properties": {
                    "CidrBlock": {
                        "Ref": "VpcCidr"
                    }
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_join_macro() {
    use stratosphere::template::ParameterType;

    let template = Template::build(|template| {
        let parameter_a = &template.parameter(
            "ParameterA",
            stratosphere::Parameter! {
                description: "First parameter",
                r#type: ParameterType::String
            },
        );

        let parameter_b = &template.parameter(
            "ParameterB",
            stratosphere::Parameter! {
                description: "Second parameter",
                r#type: ParameterType::String
            },
        );

        template.output(
            "JoinedOutput",
            stratosphere::Output! {
                description: "Joined parameters with delimiter",
                value: stratosphere::fn_join![
                    ":",
                    [
                        "prefix",
                        parameter_a,
                        parameter_b,
                        "suffix",
                    ]
                ],
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Outputs": {
            "JoinedOutput": {
                "Description": "Joined parameters with delimiter",
                "Value": {
                    "Fn::Join": [
                        ":",
                        [
                            "prefix",
                            {"Ref": "ParameterA"},
                            {"Ref": "ParameterB"},
                            "suffix"
                        ]
                    ]
                }
            }
        },
        "Parameters": {
            "ParameterA": {
                "Description": "First parameter",
                "Type": "String"
            },
            "ParameterB": {
                "Description": "Second parameter",
                "Type": "String"
            }
        },
        "Resources": {}
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_sub_macro() {
    use stratosphere::template::ParameterType;

    let template = Template::build(|template| {
        let _bucket_name = &template.parameter(
            "BucketName",
            stratosphere::Parameter! {
                description: "S3 bucket name",
                r#type: ParameterType::String
            },
        );

        template.output(
            "BucketArn",
            stratosphere::Output! {
                description: "ARN of the S3 bucket",
                value: stratosphere::fn_sub!("arn:aws:s3:::${BucketName}/*"),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Outputs": {
            "BucketArn": {
                "Description": "ARN of the S3 bucket",
                "Value": {
                    "Fn::Sub": "arn:aws:s3:::${BucketName}/*"
                }
            }
        },
        "Parameters": {
            "BucketName": {
                "Description": "S3 bucket name",
                "Type": "String"
            }
        },
        "Resources": {}
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_select_macro() {
    use stratosphere::template::ParameterType;

    let template = Template::build(|template| {
        let _environment = &template.parameter(
            "Environment",
            stratosphere::Parameter! {
                description: "Environment name",
                r#type: ParameterType::String
            },
        );

        template.output(
            "SelectedValue",
            stratosphere::Output! {
                description: "Selected availability zone",
                value: stratosphere::fn_select!(
                    0,
                    [
                        "us-east-1a",
                        "us-east-1b",
                        "us-east-1c"
                    ]
                ),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Outputs": {
            "SelectedValue": {
                "Description": "Selected availability zone",
                "Value": {
                    "Fn::Select": [
                        0,
                        [
                            "us-east-1a",
                            "us-east-1b",
                            "us-east-1c"
                        ]
                    ]
                }
            }
        },
        "Parameters": {
            "Environment": {
                "Description": "Environment name",
                "Type": "String"
            }
        },
        "Resources": {}
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_get_att_macro() {
    use cloudformation::aws::ec2;

    let template = Template::build(|template| {
        let _vpc = &template.resource(
            "MyVpc",
            ec2::VPC! {
                cidr_block: "10.0.0.0/16"
            },
        );

        template.output(
            "VpcArn",
            stratosphere::Output! {
                description: "ARN of the VPC",
                value: stratosphere::fn_get_att!("MyVpc", "Arn"),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Outputs": {
            "VpcArn": {
                "Description": "ARN of the VPC",
                "Value": {
                    "Fn::GetAtt": ["MyVpc", "Arn"]
                }
            }
        },
        "Resources": {
            "MyVpc": {
                "Type": "AWS::EC2::VPC",
                "Properties": {
                    "CidrBlock": "10.0.0.0/16"
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_generation() {
    use stratosphere_core::resource_specification::*;

    let stream = stratosphere_core::token::token_stream(Target::for_services(
        instance(),
        [
            ServiceIdentifier {
                vendor_name: VendorName("AWS"),
                service_name: ServiceName("CertificateManager"),
            },
            ServiceIdentifier {
                vendor_name: VendorName("AWS"),
                service_name: ServiceName("SecretsManager"),
            },
        ]
        .into(),
    ));

    match syn::parse2(stream.clone()) {
        Ok(abstract_file) => {
            insta::assert_snapshot!(prettyplease::unparse(&abstract_file));
        }
        Err(error) => {
            panic!("Code failed to parse with error: {error:#?}\nCode:\n{stream}");
        }
    }
}
