// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssociateCreatedArtifact`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`associate_created_artifact`](crate::client::Client::associate_created_artifact).
            ///
            /// `ParseStrictResponse` impl for `AssociateCreatedArtifact`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssociateCreatedArtifact {
    _private: ()
}
impl AssociateCreatedArtifact {
    /// Creates a new builder-style object to manufacture [`AssociateCreatedArtifactInput`](crate::input::AssociateCreatedArtifactInput).
    pub fn builder() -> crate::input::associate_created_artifact_input::Builder {
        crate::input::associate_created_artifact_input::Builder::default()
    }
    /// Creates a new `AssociateCreatedArtifact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateCreatedArtifact {
                type Output = std::result::Result<crate::output::AssociateCreatedArtifactOutput, crate::error::AssociateCreatedArtifactError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_associate_created_artifact_error(response)
                     } else {
                        crate::operation_deser::parse_associate_created_artifact_response(response)
                     }
                }
            }

/// Operation shape for `AssociateDiscoveredResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`associate_discovered_resource`](crate::client::Client::associate_discovered_resource).
            ///
            /// `ParseStrictResponse` impl for `AssociateDiscoveredResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssociateDiscoveredResource {
    _private: ()
}
impl AssociateDiscoveredResource {
    /// Creates a new builder-style object to manufacture [`AssociateDiscoveredResourceInput`](crate::input::AssociateDiscoveredResourceInput).
    pub fn builder() -> crate::input::associate_discovered_resource_input::Builder {
        crate::input::associate_discovered_resource_input::Builder::default()
    }
    /// Creates a new `AssociateDiscoveredResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateDiscoveredResource {
                type Output = std::result::Result<crate::output::AssociateDiscoveredResourceOutput, crate::error::AssociateDiscoveredResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_associate_discovered_resource_error(response)
                     } else {
                        crate::operation_deser::parse_associate_discovered_resource_response(response)
                     }
                }
            }

/// Operation shape for `CreateProgressUpdateStream`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_progress_update_stream`](crate::client::Client::create_progress_update_stream).
            ///
            /// `ParseStrictResponse` impl for `CreateProgressUpdateStream`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateProgressUpdateStream {
    _private: ()
}
impl CreateProgressUpdateStream {
    /// Creates a new builder-style object to manufacture [`CreateProgressUpdateStreamInput`](crate::input::CreateProgressUpdateStreamInput).
    pub fn builder() -> crate::input::create_progress_update_stream_input::Builder {
        crate::input::create_progress_update_stream_input::Builder::default()
    }
    /// Creates a new `CreateProgressUpdateStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateProgressUpdateStream {
                type Output = std::result::Result<crate::output::CreateProgressUpdateStreamOutput, crate::error::CreateProgressUpdateStreamError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_progress_update_stream_error(response)
                     } else {
                        crate::operation_deser::parse_create_progress_update_stream_response(response)
                     }
                }
            }

/// Operation shape for `DeleteProgressUpdateStream`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_progress_update_stream`](crate::client::Client::delete_progress_update_stream).
            ///
            /// `ParseStrictResponse` impl for `DeleteProgressUpdateStream`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteProgressUpdateStream {
    _private: ()
}
impl DeleteProgressUpdateStream {
    /// Creates a new builder-style object to manufacture [`DeleteProgressUpdateStreamInput`](crate::input::DeleteProgressUpdateStreamInput).
    pub fn builder() -> crate::input::delete_progress_update_stream_input::Builder {
        crate::input::delete_progress_update_stream_input::Builder::default()
    }
    /// Creates a new `DeleteProgressUpdateStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteProgressUpdateStream {
                type Output = std::result::Result<crate::output::DeleteProgressUpdateStreamOutput, crate::error::DeleteProgressUpdateStreamError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_progress_update_stream_error(response)
                     } else {
                        crate::operation_deser::parse_delete_progress_update_stream_response(response)
                     }
                }
            }

/// Operation shape for `DescribeApplicationState`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_application_state`](crate::client::Client::describe_application_state).
            ///
            /// `ParseStrictResponse` impl for `DescribeApplicationState`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeApplicationState {
    _private: ()
}
impl DescribeApplicationState {
    /// Creates a new builder-style object to manufacture [`DescribeApplicationStateInput`](crate::input::DescribeApplicationStateInput).
    pub fn builder() -> crate::input::describe_application_state_input::Builder {
        crate::input::describe_application_state_input::Builder::default()
    }
    /// Creates a new `DescribeApplicationState` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeApplicationState {
                type Output = std::result::Result<crate::output::DescribeApplicationStateOutput, crate::error::DescribeApplicationStateError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_application_state_error(response)
                     } else {
                        crate::operation_deser::parse_describe_application_state_response(response)
                     }
                }
            }

/// Operation shape for `DescribeMigrationTask`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_migration_task`](crate::client::Client::describe_migration_task).
            ///
            /// `ParseStrictResponse` impl for `DescribeMigrationTask`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeMigrationTask {
    _private: ()
}
impl DescribeMigrationTask {
    /// Creates a new builder-style object to manufacture [`DescribeMigrationTaskInput`](crate::input::DescribeMigrationTaskInput).
    pub fn builder() -> crate::input::describe_migration_task_input::Builder {
        crate::input::describe_migration_task_input::Builder::default()
    }
    /// Creates a new `DescribeMigrationTask` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeMigrationTask {
                type Output = std::result::Result<crate::output::DescribeMigrationTaskOutput, crate::error::DescribeMigrationTaskError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_migration_task_error(response)
                     } else {
                        crate::operation_deser::parse_describe_migration_task_response(response)
                     }
                }
            }

/// Operation shape for `DisassociateCreatedArtifact`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`disassociate_created_artifact`](crate::client::Client::disassociate_created_artifact).
            ///
            /// `ParseStrictResponse` impl for `DisassociateCreatedArtifact`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisassociateCreatedArtifact {
    _private: ()
}
impl DisassociateCreatedArtifact {
    /// Creates a new builder-style object to manufacture [`DisassociateCreatedArtifactInput`](crate::input::DisassociateCreatedArtifactInput).
    pub fn builder() -> crate::input::disassociate_created_artifact_input::Builder {
        crate::input::disassociate_created_artifact_input::Builder::default()
    }
    /// Creates a new `DisassociateCreatedArtifact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateCreatedArtifact {
                type Output = std::result::Result<crate::output::DisassociateCreatedArtifactOutput, crate::error::DisassociateCreatedArtifactError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_disassociate_created_artifact_error(response)
                     } else {
                        crate::operation_deser::parse_disassociate_created_artifact_response(response)
                     }
                }
            }

/// Operation shape for `DisassociateDiscoveredResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`disassociate_discovered_resource`](crate::client::Client::disassociate_discovered_resource).
            ///
            /// `ParseStrictResponse` impl for `DisassociateDiscoveredResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisassociateDiscoveredResource {
    _private: ()
}
impl DisassociateDiscoveredResource {
    /// Creates a new builder-style object to manufacture [`DisassociateDiscoveredResourceInput`](crate::input::DisassociateDiscoveredResourceInput).
    pub fn builder() -> crate::input::disassociate_discovered_resource_input::Builder {
        crate::input::disassociate_discovered_resource_input::Builder::default()
    }
    /// Creates a new `DisassociateDiscoveredResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateDiscoveredResource {
                type Output = std::result::Result<crate::output::DisassociateDiscoveredResourceOutput, crate::error::DisassociateDiscoveredResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_disassociate_discovered_resource_error(response)
                     } else {
                        crate::operation_deser::parse_disassociate_discovered_resource_response(response)
                     }
                }
            }

/// Operation shape for `ImportMigrationTask`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`import_migration_task`](crate::client::Client::import_migration_task).
            ///
            /// `ParseStrictResponse` impl for `ImportMigrationTask`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ImportMigrationTask {
    _private: ()
}
impl ImportMigrationTask {
    /// Creates a new builder-style object to manufacture [`ImportMigrationTaskInput`](crate::input::ImportMigrationTaskInput).
    pub fn builder() -> crate::input::import_migration_task_input::Builder {
        crate::input::import_migration_task_input::Builder::default()
    }
    /// Creates a new `ImportMigrationTask` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ImportMigrationTask {
                type Output = std::result::Result<crate::output::ImportMigrationTaskOutput, crate::error::ImportMigrationTaskError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_import_migration_task_error(response)
                     } else {
                        crate::operation_deser::parse_import_migration_task_response(response)
                     }
                }
            }

/// Operation shape for `ListApplicationStates`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_application_states`](crate::client::Client::list_application_states).
            ///
            /// `ParseStrictResponse` impl for `ListApplicationStates`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListApplicationStates {
    _private: ()
}
impl ListApplicationStates {
    /// Creates a new builder-style object to manufacture [`ListApplicationStatesInput`](crate::input::ListApplicationStatesInput).
    pub fn builder() -> crate::input::list_application_states_input::Builder {
        crate::input::list_application_states_input::Builder::default()
    }
    /// Creates a new `ListApplicationStates` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListApplicationStates {
                type Output = std::result::Result<crate::output::ListApplicationStatesOutput, crate::error::ListApplicationStatesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_application_states_error(response)
                     } else {
                        crate::operation_deser::parse_list_application_states_response(response)
                     }
                }
            }

/// Operation shape for `ListCreatedArtifacts`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_created_artifacts`](crate::client::Client::list_created_artifacts).
            ///
            /// `ParseStrictResponse` impl for `ListCreatedArtifacts`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListCreatedArtifacts {
    _private: ()
}
impl ListCreatedArtifacts {
    /// Creates a new builder-style object to manufacture [`ListCreatedArtifactsInput`](crate::input::ListCreatedArtifactsInput).
    pub fn builder() -> crate::input::list_created_artifacts_input::Builder {
        crate::input::list_created_artifacts_input::Builder::default()
    }
    /// Creates a new `ListCreatedArtifacts` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListCreatedArtifacts {
                type Output = std::result::Result<crate::output::ListCreatedArtifactsOutput, crate::error::ListCreatedArtifactsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_created_artifacts_error(response)
                     } else {
                        crate::operation_deser::parse_list_created_artifacts_response(response)
                     }
                }
            }

/// Operation shape for `ListDiscoveredResources`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_discovered_resources`](crate::client::Client::list_discovered_resources).
            ///
            /// `ParseStrictResponse` impl for `ListDiscoveredResources`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListDiscoveredResources {
    _private: ()
}
impl ListDiscoveredResources {
    /// Creates a new builder-style object to manufacture [`ListDiscoveredResourcesInput`](crate::input::ListDiscoveredResourcesInput).
    pub fn builder() -> crate::input::list_discovered_resources_input::Builder {
        crate::input::list_discovered_resources_input::Builder::default()
    }
    /// Creates a new `ListDiscoveredResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDiscoveredResources {
                type Output = std::result::Result<crate::output::ListDiscoveredResourcesOutput, crate::error::ListDiscoveredResourcesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_discovered_resources_error(response)
                     } else {
                        crate::operation_deser::parse_list_discovered_resources_response(response)
                     }
                }
            }

/// Operation shape for `ListMigrationTasks`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_migration_tasks`](crate::client::Client::list_migration_tasks).
            ///
            /// `ParseStrictResponse` impl for `ListMigrationTasks`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListMigrationTasks {
    _private: ()
}
impl ListMigrationTasks {
    /// Creates a new builder-style object to manufacture [`ListMigrationTasksInput`](crate::input::ListMigrationTasksInput).
    pub fn builder() -> crate::input::list_migration_tasks_input::Builder {
        crate::input::list_migration_tasks_input::Builder::default()
    }
    /// Creates a new `ListMigrationTasks` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListMigrationTasks {
                type Output = std::result::Result<crate::output::ListMigrationTasksOutput, crate::error::ListMigrationTasksError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_migration_tasks_error(response)
                     } else {
                        crate::operation_deser::parse_list_migration_tasks_response(response)
                     }
                }
            }

/// Operation shape for `ListProgressUpdateStreams`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_progress_update_streams`](crate::client::Client::list_progress_update_streams).
            ///
            /// `ParseStrictResponse` impl for `ListProgressUpdateStreams`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListProgressUpdateStreams {
    _private: ()
}
impl ListProgressUpdateStreams {
    /// Creates a new builder-style object to manufacture [`ListProgressUpdateStreamsInput`](crate::input::ListProgressUpdateStreamsInput).
    pub fn builder() -> crate::input::list_progress_update_streams_input::Builder {
        crate::input::list_progress_update_streams_input::Builder::default()
    }
    /// Creates a new `ListProgressUpdateStreams` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListProgressUpdateStreams {
                type Output = std::result::Result<crate::output::ListProgressUpdateStreamsOutput, crate::error::ListProgressUpdateStreamsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_progress_update_streams_error(response)
                     } else {
                        crate::operation_deser::parse_list_progress_update_streams_response(response)
                     }
                }
            }

/// Operation shape for `NotifyApplicationState`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`notify_application_state`](crate::client::Client::notify_application_state).
            ///
            /// `ParseStrictResponse` impl for `NotifyApplicationState`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct NotifyApplicationState {
    _private: ()
}
impl NotifyApplicationState {
    /// Creates a new builder-style object to manufacture [`NotifyApplicationStateInput`](crate::input::NotifyApplicationStateInput).
    pub fn builder() -> crate::input::notify_application_state_input::Builder {
        crate::input::notify_application_state_input::Builder::default()
    }
    /// Creates a new `NotifyApplicationState` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for NotifyApplicationState {
                type Output = std::result::Result<crate::output::NotifyApplicationStateOutput, crate::error::NotifyApplicationStateError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_notify_application_state_error(response)
                     } else {
                        crate::operation_deser::parse_notify_application_state_response(response)
                     }
                }
            }

/// Operation shape for `NotifyMigrationTaskState`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`notify_migration_task_state`](crate::client::Client::notify_migration_task_state).
            ///
            /// `ParseStrictResponse` impl for `NotifyMigrationTaskState`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct NotifyMigrationTaskState {
    _private: ()
}
impl NotifyMigrationTaskState {
    /// Creates a new builder-style object to manufacture [`NotifyMigrationTaskStateInput`](crate::input::NotifyMigrationTaskStateInput).
    pub fn builder() -> crate::input::notify_migration_task_state_input::Builder {
        crate::input::notify_migration_task_state_input::Builder::default()
    }
    /// Creates a new `NotifyMigrationTaskState` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for NotifyMigrationTaskState {
                type Output = std::result::Result<crate::output::NotifyMigrationTaskStateOutput, crate::error::NotifyMigrationTaskStateError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_notify_migration_task_state_error(response)
                     } else {
                        crate::operation_deser::parse_notify_migration_task_state_response(response)
                     }
                }
            }

/// Operation shape for `PutResourceAttributes`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_resource_attributes`](crate::client::Client::put_resource_attributes).
            ///
            /// `ParseStrictResponse` impl for `PutResourceAttributes`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutResourceAttributes {
    _private: ()
}
impl PutResourceAttributes {
    /// Creates a new builder-style object to manufacture [`PutResourceAttributesInput`](crate::input::PutResourceAttributesInput).
    pub fn builder() -> crate::input::put_resource_attributes_input::Builder {
        crate::input::put_resource_attributes_input::Builder::default()
    }
    /// Creates a new `PutResourceAttributes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutResourceAttributes {
                type Output = std::result::Result<crate::output::PutResourceAttributesOutput, crate::error::PutResourceAttributesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_resource_attributes_error(response)
                     } else {
                        crate::operation_deser::parse_put_resource_attributes_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

