// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`GetRoutingControlStateInput`](crate::input::GetRoutingControlStateInput)
pub mod get_routing_control_state_input {
    /// A builder for [`GetRoutingControlStateInput`](crate::input::GetRoutingControlStateInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) routing_control_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The Amazon Resource Number (ARN) for the routing control that you want to get the state for.</p>
        pub fn routing_control_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.routing_control_arn = Some(input.into());
            self
        }
        pub fn set_routing_control_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.routing_control_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`GetRoutingControlStateInput`](crate::input::GetRoutingControlStateInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::GetRoutingControlStateInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::GetRoutingControlStateInput {
                routing_control_arn: self.routing_control_arn,
            })
        }
    }
}
#[doc(hidden)]
pub type GetRoutingControlStateInputOperationOutputAlias = crate::operation::GetRoutingControlState;
#[doc(hidden)]
pub type GetRoutingControlStateInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl GetRoutingControlStateInput {
    /// Consumes the builder and constructs an Operation<[`GetRoutingControlState`](crate::operation::GetRoutingControlState)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::GetRoutingControlState,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_crate_operation_get_routing_control_state(&self).map_err(|err|smithy_http::operation::BuildError::SerializationError(err.into()))?
            ;
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request = smithy_http::operation::Request::from_parts(
                request.map(smithy_http::body::SdkBody::from),
                properties,
            );
            request.properties_mut().insert(
                aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ),
            );
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.properties_mut().insert(signing_config);
            request
                .properties_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.properties_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.properties_mut().insert(region.clone());
            }
            aws_auth::set_provider(
                &mut request.properties_mut(),
                _config.credentials_provider.clone(),
            );
            let op = smithy_http::operation::Operation::new(
                request,
                crate::operation::GetRoutingControlState::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "GetRoutingControlState",
                "route53recoverycluster",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/").expect("formatting should succeed");
        Ok(())
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        Ok(builder.method("POST").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        builder = smithy_http::header::set_header_if_absent(
            builder,
            http::header::HeaderName::from_static("content-type"),
            "application/x-amz-json-1.0",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            http::header::HeaderName::from_static("x-amz-target"),
            "ToggleCustomerAPI.GetRoutingControlState",
        );
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`GetRoutingControlStateInput`](crate::input::GetRoutingControlStateInput)
    pub fn builder() -> crate::input::get_routing_control_state_input::Builder {
        crate::input::get_routing_control_state_input::Builder::default()
    }
}

/// See [`UpdateRoutingControlStateInput`](crate::input::UpdateRoutingControlStateInput)
pub mod update_routing_control_state_input {
    /// A builder for [`UpdateRoutingControlStateInput`](crate::input::UpdateRoutingControlStateInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) routing_control_arn: std::option::Option<std::string::String>,
        pub(crate) routing_control_state: std::option::Option<crate::model::RoutingControlState>,
    }
    impl Builder {
        /// <p>The Amazon Resource Number (ARN) for the routing control that you want to update the state for.</p>
        pub fn routing_control_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.routing_control_arn = Some(input.into());
            self
        }
        pub fn set_routing_control_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.routing_control_arn = input;
            self
        }
        /// <p>The state of the routing control. You can set the value to be On or Off.</p>
        pub fn routing_control_state(mut self, input: crate::model::RoutingControlState) -> Self {
            self.routing_control_state = Some(input);
            self
        }
        pub fn set_routing_control_state(
            mut self,
            input: std::option::Option<crate::model::RoutingControlState>,
        ) -> Self {
            self.routing_control_state = input;
            self
        }
        /// Consumes the builder and constructs a [`UpdateRoutingControlStateInput`](crate::input::UpdateRoutingControlStateInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::UpdateRoutingControlStateInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::UpdateRoutingControlStateInput {
                routing_control_arn: self.routing_control_arn,
                routing_control_state: self.routing_control_state,
            })
        }
    }
}
#[doc(hidden)]
pub type UpdateRoutingControlStateInputOperationOutputAlias =
    crate::operation::UpdateRoutingControlState;
#[doc(hidden)]
pub type UpdateRoutingControlStateInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl UpdateRoutingControlStateInput {
    /// Consumes the builder and constructs an Operation<[`UpdateRoutingControlState`](crate::operation::UpdateRoutingControlState)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::UpdateRoutingControlState,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_crate_operation_update_routing_control_state(&self).map_err(|err|smithy_http::operation::BuildError::SerializationError(err.into()))?
            ;
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request = smithy_http::operation::Request::from_parts(
                request.map(smithy_http::body::SdkBody::from),
                properties,
            );
            request.properties_mut().insert(
                aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ),
            );
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.properties_mut().insert(signing_config);
            request
                .properties_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.properties_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.properties_mut().insert(region.clone());
            }
            aws_auth::set_provider(
                &mut request.properties_mut(),
                _config.credentials_provider.clone(),
            );
            let op = smithy_http::operation::Operation::new(
                request,
                crate::operation::UpdateRoutingControlState::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "UpdateRoutingControlState",
                "route53recoverycluster",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/").expect("formatting should succeed");
        Ok(())
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        Ok(builder.method("POST").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        builder = smithy_http::header::set_header_if_absent(
            builder,
            http::header::HeaderName::from_static("content-type"),
            "application/x-amz-json-1.0",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            http::header::HeaderName::from_static("x-amz-target"),
            "ToggleCustomerAPI.UpdateRoutingControlState",
        );
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`UpdateRoutingControlStateInput`](crate::input::UpdateRoutingControlStateInput)
    pub fn builder() -> crate::input::update_routing_control_state_input::Builder {
        crate::input::update_routing_control_state_input::Builder::default()
    }
}

/// See [`UpdateRoutingControlStatesInput`](crate::input::UpdateRoutingControlStatesInput)
pub mod update_routing_control_states_input {
    /// A builder for [`UpdateRoutingControlStatesInput`](crate::input::UpdateRoutingControlStatesInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) update_routing_control_state_entries:
            std::option::Option<std::vec::Vec<crate::model::UpdateRoutingControlStateEntry>>,
    }
    impl Builder {
        pub fn update_routing_control_state_entries(
            mut self,
            input: impl Into<crate::model::UpdateRoutingControlStateEntry>,
        ) -> Self {
            let mut v = self
                .update_routing_control_state_entries
                .unwrap_or_default();
            v.push(input.into());
            self.update_routing_control_state_entries = Some(v);
            self
        }
        pub fn set_update_routing_control_state_entries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::UpdateRoutingControlStateEntry>>,
        ) -> Self {
            self.update_routing_control_state_entries = input;
            self
        }
        /// Consumes the builder and constructs a [`UpdateRoutingControlStatesInput`](crate::input::UpdateRoutingControlStatesInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::UpdateRoutingControlStatesInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::UpdateRoutingControlStatesInput {
                update_routing_control_state_entries: self.update_routing_control_state_entries,
            })
        }
    }
}
#[doc(hidden)]
pub type UpdateRoutingControlStatesInputOperationOutputAlias =
    crate::operation::UpdateRoutingControlStates;
#[doc(hidden)]
pub type UpdateRoutingControlStatesInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl UpdateRoutingControlStatesInput {
    /// Consumes the builder and constructs an Operation<[`UpdateRoutingControlStates`](crate::operation::UpdateRoutingControlStates)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::UpdateRoutingControlStates,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_crate_operation_update_routing_control_states(&self).map_err(|err|smithy_http::operation::BuildError::SerializationError(err.into()))?
            ;
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request = smithy_http::operation::Request::from_parts(
                request.map(smithy_http::body::SdkBody::from),
                properties,
            );
            request.properties_mut().insert(
                aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ),
            );
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.properties_mut().insert(signing_config);
            request
                .properties_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.properties_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.properties_mut().insert(region.clone());
            }
            aws_auth::set_provider(
                &mut request.properties_mut(),
                _config.credentials_provider.clone(),
            );
            let op = smithy_http::operation::Operation::new(
                request,
                crate::operation::UpdateRoutingControlStates::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "UpdateRoutingControlStates",
                "route53recoverycluster",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/").expect("formatting should succeed");
        Ok(())
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        Ok(builder.method("POST").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        builder = smithy_http::header::set_header_if_absent(
            builder,
            http::header::HeaderName::from_static("content-type"),
            "application/x-amz-json-1.0",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            http::header::HeaderName::from_static("x-amz-target"),
            "ToggleCustomerAPI.UpdateRoutingControlStates",
        );
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`UpdateRoutingControlStatesInput`](crate::input::UpdateRoutingControlStatesInput)
    pub fn builder() -> crate::input::update_routing_control_states_input::Builder {
        crate::input::update_routing_control_states_input::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateRoutingControlStatesInput {
    /// <p>A set of routing control entries that you want to update.</p>
    pub update_routing_control_state_entries:
        std::option::Option<std::vec::Vec<crate::model::UpdateRoutingControlStateEntry>>,
}
impl std::fmt::Debug for UpdateRoutingControlStatesInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateRoutingControlStatesInput");
        formatter.field(
            "update_routing_control_state_entries",
            &self.update_routing_control_state_entries,
        );
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateRoutingControlStateInput {
    /// <p>The Amazon Resource Number (ARN) for the routing control that you want to update the state for.</p>
    pub routing_control_arn: std::option::Option<std::string::String>,
    /// <p>The state of the routing control. You can set the value to be On or Off.</p>
    pub routing_control_state: std::option::Option<crate::model::RoutingControlState>,
}
impl std::fmt::Debug for UpdateRoutingControlStateInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateRoutingControlStateInput");
        formatter.field("routing_control_arn", &self.routing_control_arn);
        formatter.field("routing_control_state", &self.routing_control_state);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetRoutingControlStateInput {
    /// <p>The Amazon Resource Number (ARN) for the routing control that you want to get the state for.</p>
    pub routing_control_arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for GetRoutingControlStateInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetRoutingControlStateInput");
        formatter.field("routing_control_arn", &self.routing_control_arn);
        formatter.finish()
    }
}
