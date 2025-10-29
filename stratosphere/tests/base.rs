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
    });

    assert_eq!(EXPECTED, serde_json::to_string_pretty(&template).unwrap())
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
