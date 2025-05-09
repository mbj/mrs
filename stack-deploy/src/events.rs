// Clone of https://github.com/mbj/stack-deploy/blob/master/src/StackDeploy/Events.hs

use crate::types::{ClientRequestToken, StackId};
use aws_sdk_cloudformation::types::{ResourceStatus, StackEvent};

pub(crate) struct Poll {
    pub(crate) client_request_token: Option<ClientRequestToken>,
    pub(crate) stack_id: StackId,
    pub(crate) start_condition: fn(&StackEvent) -> bool,
    pub(crate) stop_condition: fn(&StackEvent) -> bool,
}

impl Poll {
    pub(crate) fn default(stack_id: StackId) -> Poll {
        Self {
            client_request_token: None,
            stack_id,
            start_condition: (|_event| false),
            stop_condition: (|_event| false),
        }
    }

    pub(crate) fn wait_for_remote_operation(
        remote_operation: crate::instance_spec::RemoteOperation,
    ) -> Poll {
        fn is_stack_resource_type(stack_event: &StackEvent) -> bool {
            stack_event.resource_type.as_deref() == Some("AWS::CloudFormation::Stack")
        }

        fn is_known_unknown(value: &str) -> bool {
            match value {
                // AWS boto misses these cases for nearly a decade!
                "UPDATE_COMPLETE_CLEANUP_IN_PROGRESS" => false,
                "UPDATE_ROLLBACK_COMPLETE_CLEANUP_IN_PROGRESS" => false,
                _other => panic!("Unexpected resource status: {:#?}", value),
            }
        }

        fn is_initial(stack_event: &StackEvent) -> bool {
            if is_stack_resource_type(stack_event) {
                match &stack_event.resource_status.as_ref().unwrap() {
                    ResourceStatus::CreateComplete => false,
                    ResourceStatus::DeleteComplete => false,
                    ResourceStatus::UpdateComplete => false,
                    ResourceStatus::UpdateRollbackComplete => false,
                    ResourceStatus::RollbackComplete => false,
                    ResourceStatus::CreateInProgress => true,
                    ResourceStatus::DeleteInProgress => true,
                    ResourceStatus::RollbackInProgress => true,
                    ResourceStatus::UpdateInProgress => false,
                    ResourceStatus::UpdateRollbackInProgress => true,
                    unknown => is_known_unknown(unknown.as_str()),
                }
            } else {
                false
            }
        }

        fn is_final(stack_event: &StackEvent) -> bool {
            if is_stack_resource_type(stack_event) {
                match &stack_event.resource_status.as_ref().unwrap() {
                    ResourceStatus::CreateComplete => true,
                    ResourceStatus::DeleteComplete => true,
                    ResourceStatus::UpdateComplete => true,
                    ResourceStatus::UpdateRollbackComplete => true,
                    ResourceStatus::RollbackComplete => true,
                    ResourceStatus::CreateInProgress => false,
                    ResourceStatus::DeleteInProgress => false,
                    ResourceStatus::RollbackInProgress => false,
                    ResourceStatus::UpdateInProgress => false,
                    ResourceStatus::UpdateRollbackInProgress => false,
                    unknown => is_known_unknown(unknown.as_str()),
                }
            } else {
                false
            }
        }

        Self {
            client_request_token: Some(remote_operation.client_request_token),
            stack_id: remote_operation.stack_id,
            start_condition: is_initial,
            stop_condition: is_final,
        }
    }
}

enum ReadPagesStatus {
    Continue,
    Stop,
}

impl Poll {
    pub(crate) async fn run(
        &self,
        cloudformation: &aws_sdk_cloudformation::client::Client,
        action: fn(&StackEvent),
    ) {
        let mut initial_pages = vec![];

        self.read_pages(cloudformation, |stack_events| {
            let mut page = vec![];

            let mut status = ReadPagesStatus::Continue;

            for stack_event in stack_events.iter() {
                if self.allow_client_request_token(stack_event) {
                    page.push(stack_event.clone());
                    if (self.start_condition)(stack_event) {
                        status = ReadPagesStatus::Stop;
                        break;
                    }
                }
            }

            initial_pages.push(page);

            status
        })
        .await;

        for stack_events in initial_pages.iter().rev() {
            for stack_event in stack_events.iter().rev() {
                action(stack_event);
                if (self.stop_condition)(stack_event) {
                    return;
                }
            }
        }

        let mut youngest: Option<String> = initial_pages
            .first()
            .unwrap()
            .first()
            .unwrap()
            .event_id
            .clone();

        loop {
            let mut new_pages = vec![];

            self.read_pages(cloudformation, |stack_events| {
                let mut page = vec![];

                let mut status = ReadPagesStatus::Continue;

                for stack_event in stack_events.iter() {
                    if stack_event.event_id == youngest {
                        status = ReadPagesStatus::Stop;
                        break;
                    }
                    page.push(stack_event.clone())
                }

                if !page.is_empty() {
                    new_pages.push(page);
                }

                status
            })
            .await;

            for stack_events in new_pages.iter().rev() {
                for stack_event in stack_events.iter().rev() {
                    if self.allow_client_request_token(stack_event) {
                        action(stack_event);
                        if (self.stop_condition)(stack_event) {
                            return;
                        }
                    }
                }
            }

            if !new_pages.is_empty() {
                youngest = new_pages.first().unwrap().first().unwrap().event_id.clone();
            }

            tokio::time::sleep(std::time::Duration::new(1, 0)).await
        }
    }

    async fn read_pages(
        &self,
        cloudformation: &aws_sdk_cloudformation::client::Client,
        mut process_page: impl FnMut(Vec<StackEvent>) -> ReadPagesStatus,
    ) {
        let mut next_token = None;

        loop {
            let output = cloudformation
                .describe_stack_events()
                .stack_name(&self.stack_id)
                .set_next_token(next_token)
                .send()
                .await
                .unwrap();

            match process_page(output.stack_events.unwrap()) {
                ReadPagesStatus::Continue => (),
                ReadPagesStatus::Stop => break,
            }

            match output.next_token {
                None => break,
                Some(output_next_token) => next_token = Some(output_next_token),
            }
        }
    }

    fn allow_client_request_token(&self, stack_event: &StackEvent) -> bool {
        match (
            &self.client_request_token,
            &stack_event.client_request_token,
        ) {
            (Some(ClientRequestToken(expected)), Some(provided)) => expected == provided,
            _other => true,
        }
    }
}

pub(crate) fn print_event(stack_event: &StackEvent) {
    log::info!(
        "{} {} {} {} {}",
        stack_event
            .timestamp
            .map(|value| value
                .fmt(aws_sdk_cloudformation::primitives::DateTimeFormat::DateTime)
                .unwrap())
            .unwrap_or_else(|| "[event-time-missing]".to_string()),
        stack_event
            .physical_resource_id
            .as_deref()
            .unwrap_or("[unknown-physical-resource-id]"),
        stack_event
            .logical_resource_id
            .as_deref()
            .unwrap_or("[unknown-logical-resource-id]"),
        stack_event
            .resource_type
            .as_deref()
            .unwrap_or("[unknown-resource-type]"),
        stack_event
            .resource_status
            .as_ref()
            .map(|value| value.as_str())
            .unwrap_or("[unknown-resource-status]"),
    );

    if let Some(ref message) = stack_event.resource_status_reason {
        log::info!("- {}", message)
    }
}
