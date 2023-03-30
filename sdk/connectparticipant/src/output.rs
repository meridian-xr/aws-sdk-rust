// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct StartAttachmentUploadOutput  {
    /// <p>A unique identifier for the attachment.</p>
    #[doc(hidden)]
    pub attachment_id: std::option::Option<std::string::String>,
    /// <p>Fields to be used while uploading the attachment.</p>
    #[doc(hidden)]
    pub upload_metadata: std::option::Option<crate::model::UploadMetadata>,
    _request_id: Option<String>,
}
impl StartAttachmentUploadOutput {
    /// <p>A unique identifier for the attachment.</p>
    pub fn attachment_id(&self) -> std::option::Option<& str> {
        self.attachment_id.as_deref()
    }
    /// <p>Fields to be used while uploading the attachment.</p>
    pub fn upload_metadata(&self) -> std::option::Option<& crate::model::UploadMetadata> {
        self.upload_metadata.as_ref()
    }
}
impl aws_http::request_id::RequestId for StartAttachmentUploadOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl StartAttachmentUploadOutput {
    /// Creates a new builder-style object to manufacture [`StartAttachmentUploadOutput`](crate::output::StartAttachmentUploadOutput).
    pub fn builder() -> crate::output::start_attachment_upload_output::Builder {
        crate::output::start_attachment_upload_output::Builder::default()
    }
}

/// See [`StartAttachmentUploadOutput`](crate::output::StartAttachmentUploadOutput).
pub mod start_attachment_upload_output {
    
    /// A builder for [`StartAttachmentUploadOutput`](crate::output::StartAttachmentUploadOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) attachment_id: std::option::Option<std::string::String>,
        pub(crate) upload_metadata: std::option::Option<crate::model::UploadMetadata>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// <p>A unique identifier for the attachment.</p>
        pub fn attachment_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.attachment_id = Some(input.into());
            self
        }
        /// <p>A unique identifier for the attachment.</p>
        pub fn set_attachment_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.attachment_id = input; self
        }
        /// <p>Fields to be used while uploading the attachment.</p>
        pub fn upload_metadata(mut self, input: crate::model::UploadMetadata) -> Self {
            self.upload_metadata = Some(input);
            self
        }
        /// <p>Fields to be used while uploading the attachment.</p>
        pub fn set_upload_metadata(mut self, input: std::option::Option<crate::model::UploadMetadata>) -> Self {
            self.upload_metadata = input; self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`StartAttachmentUploadOutput`](crate::output::StartAttachmentUploadOutput).
        pub fn build(self) -> crate::output::StartAttachmentUploadOutput {
            crate::output::StartAttachmentUploadOutput {
                attachment_id: self.attachment_id
                ,
                upload_metadata: self.upload_metadata
                ,
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct SendMessageOutput  {
    /// <p>The ID of the message.</p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
    /// <p>The time when the message was sent.</p> 
    /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[doc(hidden)]
    pub absolute_time: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl SendMessageOutput {
    /// <p>The ID of the message.</p>
    pub fn id(&self) -> std::option::Option<& str> {
        self.id.as_deref()
    }
    /// <p>The time when the message was sent.</p> 
    /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    pub fn absolute_time(&self) -> std::option::Option<& str> {
        self.absolute_time.as_deref()
    }
}
impl aws_http::request_id::RequestId for SendMessageOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl SendMessageOutput {
    /// Creates a new builder-style object to manufacture [`SendMessageOutput`](crate::output::SendMessageOutput).
    pub fn builder() -> crate::output::send_message_output::Builder {
        crate::output::send_message_output::Builder::default()
    }
}

/// See [`SendMessageOutput`](crate::output::SendMessageOutput).
pub mod send_message_output {
    
    /// A builder for [`SendMessageOutput`](crate::output::SendMessageOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) id: std::option::Option<std::string::String>,
        pub(crate) absolute_time: std::option::Option<std::string::String>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// <p>The ID of the message.</p>
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        /// <p>The ID of the message.</p>
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input; self
        }
        /// <p>The time when the message was sent.</p> 
        /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
        pub fn absolute_time(mut self, input: impl Into<std::string::String>) -> Self {
            self.absolute_time = Some(input.into());
            self
        }
        /// <p>The time when the message was sent.</p> 
        /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
        pub fn set_absolute_time(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.absolute_time = input; self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`SendMessageOutput`](crate::output::SendMessageOutput).
        pub fn build(self) -> crate::output::SendMessageOutput {
            crate::output::SendMessageOutput {
                id: self.id
                ,
                absolute_time: self.absolute_time
                ,
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct SendEventOutput  {
    /// <p>The ID of the response.</p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
    /// <p>The time when the event was sent.</p> 
    /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[doc(hidden)]
    pub absolute_time: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl SendEventOutput {
    /// <p>The ID of the response.</p>
    pub fn id(&self) -> std::option::Option<& str> {
        self.id.as_deref()
    }
    /// <p>The time when the event was sent.</p> 
    /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    pub fn absolute_time(&self) -> std::option::Option<& str> {
        self.absolute_time.as_deref()
    }
}
impl aws_http::request_id::RequestId for SendEventOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl SendEventOutput {
    /// Creates a new builder-style object to manufacture [`SendEventOutput`](crate::output::SendEventOutput).
    pub fn builder() -> crate::output::send_event_output::Builder {
        crate::output::send_event_output::Builder::default()
    }
}

/// See [`SendEventOutput`](crate::output::SendEventOutput).
pub mod send_event_output {
    
    /// A builder for [`SendEventOutput`](crate::output::SendEventOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) id: std::option::Option<std::string::String>,
        pub(crate) absolute_time: std::option::Option<std::string::String>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// <p>The ID of the response.</p>
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        /// <p>The ID of the response.</p>
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input; self
        }
        /// <p>The time when the event was sent.</p> 
        /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
        pub fn absolute_time(mut self, input: impl Into<std::string::String>) -> Self {
            self.absolute_time = Some(input.into());
            self
        }
        /// <p>The time when the event was sent.</p> 
        /// <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
        pub fn set_absolute_time(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.absolute_time = input; self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`SendEventOutput`](crate::output::SendEventOutput).
        pub fn build(self) -> crate::output::SendEventOutput {
            crate::output::SendEventOutput {
                id: self.id
                ,
                absolute_time: self.absolute_time
                ,
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetTranscriptOutput  {
    /// <p>The initial contact ID for the contact. </p>
    #[doc(hidden)]
    pub initial_contact_id: std::option::Option<std::string::String>,
    /// <p>The list of messages in the session.</p>
    #[doc(hidden)]
    pub transcript: std::option::Option<std::vec::Vec<crate::model::Item>>,
    /// <p>The pagination token. Use the value returned previously in the next subsequent request to retrieve the next set of results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetTranscriptOutput {
    /// <p>The initial contact ID for the contact. </p>
    pub fn initial_contact_id(&self) -> std::option::Option<& str> {
        self.initial_contact_id.as_deref()
    }
    /// <p>The list of messages in the session.</p>
    pub fn transcript(&self) -> std::option::Option<& [crate::model::Item]> {
        self.transcript.as_deref()
    }
    /// <p>The pagination token. Use the value returned previously in the next subsequent request to retrieve the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetTranscriptOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl GetTranscriptOutput {
    /// Creates a new builder-style object to manufacture [`GetTranscriptOutput`](crate::output::GetTranscriptOutput).
    pub fn builder() -> crate::output::get_transcript_output::Builder {
        crate::output::get_transcript_output::Builder::default()
    }
}

/// See [`GetTranscriptOutput`](crate::output::GetTranscriptOutput).
pub mod get_transcript_output {
    
    /// A builder for [`GetTranscriptOutput`](crate::output::GetTranscriptOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) initial_contact_id: std::option::Option<std::string::String>,
        pub(crate) transcript: std::option::Option<std::vec::Vec<crate::model::Item>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// <p>The initial contact ID for the contact. </p>
        pub fn initial_contact_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.initial_contact_id = Some(input.into());
            self
        }
        /// <p>The initial contact ID for the contact. </p>
        pub fn set_initial_contact_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.initial_contact_id = input; self
        }
        /// Appends an item to `transcript`.
        ///
        /// To override the contents of this collection use [`set_transcript`](Self::set_transcript).
        ///
        /// <p>The list of messages in the session.</p>
        pub fn transcript(mut self, input: crate::model::Item) -> Self {
            let mut v = self.transcript.unwrap_or_default();
                            v.push(input);
                            self.transcript = Some(v);
                            self
        }
        /// <p>The list of messages in the session.</p>
        pub fn set_transcript(mut self, input: std::option::Option<std::vec::Vec<crate::model::Item>>) -> Self {
            self.transcript = input; self
        }
        /// <p>The pagination token. Use the value returned previously in the next subsequent request to retrieve the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The pagination token. Use the value returned previously in the next subsequent request to retrieve the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`GetTranscriptOutput`](crate::output::GetTranscriptOutput).
        pub fn build(self) -> crate::output::GetTranscriptOutput {
            crate::output::GetTranscriptOutput {
                initial_contact_id: self.initial_contact_id
                ,
                transcript: self.transcript
                ,
                next_token: self.next_token
                ,
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetAttachmentOutput  {
    /// <p>This is the pre-signed URL that can be used for uploading the file to Amazon S3 when used in response to <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_StartAttachmentUpload.html">StartAttachmentUpload</a>.</p>
    #[doc(hidden)]
    pub url: std::option::Option<std::string::String>,
    /// <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    #[doc(hidden)]
    pub url_expiry: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetAttachmentOutput {
    /// <p>This is the pre-signed URL that can be used for uploading the file to Amazon S3 when used in response to <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_StartAttachmentUpload.html">StartAttachmentUpload</a>.</p>
    pub fn url(&self) -> std::option::Option<& str> {
        self.url.as_deref()
    }
    /// <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    pub fn url_expiry(&self) -> std::option::Option<& str> {
        self.url_expiry.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetAttachmentOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl GetAttachmentOutput {
    /// Creates a new builder-style object to manufacture [`GetAttachmentOutput`](crate::output::GetAttachmentOutput).
    pub fn builder() -> crate::output::get_attachment_output::Builder {
        crate::output::get_attachment_output::Builder::default()
    }
}

/// See [`GetAttachmentOutput`](crate::output::GetAttachmentOutput).
pub mod get_attachment_output {
    
    /// A builder for [`GetAttachmentOutput`](crate::output::GetAttachmentOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) url: std::option::Option<std::string::String>,
        pub(crate) url_expiry: std::option::Option<std::string::String>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// <p>This is the pre-signed URL that can be used for uploading the file to Amazon S3 when used in response to <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_StartAttachmentUpload.html">StartAttachmentUpload</a>.</p>
        pub fn url(mut self, input: impl Into<std::string::String>) -> Self {
            self.url = Some(input.into());
            self
        }
        /// <p>This is the pre-signed URL that can be used for uploading the file to Amazon S3 when used in response to <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_StartAttachmentUpload.html">StartAttachmentUpload</a>.</p>
        pub fn set_url(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.url = input; self
        }
        /// <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
        pub fn url_expiry(mut self, input: impl Into<std::string::String>) -> Self {
            self.url_expiry = Some(input.into());
            self
        }
        /// <p>The expiration time of the URL in ISO timestamp. It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
        pub fn set_url_expiry(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.url_expiry = input; self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`GetAttachmentOutput`](crate::output::GetAttachmentOutput).
        pub fn build(self) -> crate::output::GetAttachmentOutput {
            crate::output::GetAttachmentOutput {
                url: self.url
                ,
                url_expiry: self.url_expiry
                ,
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DisconnectParticipantOutput  {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for DisconnectParticipantOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl DisconnectParticipantOutput {
    /// Creates a new builder-style object to manufacture [`DisconnectParticipantOutput`](crate::output::DisconnectParticipantOutput).
    pub fn builder() -> crate::output::disconnect_participant_output::Builder {
        crate::output::disconnect_participant_output::Builder::default()
    }
}

/// See [`DisconnectParticipantOutput`](crate::output::DisconnectParticipantOutput).
pub mod disconnect_participant_output {
    
    /// A builder for [`DisconnectParticipantOutput`](crate::output::DisconnectParticipantOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        _request_id: Option<String>,
    }
    impl Builder {
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`DisconnectParticipantOutput`](crate::output::DisconnectParticipantOutput).
        pub fn build(self) -> crate::output::DisconnectParticipantOutput {
            crate::output::DisconnectParticipantOutput {
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateParticipantConnectionOutput  {
    /// <p>Creates the participant's websocket connection.</p>
    #[doc(hidden)]
    pub websocket: std::option::Option<crate::model::Websocket>,
    /// <p>Creates the participant's connection credentials. The authentication token associated with the participant's connection.</p>
    #[doc(hidden)]
    pub connection_credentials: std::option::Option<crate::model::ConnectionCredentials>,
    _request_id: Option<String>,
}
impl CreateParticipantConnectionOutput {
    /// <p>Creates the participant's websocket connection.</p>
    pub fn websocket(&self) -> std::option::Option<& crate::model::Websocket> {
        self.websocket.as_ref()
    }
    /// <p>Creates the participant's connection credentials. The authentication token associated with the participant's connection.</p>
    pub fn connection_credentials(&self) -> std::option::Option<& crate::model::ConnectionCredentials> {
        self.connection_credentials.as_ref()
    }
}
impl aws_http::request_id::RequestId for CreateParticipantConnectionOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl CreateParticipantConnectionOutput {
    /// Creates a new builder-style object to manufacture [`CreateParticipantConnectionOutput`](crate::output::CreateParticipantConnectionOutput).
    pub fn builder() -> crate::output::create_participant_connection_output::Builder {
        crate::output::create_participant_connection_output::Builder::default()
    }
}

/// See [`CreateParticipantConnectionOutput`](crate::output::CreateParticipantConnectionOutput).
pub mod create_participant_connection_output {
    
    /// A builder for [`CreateParticipantConnectionOutput`](crate::output::CreateParticipantConnectionOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) websocket: std::option::Option<crate::model::Websocket>,
        pub(crate) connection_credentials: std::option::Option<crate::model::ConnectionCredentials>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// <p>Creates the participant's websocket connection.</p>
        pub fn websocket(mut self, input: crate::model::Websocket) -> Self {
            self.websocket = Some(input);
            self
        }
        /// <p>Creates the participant's websocket connection.</p>
        pub fn set_websocket(mut self, input: std::option::Option<crate::model::Websocket>) -> Self {
            self.websocket = input; self
        }
        /// <p>Creates the participant's connection credentials. The authentication token associated with the participant's connection.</p>
        pub fn connection_credentials(mut self, input: crate::model::ConnectionCredentials) -> Self {
            self.connection_credentials = Some(input);
            self
        }
        /// <p>Creates the participant's connection credentials. The authentication token associated with the participant's connection.</p>
        pub fn set_connection_credentials(mut self, input: std::option::Option<crate::model::ConnectionCredentials>) -> Self {
            self.connection_credentials = input; self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`CreateParticipantConnectionOutput`](crate::output::CreateParticipantConnectionOutput).
        pub fn build(self) -> crate::output::CreateParticipantConnectionOutput {
            crate::output::CreateParticipantConnectionOutput {
                websocket: self.websocket
                ,
                connection_credentials: self.connection_credentials
                ,
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CompleteAttachmentUploadOutput  {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for CompleteAttachmentUploadOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl CompleteAttachmentUploadOutput {
    /// Creates a new builder-style object to manufacture [`CompleteAttachmentUploadOutput`](crate::output::CompleteAttachmentUploadOutput).
    pub fn builder() -> crate::output::complete_attachment_upload_output::Builder {
        crate::output::complete_attachment_upload_output::Builder::default()
    }
}

/// See [`CompleteAttachmentUploadOutput`](crate::output::CompleteAttachmentUploadOutput).
pub mod complete_attachment_upload_output {
    
    /// A builder for [`CompleteAttachmentUploadOutput`](crate::output::CompleteAttachmentUploadOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        _request_id: Option<String>,
    }
    impl Builder {
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`CompleteAttachmentUploadOutput`](crate::output::CompleteAttachmentUploadOutput).
        pub fn build(self) -> crate::output::CompleteAttachmentUploadOutput {
            crate::output::CompleteAttachmentUploadOutput {
                _request_id: self._request_id,
            }
        }
    }
    
    
}

