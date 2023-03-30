// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DeleteAlternateContact`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_alternate_contact`](crate::client::Client::delete_alternate_contact).
            ///
            /// `ParseStrictResponse` impl for `DeleteAlternateContact`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteAlternateContact {
    _private: ()
}
impl DeleteAlternateContact {
    /// Creates a new builder-style object to manufacture [`DeleteAlternateContactInput`](crate::input::DeleteAlternateContactInput).
    pub fn builder() -> crate::input::delete_alternate_contact_input::Builder {
        crate::input::delete_alternate_contact_input::Builder::default()
    }
    /// Creates a new `DeleteAlternateContact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAlternateContact {
                type Output = std::result::Result<crate::output::DeleteAlternateContactOutput, crate::error::DeleteAlternateContactError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_alternate_contact_error(response)
                     } else {
                        crate::operation_deser::parse_delete_alternate_contact_response(response)
                     }
                }
            }

/// Operation shape for `GetAlternateContact`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_alternate_contact`](crate::client::Client::get_alternate_contact).
            ///
            /// `ParseStrictResponse` impl for `GetAlternateContact`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAlternateContact {
    _private: ()
}
impl GetAlternateContact {
    /// Creates a new builder-style object to manufacture [`GetAlternateContactInput`](crate::input::GetAlternateContactInput).
    pub fn builder() -> crate::input::get_alternate_contact_input::Builder {
        crate::input::get_alternate_contact_input::Builder::default()
    }
    /// Creates a new `GetAlternateContact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAlternateContact {
                type Output = std::result::Result<crate::output::GetAlternateContactOutput, crate::error::GetAlternateContactError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_alternate_contact_error(response)
                     } else {
                        crate::operation_deser::parse_get_alternate_contact_response(response)
                     }
                }
            }

/// Operation shape for `GetContactInformation`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_contact_information`](crate::client::Client::get_contact_information).
            ///
            /// `ParseStrictResponse` impl for `GetContactInformation`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetContactInformation {
    _private: ()
}
impl GetContactInformation {
    /// Creates a new builder-style object to manufacture [`GetContactInformationInput`](crate::input::GetContactInformationInput).
    pub fn builder() -> crate::input::get_contact_information_input::Builder {
        crate::input::get_contact_information_input::Builder::default()
    }
    /// Creates a new `GetContactInformation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetContactInformation {
                type Output = std::result::Result<crate::output::GetContactInformationOutput, crate::error::GetContactInformationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_contact_information_error(response)
                     } else {
                        crate::operation_deser::parse_get_contact_information_response(response)
                     }
                }
            }

/// Operation shape for `PutAlternateContact`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_alternate_contact`](crate::client::Client::put_alternate_contact).
            ///
            /// `ParseStrictResponse` impl for `PutAlternateContact`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutAlternateContact {
    _private: ()
}
impl PutAlternateContact {
    /// Creates a new builder-style object to manufacture [`PutAlternateContactInput`](crate::input::PutAlternateContactInput).
    pub fn builder() -> crate::input::put_alternate_contact_input::Builder {
        crate::input::put_alternate_contact_input::Builder::default()
    }
    /// Creates a new `PutAlternateContact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutAlternateContact {
                type Output = std::result::Result<crate::output::PutAlternateContactOutput, crate::error::PutAlternateContactError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_alternate_contact_error(response)
                     } else {
                        crate::operation_deser::parse_put_alternate_contact_response(response)
                     }
                }
            }

/// Operation shape for `PutContactInformation`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_contact_information`](crate::client::Client::put_contact_information).
            ///
            /// `ParseStrictResponse` impl for `PutContactInformation`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutContactInformation {
    _private: ()
}
impl PutContactInformation {
    /// Creates a new builder-style object to manufacture [`PutContactInformationInput`](crate::input::PutContactInformationInput).
    pub fn builder() -> crate::input::put_contact_information_input::Builder {
        crate::input::put_contact_information_input::Builder::default()
    }
    /// Creates a new `PutContactInformation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutContactInformation {
                type Output = std::result::Result<crate::output::PutContactInformationOutput, crate::error::PutContactInformationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_contact_information_error(response)
                     } else {
                        crate::operation_deser::parse_put_contact_information_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

