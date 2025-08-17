#![feature(default_field_values)]

use pretty_assertions::assert_eq;
use stratosphere::template::*;

use stratosphere::value::*;

stratosphere_generator::generate_service!();

#[test]
fn test_generation() {
    let template = Template::new()
        .resource(
            "SecurityGroupA",
            cloudformation::aws::ec2::SecurityGroup {
                GroupDescription: "Test Description".to_exp(),
                ..
            },
        )
        .resource(
            "SecurityGroupB",
            cloudformation::aws::ec2::SecurityGroup {
                GroupDescription: "Test Description".to_exp(),
                SecurityGroupIngress: Some(vec![
                    cloudformation::aws::ec2::securitygroup::Ingress {
                        IpProtocol: "tcp".to_exp(),
                        CidrIp: Some("127.0.0.1".to_exp()),
                        ..
                    },
                ]),
                ..
            },
        );

    assert_eq!(
        r#"{
  "Version": "2010-09-09",
  "Resources": {
    "SecurityGroupA": {
      "Type": "AWS::EC2::SecurityGroup",
      "Properties": {
        "GroupDescription": "Test Description"
      }
    },
    "SecurityGroupB": {
      "Type": "AWS::EC2::SecurityGroup",
      "Properties": {
        "GroupDescription": "Test Description",
        "SecurityGroupIngress": [
          {
            "CidrIp": "127.0.0.1",
            "IpProtocol": "tcp"
          }
        ]
      }
    }
  }
}"#,
        serde_json::to_string_pretty(&template).unwrap()
    )
}
