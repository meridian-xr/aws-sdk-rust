// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DeleteConnection`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_connection`](crate::client::Client::delete_connection).
            ///
            /// `ParseStrictResponse` impl for `DeleteConnection`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteConnection {
    _private: ()
}
impl DeleteConnection {
    /// Creates a new builder-style object to manufacture [`DeleteConnectionInput`](crate::input::DeleteConnectionInput).
    pub fn builder() -> crate::input::delete_connection_input::Builder {
        crate::input::delete_connection_input::Builder::default()
    }
    /// Creates a new `DeleteConnection` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteConnection {
                type Output = std::result::Result<crate::output::DeleteConnectionOutput, crate::error::DeleteConnectionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 204 {
                        crate::operation_deser::parse_delete_connection_error(response)
                     } else {
                        crate::operation_deser::parse_delete_connection_response(response)
                     }
                }
            }

/// Operation shape for `GetConnection`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_connection`](crate::client::Client::get_connection).
            ///
            /// `ParseStrictResponse` impl for `GetConnection`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetConnection {
    _private: ()
}
impl GetConnection {
    /// Creates a new builder-style object to manufacture [`GetConnectionInput`](crate::input::GetConnectionInput).
    pub fn builder() -> crate::input::get_connection_input::Builder {
        crate::input::get_connection_input::Builder::default()
    }
    /// Creates a new `GetConnection` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetConnection {
                type Output = std::result::Result<crate::output::GetConnectionOutput, crate::error::GetConnectionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_connection_error(response)
                     } else {
                        crate::operation_deser::parse_get_connection_response(response)
                     }
                }
            }

/// Operation shape for `PostToConnection`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`post_to_connection`](crate::client::Client::post_to_connection).
            ///
            /// `ParseStrictResponse` impl for `PostToConnection`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PostToConnection {
    _private: ()
}
impl PostToConnection {
    /// Creates a new builder-style object to manufacture [`PostToConnectionInput`](crate::input::PostToConnectionInput).
    pub fn builder() -> crate::input::post_to_connection_input::Builder {
        crate::input::post_to_connection_input::Builder::default()
    }
    /// Creates a new `PostToConnection` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PostToConnection {
                type Output = std::result::Result<crate::output::PostToConnectionOutput, crate::error::PostToConnectionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_post_to_connection_error(response)
                     } else {
                        crate::operation_deser::parse_post_to_connection_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

