// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateExperimentTemplate`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_experiment_template`](crate::client::Client::create_experiment_template).
            ///
            /// `ParseStrictResponse` impl for `CreateExperimentTemplate`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateExperimentTemplate {
    _private: ()
}
impl CreateExperimentTemplate {
    /// Creates a new builder-style object to manufacture [`CreateExperimentTemplateInput`](crate::input::CreateExperimentTemplateInput).
    pub fn builder() -> crate::input::create_experiment_template_input::Builder {
        crate::input::create_experiment_template_input::Builder::default()
    }
    /// Creates a new `CreateExperimentTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateExperimentTemplate {
                type Output = std::result::Result<crate::output::CreateExperimentTemplateOutput, crate::error::CreateExperimentTemplateError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_experiment_template_error(response)
                     } else {
                        crate::operation_deser::parse_create_experiment_template_response(response)
                     }
                }
            }

/// Operation shape for `DeleteExperimentTemplate`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_experiment_template`](crate::client::Client::delete_experiment_template).
            ///
            /// `ParseStrictResponse` impl for `DeleteExperimentTemplate`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteExperimentTemplate {
    _private: ()
}
impl DeleteExperimentTemplate {
    /// Creates a new builder-style object to manufacture [`DeleteExperimentTemplateInput`](crate::input::DeleteExperimentTemplateInput).
    pub fn builder() -> crate::input::delete_experiment_template_input::Builder {
        crate::input::delete_experiment_template_input::Builder::default()
    }
    /// Creates a new `DeleteExperimentTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteExperimentTemplate {
                type Output = std::result::Result<crate::output::DeleteExperimentTemplateOutput, crate::error::DeleteExperimentTemplateError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_experiment_template_error(response)
                     } else {
                        crate::operation_deser::parse_delete_experiment_template_response(response)
                     }
                }
            }

/// Operation shape for `GetAction`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_action`](crate::client::Client::get_action).
            ///
            /// `ParseStrictResponse` impl for `GetAction`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAction {
    _private: ()
}
impl GetAction {
    /// Creates a new builder-style object to manufacture [`GetActionInput`](crate::input::GetActionInput).
    pub fn builder() -> crate::input::get_action_input::Builder {
        crate::input::get_action_input::Builder::default()
    }
    /// Creates a new `GetAction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAction {
                type Output = std::result::Result<crate::output::GetActionOutput, crate::error::GetActionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_action_error(response)
                     } else {
                        crate::operation_deser::parse_get_action_response(response)
                     }
                }
            }

/// Operation shape for `GetExperiment`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_experiment`](crate::client::Client::get_experiment).
            ///
            /// `ParseStrictResponse` impl for `GetExperiment`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetExperiment {
    _private: ()
}
impl GetExperiment {
    /// Creates a new builder-style object to manufacture [`GetExperimentInput`](crate::input::GetExperimentInput).
    pub fn builder() -> crate::input::get_experiment_input::Builder {
        crate::input::get_experiment_input::Builder::default()
    }
    /// Creates a new `GetExperiment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetExperiment {
                type Output = std::result::Result<crate::output::GetExperimentOutput, crate::error::GetExperimentError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_experiment_error(response)
                     } else {
                        crate::operation_deser::parse_get_experiment_response(response)
                     }
                }
            }

/// Operation shape for `GetExperimentTemplate`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_experiment_template`](crate::client::Client::get_experiment_template).
            ///
            /// `ParseStrictResponse` impl for `GetExperimentTemplate`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetExperimentTemplate {
    _private: ()
}
impl GetExperimentTemplate {
    /// Creates a new builder-style object to manufacture [`GetExperimentTemplateInput`](crate::input::GetExperimentTemplateInput).
    pub fn builder() -> crate::input::get_experiment_template_input::Builder {
        crate::input::get_experiment_template_input::Builder::default()
    }
    /// Creates a new `GetExperimentTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetExperimentTemplate {
                type Output = std::result::Result<crate::output::GetExperimentTemplateOutput, crate::error::GetExperimentTemplateError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_experiment_template_error(response)
                     } else {
                        crate::operation_deser::parse_get_experiment_template_response(response)
                     }
                }
            }

/// Operation shape for `GetTargetResourceType`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_target_resource_type`](crate::client::Client::get_target_resource_type).
            ///
            /// `ParseStrictResponse` impl for `GetTargetResourceType`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetTargetResourceType {
    _private: ()
}
impl GetTargetResourceType {
    /// Creates a new builder-style object to manufacture [`GetTargetResourceTypeInput`](crate::input::GetTargetResourceTypeInput).
    pub fn builder() -> crate::input::get_target_resource_type_input::Builder {
        crate::input::get_target_resource_type_input::Builder::default()
    }
    /// Creates a new `GetTargetResourceType` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetTargetResourceType {
                type Output = std::result::Result<crate::output::GetTargetResourceTypeOutput, crate::error::GetTargetResourceTypeError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_target_resource_type_error(response)
                     } else {
                        crate::operation_deser::parse_get_target_resource_type_response(response)
                     }
                }
            }

/// Operation shape for `ListActions`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_actions`](crate::client::Client::list_actions).
            ///
            /// `ParseStrictResponse` impl for `ListActions`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListActions {
    _private: ()
}
impl ListActions {
    /// Creates a new builder-style object to manufacture [`ListActionsInput`](crate::input::ListActionsInput).
    pub fn builder() -> crate::input::list_actions_input::Builder {
        crate::input::list_actions_input::Builder::default()
    }
    /// Creates a new `ListActions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListActions {
                type Output = std::result::Result<crate::output::ListActionsOutput, crate::error::ListActionsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_actions_error(response)
                     } else {
                        crate::operation_deser::parse_list_actions_response(response)
                     }
                }
            }

/// Operation shape for `ListExperiments`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_experiments`](crate::client::Client::list_experiments).
            ///
            /// `ParseStrictResponse` impl for `ListExperiments`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListExperiments {
    _private: ()
}
impl ListExperiments {
    /// Creates a new builder-style object to manufacture [`ListExperimentsInput`](crate::input::ListExperimentsInput).
    pub fn builder() -> crate::input::list_experiments_input::Builder {
        crate::input::list_experiments_input::Builder::default()
    }
    /// Creates a new `ListExperiments` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListExperiments {
                type Output = std::result::Result<crate::output::ListExperimentsOutput, crate::error::ListExperimentsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_experiments_error(response)
                     } else {
                        crate::operation_deser::parse_list_experiments_response(response)
                     }
                }
            }

/// Operation shape for `ListExperimentTemplates`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_experiment_templates`](crate::client::Client::list_experiment_templates).
            ///
            /// `ParseStrictResponse` impl for `ListExperimentTemplates`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListExperimentTemplates {
    _private: ()
}
impl ListExperimentTemplates {
    /// Creates a new builder-style object to manufacture [`ListExperimentTemplatesInput`](crate::input::ListExperimentTemplatesInput).
    pub fn builder() -> crate::input::list_experiment_templates_input::Builder {
        crate::input::list_experiment_templates_input::Builder::default()
    }
    /// Creates a new `ListExperimentTemplates` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListExperimentTemplates {
                type Output = std::result::Result<crate::output::ListExperimentTemplatesOutput, crate::error::ListExperimentTemplatesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_experiment_templates_error(response)
                     } else {
                        crate::operation_deser::parse_list_experiment_templates_response(response)
                     }
                }
            }

/// Operation shape for `ListTagsForResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
            ///
            /// `ParseStrictResponse` impl for `ListTagsForResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: ()
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
                type Output = std::result::Result<crate::output::ListTagsForResourceOutput, crate::error::ListTagsForResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_tags_for_resource_error(response)
                     } else {
                        crate::operation_deser::parse_list_tags_for_resource_response(response)
                     }
                }
            }

/// Operation shape for `ListTargetResourceTypes`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_target_resource_types`](crate::client::Client::list_target_resource_types).
            ///
            /// `ParseStrictResponse` impl for `ListTargetResourceTypes`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTargetResourceTypes {
    _private: ()
}
impl ListTargetResourceTypes {
    /// Creates a new builder-style object to manufacture [`ListTargetResourceTypesInput`](crate::input::ListTargetResourceTypesInput).
    pub fn builder() -> crate::input::list_target_resource_types_input::Builder {
        crate::input::list_target_resource_types_input::Builder::default()
    }
    /// Creates a new `ListTargetResourceTypes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTargetResourceTypes {
                type Output = std::result::Result<crate::output::ListTargetResourceTypesOutput, crate::error::ListTargetResourceTypesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_target_resource_types_error(response)
                     } else {
                        crate::operation_deser::parse_list_target_resource_types_response(response)
                     }
                }
            }

/// Operation shape for `StartExperiment`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_experiment`](crate::client::Client::start_experiment).
            ///
            /// `ParseStrictResponse` impl for `StartExperiment`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartExperiment {
    _private: ()
}
impl StartExperiment {
    /// Creates a new builder-style object to manufacture [`StartExperimentInput`](crate::input::StartExperimentInput).
    pub fn builder() -> crate::input::start_experiment_input::Builder {
        crate::input::start_experiment_input::Builder::default()
    }
    /// Creates a new `StartExperiment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartExperiment {
                type Output = std::result::Result<crate::output::StartExperimentOutput, crate::error::StartExperimentError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_start_experiment_error(response)
                     } else {
                        crate::operation_deser::parse_start_experiment_response(response)
                     }
                }
            }

/// Operation shape for `StopExperiment`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`stop_experiment`](crate::client::Client::stop_experiment).
            ///
            /// `ParseStrictResponse` impl for `StopExperiment`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StopExperiment {
    _private: ()
}
impl StopExperiment {
    /// Creates a new builder-style object to manufacture [`StopExperimentInput`](crate::input::StopExperimentInput).
    pub fn builder() -> crate::input::stop_experiment_input::Builder {
        crate::input::stop_experiment_input::Builder::default()
    }
    /// Creates a new `StopExperiment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopExperiment {
                type Output = std::result::Result<crate::output::StopExperimentOutput, crate::error::StopExperimentError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_stop_experiment_error(response)
                     } else {
                        crate::operation_deser::parse_stop_experiment_response(response)
                     }
                }
            }

/// Operation shape for `TagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`tag_resource`](crate::client::Client::tag_resource).
            ///
            /// `ParseStrictResponse` impl for `TagResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagResource {
    _private: ()
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
                type Output = std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_tag_resource_error(response)
                     } else {
                        crate::operation_deser::parse_tag_resource_response(response)
                     }
                }
            }

/// Operation shape for `UntagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`untag_resource`](crate::client::Client::untag_resource).
            ///
            /// `ParseStrictResponse` impl for `UntagResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagResource {
    _private: ()
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
                type Output = std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_untag_resource_error(response)
                     } else {
                        crate::operation_deser::parse_untag_resource_response(response)
                     }
                }
            }

/// Operation shape for `UpdateExperimentTemplate`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_experiment_template`](crate::client::Client::update_experiment_template).
            ///
            /// `ParseStrictResponse` impl for `UpdateExperimentTemplate`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateExperimentTemplate {
    _private: ()
}
impl UpdateExperimentTemplate {
    /// Creates a new builder-style object to manufacture [`UpdateExperimentTemplateInput`](crate::input::UpdateExperimentTemplateInput).
    pub fn builder() -> crate::input::update_experiment_template_input::Builder {
        crate::input::update_experiment_template_input::Builder::default()
    }
    /// Creates a new `UpdateExperimentTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateExperimentTemplate {
                type Output = std::result::Result<crate::output::UpdateExperimentTemplateOutput, crate::error::UpdateExperimentTemplateError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_experiment_template_error(response)
                     } else {
                        crate::operation_deser::parse_update_experiment_template_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

