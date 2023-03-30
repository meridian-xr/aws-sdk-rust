// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateEnvironment`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_environment`](crate::client::Client::create_environment).
            ///
            /// `ParseStrictResponse` impl for `CreateEnvironment`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateEnvironment {
    _private: ()
}
impl CreateEnvironment {
    /// Creates a new builder-style object to manufacture [`CreateEnvironmentInput`](crate::input::CreateEnvironmentInput).
    pub fn builder() -> crate::input::create_environment_input::Builder {
        crate::input::create_environment_input::Builder::default()
    }
    /// Creates a new `CreateEnvironment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateEnvironment {
                type Output = std::result::Result<crate::output::CreateEnvironmentOutput, crate::error::CreateEnvironmentError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_environment_error(response)
                     } else {
                        crate::operation_deser::parse_create_environment_response(response)
                     }
                }
            }

/// Operation shape for `DeleteEnvironment`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_environment`](crate::client::Client::delete_environment).
            ///
            /// `ParseStrictResponse` impl for `DeleteEnvironment`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteEnvironment {
    _private: ()
}
impl DeleteEnvironment {
    /// Creates a new builder-style object to manufacture [`DeleteEnvironmentInput`](crate::input::DeleteEnvironmentInput).
    pub fn builder() -> crate::input::delete_environment_input::Builder {
        crate::input::delete_environment_input::Builder::default()
    }
    /// Creates a new `DeleteEnvironment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteEnvironment {
                type Output = std::result::Result<crate::output::DeleteEnvironmentOutput, crate::error::DeleteEnvironmentError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_environment_error(response)
                     } else {
                        crate::operation_deser::parse_delete_environment_response(response)
                     }
                }
            }

/// Operation shape for `GetEnvironment`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_environment`](crate::client::Client::get_environment).
            ///
            /// `ParseStrictResponse` impl for `GetEnvironment`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetEnvironment {
    _private: ()
}
impl GetEnvironment {
    /// Creates a new builder-style object to manufacture [`GetEnvironmentInput`](crate::input::GetEnvironmentInput).
    pub fn builder() -> crate::input::get_environment_input::Builder {
        crate::input::get_environment_input::Builder::default()
    }
    /// Creates a new `GetEnvironment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEnvironment {
                type Output = std::result::Result<crate::output::GetEnvironmentOutput, crate::error::GetEnvironmentError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_environment_error(response)
                     } else {
                        crate::operation_deser::parse_get_environment_response(response)
                     }
                }
            }

/// Operation shape for `ListEnvironments`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_environments`](crate::client::Client::list_environments).
            ///
            /// `ParseStrictResponse` impl for `ListEnvironments`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListEnvironments {
    _private: ()
}
impl ListEnvironments {
    /// Creates a new builder-style object to manufacture [`ListEnvironmentsInput`](crate::input::ListEnvironmentsInput).
    pub fn builder() -> crate::input::list_environments_input::Builder {
        crate::input::list_environments_input::Builder::default()
    }
    /// Creates a new `ListEnvironments` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListEnvironments {
                type Output = std::result::Result<crate::output::ListEnvironmentsOutput, crate::error::ListEnvironmentsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_environments_error(response)
                     } else {
                        crate::operation_deser::parse_list_environments_response(response)
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

/// Operation shape for `UpdateEnvironment`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_environment`](crate::client::Client::update_environment).
            ///
            /// `ParseStrictResponse` impl for `UpdateEnvironment`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateEnvironment {
    _private: ()
}
impl UpdateEnvironment {
    /// Creates a new builder-style object to manufacture [`UpdateEnvironmentInput`](crate::input::UpdateEnvironmentInput).
    pub fn builder() -> crate::input::update_environment_input::Builder {
        crate::input::update_environment_input::Builder::default()
    }
    /// Creates a new `UpdateEnvironment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateEnvironment {
                type Output = std::result::Result<crate::output::UpdateEnvironmentOutput, crate::error::UpdateEnvironmentError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_environment_error(response)
                     } else {
                        crate::operation_deser::parse_update_environment_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

