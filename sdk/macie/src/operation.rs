// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssociateMemberAccount`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`associate_member_account`](crate::client::Client::associate_member_account).
            ///
            /// `ParseStrictResponse` impl for `AssociateMemberAccount`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssociateMemberAccount {
    _private: ()
}
impl AssociateMemberAccount {
    /// Creates a new builder-style object to manufacture [`AssociateMemberAccountInput`](crate::input::AssociateMemberAccountInput).
    pub fn builder() -> crate::input::associate_member_account_input::Builder {
        crate::input::associate_member_account_input::Builder::default()
    }
    /// Creates a new `AssociateMemberAccount` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateMemberAccount {
                type Output = std::result::Result<crate::output::AssociateMemberAccountOutput, crate::error::AssociateMemberAccountError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_associate_member_account_error(response)
                     } else {
                        crate::operation_deser::parse_associate_member_account_response(response)
                     }
                }
            }

/// Operation shape for `AssociateS3Resources`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`associate_s3_resources`](crate::client::Client::associate_s3_resources).
            ///
            /// `ParseStrictResponse` impl for `AssociateS3Resources`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssociateS3Resources {
    _private: ()
}
impl AssociateS3Resources {
    /// Creates a new builder-style object to manufacture [`AssociateS3ResourcesInput`](crate::input::AssociateS3ResourcesInput).
    pub fn builder() -> crate::input::associate_s3_resources_input::Builder {
        crate::input::associate_s3_resources_input::Builder::default()
    }
    /// Creates a new `AssociateS3Resources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateS3Resources {
                type Output = std::result::Result<crate::output::AssociateS3ResourcesOutput, crate::error::AssociateS3ResourcesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_associate_s3_resources_error(response)
                     } else {
                        crate::operation_deser::parse_associate_s3_resources_response(response)
                     }
                }
            }

/// Operation shape for `DisassociateMemberAccount`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`disassociate_member_account`](crate::client::Client::disassociate_member_account).
            ///
            /// `ParseStrictResponse` impl for `DisassociateMemberAccount`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisassociateMemberAccount {
    _private: ()
}
impl DisassociateMemberAccount {
    /// Creates a new builder-style object to manufacture [`DisassociateMemberAccountInput`](crate::input::DisassociateMemberAccountInput).
    pub fn builder() -> crate::input::disassociate_member_account_input::Builder {
        crate::input::disassociate_member_account_input::Builder::default()
    }
    /// Creates a new `DisassociateMemberAccount` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateMemberAccount {
                type Output = std::result::Result<crate::output::DisassociateMemberAccountOutput, crate::error::DisassociateMemberAccountError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_disassociate_member_account_error(response)
                     } else {
                        crate::operation_deser::parse_disassociate_member_account_response(response)
                     }
                }
            }

/// Operation shape for `DisassociateS3Resources`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`disassociate_s3_resources`](crate::client::Client::disassociate_s3_resources).
            ///
            /// `ParseStrictResponse` impl for `DisassociateS3Resources`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisassociateS3Resources {
    _private: ()
}
impl DisassociateS3Resources {
    /// Creates a new builder-style object to manufacture [`DisassociateS3ResourcesInput`](crate::input::DisassociateS3ResourcesInput).
    pub fn builder() -> crate::input::disassociate_s3_resources_input::Builder {
        crate::input::disassociate_s3_resources_input::Builder::default()
    }
    /// Creates a new `DisassociateS3Resources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateS3Resources {
                type Output = std::result::Result<crate::output::DisassociateS3ResourcesOutput, crate::error::DisassociateS3ResourcesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_disassociate_s3_resources_error(response)
                     } else {
                        crate::operation_deser::parse_disassociate_s3_resources_response(response)
                     }
                }
            }

/// Operation shape for `ListMemberAccounts`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_member_accounts`](crate::client::Client::list_member_accounts).
            ///
            /// `ParseStrictResponse` impl for `ListMemberAccounts`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListMemberAccounts {
    _private: ()
}
impl ListMemberAccounts {
    /// Creates a new builder-style object to manufacture [`ListMemberAccountsInput`](crate::input::ListMemberAccountsInput).
    pub fn builder() -> crate::input::list_member_accounts_input::Builder {
        crate::input::list_member_accounts_input::Builder::default()
    }
    /// Creates a new `ListMemberAccounts` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListMemberAccounts {
                type Output = std::result::Result<crate::output::ListMemberAccountsOutput, crate::error::ListMemberAccountsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_member_accounts_error(response)
                     } else {
                        crate::operation_deser::parse_list_member_accounts_response(response)
                     }
                }
            }

/// Operation shape for `ListS3Resources`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_s3_resources`](crate::client::Client::list_s3_resources).
            ///
            /// `ParseStrictResponse` impl for `ListS3Resources`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListS3Resources {
    _private: ()
}
impl ListS3Resources {
    /// Creates a new builder-style object to manufacture [`ListS3ResourcesInput`](crate::input::ListS3ResourcesInput).
    pub fn builder() -> crate::input::list_s3_resources_input::Builder {
        crate::input::list_s3_resources_input::Builder::default()
    }
    /// Creates a new `ListS3Resources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListS3Resources {
                type Output = std::result::Result<crate::output::ListS3ResourcesOutput, crate::error::ListS3ResourcesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_s3_resources_error(response)
                     } else {
                        crate::operation_deser::parse_list_s3_resources_response(response)
                     }
                }
            }

/// Operation shape for `UpdateS3Resources`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_s3_resources`](crate::client::Client::update_s3_resources).
            ///
            /// `ParseStrictResponse` impl for `UpdateS3Resources`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateS3Resources {
    _private: ()
}
impl UpdateS3Resources {
    /// Creates a new builder-style object to manufacture [`UpdateS3ResourcesInput`](crate::input::UpdateS3ResourcesInput).
    pub fn builder() -> crate::input::update_s3_resources_input::Builder {
        crate::input::update_s3_resources_input::Builder::default()
    }
    /// Creates a new `UpdateS3Resources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateS3Resources {
                type Output = std::result::Result<crate::output::UpdateS3ResourcesOutput, crate::error::UpdateS3ResourcesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_s3_resources_error(response)
                     } else {
                        crate::operation_deser::parse_update_s3_resources_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

