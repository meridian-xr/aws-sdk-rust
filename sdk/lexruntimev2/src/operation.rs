// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DeleteSession`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_session`](crate::client::Client::delete_session).
            ///
            /// `ParseStrictResponse` impl for `DeleteSession`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteSession {
    _private: ()
}
impl DeleteSession {
    /// Creates a new builder-style object to manufacture [`DeleteSessionInput`](crate::input::DeleteSessionInput).
    pub fn builder() -> crate::input::delete_session_input::Builder {
        crate::input::delete_session_input::Builder::default()
    }
    /// Creates a new `DeleteSession` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSession {
                type Output = std::result::Result<crate::output::DeleteSessionOutput, crate::error::DeleteSessionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_session_error(response)
                     } else {
                        crate::operation_deser::parse_delete_session_response(response)
                     }
                }
            }

/// Operation shape for `GetSession`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_session`](crate::client::Client::get_session).
            ///
            /// `ParseStrictResponse` impl for `GetSession`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetSession {
    _private: ()
}
impl GetSession {
    /// Creates a new builder-style object to manufacture [`GetSessionInput`](crate::input::GetSessionInput).
    pub fn builder() -> crate::input::get_session_input::Builder {
        crate::input::get_session_input::Builder::default()
    }
    /// Creates a new `GetSession` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSession {
                type Output = std::result::Result<crate::output::GetSessionOutput, crate::error::GetSessionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_session_error(response)
                     } else {
                        crate::operation_deser::parse_get_session_response(response)
                     }
                }
            }

/// Operation shape for `PutSession`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_session`](crate::client::Client::put_session).
            ///
            /// `ParseStrictResponse` impl for `PutSession`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutSession {
    _private: ()
}
impl PutSession {
    /// Creates a new builder-style object to manufacture [`PutSessionInput`](crate::input::PutSessionInput).
    pub fn builder() -> crate::input::put_session_input::Builder {
        crate::input::put_session_input::Builder::default()
    }
    /// Creates a new `PutSession` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseHttpResponse for PutSession {
                type Output = std::result::Result<crate::output::PutSessionOutput, crate::error::PutSessionError>;
                fn parse_unloaded(&self, response: &mut aws_smithy_http::operation::Response) -> Option<Self::Output> {
                    // This is an error, defer to the non-streaming parser
                    if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
                        return None;
                    }
                    Some(crate::operation_deser::parse_put_session(response))
                }
                fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                    // if streaming, we only hit this case if its an error
                    crate::operation_deser::parse_put_session_error(response)
                }
            }

/// Operation shape for `RecognizeText`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`recognize_text`](crate::client::Client::recognize_text).
            ///
            /// `ParseStrictResponse` impl for `RecognizeText`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RecognizeText {
    _private: ()
}
impl RecognizeText {
    /// Creates a new builder-style object to manufacture [`RecognizeTextInput`](crate::input::RecognizeTextInput).
    pub fn builder() -> crate::input::recognize_text_input::Builder {
        crate::input::recognize_text_input::Builder::default()
    }
    /// Creates a new `RecognizeText` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RecognizeText {
                type Output = std::result::Result<crate::output::RecognizeTextOutput, crate::error::RecognizeTextError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_recognize_text_error(response)
                     } else {
                        crate::operation_deser::parse_recognize_text_response(response)
                     }
                }
            }

/// Operation shape for `RecognizeUtterance`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`recognize_utterance`](crate::client::Client::recognize_utterance).
            ///
            /// `ParseStrictResponse` impl for `RecognizeUtterance`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RecognizeUtterance {
    _private: ()
}
impl RecognizeUtterance {
    /// Creates a new builder-style object to manufacture [`RecognizeUtteranceInput`](crate::input::RecognizeUtteranceInput).
    pub fn builder() -> crate::input::recognize_utterance_input::Builder {
        crate::input::recognize_utterance_input::Builder::default()
    }
    /// Creates a new `RecognizeUtterance` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseHttpResponse for RecognizeUtterance {
                type Output = std::result::Result<crate::output::RecognizeUtteranceOutput, crate::error::RecognizeUtteranceError>;
                fn parse_unloaded(&self, response: &mut aws_smithy_http::operation::Response) -> Option<Self::Output> {
                    // This is an error, defer to the non-streaming parser
                    if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
                        return None;
                    }
                    Some(crate::operation_deser::parse_recognize_utterance(response))
                }
                fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                    // if streaming, we only hit this case if its an error
                    crate::operation_deser::parse_recognize_utterance_error(response)
                }
            }

/// Operation customization and supporting types
pub mod customize;

