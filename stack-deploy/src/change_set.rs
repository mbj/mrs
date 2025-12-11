use aws_sdk_cloudformation::operation::describe_change_set::DescribeChangeSetOutput;
use aws_sdk_cloudformation::types::{
    ResourceChange, ResourceChangeDetail, ResourceTargetDefinition,
};
use std::fmt::Display;
use std::io::{self, Write};

struct IndentWriter<'a> {
    writer: &'a mut dyn Write,
    level: usize,
    indent_str: &'static str,
}

impl<'a> IndentWriter<'a> {
    fn new(writer: &'a mut dyn Write) -> Self {
        IndentWriter {
            writer,
            level: 0,
            indent_str: "  ",
        }
    }

    fn indent(&mut self) -> IndentWriter<'_> {
        IndentWriter {
            writer: self.writer,
            level: self.level + 1,
            indent_str: self.indent_str,
        }
    }

    fn write_line<T: Display>(&mut self, value: T) -> io::Result<()> {
        for _ in 0..self.level {
            write!(self.writer, "{}", self.indent_str)?;
        }
        writeln!(self.writer, "{value}")
    }
}

pub fn print_change_set_output(output: &DescribeChangeSetOutput) {
    let mut stdout = io::stdout();
    let mut writer = IndentWriter::new(&mut stdout);
    writer.write_line("=== Change Set Details ===").unwrap();
    print_basic_details(output, &mut writer);
    print_changes(output, &mut writer);
}

fn print_basic_details(output: &DescribeChangeSetOutput, writer: &mut IndentWriter<'_>) {
    if let Some(name) = &output.change_set_name {
        writer
            .write_line(format!("Change Set Name: {name}"))
            .unwrap();
    }
    if let Some(id) = &output.change_set_id {
        writer.write_line(format!("Change Set ID: {id}")).unwrap();
    }
    if let Some(stack_name) = &output.stack_name {
        writer
            .write_line(format!("Stack Name: {stack_name}"))
            .unwrap();
    }
    if let Some(execution_status) = &output.execution_status {
        writer
            .write_line(format!("Execution Status: {execution_status}"))
            .unwrap();
    }
    if let Some(status) = &output.status {
        writer.write_line(format!("Status: {status:?}")).unwrap();
    }
    if let Some(status_reason) = &output.status_reason {
        writer
            .write_line(format!("Status Reason: {status_reason}"))
            .unwrap();
    }
    if let Some(creation_time) = &output.creation_time {
        writer
            .write_line(format!("Creation Time: {creation_time}"))
            .unwrap();
    }
    if let Some(description) = &output.description {
        writer
            .write_line(format!("Description: {description}"))
            .unwrap();
    }
}

fn print_changes(output: &DescribeChangeSetOutput, writer: &mut IndentWriter<'_>) {
    if let Some(changes) = &output.changes {
        for change in changes {
            if let Some(resource_change) = &change.resource_change {
                print_resource_change(resource_change, writer);
            }
        }
    }
}

fn print_resource_change(resource_change: &ResourceChange, writer: &mut IndentWriter<'_>) {
    let logical_resource_id = resource_change.logical_resource_id.as_deref().unwrap();
    let resource_type = resource_change.resource_type.as_deref().unwrap();
    let action = resource_change.action.as_ref().unwrap();

    let mut indented_writer = writer.indent();
    indented_writer
        .write_line(format!("{logical_resource_id}:"))
        .unwrap();

    let mut detail_writer = indented_writer.indent();
    detail_writer
        .write_line(format!("Resource Type: {resource_type}"))
        .unwrap();
    detail_writer
        .write_line(format!("Action: {action:?}"))
        .unwrap();

    if let Some(replacement) = &resource_change.replacement {
        detail_writer
            .write_line(format!("Replacement: {replacement:?}"))
            .unwrap();
    }

    if let Some(details) = &resource_change.details
        && !details.is_empty()
    {
        detail_writer.write_line("Details:").unwrap();
        for detail in details {
            print_resource_change_detail(detail, &mut detail_writer);
        }
    }
}

fn print_resource_change_detail(detail: &ResourceChangeDetail, writer: &mut IndentWriter<'_>) {
    let mut indented_writer = writer.indent();
    indented_writer.write_line("Detail:").unwrap();

    let mut inner_writer = indented_writer.indent();
    if let Some(target) = &detail.target {
        print_target(target, &mut inner_writer);
    }
    if let Some(change_source) = &detail.change_source {
        inner_writer
            .write_line(format!("Change Source: {change_source:?}"))
            .unwrap();
    }
    if let Some(causing_entity) = &detail.causing_entity {
        inner_writer
            .write_line(format!("Causing Entity: {causing_entity}"))
            .unwrap();
    }
    if let Some(evaluation) = &detail.evaluation {
        inner_writer
            .write_line(format!("Evaluation: {evaluation:?}"))
            .unwrap();
    }
}

fn print_target(target: &ResourceTargetDefinition, writer: &mut IndentWriter<'_>) {
    writer.write_line("Target:").unwrap();

    let mut indented_writer = writer.indent();
    if let Some(attribute) = &target.attribute {
        indented_writer
            .write_line(format!("Attribute: {attribute:?}"))
            .unwrap();
    }
    print_optional_field("Name", &target.name, &mut indented_writer);
    if let Some(requires_recreation) = &target.requires_recreation {
        indented_writer
            .write_line(format!("Requires Recreation: {requires_recreation}"))
            .unwrap();
    }
    print_optional_field("Path", &target.path, &mut indented_writer);
    print_optional_field("Before Value", &target.before_value, &mut indented_writer);
    print_optional_field("After Value", &target.after_value, &mut indented_writer);
    print_optional_field(
        "Attribute Change Type",
        &target.attribute_change_type,
        &mut indented_writer,
    );
}

fn print_optional_field<T: Display>(
    field_name: &str,
    field: &Option<T>,
    writer: &mut IndentWriter<'_>,
) {
    if let Some(value) = field {
        writer.write_line(format!("{field_name}: {value}")).unwrap();
    }
}
