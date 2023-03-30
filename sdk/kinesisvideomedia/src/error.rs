// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Do not use this.
            ///
            /// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
            #[deprecated(note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).")]
            pub type GetMediaErrorKind = GetMediaError;
/// Error type for the `GetMediaError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetMediaError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceededException(crate::error::ClientLimitExceededException),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client connections.</p>
    ConnectionLimitExceededException(crate::error::ConnectionLimitExceededException),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgumentException(crate::error::InvalidArgumentException),
    /// <p> Status Code: 400, Caller used wrong endpoint to write data to a stream. On receiving such an exception, the user must call <code>GetDataEndpoint</code> with <code>AccessMode</code> set to "READ" and use the endpoint Kinesis Video returns in the next <code>GetMedia</code> call. </p>
    InvalidEndpointException(crate::error::InvalidEndpointException),
    /// <p>Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired.</p>
    NotAuthorizedException(crate::error::NotAuthorizedException),
    /// <p>Status Code: 404, The stream with the given name does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
                    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for GetMediaError {
    
                    fn create_unhandled_error(
                        source: Box<dyn std::error::Error + Send + Sync + 'static>,
                        meta: Option<aws_smithy_types::error::ErrorMetadata>
                    ) -> Self
                     {
        Self::Unhandled({
                                let mut builder = aws_smithy_types::error::Unhandled::builder().source(source);
                                builder.set_meta(meta);
                                builder.build()
                            })
    }
}
impl std::fmt::Display for GetMediaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClientLimitExceededException(_inner) =>
            _inner.fmt(f)
            ,
            Self::ConnectionLimitExceededException(_inner) =>
            _inner.fmt(f)
            ,
            Self::InvalidArgumentException(_inner) =>
            _inner.fmt(f)
            ,
            Self::InvalidEndpointException(_inner) =>
            _inner.fmt(f)
            ,
            Self::NotAuthorizedException(_inner) =>
            _inner.fmt(f)
            ,
            Self::ResourceNotFoundException(_inner) =>
            _inner.fmt(f)
            ,
            Self::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for GetMediaError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ClientLimitExceededException(_inner) =>
            aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            ,
            Self::ConnectionLimitExceededException(_inner) =>
            aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            ,
            Self::InvalidArgumentException(_inner) =>
            aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            ,
            Self::InvalidEndpointException(_inner) =>
            aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            ,
            Self::NotAuthorizedException(_inner) =>
            aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            ,
            Self::ResourceNotFoundException(_inner) =>
            aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            ,
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::error::GetMediaError {
                            fn request_id(&self) -> Option<&str> {
                                self.meta().request_id()
                            }
                        }
impl aws_smithy_types::retry::ProvideErrorKind for GetMediaError {
    fn code(&self) -> Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetMediaError {
    /// Creates the `GetMediaError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self::Unhandled(aws_smithy_types::error::Unhandled::builder().source(err).build())
                    }
    
                    /// Creates the `GetMediaError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
                    pub fn generic(err: aws_smithy_types::error::ErrorMetadata) -> Self {
                        Self::Unhandled(aws_smithy_types::error::Unhandled::builder().source(err.clone()).meta(err).build())
                    }
    /// 
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    /// 
    pub fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::ClientLimitExceededException(e) => e.meta(),
            Self::ConnectionLimitExceededException(e) => e.meta(),
            Self::InvalidArgumentException(e) => e.meta(),
            Self::InvalidEndpointException(e) => e.meta(),
            Self::NotAuthorizedException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `GetMediaError::ClientLimitExceededException`.
    pub fn is_client_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::ClientLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `GetMediaError::ConnectionLimitExceededException`.
    pub fn is_connection_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::ConnectionLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `GetMediaError::InvalidArgumentException`.
    pub fn is_invalid_argument_exception(&self) -> bool {
        matches!(self, Self::InvalidArgumentException(_))
    }
    /// Returns `true` if the error kind is `GetMediaError::InvalidEndpointException`.
    pub fn is_invalid_endpoint_exception(&self) -> bool {
        matches!(self, Self::InvalidEndpointException(_))
    }
    /// Returns `true` if the error kind is `GetMediaError::NotAuthorizedException`.
    pub fn is_not_authorized_exception(&self) -> bool {
        matches!(self, Self::NotAuthorizedException(_))
    }
    /// Returns `true` if the error kind is `GetMediaError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
}
impl std::error::Error for GetMediaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ClientLimitExceededException(_inner) =>
            Some(_inner)
            ,
            Self::ConnectionLimitExceededException(_inner) =>
            Some(_inner)
            ,
            Self::InvalidArgumentException(_inner) =>
            Some(_inner)
            ,
            Self::InvalidEndpointException(_inner) =>
            Some(_inner)
            ,
            Self::NotAuthorizedException(_inner) =>
            Some(_inner)
            ,
            Self::ResourceNotFoundException(_inner) =>
            Some(_inner)
            ,
            Self::Unhandled(_inner) => {
                Some(_inner)
            }
        }
    }
}

/// <p>Status Code: 404, The stream with the given name does not exist.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ResourceNotFoundException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl ResourceNotFoundException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_1) = &self.message {
             {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFoundException {}
impl aws_http::request_id::RequestId for crate::error::ResourceNotFoundException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for ResourceNotFoundException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata { &self.meta }
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
    pub fn builder() -> crate::error::resource_not_found_exception::Builder {
        crate::error::resource_not_found_exception::Builder::default()
    }
}

/// See [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
pub mod resource_not_found_exception {
    
    /// A builder for [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        meta: Option<aws_smithy_types::error::ErrorMetadata>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Sets error metadata
                                                pub fn meta(mut self, meta: aws_smithy_types::error::ErrorMetadata) -> Self {
                                                    self.meta = Some(meta);
                                                    self
                                                }
        
                                                /// Sets error metadata
                                                pub fn set_meta(&mut self, meta: Option<aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
                                                    self.meta = meta;
                                                    self
                                                }
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message
                ,
                meta: self.meta.unwrap_or_default(),
            }
        }
    }
    
    
}

/// <p>Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct NotAuthorizedException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl NotAuthorizedException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for NotAuthorizedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotAuthorizedException")?;
        if let Some(inner_2) = &self.message {
             {
                write!(f, ": {}", inner_2)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for NotAuthorizedException {}
impl aws_http::request_id::RequestId for crate::error::NotAuthorizedException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for NotAuthorizedException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata { &self.meta }
}
impl NotAuthorizedException {
    /// Creates a new builder-style object to manufacture [`NotAuthorizedException`](crate::error::NotAuthorizedException).
    pub fn builder() -> crate::error::not_authorized_exception::Builder {
        crate::error::not_authorized_exception::Builder::default()
    }
}

/// See [`NotAuthorizedException`](crate::error::NotAuthorizedException).
pub mod not_authorized_exception {
    
    /// A builder for [`NotAuthorizedException`](crate::error::NotAuthorizedException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        meta: Option<aws_smithy_types::error::ErrorMetadata>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Sets error metadata
                                                pub fn meta(mut self, meta: aws_smithy_types::error::ErrorMetadata) -> Self {
                                                    self.meta = Some(meta);
                                                    self
                                                }
        
                                                /// Sets error metadata
                                                pub fn set_meta(&mut self, meta: Option<aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
                                                    self.meta = meta;
                                                    self
                                                }
        /// Consumes the builder and constructs a [`NotAuthorizedException`](crate::error::NotAuthorizedException).
        pub fn build(self) -> crate::error::NotAuthorizedException {
            crate::error::NotAuthorizedException {
                message: self.message
                ,
                meta: self.meta.unwrap_or_default(),
            }
        }
    }
    
    
}

/// <p> Status Code: 400, Caller used wrong endpoint to write data to a stream. On receiving such an exception, the user must call <code>GetDataEndpoint</code> with <code>AccessMode</code> set to "READ" and use the endpoint Kinesis Video returns in the next <code>GetMedia</code> call. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InvalidEndpointException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl InvalidEndpointException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for InvalidEndpointException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidEndpointException")?;
        if let Some(inner_3) = &self.message {
             {
                write!(f, ": {}", inner_3)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InvalidEndpointException {}
impl aws_http::request_id::RequestId for crate::error::InvalidEndpointException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for InvalidEndpointException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata { &self.meta }
}
impl InvalidEndpointException {
    /// Creates a new builder-style object to manufacture [`InvalidEndpointException`](crate::error::InvalidEndpointException).
    pub fn builder() -> crate::error::invalid_endpoint_exception::Builder {
        crate::error::invalid_endpoint_exception::Builder::default()
    }
}

/// See [`InvalidEndpointException`](crate::error::InvalidEndpointException).
pub mod invalid_endpoint_exception {
    
    /// A builder for [`InvalidEndpointException`](crate::error::InvalidEndpointException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        meta: Option<aws_smithy_types::error::ErrorMetadata>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Sets error metadata
                                                pub fn meta(mut self, meta: aws_smithy_types::error::ErrorMetadata) -> Self {
                                                    self.meta = Some(meta);
                                                    self
                                                }
        
                                                /// Sets error metadata
                                                pub fn set_meta(&mut self, meta: Option<aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
                                                    self.meta = meta;
                                                    self
                                                }
        /// Consumes the builder and constructs a [`InvalidEndpointException`](crate::error::InvalidEndpointException).
        pub fn build(self) -> crate::error::InvalidEndpointException {
            crate::error::InvalidEndpointException {
                message: self.message
                ,
                meta: self.meta.unwrap_or_default(),
            }
        }
    }
    
    
}

/// <p>The value for this input parameter is invalid.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InvalidArgumentException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl InvalidArgumentException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for InvalidArgumentException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidArgumentException")?;
        if let Some(inner_4) = &self.message {
             {
                write!(f, ": {}", inner_4)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InvalidArgumentException {}
impl aws_http::request_id::RequestId for crate::error::InvalidArgumentException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for InvalidArgumentException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata { &self.meta }
}
impl InvalidArgumentException {
    /// Creates a new builder-style object to manufacture [`InvalidArgumentException`](crate::error::InvalidArgumentException).
    pub fn builder() -> crate::error::invalid_argument_exception::Builder {
        crate::error::invalid_argument_exception::Builder::default()
    }
}

/// See [`InvalidArgumentException`](crate::error::InvalidArgumentException).
pub mod invalid_argument_exception {
    
    /// A builder for [`InvalidArgumentException`](crate::error::InvalidArgumentException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        meta: Option<aws_smithy_types::error::ErrorMetadata>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Sets error metadata
                                                pub fn meta(mut self, meta: aws_smithy_types::error::ErrorMetadata) -> Self {
                                                    self.meta = Some(meta);
                                                    self
                                                }
        
                                                /// Sets error metadata
                                                pub fn set_meta(&mut self, meta: Option<aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
                                                    self.meta = meta;
                                                    self
                                                }
        /// Consumes the builder and constructs a [`InvalidArgumentException`](crate::error::InvalidArgumentException).
        pub fn build(self) -> crate::error::InvalidArgumentException {
            crate::error::InvalidArgumentException {
                message: self.message
                ,
                meta: self.meta.unwrap_or_default(),
            }
        }
    }
    
    
}

/// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client connections.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ConnectionLimitExceededException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl ConnectionLimitExceededException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for ConnectionLimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionLimitExceededException")?;
        if let Some(inner_5) = &self.message {
             {
                write!(f, ": {}", inner_5)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ConnectionLimitExceededException {}
impl aws_http::request_id::RequestId for crate::error::ConnectionLimitExceededException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for ConnectionLimitExceededException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata { &self.meta }
}
impl ConnectionLimitExceededException {
    /// Creates a new builder-style object to manufacture [`ConnectionLimitExceededException`](crate::error::ConnectionLimitExceededException).
    pub fn builder() -> crate::error::connection_limit_exceeded_exception::Builder {
        crate::error::connection_limit_exceeded_exception::Builder::default()
    }
}

/// See [`ConnectionLimitExceededException`](crate::error::ConnectionLimitExceededException).
pub mod connection_limit_exceeded_exception {
    
    /// A builder for [`ConnectionLimitExceededException`](crate::error::ConnectionLimitExceededException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        meta: Option<aws_smithy_types::error::ErrorMetadata>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Sets error metadata
                                                pub fn meta(mut self, meta: aws_smithy_types::error::ErrorMetadata) -> Self {
                                                    self.meta = Some(meta);
                                                    self
                                                }
        
                                                /// Sets error metadata
                                                pub fn set_meta(&mut self, meta: Option<aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
                                                    self.meta = meta;
                                                    self
                                                }
        /// Consumes the builder and constructs a [`ConnectionLimitExceededException`](crate::error::ConnectionLimitExceededException).
        pub fn build(self) -> crate::error::ConnectionLimitExceededException {
            crate::error::ConnectionLimitExceededException {
                message: self.message
                ,
                meta: self.meta.unwrap_or_default(),
            }
        }
    }
    
    
}

/// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ClientLimitExceededException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl ClientLimitExceededException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for ClientLimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientLimitExceededException")?;
        if let Some(inner_6) = &self.message {
             {
                write!(f, ": {}", inner_6)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ClientLimitExceededException {}
impl aws_http::request_id::RequestId for crate::error::ClientLimitExceededException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for ClientLimitExceededException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata { &self.meta }
}
impl ClientLimitExceededException {
    /// Creates a new builder-style object to manufacture [`ClientLimitExceededException`](crate::error::ClientLimitExceededException).
    pub fn builder() -> crate::error::client_limit_exceeded_exception::Builder {
        crate::error::client_limit_exceeded_exception::Builder::default()
    }
}

/// See [`ClientLimitExceededException`](crate::error::ClientLimitExceededException).
pub mod client_limit_exceeded_exception {
    
    /// A builder for [`ClientLimitExceededException`](crate::error::ClientLimitExceededException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        meta: Option<aws_smithy_types::error::ErrorMetadata>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Sets error metadata
                                                pub fn meta(mut self, meta: aws_smithy_types::error::ErrorMetadata) -> Self {
                                                    self.meta = Some(meta);
                                                    self
                                                }
        
                                                /// Sets error metadata
                                                pub fn set_meta(&mut self, meta: Option<aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
                                                    self.meta = meta;
                                                    self
                                                }
        /// Consumes the builder and constructs a [`ClientLimitExceededException`](crate::error::ClientLimitExceededException).
        pub fn build(self) -> crate::error::ClientLimitExceededException {
            crate::error::ClientLimitExceededException {
                message: self.message
                ,
                meta: self.meta.unwrap_or_default(),
            }
        }
    }
    
    
}

