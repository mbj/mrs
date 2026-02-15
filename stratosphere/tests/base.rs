use pretty_assertions::assert_eq;
use stratosphere::services as cloudformation;
use stratosphere::template::*;

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
                    [stratosphere::Tag_ {
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
                condition: None,
                description: "Id of the security group A".into(),
                export: None,
                value: stratosphere::value::ExpString::Ref("SecurityGroupA".into()),
            },
        );

    assert_eq!(serde_json::to_string_pretty(&template).unwrap(), EXPECTED)
}

#[test]
fn test_template_builder() {
    use cloudformation::aws::ec2;
    use stratosphere::Tag;
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
fn test_get_att_arn_macro() {
    use cloudformation::aws::ec2;

    let template = Template::build(|template| {
        let _security_group = &template.resource(
            "MySecurityGroup",
            ec2::SecurityGroup! {
                group_description: "Test security group",
                vpc_id: "vpc-12345678"
            },
        );

        template.output(
            "SecurityGroupArn",
            stratosphere::Output! {
                description: "ARN of the security group",
                value: stratosphere::fn_get_att_arn!("MySecurityGroup"),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Outputs": {
            "SecurityGroupArn": {
                "Description": "ARN of the security group",
                "Value": {
                    "Fn::GetAtt": ["MySecurityGroup", "Arn"]
                }
            }
        },
        "Resources": {
            "MySecurityGroup": {
                "Type": "AWS::EC2::SecurityGroup",
                "Properties": {
                    "GroupDescription": "Test security group",
                    "VpcId": "vpc-12345678"
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_import_value_macro() {
    use cloudformation::aws::ec2;

    let template = Template::build(|template| {
        let _security_group = &template.resource(
            "MySecurityGroup",
            ec2::SecurityGroup! {
                group_description: "Test security group",
                vpc_id: stratosphere::fn_import_value!("NetworkStack-VpcId")
            },
        );

        template.output(
            "SecurityGroupId",
            stratosphere::Output! {
                description: "Security group ID",
                value: "MySecurityGroup",
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Outputs": {
            "SecurityGroupId": {
                "Description": "Security group ID",
                "Value": "MySecurityGroup"
            }
        },
        "Resources": {
            "MySecurityGroup": {
                "Type": "AWS::EC2::SecurityGroup",
                "Properties": {
                    "GroupDescription": "Test security group",
                    "VpcId": {
                        "Fn::ImportValue": "NetworkStack-VpcId"
                    }
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_base64_macro() {
    let template = Template::build(|template| {
        template.output(
            "EncodedData",
            stratosphere::Output! {
                description: "Base64 encoded user data",
                value: stratosphere::fn_base64!("#!/bin/bash\necho 'Hello World'"),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Outputs": {
            "EncodedData": {
                "Description": "Base64 encoded user data",
                "Value": {
                    "Fn::Base64": "#!/bin/bash\necho 'Hello World'"
                }
            }
        },
        "Resources": {}
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_mk_name_macro() {
    let template = Template::build(|template| {
        template.output(
            "ResourceName",
            stratosphere::Output! {
                description: "Stack-prefixed resource name",
                value: stratosphere::mk_name!("MyBucket"),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Outputs": {
            "ResourceName": {
                "Description": "Stack-prefixed resource name",
                "Value": {
                    "Fn::Join": [
                        "-",
                        [
                            {"Ref": "AWS::StackName"},
                            "MyBucket"
                        ]
                    ]
                }
            }
        },
        "Resources": {}
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_if_macro() {
    let template = Template::build(|template| {
        template.output(
            "InstanceType",
            stratosphere::Output! {
                description: "Instance type based on environment",
                value: stratosphere::fn_if!(
                    "IsProduction",
                    "m5.large",
                    "t3.micro"
                ),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Outputs": {
            "InstanceType": {
                "Description": "Instance type based on environment",
                "Value": {
                    "Fn::If": [
                        "IsProduction",
                        "m5.large",
                        "t3.micro"
                    ]
                }
            }
        },
        "Resources": {}
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_and_macro() {
    use stratosphere::value::ToValue;

    let condition = stratosphere::fn_and!(true, false);
    let value = condition.to_value();

    let expected = serde_json::json!({
        "Fn::And": [true, false]
    });

    assert_eq!(expected, value);
}

#[test]
fn test_fn_or_macro() {
    use stratosphere::value::ToValue;

    let condition = stratosphere::fn_or!([true, false, true]);
    let value = condition.to_value();

    let expected = serde_json::json!({
        "Fn::Or": [true, false, true]
    });

    assert_eq!(expected, value);
}

#[test]
fn test_fn_not_macro() {
    use stratosphere::value::ToValue;

    let condition = stratosphere::fn_not!(true);
    let value = condition.to_value();

    let expected = serde_json::json!({
        "Fn::Not": [true]
    });

    assert_eq!(expected, value);
}

#[test]
fn test_fn_equals_string_macro() {
    use stratosphere::value::ToValue;

    // Test the macro directly
    let condition = stratosphere::fn_equals_string!("production", "production");
    let value = condition.to_value();

    let expected = serde_json::json!({
        "Fn::Equals": ["production", "production"]
    });

    assert_eq!(expected, value);
}

#[test]
fn test_fn_equals_bool_macro() {
    use stratosphere::value::ToValue;

    let condition = stratosphere::fn_equals_bool!(true, false);
    let value = condition.to_value();

    let expected = serde_json::json!({
        "Fn::Equals": [true, false]
    });

    assert_eq!(expected, value);
}

#[test]
fn test_timestamp_values() {
    use chrono::{TimeZone, Utc};
    use cloudformation::aws::applicationautoscaling;

    let template = Template::build(|template| {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 10, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 12, 31, 22, 0, 0).unwrap();

        let _scalable_target = &template.resource(
            "MyScalableTarget",
            applicationautoscaling::ScalableTarget! {
                max_capacity: 10,
                min_capacity: 1,
                resource_id: "service/my-cluster/my-service",
                scalable_dimension: "ecs:service:DesiredCount",
                service_namespace: "ecs",
                scheduled_actions: vec![
                    applicationautoscaling::scalabletarget::ScheduledAction! {
                        schedule: "cron(0 10 * * ? *)",
                        scheduled_action_name: "ScaleUp",
                        start_time: start,
                        end_time: end
                    }
                ]
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Resources": {
            "MyScalableTarget": {
                "Type": "AWS::ApplicationAutoScaling::ScalableTarget",
                "Properties": {
                    "MaxCapacity": 10,
                    "MinCapacity": 1,
                    "ResourceId": "service/my-cluster/my-service",
                    "ScalableDimension": "ecs:service:DesiredCount",
                    "ServiceNamespace": "ecs",
                    "ScheduledActions": [
                        {
                            "Schedule": "cron(0 10 * * ? *)",
                            "ScheduledActionName": "ScaleUp",
                            "StartTime": "2024-01-01T10:00:00+00:00",
                            "EndTime": "2024-12-31T22:00:00+00:00"
                        }
                    ]
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_list_property() {
    use cloudformation::aws::ec2;

    let template = Template::build(|template| {
        let _instance = &template.resource(
            "MyInstance",
            ec2::Instance! {
                image_id: "ami-12345678",
                instance_type: "t2.micro",
                security_group_ids: vec![
                    "sg-12345678".into(),
                    "sg-87654321".into()
                ]
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Resources": {
            "MyInstance": {
                "Type": "AWS::EC2::Instance",
                "Properties": {
                    "ImageId": "ami-12345678",
                    "InstanceType": "t2.micro",
                    "SecurityGroupIds": ["sg-12345678", "sg-87654321"]
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_if_bool_macro() {
    use cloudformation::aws::ec2::VPC;
    use stratosphere::value::fn_if_bool;

    let template = Template::build(|template| {
        template.resource(
            "Vpc",
            VPC! {
                cidr_block: "10.0.0.0/16",
                enable_dns_support: fn_if_bool("IsProduction", true, false),
                enable_dns_hostnames: fn_if_bool("IsProduction", true, false),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Resources": {
            "Vpc": {
                "Type": "AWS::EC2::VPC",
                "Properties": {
                    "CidrBlock": "10.0.0.0/16",
                    "EnableDnsSupport": {
                        "Fn::If": [
                            "IsProduction",
                            true,
                            false
                        ]
                    },
                    "EnableDnsHostnames": {
                        "Fn::If": [
                            "IsProduction",
                            true,
                            false
                        ]
                    }
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_select_bool() {
    use cloudformation::aws::ec2::VPC;
    use stratosphere::value::fn_select_bool;

    let template = Template::build(|template| {
        template.resource(
            "Vpc",
            VPC! {
                cidr_block: "10.0.0.0/16",
                enable_dns_support: fn_select_bool(0, vec![true.into(), false.into()]),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Resources": {
            "Vpc": {
                "Type": "AWS::EC2::VPC",
                "Properties": {
                    "CidrBlock": "10.0.0.0/16",
                    "EnableDnsSupport": {
                        "Fn::Select": [
                            0,
                            [true, false]
                        ]
                    }
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_select_string() {
    use cloudformation::aws::ec2::VPC;
    use stratosphere::value::fn_select_string;

    let template = Template::build(|template| {
        template.resource(
            "Vpc",
            VPC! {
                cidr_block: fn_select_string(0, vec!["10.0.0.0/16".into(), "10.1.0.0/16".into()]),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Resources": {
            "Vpc": {
                "Type": "AWS::EC2::VPC",
                "Properties": {
                    "CidrBlock": {
                        "Fn::Select": [
                            0,
                            ["10.0.0.0/16", "10.1.0.0/16"]
                        ]
                    }
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_split_string() {
    use cloudformation::aws::ec2::VPC;
    use stratosphere::value::{fn_select_string, fn_split};

    let template = Template::build(|template| {
        template.resource(
            "Vpc",
            VPC! {
                cidr_block: fn_select_string(0, vec![fn_split(",", "10.0.0.0/16,10.1.0.0/16")]),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Resources": {
            "Vpc": {
                "Type": "AWS::EC2::VPC",
                "Properties": {
                    "CidrBlock": {
                        "Fn::Select": [
                            0,
                            [
                                {
                                    "Fn::Split": [",", "10.0.0.0/16,10.1.0.0/16"]
                                }
                            ]
                        ]
                    }
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_find_in_map_string() {
    use cloudformation::aws::ec2::VPC;
    use stratosphere::value::fn_find_in_map_string;

    let template = Template::build(|template| {
        let region_map = template.mapping(
            "RegionMap",
            [(
                "us-east-1".to_string(),
                [("CIDR".to_string(), serde_json::json!("10.0.0.0/16"))].into(),
            )]
            .into(),
        );

        template.resource(
            "Vpc",
            VPC! {
                cidr_block: fn_find_in_map_string(&region_map, "us-east-1", "CIDR"),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Mappings": {
            "RegionMap": {
                "us-east-1": {
                    "CIDR": "10.0.0.0/16"
                }
            }
        },
        "Resources": {
            "Vpc": {
                "Type": "AWS::EC2::VPC",
                "Properties": {
                    "CidrBlock": {
                        "Fn::FindInMap": ["RegionMap", "us-east-1", "CIDR"]
                    }
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_find_in_map_bool() {
    use cloudformation::aws::ec2::VPC;
    use stratosphere::value::fn_find_in_map_bool;

    let template = Template::build(|template| {
        let config_map = template.mapping(
            "ConfigMap",
            [(
                "us-east-1".to_string(),
                [("DnsEnabled".to_string(), serde_json::json!(true))].into(),
            )]
            .into(),
        );

        template.resource(
            "Vpc",
            VPC! {
                cidr_block: "10.0.0.0/16",
                enable_dns_support: fn_find_in_map_bool(&config_map, "us-east-1", "DnsEnabled"),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Mappings": {
            "ConfigMap": {
                "us-east-1": {
                    "DnsEnabled": true
                }
            }
        },
        "Resources": {
            "Vpc": {
                "Type": "AWS::EC2::VPC",
                "Properties": {
                    "CidrBlock": "10.0.0.0/16",
                    "EnableDnsSupport": {
                        "Fn::FindInMap": ["ConfigMap", "us-east-1", "DnsEnabled"]
                    }
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_find_in_map_with_ref() {
    use cloudformation::aws::ec2::VPC;
    use stratosphere::value::{AWS_REGION, fn_find_in_map_string};

    let template = Template::build(|template| {
        let region_map = template.mapping(
            "RegionMap",
            [
                (
                    "us-east-1".to_string(),
                    [("CIDR".to_string(), serde_json::json!("10.0.0.0/16"))].into(),
                ),
                (
                    "us-west-2".to_string(),
                    [("CIDR".to_string(), serde_json::json!("10.1.0.0/16"))].into(),
                ),
            ]
            .into(),
        );

        template.resource(
            "Vpc",
            VPC! {
                cidr_block: fn_find_in_map_string(&region_map, AWS_REGION, "CIDR"),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Mappings": {
            "RegionMap": {
                "us-east-1": {
                    "CIDR": "10.0.0.0/16"
                },
                "us-west-2": {
                    "CIDR": "10.1.0.0/16"
                }
            }
        },
        "Resources": {
            "Vpc": {
                "Type": "AWS::EC2::VPC",
                "Properties": {
                    "CidrBlock": {
                        "Fn::FindInMap": [
                            "RegionMap",
                            {"Ref": "AWS::Region"},
                            "CIDR"
                        ]
                    }
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_get_azs_current_region() {
    use cloudformation::aws::ec2::Subnet;
    use stratosphere::value::{fn_get_azs, fn_select_string};

    let template = Template::build(|template| {
        template.resource(
            "Subnet",
            Subnet! {
                vpc_id: "vpc-12345",
                cidr_block: "10.0.1.0/24",
                availability_zone: fn_select_string(0, fn_get_azs("")),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Resources": {
            "Subnet": {
                "Type": "AWS::EC2::Subnet",
                "Properties": {
                    "VpcId": "vpc-12345",
                    "CidrBlock": "10.0.1.0/24",
                    "AvailabilityZone": {
                        "Fn::Select": [
                            0,
                            {"Fn::GetAZs": ""}
                        ]
                    }
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_get_azs_specific_region() {
    use cloudformation::aws::ec2::Subnet;
    use stratosphere::value::{fn_get_azs, fn_select_string};

    let template = Template::build(|template| {
        template.resource(
            "Subnet",
            Subnet! {
                vpc_id: "vpc-12345",
                cidr_block: "10.0.1.0/24",
                availability_zone: fn_select_string(0, fn_get_azs("us-west-2")),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Resources": {
            "Subnet": {
                "Type": "AWS::EC2::Subnet",
                "Properties": {
                    "VpcId": "vpc-12345",
                    "CidrBlock": "10.0.1.0/24",
                    "AvailabilityZone": {
                        "Fn::Select": [
                            0,
                            {"Fn::GetAZs": "us-west-2"}
                        ]
                    }
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_get_azs_with_ref() {
    use cloudformation::aws::ec2::Subnet;
    use stratosphere::value::{AWS_REGION, fn_get_azs, fn_select_string};

    let template = Template::build(|template| {
        template.resource(
            "Subnet",
            Subnet! {
                vpc_id: "vpc-12345",
                cidr_block: "10.0.1.0/24",
                availability_zone: fn_select_string(1, fn_get_azs(AWS_REGION)),
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Resources": {
            "Subnet": {
                "Type": "AWS::EC2::Subnet",
                "Properties": {
                    "VpcId": "vpc-12345",
                    "CidrBlock": "10.0.1.0/24",
                    "AvailabilityZone": {
                        "Fn::Select": [
                            1,
                            {"Fn::GetAZs": {"Ref": "AWS::Region"}}
                        ]
                    }
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_cidr_basic() {
    use cloudformation::aws::ec2::Subnet;
    use stratosphere::value::{fn_cidr, fn_select_string};

    let template = Template::build(|template| {
        template.resource(
            "Subnet",
            Subnet! {
                vpc_id: "vpc-12345",
                cidr_block: fn_select_string(0, fn_cidr("10.0.0.0/16", 6, 8)),
                availability_zone: "us-east-1a",
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Resources": {
            "Subnet": {
                "Type": "AWS::EC2::Subnet",
                "Properties": {
                    "VpcId": "vpc-12345",
                    "CidrBlock": {
                        "Fn::Select": [
                            0,
                            {"Fn::Cidr": ["10.0.0.0/16", 6, 8]}
                        ]
                    },
                    "AvailabilityZone": "us-east-1a"
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_cidr_with_ref() {
    use cloudformation::aws::ec2::Subnet;
    use stratosphere::value::{fn_cidr, fn_select_string};

    let template = Template::build(|template| {
        let vpc_cidr = template.parameter(
            "VpcCidr",
            stratosphere::template::Parameter {
                description: Some("VPC CIDR block".to_string()),
                r#type: stratosphere::template::ParameterType::String,
                allowed_pattern: None,
            },
        );

        template.resource(
            "Subnet1",
            Subnet! {
                vpc_id: "vpc-12345",
                cidr_block: fn_select_string(0, fn_cidr(&vpc_cidr, 6, 8)),
                availability_zone: "us-east-1a",
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Parameters": {
            "VpcCidr": {
                "Description": "VPC CIDR block",
                "Type": "String"
            }
        },
        "Resources": {
            "Subnet1": {
                "Type": "AWS::EC2::Subnet",
                "Properties": {
                    "VpcId": "vpc-12345",
                    "CidrBlock": {
                        "Fn::Select": [
                            0,
                            {"Fn::Cidr": [{"Ref": "VpcCidr"}, 6, 8]}
                        ]
                    },
                    "AvailabilityZone": "us-east-1a"
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_fn_cidr_multiple_subnets() {
    use cloudformation::aws::ec2::Subnet;
    use stratosphere::value::{fn_cidr, fn_select_string};

    let template = Template::build(|template| {
        template.resource(
            "Subnet1",
            Subnet! {
                vpc_id: "vpc-12345",
                cidr_block: fn_select_string(0, fn_cidr("10.0.0.0/16", 6, 8)),
                availability_zone: "us-east-1a",
            },
        );

        template.resource(
            "Subnet2",
            Subnet! {
                vpc_id: "vpc-12345",
                cidr_block: fn_select_string(1, fn_cidr("10.0.0.0/16", 6, 8)),
                availability_zone: "us-east-1b",
            },
        );

        template.resource(
            "Subnet3",
            Subnet! {
                vpc_id: "vpc-12345",
                cidr_block: fn_select_string(2, fn_cidr("10.0.0.0/16", 6, 8)),
                availability_zone: "us-east-1c",
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Resources": {
            "Subnet1": {
                "Type": "AWS::EC2::Subnet",
                "Properties": {
                    "VpcId": "vpc-12345",
                    "CidrBlock": {
                        "Fn::Select": [
                            0,
                            {"Fn::Cidr": ["10.0.0.0/16", 6, 8]}
                        ]
                    },
                    "AvailabilityZone": "us-east-1a"
                }
            },
            "Subnet2": {
                "Type": "AWS::EC2::Subnet",
                "Properties": {
                    "VpcId": "vpc-12345",
                    "CidrBlock": {
                        "Fn::Select": [
                            1,
                            {"Fn::Cidr": ["10.0.0.0/16", 6, 8]}
                        ]
                    },
                    "AvailabilityZone": "us-east-1b"
                }
            },
            "Subnet3": {
                "Type": "AWS::EC2::Subnet",
                "Properties": {
                    "VpcId": "vpc-12345",
                    "CidrBlock": {
                        "Fn::Select": [
                            2,
                            {"Fn::Cidr": ["10.0.0.0/16", 6, 8]}
                        ]
                    },
                    "AvailabilityZone": "us-east-1c"
                }
            }
        }
    });

    assert_eq!(expected, serde_json::to_value(&template).unwrap());
}

#[test]
fn test_conditions_section() {
    use stratosphere::template::ParameterType;
    use stratosphere::value::equals_string;

    let template = Template::build(|template| {
        let env = &template.parameter(
            "Environment",
            stratosphere::Parameter! {
                description: "Environment name",
                r#type: ParameterType::String,
            },
        );

        template.condition("IsProduction", equals_string(env, "production"));
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Conditions": {
            "IsProduction": {
                "Fn::Equals": [{"Ref": "Environment"}, "production"]
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
fn test_condition_referencing_another_condition() {
    use stratosphere::template::ParameterType;
    use stratosphere::value::{ExpBool, equals_string};

    let template = Template::build(|template| {
        let env = &template.parameter(
            "Environment",
            stratosphere::Parameter! {
                description: "Environment name",
                r#type: ParameterType::String,
            },
        );

        let is_prod = template.condition("IsProduction", equals_string(env, "production"));

        template.condition(
            "IsNotProduction",
            ExpBool::Not(Box::new(ExpBool::Condition(is_prod))),
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Conditions": {
            "IsNotProduction": {
                "Fn::Not": [{"Condition": "IsProduction"}]
            },
            "IsProduction": {
                "Fn::Equals": [{"Ref": "Environment"}, "production"]
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
fn test_conditional_resource() {
    use cloudformation::aws::ec2;
    use stratosphere::template::ParameterType;
    use stratosphere::value::equals_string;

    let template = Template::build(|template| {
        let env = &template.parameter(
            "Environment",
            stratosphere::Parameter! {
                description: "Environment name",
                r#type: ParameterType::String,
            },
        );

        let is_prod = template.condition("IsProduction", equals_string(env, "production"));

        template.conditional_resource(
            "ProdVpc",
            &is_prod,
            ec2::VPC_ {
                cidr_block: Some("10.0.0.0/16".into()),
                enable_dns_hostnames: None,
                enable_dns_support: None,
                instance_tenancy: None,
                ipv4_ipam_pool_id: None,
                ipv4_netmask_length: None,
                tags: None,
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Conditions": {
            "IsProduction": {
                "Fn::Equals": [{"Ref": "Environment"}, "production"]
            }
        },
        "Parameters": {
            "Environment": {
                "Description": "Environment name",
                "Type": "String"
            }
        },
        "Resources": {
            "ProdVpc": {
                "Condition": "IsProduction",
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
fn test_conditional_output() {
    use stratosphere::template::ParameterType;
    use stratosphere::value::equals_string;

    let template = Template::build(|template| {
        let env = &template.parameter(
            "Environment",
            stratosphere::Parameter! {
                description: "Environment name",
                r#type: ParameterType::String,
            },
        );

        let is_prod = template.condition("IsProduction", equals_string(env, "production"));

        template.output(
            "EnvInfo",
            stratosphere::Output! {
                condition: &is_prod,
                description: "Production environment info",
                value: env,
            },
        );
    });

    let expected = serde_json::json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Conditions": {
            "IsProduction": {
                "Fn::Equals": [{"Ref": "Environment"}, "production"]
            }
        },
        "Outputs": {
            "EnvInfo": {
                "Condition": "IsProduction",
                "Description": "Production environment info",
                "Value": {"Ref": "Environment"}
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
