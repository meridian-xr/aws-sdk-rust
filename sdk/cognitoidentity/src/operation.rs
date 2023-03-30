// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateIdentityPool`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_identity_pool`](crate::client::Client::create_identity_pool).
            ///
            /// `ParseStrictResponse` impl for `CreateIdentityPool`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateIdentityPool {
    _private: ()
}
impl CreateIdentityPool {
    /// Creates a new builder-style object to manufacture [`CreateIdentityPoolInput`](crate::input::CreateIdentityPoolInput).
    pub fn builder() -> crate::input::create_identity_pool_input::Builder {
        crate::input::create_identity_pool_input::Builder::default()
    }
    /// Creates a new `CreateIdentityPool` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateIdentityPool {
                type Output = std::result::Result<crate::output::CreateIdentityPoolOutput, crate::error::CreateIdentityPoolError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_identity_pool_error(response)
                     } else {
                        crate::operation_deser::parse_create_identity_pool_response(response)
                     }
                }
            }

/// Operation shape for `DeleteIdentities`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_identities`](crate::client::Client::delete_identities).
            ///
            /// `ParseStrictResponse` impl for `DeleteIdentities`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteIdentities {
    _private: ()
}
impl DeleteIdentities {
    /// Creates a new builder-style object to manufacture [`DeleteIdentitiesInput`](crate::input::DeleteIdentitiesInput).
    pub fn builder() -> crate::input::delete_identities_input::Builder {
        crate::input::delete_identities_input::Builder::default()
    }
    /// Creates a new `DeleteIdentities` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteIdentities {
                type Output = std::result::Result<crate::output::DeleteIdentitiesOutput, crate::error::DeleteIdentitiesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_identities_error(response)
                     } else {
                        crate::operation_deser::parse_delete_identities_response(response)
                     }
                }
            }

/// Operation shape for `DeleteIdentityPool`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_identity_pool`](crate::client::Client::delete_identity_pool).
            ///
            /// `ParseStrictResponse` impl for `DeleteIdentityPool`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteIdentityPool {
    _private: ()
}
impl DeleteIdentityPool {
    /// Creates a new builder-style object to manufacture [`DeleteIdentityPoolInput`](crate::input::DeleteIdentityPoolInput).
    pub fn builder() -> crate::input::delete_identity_pool_input::Builder {
        crate::input::delete_identity_pool_input::Builder::default()
    }
    /// Creates a new `DeleteIdentityPool` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteIdentityPool {
                type Output = std::result::Result<crate::output::DeleteIdentityPoolOutput, crate::error::DeleteIdentityPoolError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_identity_pool_error(response)
                     } else {
                        crate::operation_deser::parse_delete_identity_pool_response(response)
                     }
                }
            }

/// Operation shape for `DescribeIdentity`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_identity`](crate::client::Client::describe_identity).
            ///
            /// `ParseStrictResponse` impl for `DescribeIdentity`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeIdentity {
    _private: ()
}
impl DescribeIdentity {
    /// Creates a new builder-style object to manufacture [`DescribeIdentityInput`](crate::input::DescribeIdentityInput).
    pub fn builder() -> crate::input::describe_identity_input::Builder {
        crate::input::describe_identity_input::Builder::default()
    }
    /// Creates a new `DescribeIdentity` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeIdentity {
                type Output = std::result::Result<crate::output::DescribeIdentityOutput, crate::error::DescribeIdentityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_identity_error(response)
                     } else {
                        crate::operation_deser::parse_describe_identity_response(response)
                     }
                }
            }

/// Operation shape for `DescribeIdentityPool`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_identity_pool`](crate::client::Client::describe_identity_pool).
            ///
            /// `ParseStrictResponse` impl for `DescribeIdentityPool`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeIdentityPool {
    _private: ()
}
impl DescribeIdentityPool {
    /// Creates a new builder-style object to manufacture [`DescribeIdentityPoolInput`](crate::input::DescribeIdentityPoolInput).
    pub fn builder() -> crate::input::describe_identity_pool_input::Builder {
        crate::input::describe_identity_pool_input::Builder::default()
    }
    /// Creates a new `DescribeIdentityPool` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeIdentityPool {
                type Output = std::result::Result<crate::output::DescribeIdentityPoolOutput, crate::error::DescribeIdentityPoolError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_identity_pool_error(response)
                     } else {
                        crate::operation_deser::parse_describe_identity_pool_response(response)
                     }
                }
            }

/// Operation shape for `GetCredentialsForIdentity`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_credentials_for_identity`](crate::client::Client::get_credentials_for_identity).
            ///
            /// `ParseStrictResponse` impl for `GetCredentialsForIdentity`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetCredentialsForIdentity {
    _private: ()
}
impl GetCredentialsForIdentity {
    /// Creates a new builder-style object to manufacture [`GetCredentialsForIdentityInput`](crate::input::GetCredentialsForIdentityInput).
    pub fn builder() -> crate::input::get_credentials_for_identity_input::Builder {
        crate::input::get_credentials_for_identity_input::Builder::default()
    }
    /// Creates a new `GetCredentialsForIdentity` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetCredentialsForIdentity {
                type Output = std::result::Result<crate::output::GetCredentialsForIdentityOutput, crate::error::GetCredentialsForIdentityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_credentials_for_identity_error(response)
                     } else {
                        crate::operation_deser::parse_get_credentials_for_identity_response(response)
                     }
                }
            }

/// Operation shape for `GetId`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_id`](crate::client::Client::get_id).
            ///
            /// `ParseStrictResponse` impl for `GetId`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetId {
    _private: ()
}
impl GetId {
    /// Creates a new builder-style object to manufacture [`GetIdInput`](crate::input::GetIdInput).
    pub fn builder() -> crate::input::get_id_input::Builder {
        crate::input::get_id_input::Builder::default()
    }
    /// Creates a new `GetId` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetId {
                type Output = std::result::Result<crate::output::GetIdOutput, crate::error::GetIdError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_id_error(response)
                     } else {
                        crate::operation_deser::parse_get_id_response(response)
                     }
                }
            }

/// Operation shape for `GetIdentityPoolRoles`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_identity_pool_roles`](crate::client::Client::get_identity_pool_roles).
            ///
            /// `ParseStrictResponse` impl for `GetIdentityPoolRoles`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetIdentityPoolRoles {
    _private: ()
}
impl GetIdentityPoolRoles {
    /// Creates a new builder-style object to manufacture [`GetIdentityPoolRolesInput`](crate::input::GetIdentityPoolRolesInput).
    pub fn builder() -> crate::input::get_identity_pool_roles_input::Builder {
        crate::input::get_identity_pool_roles_input::Builder::default()
    }
    /// Creates a new `GetIdentityPoolRoles` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetIdentityPoolRoles {
                type Output = std::result::Result<crate::output::GetIdentityPoolRolesOutput, crate::error::GetIdentityPoolRolesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_identity_pool_roles_error(response)
                     } else {
                        crate::operation_deser::parse_get_identity_pool_roles_response(response)
                     }
                }
            }

/// Operation shape for `GetOpenIdToken`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_open_id_token`](crate::client::Client::get_open_id_token).
            ///
            /// `ParseStrictResponse` impl for `GetOpenIdToken`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetOpenIdToken {
    _private: ()
}
impl GetOpenIdToken {
    /// Creates a new builder-style object to manufacture [`GetOpenIdTokenInput`](crate::input::GetOpenIdTokenInput).
    pub fn builder() -> crate::input::get_open_id_token_input::Builder {
        crate::input::get_open_id_token_input::Builder::default()
    }
    /// Creates a new `GetOpenIdToken` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetOpenIdToken {
                type Output = std::result::Result<crate::output::GetOpenIdTokenOutput, crate::error::GetOpenIdTokenError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_open_id_token_error(response)
                     } else {
                        crate::operation_deser::parse_get_open_id_token_response(response)
                     }
                }
            }

/// Operation shape for `GetOpenIdTokenForDeveloperIdentity`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_open_id_token_for_developer_identity`](crate::client::Client::get_open_id_token_for_developer_identity).
            ///
            /// `ParseStrictResponse` impl for `GetOpenIdTokenForDeveloperIdentity`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetOpenIdTokenForDeveloperIdentity {
    _private: ()
}
impl GetOpenIdTokenForDeveloperIdentity {
    /// Creates a new builder-style object to manufacture [`GetOpenIdTokenForDeveloperIdentityInput`](crate::input::GetOpenIdTokenForDeveloperIdentityInput).
    pub fn builder() -> crate::input::get_open_id_token_for_developer_identity_input::Builder {
        crate::input::get_open_id_token_for_developer_identity_input::Builder::default()
    }
    /// Creates a new `GetOpenIdTokenForDeveloperIdentity` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetOpenIdTokenForDeveloperIdentity {
                type Output = std::result::Result<crate::output::GetOpenIdTokenForDeveloperIdentityOutput, crate::error::GetOpenIdTokenForDeveloperIdentityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_open_id_token_for_developer_identity_error(response)
                     } else {
                        crate::operation_deser::parse_get_open_id_token_for_developer_identity_response(response)
                     }
                }
            }

/// Operation shape for `GetPrincipalTagAttributeMap`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_principal_tag_attribute_map`](crate::client::Client::get_principal_tag_attribute_map).
            ///
            /// `ParseStrictResponse` impl for `GetPrincipalTagAttributeMap`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetPrincipalTagAttributeMap {
    _private: ()
}
impl GetPrincipalTagAttributeMap {
    /// Creates a new builder-style object to manufacture [`GetPrincipalTagAttributeMapInput`](crate::input::GetPrincipalTagAttributeMapInput).
    pub fn builder() -> crate::input::get_principal_tag_attribute_map_input::Builder {
        crate::input::get_principal_tag_attribute_map_input::Builder::default()
    }
    /// Creates a new `GetPrincipalTagAttributeMap` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetPrincipalTagAttributeMap {
                type Output = std::result::Result<crate::output::GetPrincipalTagAttributeMapOutput, crate::error::GetPrincipalTagAttributeMapError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_principal_tag_attribute_map_error(response)
                     } else {
                        crate::operation_deser::parse_get_principal_tag_attribute_map_response(response)
                     }
                }
            }

/// Operation shape for `ListIdentities`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_identities`](crate::client::Client::list_identities).
            ///
            /// `ParseStrictResponse` impl for `ListIdentities`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListIdentities {
    _private: ()
}
impl ListIdentities {
    /// Creates a new builder-style object to manufacture [`ListIdentitiesInput`](crate::input::ListIdentitiesInput).
    pub fn builder() -> crate::input::list_identities_input::Builder {
        crate::input::list_identities_input::Builder::default()
    }
    /// Creates a new `ListIdentities` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListIdentities {
                type Output = std::result::Result<crate::output::ListIdentitiesOutput, crate::error::ListIdentitiesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_identities_error(response)
                     } else {
                        crate::operation_deser::parse_list_identities_response(response)
                     }
                }
            }

/// Operation shape for `ListIdentityPools`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_identity_pools`](crate::client::Client::list_identity_pools).
            ///
            /// `ParseStrictResponse` impl for `ListIdentityPools`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListIdentityPools {
    _private: ()
}
impl ListIdentityPools {
    /// Creates a new builder-style object to manufacture [`ListIdentityPoolsInput`](crate::input::ListIdentityPoolsInput).
    pub fn builder() -> crate::input::list_identity_pools_input::Builder {
        crate::input::list_identity_pools_input::Builder::default()
    }
    /// Creates a new `ListIdentityPools` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListIdentityPools {
                type Output = std::result::Result<crate::output::ListIdentityPoolsOutput, crate::error::ListIdentityPoolsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_identity_pools_error(response)
                     } else {
                        crate::operation_deser::parse_list_identity_pools_response(response)
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

/// Operation shape for `LookupDeveloperIdentity`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`lookup_developer_identity`](crate::client::Client::lookup_developer_identity).
            ///
            /// `ParseStrictResponse` impl for `LookupDeveloperIdentity`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct LookupDeveloperIdentity {
    _private: ()
}
impl LookupDeveloperIdentity {
    /// Creates a new builder-style object to manufacture [`LookupDeveloperIdentityInput`](crate::input::LookupDeveloperIdentityInput).
    pub fn builder() -> crate::input::lookup_developer_identity_input::Builder {
        crate::input::lookup_developer_identity_input::Builder::default()
    }
    /// Creates a new `LookupDeveloperIdentity` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for LookupDeveloperIdentity {
                type Output = std::result::Result<crate::output::LookupDeveloperIdentityOutput, crate::error::LookupDeveloperIdentityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_lookup_developer_identity_error(response)
                     } else {
                        crate::operation_deser::parse_lookup_developer_identity_response(response)
                     }
                }
            }

/// Operation shape for `MergeDeveloperIdentities`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`merge_developer_identities`](crate::client::Client::merge_developer_identities).
            ///
            /// `ParseStrictResponse` impl for `MergeDeveloperIdentities`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct MergeDeveloperIdentities {
    _private: ()
}
impl MergeDeveloperIdentities {
    /// Creates a new builder-style object to manufacture [`MergeDeveloperIdentitiesInput`](crate::input::MergeDeveloperIdentitiesInput).
    pub fn builder() -> crate::input::merge_developer_identities_input::Builder {
        crate::input::merge_developer_identities_input::Builder::default()
    }
    /// Creates a new `MergeDeveloperIdentities` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for MergeDeveloperIdentities {
                type Output = std::result::Result<crate::output::MergeDeveloperIdentitiesOutput, crate::error::MergeDeveloperIdentitiesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_merge_developer_identities_error(response)
                     } else {
                        crate::operation_deser::parse_merge_developer_identities_response(response)
                     }
                }
            }

/// Operation shape for `SetIdentityPoolRoles`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`set_identity_pool_roles`](crate::client::Client::set_identity_pool_roles).
            ///
            /// `ParseStrictResponse` impl for `SetIdentityPoolRoles`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct SetIdentityPoolRoles {
    _private: ()
}
impl SetIdentityPoolRoles {
    /// Creates a new builder-style object to manufacture [`SetIdentityPoolRolesInput`](crate::input::SetIdentityPoolRolesInput).
    pub fn builder() -> crate::input::set_identity_pool_roles_input::Builder {
        crate::input::set_identity_pool_roles_input::Builder::default()
    }
    /// Creates a new `SetIdentityPoolRoles` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SetIdentityPoolRoles {
                type Output = std::result::Result<crate::output::SetIdentityPoolRolesOutput, crate::error::SetIdentityPoolRolesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_set_identity_pool_roles_error(response)
                     } else {
                        crate::operation_deser::parse_set_identity_pool_roles_response(response)
                     }
                }
            }

/// Operation shape for `SetPrincipalTagAttributeMap`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`set_principal_tag_attribute_map`](crate::client::Client::set_principal_tag_attribute_map).
            ///
            /// `ParseStrictResponse` impl for `SetPrincipalTagAttributeMap`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct SetPrincipalTagAttributeMap {
    _private: ()
}
impl SetPrincipalTagAttributeMap {
    /// Creates a new builder-style object to manufacture [`SetPrincipalTagAttributeMapInput`](crate::input::SetPrincipalTagAttributeMapInput).
    pub fn builder() -> crate::input::set_principal_tag_attribute_map_input::Builder {
        crate::input::set_principal_tag_attribute_map_input::Builder::default()
    }
    /// Creates a new `SetPrincipalTagAttributeMap` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SetPrincipalTagAttributeMap {
                type Output = std::result::Result<crate::output::SetPrincipalTagAttributeMapOutput, crate::error::SetPrincipalTagAttributeMapError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_set_principal_tag_attribute_map_error(response)
                     } else {
                        crate::operation_deser::parse_set_principal_tag_attribute_map_response(response)
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

/// Operation shape for `UnlinkDeveloperIdentity`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`unlink_developer_identity`](crate::client::Client::unlink_developer_identity).
            ///
            /// `ParseStrictResponse` impl for `UnlinkDeveloperIdentity`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UnlinkDeveloperIdentity {
    _private: ()
}
impl UnlinkDeveloperIdentity {
    /// Creates a new builder-style object to manufacture [`UnlinkDeveloperIdentityInput`](crate::input::UnlinkDeveloperIdentityInput).
    pub fn builder() -> crate::input::unlink_developer_identity_input::Builder {
        crate::input::unlink_developer_identity_input::Builder::default()
    }
    /// Creates a new `UnlinkDeveloperIdentity` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UnlinkDeveloperIdentity {
                type Output = std::result::Result<crate::output::UnlinkDeveloperIdentityOutput, crate::error::UnlinkDeveloperIdentityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_unlink_developer_identity_error(response)
                     } else {
                        crate::operation_deser::parse_unlink_developer_identity_response(response)
                     }
                }
            }

/// Operation shape for `UnlinkIdentity`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`unlink_identity`](crate::client::Client::unlink_identity).
            ///
            /// `ParseStrictResponse` impl for `UnlinkIdentity`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UnlinkIdentity {
    _private: ()
}
impl UnlinkIdentity {
    /// Creates a new builder-style object to manufacture [`UnlinkIdentityInput`](crate::input::UnlinkIdentityInput).
    pub fn builder() -> crate::input::unlink_identity_input::Builder {
        crate::input::unlink_identity_input::Builder::default()
    }
    /// Creates a new `UnlinkIdentity` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UnlinkIdentity {
                type Output = std::result::Result<crate::output::UnlinkIdentityOutput, crate::error::UnlinkIdentityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_unlink_identity_error(response)
                     } else {
                        crate::operation_deser::parse_unlink_identity_response(response)
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

/// Operation shape for `UpdateIdentityPool`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_identity_pool`](crate::client::Client::update_identity_pool).
            ///
            /// `ParseStrictResponse` impl for `UpdateIdentityPool`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateIdentityPool {
    _private: ()
}
impl UpdateIdentityPool {
    /// Creates a new builder-style object to manufacture [`UpdateIdentityPoolInput`](crate::input::UpdateIdentityPoolInput).
    pub fn builder() -> crate::input::update_identity_pool_input::Builder {
        crate::input::update_identity_pool_input::Builder::default()
    }
    /// Creates a new `UpdateIdentityPool` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateIdentityPool {
                type Output = std::result::Result<crate::output::UpdateIdentityPoolOutput, crate::error::UpdateIdentityPoolError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_identity_pool_error(response)
                     } else {
                        crate::operation_deser::parse_update_identity_pool_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

