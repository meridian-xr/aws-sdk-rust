// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DisableControl`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`disable_control`](crate::client::Client::disable_control).
            ///
            /// `ParseStrictResponse` impl for `DisableControl`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisableControl {
    _private: ()
}
impl DisableControl {
    /// Creates a new builder-style object to manufacture [`DisableControlInput`](crate::input::DisableControlInput).
    pub fn builder() -> crate::input::disable_control_input::Builder {
        crate::input::disable_control_input::Builder::default()
    }
    /// Creates a new `DisableControl` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisableControl {
                type Output = std::result::Result<crate::output::DisableControlOutput, crate::error::DisableControlError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_disable_control_error(response)
                     } else {
                        crate::operation_deser::parse_disable_control_response(response)
                     }
                }
            }

/// Operation shape for `EnableControl`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`enable_control`](crate::client::Client::enable_control).
            ///
            /// `ParseStrictResponse` impl for `EnableControl`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct EnableControl {
    _private: ()
}
impl EnableControl {
    /// Creates a new builder-style object to manufacture [`EnableControlInput`](crate::input::EnableControlInput).
    pub fn builder() -> crate::input::enable_control_input::Builder {
        crate::input::enable_control_input::Builder::default()
    }
    /// Creates a new `EnableControl` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for EnableControl {
                type Output = std::result::Result<crate::output::EnableControlOutput, crate::error::EnableControlError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_enable_control_error(response)
                     } else {
                        crate::operation_deser::parse_enable_control_response(response)
                     }
                }
            }

/// Operation shape for `GetControlOperation`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_control_operation`](crate::client::Client::get_control_operation).
            ///
            /// `ParseStrictResponse` impl for `GetControlOperation`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetControlOperation {
    _private: ()
}
impl GetControlOperation {
    /// Creates a new builder-style object to manufacture [`GetControlOperationInput`](crate::input::GetControlOperationInput).
    pub fn builder() -> crate::input::get_control_operation_input::Builder {
        crate::input::get_control_operation_input::Builder::default()
    }
    /// Creates a new `GetControlOperation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetControlOperation {
                type Output = std::result::Result<crate::output::GetControlOperationOutput, crate::error::GetControlOperationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_control_operation_error(response)
                     } else {
                        crate::operation_deser::parse_get_control_operation_response(response)
                     }
                }
            }

/// Operation shape for `ListEnabledControls`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_enabled_controls`](crate::client::Client::list_enabled_controls).
            ///
            /// `ParseStrictResponse` impl for `ListEnabledControls`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListEnabledControls {
    _private: ()
}
impl ListEnabledControls {
    /// Creates a new builder-style object to manufacture [`ListEnabledControlsInput`](crate::input::ListEnabledControlsInput).
    pub fn builder() -> crate::input::list_enabled_controls_input::Builder {
        crate::input::list_enabled_controls_input::Builder::default()
    }
    /// Creates a new `ListEnabledControls` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListEnabledControls {
                type Output = std::result::Result<crate::output::ListEnabledControlsOutput, crate::error::ListEnabledControlsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_enabled_controls_error(response)
                     } else {
                        crate::operation_deser::parse_list_enabled_controls_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

