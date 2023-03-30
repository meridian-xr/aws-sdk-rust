// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Do not use this.
            ///
            /// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
            #[deprecated(note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).")]
            pub type GetEntitlementsErrorKind = GetEntitlementsError;
/// Error type for the `GetEntitlementsError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetEntitlementsError {
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceErrorException(crate::error::InternalServiceErrorException),
    /// <p>One or more parameters in your request was invalid.</p>
    InvalidParameterException(crate::error::InvalidParameterException),
    /// <p>The calls to the GetEntitlements API are throttled.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
                    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for GetEntitlementsError {
    
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
impl std::fmt::Display for GetEntitlementsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalServiceErrorException(_inner) =>
            _inner.fmt(f)
            ,
            Self::InvalidParameterException(_inner) =>
            _inner.fmt(f)
            ,
            Self::ThrottlingException(_inner) =>
            _inner.fmt(f)
            ,
            Self::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for GetEntitlementsError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InternalServiceErrorException(_inner) =>
            aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            ,
            Self::InvalidParameterException(_inner) =>
            aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            ,
            Self::ThrottlingException(_inner) =>
            aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            ,
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::error::GetEntitlementsError {
                            fn request_id(&self) -> Option<&str> {
                                self.meta().request_id()
                            }
                        }
impl aws_smithy_types::retry::ProvideErrorKind for GetEntitlementsError {
    fn code(&self) -> Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetEntitlementsError {
    /// Creates the `GetEntitlementsError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self::Unhandled(aws_smithy_types::error::Unhandled::builder().source(err).build())
                    }
    
                    /// Creates the `GetEntitlementsError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::InternalServiceErrorException(e) => e.meta(),
            Self::InvalidParameterException(e) => e.meta(),
            Self::ThrottlingException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `GetEntitlementsError::InternalServiceErrorException`.
    pub fn is_internal_service_error_exception(&self) -> bool {
        matches!(self, Self::InternalServiceErrorException(_))
    }
    /// Returns `true` if the error kind is `GetEntitlementsError::InvalidParameterException`.
    pub fn is_invalid_parameter_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterException(_))
    }
    /// Returns `true` if the error kind is `GetEntitlementsError::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(self, Self::ThrottlingException(_))
    }
}
impl std::error::Error for GetEntitlementsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InternalServiceErrorException(_inner) =>
            Some(_inner)
            ,
            Self::InvalidParameterException(_inner) =>
            Some(_inner)
            ,
            Self::ThrottlingException(_inner) =>
            Some(_inner)
            ,
            Self::Unhandled(_inner) => {
                Some(_inner)
            }
        }
    }
}

/// <p>The calls to the GetEntitlements API are throttled.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ThrottlingException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl ThrottlingException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThrottlingException")?;
        if let Some(inner_1) = &self.message {
             {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ThrottlingException {}
impl aws_http::request_id::RequestId for crate::error::ThrottlingException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for ThrottlingException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata { &self.meta }
}
impl ThrottlingException {
    /// Creates a new builder-style object to manufacture [`ThrottlingException`](crate::error::ThrottlingException).
    pub fn builder() -> crate::error::throttling_exception::Builder {
        crate::error::throttling_exception::Builder::default()
    }
}

/// See [`ThrottlingException`](crate::error::ThrottlingException).
pub mod throttling_exception {
    
    /// A builder for [`ThrottlingException`](crate::error::ThrottlingException).
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
        /// Consumes the builder and constructs a [`ThrottlingException`](crate::error::ThrottlingException).
        pub fn build(self) -> crate::error::ThrottlingException {
            crate::error::ThrottlingException {
                message: self.message
                ,
                meta: self.meta.unwrap_or_default(),
            }
        }
    }
    
    
}

/// <p>One or more parameters in your request was invalid.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InvalidParameterException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl InvalidParameterException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for InvalidParameterException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidParameterException")?;
        if let Some(inner_2) = &self.message {
             {
                write!(f, ": {}", inner_2)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InvalidParameterException {}
impl aws_http::request_id::RequestId for crate::error::InvalidParameterException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for InvalidParameterException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata { &self.meta }
}
impl InvalidParameterException {
    /// Creates a new builder-style object to manufacture [`InvalidParameterException`](crate::error::InvalidParameterException).
    pub fn builder() -> crate::error::invalid_parameter_exception::Builder {
        crate::error::invalid_parameter_exception::Builder::default()
    }
}

/// See [`InvalidParameterException`](crate::error::InvalidParameterException).
pub mod invalid_parameter_exception {
    
    /// A builder for [`InvalidParameterException`](crate::error::InvalidParameterException).
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
        /// Consumes the builder and constructs a [`InvalidParameterException`](crate::error::InvalidParameterException).
        pub fn build(self) -> crate::error::InvalidParameterException {
            crate::error::InvalidParameterException {
                message: self.message
                ,
                meta: self.meta.unwrap_or_default(),
            }
        }
    }
    
    
}

/// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InternalServiceErrorException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl InternalServiceErrorException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for InternalServiceErrorException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServiceErrorException")?;
        if let Some(inner_3) = &self.message {
             {
                write!(f, ": {}", inner_3)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InternalServiceErrorException {}
impl aws_http::request_id::RequestId for crate::error::InternalServiceErrorException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for InternalServiceErrorException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata { &self.meta }
}
impl InternalServiceErrorException {
    /// Creates a new builder-style object to manufacture [`InternalServiceErrorException`](crate::error::InternalServiceErrorException).
    pub fn builder() -> crate::error::internal_service_error_exception::Builder {
        crate::error::internal_service_error_exception::Builder::default()
    }
}

/// See [`InternalServiceErrorException`](crate::error::InternalServiceErrorException).
pub mod internal_service_error_exception {
    
    /// A builder for [`InternalServiceErrorException`](crate::error::InternalServiceErrorException).
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
        /// Consumes the builder and constructs a [`InternalServiceErrorException`](crate::error::InternalServiceErrorException).
        pub fn build(self) -> crate::error::InternalServiceErrorException {
            crate::error::InternalServiceErrorException {
                message: self.message
                ,
                meta: self.meta.unwrap_or_default(),
            }
        }
    }
    
    
}

