// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CancelChangeSet`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`cancel_change_set`](crate::client::Client::cancel_change_set).
            ///
            /// `ParseStrictResponse` impl for `CancelChangeSet`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CancelChangeSet {
    _private: ()
}
impl CancelChangeSet {
    /// Creates a new builder-style object to manufacture [`CancelChangeSetInput`](crate::input::CancelChangeSetInput).
    pub fn builder() -> crate::input::cancel_change_set_input::Builder {
        crate::input::cancel_change_set_input::Builder::default()
    }
    /// Creates a new `CancelChangeSet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelChangeSet {
                type Output = std::result::Result<crate::output::CancelChangeSetOutput, crate::error::CancelChangeSetError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_cancel_change_set_error(response)
                     } else {
                        crate::operation_deser::parse_cancel_change_set_response(response)
                     }
                }
            }

/// Operation shape for `DescribeChangeSet`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_change_set`](crate::client::Client::describe_change_set).
            ///
            /// `ParseStrictResponse` impl for `DescribeChangeSet`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeChangeSet {
    _private: ()
}
impl DescribeChangeSet {
    /// Creates a new builder-style object to manufacture [`DescribeChangeSetInput`](crate::input::DescribeChangeSetInput).
    pub fn builder() -> crate::input::describe_change_set_input::Builder {
        crate::input::describe_change_set_input::Builder::default()
    }
    /// Creates a new `DescribeChangeSet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeChangeSet {
                type Output = std::result::Result<crate::output::DescribeChangeSetOutput, crate::error::DescribeChangeSetError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_change_set_error(response)
                     } else {
                        crate::operation_deser::parse_describe_change_set_response(response)
                     }
                }
            }

/// Operation shape for `DescribeEntity`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_entity`](crate::client::Client::describe_entity).
            ///
            /// `ParseStrictResponse` impl for `DescribeEntity`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEntity {
    _private: ()
}
impl DescribeEntity {
    /// Creates a new builder-style object to manufacture [`DescribeEntityInput`](crate::input::DescribeEntityInput).
    pub fn builder() -> crate::input::describe_entity_input::Builder {
        crate::input::describe_entity_input::Builder::default()
    }
    /// Creates a new `DescribeEntity` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEntity {
                type Output = std::result::Result<crate::output::DescribeEntityOutput, crate::error::DescribeEntityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_entity_error(response)
                     } else {
                        crate::operation_deser::parse_describe_entity_response(response)
                     }
                }
            }

/// Operation shape for `ListChangeSets`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_change_sets`](crate::client::Client::list_change_sets).
            ///
            /// `ParseStrictResponse` impl for `ListChangeSets`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListChangeSets {
    _private: ()
}
impl ListChangeSets {
    /// Creates a new builder-style object to manufacture [`ListChangeSetsInput`](crate::input::ListChangeSetsInput).
    pub fn builder() -> crate::input::list_change_sets_input::Builder {
        crate::input::list_change_sets_input::Builder::default()
    }
    /// Creates a new `ListChangeSets` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListChangeSets {
                type Output = std::result::Result<crate::output::ListChangeSetsOutput, crate::error::ListChangeSetsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_change_sets_error(response)
                     } else {
                        crate::operation_deser::parse_list_change_sets_response(response)
                     }
                }
            }

/// Operation shape for `ListEntities`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_entities`](crate::client::Client::list_entities).
            ///
            /// `ParseStrictResponse` impl for `ListEntities`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListEntities {
    _private: ()
}
impl ListEntities {
    /// Creates a new builder-style object to manufacture [`ListEntitiesInput`](crate::input::ListEntitiesInput).
    pub fn builder() -> crate::input::list_entities_input::Builder {
        crate::input::list_entities_input::Builder::default()
    }
    /// Creates a new `ListEntities` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListEntities {
                type Output = std::result::Result<crate::output::ListEntitiesOutput, crate::error::ListEntitiesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_entities_error(response)
                     } else {
                        crate::operation_deser::parse_list_entities_response(response)
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

/// Operation shape for `StartChangeSet`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_change_set`](crate::client::Client::start_change_set).
            ///
            /// `ParseStrictResponse` impl for `StartChangeSet`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartChangeSet {
    _private: ()
}
impl StartChangeSet {
    /// Creates a new builder-style object to manufacture [`StartChangeSetInput`](crate::input::StartChangeSetInput).
    pub fn builder() -> crate::input::start_change_set_input::Builder {
        crate::input::start_change_set_input::Builder::default()
    }
    /// Creates a new `StartChangeSet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartChangeSet {
                type Output = std::result::Result<crate::output::StartChangeSetOutput, crate::error::StartChangeSetError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_start_change_set_error(response)
                     } else {
                        crate::operation_deser::parse_start_change_set_response(response)
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

/// Operation customization and supporting types
pub mod customize;

