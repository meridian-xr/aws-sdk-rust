// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssociateAttributeGroup`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`associate_attribute_group`](crate::client::Client::associate_attribute_group).
            ///
            /// `ParseStrictResponse` impl for `AssociateAttributeGroup`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssociateAttributeGroup {
    _private: ()
}
impl AssociateAttributeGroup {
    /// Creates a new builder-style object to manufacture [`AssociateAttributeGroupInput`](crate::input::AssociateAttributeGroupInput).
    pub fn builder() -> crate::input::associate_attribute_group_input::Builder {
        crate::input::associate_attribute_group_input::Builder::default()
    }
    /// Creates a new `AssociateAttributeGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateAttributeGroup {
                type Output = std::result::Result<crate::output::AssociateAttributeGroupOutput, crate::error::AssociateAttributeGroupError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_associate_attribute_group_error(response)
                     } else {
                        crate::operation_deser::parse_associate_attribute_group_response(response)
                     }
                }
            }

/// Operation shape for `AssociateResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`associate_resource`](crate::client::Client::associate_resource).
            ///
            /// `ParseStrictResponse` impl for `AssociateResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssociateResource {
    _private: ()
}
impl AssociateResource {
    /// Creates a new builder-style object to manufacture [`AssociateResourceInput`](crate::input::AssociateResourceInput).
    pub fn builder() -> crate::input::associate_resource_input::Builder {
        crate::input::associate_resource_input::Builder::default()
    }
    /// Creates a new `AssociateResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateResource {
                type Output = std::result::Result<crate::output::AssociateResourceOutput, crate::error::AssociateResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_associate_resource_error(response)
                     } else {
                        crate::operation_deser::parse_associate_resource_response(response)
                     }
                }
            }

/// Operation shape for `CreateApplication`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_application`](crate::client::Client::create_application).
            ///
            /// `ParseStrictResponse` impl for `CreateApplication`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateApplication {
    _private: ()
}
impl CreateApplication {
    /// Creates a new builder-style object to manufacture [`CreateApplicationInput`](crate::input::CreateApplicationInput).
    pub fn builder() -> crate::input::create_application_input::Builder {
        crate::input::create_application_input::Builder::default()
    }
    /// Creates a new `CreateApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateApplication {
                type Output = std::result::Result<crate::output::CreateApplicationOutput, crate::error::CreateApplicationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 201 {
                        crate::operation_deser::parse_create_application_error(response)
                     } else {
                        crate::operation_deser::parse_create_application_response(response)
                     }
                }
            }

/// Operation shape for `CreateAttributeGroup`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_attribute_group`](crate::client::Client::create_attribute_group).
            ///
            /// `ParseStrictResponse` impl for `CreateAttributeGroup`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateAttributeGroup {
    _private: ()
}
impl CreateAttributeGroup {
    /// Creates a new builder-style object to manufacture [`CreateAttributeGroupInput`](crate::input::CreateAttributeGroupInput).
    pub fn builder() -> crate::input::create_attribute_group_input::Builder {
        crate::input::create_attribute_group_input::Builder::default()
    }
    /// Creates a new `CreateAttributeGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateAttributeGroup {
                type Output = std::result::Result<crate::output::CreateAttributeGroupOutput, crate::error::CreateAttributeGroupError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 201 {
                        crate::operation_deser::parse_create_attribute_group_error(response)
                     } else {
                        crate::operation_deser::parse_create_attribute_group_response(response)
                     }
                }
            }

/// Operation shape for `DeleteApplication`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_application`](crate::client::Client::delete_application).
            ///
            /// `ParseStrictResponse` impl for `DeleteApplication`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteApplication {
    _private: ()
}
impl DeleteApplication {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationInput`](crate::input::DeleteApplicationInput).
    pub fn builder() -> crate::input::delete_application_input::Builder {
        crate::input::delete_application_input::Builder::default()
    }
    /// Creates a new `DeleteApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteApplication {
                type Output = std::result::Result<crate::output::DeleteApplicationOutput, crate::error::DeleteApplicationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_application_error(response)
                     } else {
                        crate::operation_deser::parse_delete_application_response(response)
                     }
                }
            }

/// Operation shape for `DeleteAttributeGroup`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_attribute_group`](crate::client::Client::delete_attribute_group).
            ///
            /// `ParseStrictResponse` impl for `DeleteAttributeGroup`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteAttributeGroup {
    _private: ()
}
impl DeleteAttributeGroup {
    /// Creates a new builder-style object to manufacture [`DeleteAttributeGroupInput`](crate::input::DeleteAttributeGroupInput).
    pub fn builder() -> crate::input::delete_attribute_group_input::Builder {
        crate::input::delete_attribute_group_input::Builder::default()
    }
    /// Creates a new `DeleteAttributeGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAttributeGroup {
                type Output = std::result::Result<crate::output::DeleteAttributeGroupOutput, crate::error::DeleteAttributeGroupError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_attribute_group_error(response)
                     } else {
                        crate::operation_deser::parse_delete_attribute_group_response(response)
                     }
                }
            }

/// Operation shape for `DisassociateAttributeGroup`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`disassociate_attribute_group`](crate::client::Client::disassociate_attribute_group).
            ///
            /// `ParseStrictResponse` impl for `DisassociateAttributeGroup`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisassociateAttributeGroup {
    _private: ()
}
impl DisassociateAttributeGroup {
    /// Creates a new builder-style object to manufacture [`DisassociateAttributeGroupInput`](crate::input::DisassociateAttributeGroupInput).
    pub fn builder() -> crate::input::disassociate_attribute_group_input::Builder {
        crate::input::disassociate_attribute_group_input::Builder::default()
    }
    /// Creates a new `DisassociateAttributeGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateAttributeGroup {
                type Output = std::result::Result<crate::output::DisassociateAttributeGroupOutput, crate::error::DisassociateAttributeGroupError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_disassociate_attribute_group_error(response)
                     } else {
                        crate::operation_deser::parse_disassociate_attribute_group_response(response)
                     }
                }
            }

/// Operation shape for `DisassociateResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`disassociate_resource`](crate::client::Client::disassociate_resource).
            ///
            /// `ParseStrictResponse` impl for `DisassociateResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisassociateResource {
    _private: ()
}
impl DisassociateResource {
    /// Creates a new builder-style object to manufacture [`DisassociateResourceInput`](crate::input::DisassociateResourceInput).
    pub fn builder() -> crate::input::disassociate_resource_input::Builder {
        crate::input::disassociate_resource_input::Builder::default()
    }
    /// Creates a new `DisassociateResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateResource {
                type Output = std::result::Result<crate::output::DisassociateResourceOutput, crate::error::DisassociateResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_disassociate_resource_error(response)
                     } else {
                        crate::operation_deser::parse_disassociate_resource_response(response)
                     }
                }
            }

/// Operation shape for `GetApplication`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_application`](crate::client::Client::get_application).
            ///
            /// `ParseStrictResponse` impl for `GetApplication`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetApplication {
    _private: ()
}
impl GetApplication {
    /// Creates a new builder-style object to manufacture [`GetApplicationInput`](crate::input::GetApplicationInput).
    pub fn builder() -> crate::input::get_application_input::Builder {
        crate::input::get_application_input::Builder::default()
    }
    /// Creates a new `GetApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetApplication {
                type Output = std::result::Result<crate::output::GetApplicationOutput, crate::error::GetApplicationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_application_error(response)
                     } else {
                        crate::operation_deser::parse_get_application_response(response)
                     }
                }
            }

/// Operation shape for `GetAssociatedResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_associated_resource`](crate::client::Client::get_associated_resource).
            ///
            /// `ParseStrictResponse` impl for `GetAssociatedResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAssociatedResource {
    _private: ()
}
impl GetAssociatedResource {
    /// Creates a new builder-style object to manufacture [`GetAssociatedResourceInput`](crate::input::GetAssociatedResourceInput).
    pub fn builder() -> crate::input::get_associated_resource_input::Builder {
        crate::input::get_associated_resource_input::Builder::default()
    }
    /// Creates a new `GetAssociatedResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAssociatedResource {
                type Output = std::result::Result<crate::output::GetAssociatedResourceOutput, crate::error::GetAssociatedResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_associated_resource_error(response)
                     } else {
                        crate::operation_deser::parse_get_associated_resource_response(response)
                     }
                }
            }

/// Operation shape for `GetAttributeGroup`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_attribute_group`](crate::client::Client::get_attribute_group).
            ///
            /// `ParseStrictResponse` impl for `GetAttributeGroup`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAttributeGroup {
    _private: ()
}
impl GetAttributeGroup {
    /// Creates a new builder-style object to manufacture [`GetAttributeGroupInput`](crate::input::GetAttributeGroupInput).
    pub fn builder() -> crate::input::get_attribute_group_input::Builder {
        crate::input::get_attribute_group_input::Builder::default()
    }
    /// Creates a new `GetAttributeGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAttributeGroup {
                type Output = std::result::Result<crate::output::GetAttributeGroupOutput, crate::error::GetAttributeGroupError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_attribute_group_error(response)
                     } else {
                        crate::operation_deser::parse_get_attribute_group_response(response)
                     }
                }
            }

/// Operation shape for `GetConfiguration`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_configuration`](crate::client::Client::get_configuration).
            ///
            /// `ParseStrictResponse` impl for `GetConfiguration`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetConfiguration {
    _private: ()
}
impl GetConfiguration {
    /// Creates a new builder-style object to manufacture [`GetConfigurationInput`](crate::input::GetConfigurationInput).
    pub fn builder() -> crate::input::get_configuration_input::Builder {
        crate::input::get_configuration_input::Builder::default()
    }
    /// Creates a new `GetConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetConfiguration {
                type Output = std::result::Result<crate::output::GetConfigurationOutput, crate::error::GetConfigurationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_configuration_error(response)
                     } else {
                        crate::operation_deser::parse_get_configuration_response(response)
                     }
                }
            }

/// Operation shape for `ListApplications`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_applications`](crate::client::Client::list_applications).
            ///
            /// `ParseStrictResponse` impl for `ListApplications`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListApplications {
    _private: ()
}
impl ListApplications {
    /// Creates a new builder-style object to manufacture [`ListApplicationsInput`](crate::input::ListApplicationsInput).
    pub fn builder() -> crate::input::list_applications_input::Builder {
        crate::input::list_applications_input::Builder::default()
    }
    /// Creates a new `ListApplications` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListApplications {
                type Output = std::result::Result<crate::output::ListApplicationsOutput, crate::error::ListApplicationsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_applications_error(response)
                     } else {
                        crate::operation_deser::parse_list_applications_response(response)
                     }
                }
            }

/// Operation shape for `ListAssociatedAttributeGroups`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_associated_attribute_groups`](crate::client::Client::list_associated_attribute_groups).
            ///
            /// `ParseStrictResponse` impl for `ListAssociatedAttributeGroups`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListAssociatedAttributeGroups {
    _private: ()
}
impl ListAssociatedAttributeGroups {
    /// Creates a new builder-style object to manufacture [`ListAssociatedAttributeGroupsInput`](crate::input::ListAssociatedAttributeGroupsInput).
    pub fn builder() -> crate::input::list_associated_attribute_groups_input::Builder {
        crate::input::list_associated_attribute_groups_input::Builder::default()
    }
    /// Creates a new `ListAssociatedAttributeGroups` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAssociatedAttributeGroups {
                type Output = std::result::Result<crate::output::ListAssociatedAttributeGroupsOutput, crate::error::ListAssociatedAttributeGroupsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_associated_attribute_groups_error(response)
                     } else {
                        crate::operation_deser::parse_list_associated_attribute_groups_response(response)
                     }
                }
            }

/// Operation shape for `ListAssociatedResources`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_associated_resources`](crate::client::Client::list_associated_resources).
            ///
            /// `ParseStrictResponse` impl for `ListAssociatedResources`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListAssociatedResources {
    _private: ()
}
impl ListAssociatedResources {
    /// Creates a new builder-style object to manufacture [`ListAssociatedResourcesInput`](crate::input::ListAssociatedResourcesInput).
    pub fn builder() -> crate::input::list_associated_resources_input::Builder {
        crate::input::list_associated_resources_input::Builder::default()
    }
    /// Creates a new `ListAssociatedResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAssociatedResources {
                type Output = std::result::Result<crate::output::ListAssociatedResourcesOutput, crate::error::ListAssociatedResourcesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_associated_resources_error(response)
                     } else {
                        crate::operation_deser::parse_list_associated_resources_response(response)
                     }
                }
            }

/// Operation shape for `ListAttributeGroups`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_attribute_groups`](crate::client::Client::list_attribute_groups).
            ///
            /// `ParseStrictResponse` impl for `ListAttributeGroups`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListAttributeGroups {
    _private: ()
}
impl ListAttributeGroups {
    /// Creates a new builder-style object to manufacture [`ListAttributeGroupsInput`](crate::input::ListAttributeGroupsInput).
    pub fn builder() -> crate::input::list_attribute_groups_input::Builder {
        crate::input::list_attribute_groups_input::Builder::default()
    }
    /// Creates a new `ListAttributeGroups` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAttributeGroups {
                type Output = std::result::Result<crate::output::ListAttributeGroupsOutput, crate::error::ListAttributeGroupsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_attribute_groups_error(response)
                     } else {
                        crate::operation_deser::parse_list_attribute_groups_response(response)
                     }
                }
            }

/// Operation shape for `ListAttributeGroupsForApplication`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_attribute_groups_for_application`](crate::client::Client::list_attribute_groups_for_application).
            ///
            /// `ParseStrictResponse` impl for `ListAttributeGroupsForApplication`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListAttributeGroupsForApplication {
    _private: ()
}
impl ListAttributeGroupsForApplication {
    /// Creates a new builder-style object to manufacture [`ListAttributeGroupsForApplicationInput`](crate::input::ListAttributeGroupsForApplicationInput).
    pub fn builder() -> crate::input::list_attribute_groups_for_application_input::Builder {
        crate::input::list_attribute_groups_for_application_input::Builder::default()
    }
    /// Creates a new `ListAttributeGroupsForApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAttributeGroupsForApplication {
                type Output = std::result::Result<crate::output::ListAttributeGroupsForApplicationOutput, crate::error::ListAttributeGroupsForApplicationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_attribute_groups_for_application_error(response)
                     } else {
                        crate::operation_deser::parse_list_attribute_groups_for_application_response(response)
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

/// Operation shape for `PutConfiguration`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_configuration`](crate::client::Client::put_configuration).
            ///
            /// `ParseStrictResponse` impl for `PutConfiguration`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutConfiguration {
    _private: ()
}
impl PutConfiguration {
    /// Creates a new builder-style object to manufacture [`PutConfigurationInput`](crate::input::PutConfigurationInput).
    pub fn builder() -> crate::input::put_configuration_input::Builder {
        crate::input::put_configuration_input::Builder::default()
    }
    /// Creates a new `PutConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutConfiguration {
                type Output = std::result::Result<crate::output::PutConfigurationOutput, crate::error::PutConfigurationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_configuration_error(response)
                     } else {
                        crate::operation_deser::parse_put_configuration_response(response)
                     }
                }
            }

/// Operation shape for `SyncResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`sync_resource`](crate::client::Client::sync_resource).
            ///
            /// `ParseStrictResponse` impl for `SyncResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct SyncResource {
    _private: ()
}
impl SyncResource {
    /// Creates a new builder-style object to manufacture [`SyncResourceInput`](crate::input::SyncResourceInput).
    pub fn builder() -> crate::input::sync_resource_input::Builder {
        crate::input::sync_resource_input::Builder::default()
    }
    /// Creates a new `SyncResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SyncResource {
                type Output = std::result::Result<crate::output::SyncResourceOutput, crate::error::SyncResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_sync_resource_error(response)
                     } else {
                        crate::operation_deser::parse_sync_resource_response(response)
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

/// Operation shape for `UpdateApplication`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_application`](crate::client::Client::update_application).
            ///
            /// `ParseStrictResponse` impl for `UpdateApplication`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateApplication {
    _private: ()
}
impl UpdateApplication {
    /// Creates a new builder-style object to manufacture [`UpdateApplicationInput`](crate::input::UpdateApplicationInput).
    pub fn builder() -> crate::input::update_application_input::Builder {
        crate::input::update_application_input::Builder::default()
    }
    /// Creates a new `UpdateApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateApplication {
                type Output = std::result::Result<crate::output::UpdateApplicationOutput, crate::error::UpdateApplicationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_application_error(response)
                     } else {
                        crate::operation_deser::parse_update_application_response(response)
                     }
                }
            }

/// Operation shape for `UpdateAttributeGroup`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_attribute_group`](crate::client::Client::update_attribute_group).
            ///
            /// `ParseStrictResponse` impl for `UpdateAttributeGroup`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateAttributeGroup {
    _private: ()
}
impl UpdateAttributeGroup {
    /// Creates a new builder-style object to manufacture [`UpdateAttributeGroupInput`](crate::input::UpdateAttributeGroupInput).
    pub fn builder() -> crate::input::update_attribute_group_input::Builder {
        crate::input::update_attribute_group_input::Builder::default()
    }
    /// Creates a new `UpdateAttributeGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateAttributeGroup {
                type Output = std::result::Result<crate::output::UpdateAttributeGroupOutput, crate::error::UpdateAttributeGroupError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_attribute_group_error(response)
                     } else {
                        crate::operation_deser::parse_update_attribute_group_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

