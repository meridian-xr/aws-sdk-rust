// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`DeleteReportDefinitionInput`](crate::input::DeleteReportDefinitionInput)
pub mod delete_report_definition_input {
    /// A builder for [`DeleteReportDefinitionInput`](crate::input::DeleteReportDefinitionInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) report_name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of the report that you want to delete. The name must be unique, is case sensitive, and can't include spaces.</p>
        pub fn report_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.report_name = Some(input.into());
            self
        }
        pub fn set_report_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.report_name = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteReportDefinitionInput`](crate::input::DeleteReportDefinitionInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::DeleteReportDefinitionInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::DeleteReportDefinitionInput {
                report_name: self.report_name,
            })
        }
    }
}
#[doc(hidden)]
pub type DeleteReportDefinitionInputOperationOutputAlias = crate::operation::DeleteReportDefinition;
#[doc(hidden)]
pub type DeleteReportDefinitionInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl DeleteReportDefinitionInput {
    /// Consumes the builder and constructs an Operation<[`DeleteReportDefinition`](crate::operation::DeleteReportDefinition)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::DeleteReportDefinition,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_crate_operation_delete_report_definition(
                    &self,
                )
                .map_err(|err| {
                    smithy_http::operation::BuildError::SerializationError(err.into())
                })?;
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
                crate::operation::DeleteReportDefinition::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "DeleteReportDefinition",
                "costandusagereportservice",
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
            "application/x-amz-json-1.1",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            http::header::HeaderName::from_static("x-amz-target"),
            "AWSOrigamiServiceGatewayService.DeleteReportDefinition",
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
    /// Creates a new builder-style object to manufacture [`DeleteReportDefinitionInput`](crate::input::DeleteReportDefinitionInput)
    pub fn builder() -> crate::input::delete_report_definition_input::Builder {
        crate::input::delete_report_definition_input::Builder::default()
    }
}

/// See [`DescribeReportDefinitionsInput`](crate::input::DescribeReportDefinitionsInput)
pub mod describe_report_definitions_input {
    /// A builder for [`DescribeReportDefinitionsInput`](crate::input::DescribeReportDefinitionsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) max_results: std::option::Option<i32>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The maximum number of results that AWS returns for the operation.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// <p>A generic string.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeReportDefinitionsInput`](crate::input::DescribeReportDefinitionsInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::DescribeReportDefinitionsInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::DescribeReportDefinitionsInput {
                max_results: self.max_results,
                next_token: self.next_token,
            })
        }
    }
}
#[doc(hidden)]
pub type DescribeReportDefinitionsInputOperationOutputAlias =
    crate::operation::DescribeReportDefinitions;
#[doc(hidden)]
pub type DescribeReportDefinitionsInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl DescribeReportDefinitionsInput {
    /// Consumes the builder and constructs an Operation<[`DescribeReportDefinitions`](crate::operation::DescribeReportDefinitions)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::DescribeReportDefinitions,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_crate_operation_describe_report_definitions(&self).map_err(|err|smithy_http::operation::BuildError::SerializationError(err.into()))?
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
                crate::operation::DescribeReportDefinitions::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "DescribeReportDefinitions",
                "costandusagereportservice",
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
            "application/x-amz-json-1.1",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            http::header::HeaderName::from_static("x-amz-target"),
            "AWSOrigamiServiceGatewayService.DescribeReportDefinitions",
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
    /// Creates a new builder-style object to manufacture [`DescribeReportDefinitionsInput`](crate::input::DescribeReportDefinitionsInput)
    pub fn builder() -> crate::input::describe_report_definitions_input::Builder {
        crate::input::describe_report_definitions_input::Builder::default()
    }
}

/// See [`ModifyReportDefinitionInput`](crate::input::ModifyReportDefinitionInput)
pub mod modify_report_definition_input {
    /// A builder for [`ModifyReportDefinitionInput`](crate::input::ModifyReportDefinitionInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) report_name: std::option::Option<std::string::String>,
        pub(crate) report_definition: std::option::Option<crate::model::ReportDefinition>,
    }
    impl Builder {
        /// <p>The name of the report that you want to create. The name must be unique,
        /// is case sensitive, and can't include spaces. </p>
        pub fn report_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.report_name = Some(input.into());
            self
        }
        pub fn set_report_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.report_name = input;
            self
        }
        /// <p>The definition of AWS Cost and Usage Report. You can specify the report name,
        /// time unit, report format, compression format, S3 bucket, additional artifacts, and schema
        /// elements in the definition.
        /// </p>
        pub fn report_definition(mut self, input: crate::model::ReportDefinition) -> Self {
            self.report_definition = Some(input);
            self
        }
        pub fn set_report_definition(
            mut self,
            input: std::option::Option<crate::model::ReportDefinition>,
        ) -> Self {
            self.report_definition = input;
            self
        }
        /// Consumes the builder and constructs a [`ModifyReportDefinitionInput`](crate::input::ModifyReportDefinitionInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::ModifyReportDefinitionInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::ModifyReportDefinitionInput {
                report_name: self.report_name,
                report_definition: self.report_definition,
            })
        }
    }
}
#[doc(hidden)]
pub type ModifyReportDefinitionInputOperationOutputAlias = crate::operation::ModifyReportDefinition;
#[doc(hidden)]
pub type ModifyReportDefinitionInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl ModifyReportDefinitionInput {
    /// Consumes the builder and constructs an Operation<[`ModifyReportDefinition`](crate::operation::ModifyReportDefinition)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::ModifyReportDefinition,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_crate_operation_modify_report_definition(
                    &self,
                )
                .map_err(|err| {
                    smithy_http::operation::BuildError::SerializationError(err.into())
                })?;
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
                crate::operation::ModifyReportDefinition::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "ModifyReportDefinition",
                "costandusagereportservice",
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
            "application/x-amz-json-1.1",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            http::header::HeaderName::from_static("x-amz-target"),
            "AWSOrigamiServiceGatewayService.ModifyReportDefinition",
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
    /// Creates a new builder-style object to manufacture [`ModifyReportDefinitionInput`](crate::input::ModifyReportDefinitionInput)
    pub fn builder() -> crate::input::modify_report_definition_input::Builder {
        crate::input::modify_report_definition_input::Builder::default()
    }
}

/// See [`PutReportDefinitionInput`](crate::input::PutReportDefinitionInput)
pub mod put_report_definition_input {
    /// A builder for [`PutReportDefinitionInput`](crate::input::PutReportDefinitionInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) report_definition: std::option::Option<crate::model::ReportDefinition>,
    }
    impl Builder {
        /// <p>Represents the output of the PutReportDefinition operation. The content consists of the detailed
        /// metadata and data file information. </p>
        pub fn report_definition(mut self, input: crate::model::ReportDefinition) -> Self {
            self.report_definition = Some(input);
            self
        }
        pub fn set_report_definition(
            mut self,
            input: std::option::Option<crate::model::ReportDefinition>,
        ) -> Self {
            self.report_definition = input;
            self
        }
        /// Consumes the builder and constructs a [`PutReportDefinitionInput`](crate::input::PutReportDefinitionInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::PutReportDefinitionInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::PutReportDefinitionInput {
                report_definition: self.report_definition,
            })
        }
    }
}
#[doc(hidden)]
pub type PutReportDefinitionInputOperationOutputAlias = crate::operation::PutReportDefinition;
#[doc(hidden)]
pub type PutReportDefinitionInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl PutReportDefinitionInput {
    /// Consumes the builder and constructs an Operation<[`PutReportDefinition`](crate::operation::PutReportDefinition)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::PutReportDefinition,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let properties = smithy_http::property_bag::SharedPropertyBag::new();
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_crate_operation_put_report_definition(
                    &self,
                )
                .map_err(|err| {
                    smithy_http::operation::BuildError::SerializationError(err.into())
                })?;
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
                crate::operation::PutReportDefinition::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "PutReportDefinition",
                "costandusagereportservice",
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
            "application/x-amz-json-1.1",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            http::header::HeaderName::from_static("x-amz-target"),
            "AWSOrigamiServiceGatewayService.PutReportDefinition",
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
    /// Creates a new builder-style object to manufacture [`PutReportDefinitionInput`](crate::input::PutReportDefinitionInput)
    pub fn builder() -> crate::input::put_report_definition_input::Builder {
        crate::input::put_report_definition_input::Builder::default()
    }
}

/// <p>Creates a Cost and Usage Report.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutReportDefinitionInput {
    /// <p>Represents the output of the PutReportDefinition operation. The content consists of the detailed
    /// metadata and data file information. </p>
    pub report_definition: std::option::Option<crate::model::ReportDefinition>,
}
impl std::fmt::Debug for PutReportDefinitionInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutReportDefinitionInput");
        formatter.field("report_definition", &self.report_definition);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ModifyReportDefinitionInput {
    /// <p>The name of the report that you want to create. The name must be unique,
    /// is case sensitive, and can't include spaces. </p>
    pub report_name: std::option::Option<std::string::String>,
    /// <p>The definition of AWS Cost and Usage Report. You can specify the report name,
    /// time unit, report format, compression format, S3 bucket, additional artifacts, and schema
    /// elements in the definition.
    /// </p>
    pub report_definition: std::option::Option<crate::model::ReportDefinition>,
}
impl std::fmt::Debug for ModifyReportDefinitionInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ModifyReportDefinitionInput");
        formatter.field("report_name", &self.report_name);
        formatter.field("report_definition", &self.report_definition);
        formatter.finish()
    }
}

/// <p>Requests a list of AWS Cost and Usage reports owned by the account.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeReportDefinitionsInput {
    /// <p>The maximum number of results that AWS returns for the operation.</p>
    pub max_results: std::option::Option<i32>,
    /// <p>A generic string.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DescribeReportDefinitionsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeReportDefinitionsInput");
        formatter.field("max_results", &self.max_results);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}

/// <p>Deletes the specified report.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteReportDefinitionInput {
    /// <p>The name of the report that you want to delete. The name must be unique, is case sensitive, and can't include spaces.</p>
    pub report_name: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DeleteReportDefinitionInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteReportDefinitionInput");
        formatter.field("report_name", &self.report_name);
        formatter.finish()
    }
}
