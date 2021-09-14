// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateApplicationOutput {}
impl std::fmt::Debug for UpdateApplicationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateApplicationOutput");
        formatter.finish()
    }
}
/// See [`UpdateApplicationOutput`](crate::output::UpdateApplicationOutput)
pub mod update_application_output {
    /// A builder for [`UpdateApplicationOutput`](crate::output::UpdateApplicationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateApplicationOutput`](crate::output::UpdateApplicationOutput)
        pub fn build(self) -> crate::output::UpdateApplicationOutput {
            crate::output::UpdateApplicationOutput {}
        }
    }
}
impl UpdateApplicationOutput {
    /// Creates a new builder-style object to manufacture [`UpdateApplicationOutput`](crate::output::UpdateApplicationOutput)
    pub fn builder() -> crate::output::update_application_output::Builder {
        crate::output::update_application_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UntagResourceOutput {}
impl std::fmt::Debug for UntagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UntagResourceOutput");
        formatter.finish()
    }
}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput)
pub mod untag_resource_output {
    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput)
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {}
        }
    }
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput)
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TagResourceOutput {}
impl std::fmt::Debug for TagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TagResourceOutput");
        formatter.finish()
    }
}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput)
pub mod tag_resource_output {
    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput)
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {}
        }
    }
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput)
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

/// <p></p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StopApplicationOutput {}
impl std::fmt::Debug for StopApplicationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StopApplicationOutput");
        formatter.finish()
    }
}
/// See [`StopApplicationOutput`](crate::output::StopApplicationOutput)
pub mod stop_application_output {
    /// A builder for [`StopApplicationOutput`](crate::output::StopApplicationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`StopApplicationOutput`](crate::output::StopApplicationOutput)
        pub fn build(self) -> crate::output::StopApplicationOutput {
            crate::output::StopApplicationOutput {}
        }
    }
}
impl StopApplicationOutput {
    /// Creates a new builder-style object to manufacture [`StopApplicationOutput`](crate::output::StopApplicationOutput)
    pub fn builder() -> crate::output::stop_application_output::Builder {
        crate::output::stop_application_output::Builder::default()
    }
}

/// <p></p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StartApplicationOutput {}
impl std::fmt::Debug for StartApplicationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StartApplicationOutput");
        formatter.finish()
    }
}
/// See [`StartApplicationOutput`](crate::output::StartApplicationOutput)
pub mod start_application_output {
    /// A builder for [`StartApplicationOutput`](crate::output::StartApplicationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`StartApplicationOutput`](crate::output::StartApplicationOutput)
        pub fn build(self) -> crate::output::StartApplicationOutput {
            crate::output::StartApplicationOutput {}
        }
    }
}
impl StartApplicationOutput {
    /// Creates a new builder-style object to manufacture [`StartApplicationOutput`](crate::output::StartApplicationOutput)
    pub fn builder() -> crate::output::start_application_output::Builder {
        crate::output::start_application_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListTagsForResourceOutput {
    /// <p>The key-value tags assigned to the application.</p>
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
}
impl std::fmt::Debug for ListTagsForResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListTagsForResourceOutput");
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
pub mod list_tags_for_resource_output {
    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    }
    impl Builder {
        pub fn tags(mut self, input: impl Into<crate::model::Tag>) -> Self {
            let mut v = self.tags.unwrap_or_default();
            v.push(input.into());
            self.tags = Some(v);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput { tags: self.tags }
        }
    }
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

/// <p></p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListApplicationsOutput {
    /// <p>List of <code>ApplicationSummary</code> objects. </p>
    pub application_summaries: std::option::Option<std::vec::Vec<crate::model::ApplicationSummary>>,
    /// <p>Returns true if there are more applications to retrieve.</p>
    pub has_more_applications: std::option::Option<bool>,
}
impl std::fmt::Debug for ListApplicationsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListApplicationsOutput");
        formatter.field("application_summaries", &self.application_summaries);
        formatter.field("has_more_applications", &self.has_more_applications);
        formatter.finish()
    }
}
/// See [`ListApplicationsOutput`](crate::output::ListApplicationsOutput)
pub mod list_applications_output {
    /// A builder for [`ListApplicationsOutput`](crate::output::ListApplicationsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) application_summaries:
            std::option::Option<std::vec::Vec<crate::model::ApplicationSummary>>,
        pub(crate) has_more_applications: std::option::Option<bool>,
    }
    impl Builder {
        pub fn application_summaries(
            mut self,
            input: impl Into<crate::model::ApplicationSummary>,
        ) -> Self {
            let mut v = self.application_summaries.unwrap_or_default();
            v.push(input.into());
            self.application_summaries = Some(v);
            self
        }
        pub fn set_application_summaries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ApplicationSummary>>,
        ) -> Self {
            self.application_summaries = input;
            self
        }
        /// <p>Returns true if there are more applications to retrieve.</p>
        pub fn has_more_applications(mut self, input: bool) -> Self {
            self.has_more_applications = Some(input);
            self
        }
        pub fn set_has_more_applications(mut self, input: std::option::Option<bool>) -> Self {
            self.has_more_applications = input;
            self
        }
        /// Consumes the builder and constructs a [`ListApplicationsOutput`](crate::output::ListApplicationsOutput)
        pub fn build(self) -> crate::output::ListApplicationsOutput {
            crate::output::ListApplicationsOutput {
                application_summaries: self.application_summaries,
                has_more_applications: self.has_more_applications,
            }
        }
    }
}
impl ListApplicationsOutput {
    /// Creates a new builder-style object to manufacture [`ListApplicationsOutput`](crate::output::ListApplicationsOutput)
    pub fn builder() -> crate::output::list_applications_output::Builder {
        crate::output::list_applications_output::Builder::default()
    }
}

/// <p></p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DiscoverInputSchemaOutput {
    /// <p>Schema inferred from the streaming source. It identifies the format of the data in the streaming source and how each data element maps to corresponding columns in the in-application stream that you can create.</p>
    pub input_schema: std::option::Option<crate::model::SourceSchema>,
    /// <p>An array of elements, where each element corresponds to a row in a stream record (a stream record can have more than one row).</p>
    pub parsed_input_records:
        std::option::Option<std::vec::Vec<std::vec::Vec<std::string::String>>>,
    /// <p>Stream data that was modified by the processor specified in the <code>InputProcessingConfiguration</code> parameter.</p>
    pub processed_input_records: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Raw stream data that was sampled to infer the schema.</p>
    pub raw_input_records: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl std::fmt::Debug for DiscoverInputSchemaOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DiscoverInputSchemaOutput");
        formatter.field("input_schema", &self.input_schema);
        formatter.field("parsed_input_records", &self.parsed_input_records);
        formatter.field("processed_input_records", &self.processed_input_records);
        formatter.field("raw_input_records", &self.raw_input_records);
        formatter.finish()
    }
}
/// See [`DiscoverInputSchemaOutput`](crate::output::DiscoverInputSchemaOutput)
pub mod discover_input_schema_output {
    /// A builder for [`DiscoverInputSchemaOutput`](crate::output::DiscoverInputSchemaOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) input_schema: std::option::Option<crate::model::SourceSchema>,
        pub(crate) parsed_input_records:
            std::option::Option<std::vec::Vec<std::vec::Vec<std::string::String>>>,
        pub(crate) processed_input_records: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) raw_input_records: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p>Schema inferred from the streaming source. It identifies the format of the data in the streaming source and how each data element maps to corresponding columns in the in-application stream that you can create.</p>
        pub fn input_schema(mut self, input: crate::model::SourceSchema) -> Self {
            self.input_schema = Some(input);
            self
        }
        pub fn set_input_schema(
            mut self,
            input: std::option::Option<crate::model::SourceSchema>,
        ) -> Self {
            self.input_schema = input;
            self
        }
        pub fn parsed_input_records(
            mut self,
            input: impl Into<std::vec::Vec<std::string::String>>,
        ) -> Self {
            let mut v = self.parsed_input_records.unwrap_or_default();
            v.push(input.into());
            self.parsed_input_records = Some(v);
            self
        }
        pub fn set_parsed_input_records(
            mut self,
            input: std::option::Option<std::vec::Vec<std::vec::Vec<std::string::String>>>,
        ) -> Self {
            self.parsed_input_records = input;
            self
        }
        pub fn processed_input_records(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.processed_input_records.unwrap_or_default();
            v.push(input.into());
            self.processed_input_records = Some(v);
            self
        }
        pub fn set_processed_input_records(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.processed_input_records = input;
            self
        }
        pub fn raw_input_records(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.raw_input_records.unwrap_or_default();
            v.push(input.into());
            self.raw_input_records = Some(v);
            self
        }
        pub fn set_raw_input_records(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.raw_input_records = input;
            self
        }
        /// Consumes the builder and constructs a [`DiscoverInputSchemaOutput`](crate::output::DiscoverInputSchemaOutput)
        pub fn build(self) -> crate::output::DiscoverInputSchemaOutput {
            crate::output::DiscoverInputSchemaOutput {
                input_schema: self.input_schema,
                parsed_input_records: self.parsed_input_records,
                processed_input_records: self.processed_input_records,
                raw_input_records: self.raw_input_records,
            }
        }
    }
}
impl DiscoverInputSchemaOutput {
    /// Creates a new builder-style object to manufacture [`DiscoverInputSchemaOutput`](crate::output::DiscoverInputSchemaOutput)
    pub fn builder() -> crate::output::discover_input_schema_output::Builder {
        crate::output::discover_input_schema_output::Builder::default()
    }
}

/// <p></p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeApplicationOutput {
    /// <p>Provides a description of the application, such as the application Amazon Resource Name (ARN), status, latest version, and input and output configuration details.</p>
    pub application_detail: std::option::Option<crate::model::ApplicationDetail>,
}
impl std::fmt::Debug for DescribeApplicationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeApplicationOutput");
        formatter.field("application_detail", &self.application_detail);
        formatter.finish()
    }
}
/// See [`DescribeApplicationOutput`](crate::output::DescribeApplicationOutput)
pub mod describe_application_output {
    /// A builder for [`DescribeApplicationOutput`](crate::output::DescribeApplicationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) application_detail: std::option::Option<crate::model::ApplicationDetail>,
    }
    impl Builder {
        /// <p>Provides a description of the application, such as the application Amazon Resource Name (ARN), status, latest version, and input and output configuration details.</p>
        pub fn application_detail(mut self, input: crate::model::ApplicationDetail) -> Self {
            self.application_detail = Some(input);
            self
        }
        pub fn set_application_detail(
            mut self,
            input: std::option::Option<crate::model::ApplicationDetail>,
        ) -> Self {
            self.application_detail = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeApplicationOutput`](crate::output::DescribeApplicationOutput)
        pub fn build(self) -> crate::output::DescribeApplicationOutput {
            crate::output::DescribeApplicationOutput {
                application_detail: self.application_detail,
            }
        }
    }
}
impl DescribeApplicationOutput {
    /// Creates a new builder-style object to manufacture [`DescribeApplicationOutput`](crate::output::DescribeApplicationOutput)
    pub fn builder() -> crate::output::describe_application_output::Builder {
        crate::output::describe_application_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteApplicationReferenceDataSourceOutput {}
impl std::fmt::Debug for DeleteApplicationReferenceDataSourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteApplicationReferenceDataSourceOutput");
        formatter.finish()
    }
}
/// See [`DeleteApplicationReferenceDataSourceOutput`](crate::output::DeleteApplicationReferenceDataSourceOutput)
pub mod delete_application_reference_data_source_output {
    /// A builder for [`DeleteApplicationReferenceDataSourceOutput`](crate::output::DeleteApplicationReferenceDataSourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteApplicationReferenceDataSourceOutput`](crate::output::DeleteApplicationReferenceDataSourceOutput)
        pub fn build(self) -> crate::output::DeleteApplicationReferenceDataSourceOutput {
            crate::output::DeleteApplicationReferenceDataSourceOutput {}
        }
    }
}
impl DeleteApplicationReferenceDataSourceOutput {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationReferenceDataSourceOutput`](crate::output::DeleteApplicationReferenceDataSourceOutput)
    pub fn builder() -> crate::output::delete_application_reference_data_source_output::Builder {
        crate::output::delete_application_reference_data_source_output::Builder::default()
    }
}

/// <p></p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteApplicationOutputOutput {}
impl std::fmt::Debug for DeleteApplicationOutputOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteApplicationOutputOutput");
        formatter.finish()
    }
}
/// See [`DeleteApplicationOutputOutput`](crate::output::DeleteApplicationOutputOutput)
pub mod delete_application_output_output {
    /// A builder for [`DeleteApplicationOutputOutput`](crate::output::DeleteApplicationOutputOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteApplicationOutputOutput`](crate::output::DeleteApplicationOutputOutput)
        pub fn build(self) -> crate::output::DeleteApplicationOutputOutput {
            crate::output::DeleteApplicationOutputOutput {}
        }
    }
}
impl DeleteApplicationOutputOutput {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationOutputOutput`](crate::output::DeleteApplicationOutputOutput)
    pub fn builder() -> crate::output::delete_application_output_output::Builder {
        crate::output::delete_application_output_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteApplicationInputProcessingConfigurationOutput {}
impl std::fmt::Debug for DeleteApplicationInputProcessingConfigurationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteApplicationInputProcessingConfigurationOutput");
        formatter.finish()
    }
}
/// See [`DeleteApplicationInputProcessingConfigurationOutput`](crate::output::DeleteApplicationInputProcessingConfigurationOutput)
pub mod delete_application_input_processing_configuration_output {
    /// A builder for [`DeleteApplicationInputProcessingConfigurationOutput`](crate::output::DeleteApplicationInputProcessingConfigurationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteApplicationInputProcessingConfigurationOutput`](crate::output::DeleteApplicationInputProcessingConfigurationOutput)
        pub fn build(self) -> crate::output::DeleteApplicationInputProcessingConfigurationOutput {
            crate::output::DeleteApplicationInputProcessingConfigurationOutput {}
        }
    }
}
impl DeleteApplicationInputProcessingConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationInputProcessingConfigurationOutput`](crate::output::DeleteApplicationInputProcessingConfigurationOutput)
    pub fn builder(
    ) -> crate::output::delete_application_input_processing_configuration_output::Builder {
        crate::output::delete_application_input_processing_configuration_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteApplicationCloudWatchLoggingOptionOutput {}
impl std::fmt::Debug for DeleteApplicationCloudWatchLoggingOptionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteApplicationCloudWatchLoggingOptionOutput");
        formatter.finish()
    }
}
/// See [`DeleteApplicationCloudWatchLoggingOptionOutput`](crate::output::DeleteApplicationCloudWatchLoggingOptionOutput)
pub mod delete_application_cloud_watch_logging_option_output {
    /// A builder for [`DeleteApplicationCloudWatchLoggingOptionOutput`](crate::output::DeleteApplicationCloudWatchLoggingOptionOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteApplicationCloudWatchLoggingOptionOutput`](crate::output::DeleteApplicationCloudWatchLoggingOptionOutput)
        pub fn build(self) -> crate::output::DeleteApplicationCloudWatchLoggingOptionOutput {
            crate::output::DeleteApplicationCloudWatchLoggingOptionOutput {}
        }
    }
}
impl DeleteApplicationCloudWatchLoggingOptionOutput {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationCloudWatchLoggingOptionOutput`](crate::output::DeleteApplicationCloudWatchLoggingOptionOutput)
    pub fn builder() -> crate::output::delete_application_cloud_watch_logging_option_output::Builder
    {
        crate::output::delete_application_cloud_watch_logging_option_output::Builder::default()
    }
}

/// <p></p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteApplicationOutput {}
impl std::fmt::Debug for DeleteApplicationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteApplicationOutput");
        formatter.finish()
    }
}
/// See [`DeleteApplicationOutput`](crate::output::DeleteApplicationOutput)
pub mod delete_application_output {
    /// A builder for [`DeleteApplicationOutput`](crate::output::DeleteApplicationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteApplicationOutput`](crate::output::DeleteApplicationOutput)
        pub fn build(self) -> crate::output::DeleteApplicationOutput {
            crate::output::DeleteApplicationOutput {}
        }
    }
}
impl DeleteApplicationOutput {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationOutput`](crate::output::DeleteApplicationOutput)
    pub fn builder() -> crate::output::delete_application_output::Builder {
        crate::output::delete_application_output::Builder::default()
    }
}

/// <p>TBD</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateApplicationOutput {
    /// <p>In response to your <code>CreateApplication</code> request, Amazon Kinesis Analytics returns a
    /// response with a summary of the application it created, including the application Amazon Resource Name (ARN),
    /// name, and status.</p>
    pub application_summary: std::option::Option<crate::model::ApplicationSummary>,
}
impl std::fmt::Debug for CreateApplicationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateApplicationOutput");
        formatter.field("application_summary", &self.application_summary);
        formatter.finish()
    }
}
/// See [`CreateApplicationOutput`](crate::output::CreateApplicationOutput)
pub mod create_application_output {
    /// A builder for [`CreateApplicationOutput`](crate::output::CreateApplicationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) application_summary: std::option::Option<crate::model::ApplicationSummary>,
    }
    impl Builder {
        /// <p>In response to your <code>CreateApplication</code> request, Amazon Kinesis Analytics returns a
        /// response with a summary of the application it created, including the application Amazon Resource Name (ARN),
        /// name, and status.</p>
        pub fn application_summary(mut self, input: crate::model::ApplicationSummary) -> Self {
            self.application_summary = Some(input);
            self
        }
        pub fn set_application_summary(
            mut self,
            input: std::option::Option<crate::model::ApplicationSummary>,
        ) -> Self {
            self.application_summary = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateApplicationOutput`](crate::output::CreateApplicationOutput)
        pub fn build(self) -> crate::output::CreateApplicationOutput {
            crate::output::CreateApplicationOutput {
                application_summary: self.application_summary,
            }
        }
    }
}
impl CreateApplicationOutput {
    /// Creates a new builder-style object to manufacture [`CreateApplicationOutput`](crate::output::CreateApplicationOutput)
    pub fn builder() -> crate::output::create_application_output::Builder {
        crate::output::create_application_output::Builder::default()
    }
}

/// <p></p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AddApplicationReferenceDataSourceOutput {}
impl std::fmt::Debug for AddApplicationReferenceDataSourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AddApplicationReferenceDataSourceOutput");
        formatter.finish()
    }
}
/// See [`AddApplicationReferenceDataSourceOutput`](crate::output::AddApplicationReferenceDataSourceOutput)
pub mod add_application_reference_data_source_output {
    /// A builder for [`AddApplicationReferenceDataSourceOutput`](crate::output::AddApplicationReferenceDataSourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`AddApplicationReferenceDataSourceOutput`](crate::output::AddApplicationReferenceDataSourceOutput)
        pub fn build(self) -> crate::output::AddApplicationReferenceDataSourceOutput {
            crate::output::AddApplicationReferenceDataSourceOutput {}
        }
    }
}
impl AddApplicationReferenceDataSourceOutput {
    /// Creates a new builder-style object to manufacture [`AddApplicationReferenceDataSourceOutput`](crate::output::AddApplicationReferenceDataSourceOutput)
    pub fn builder() -> crate::output::add_application_reference_data_source_output::Builder {
        crate::output::add_application_reference_data_source_output::Builder::default()
    }
}

/// <p></p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AddApplicationOutputOutput {}
impl std::fmt::Debug for AddApplicationOutputOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AddApplicationOutputOutput");
        formatter.finish()
    }
}
/// See [`AddApplicationOutputOutput`](crate::output::AddApplicationOutputOutput)
pub mod add_application_output_output {
    /// A builder for [`AddApplicationOutputOutput`](crate::output::AddApplicationOutputOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`AddApplicationOutputOutput`](crate::output::AddApplicationOutputOutput)
        pub fn build(self) -> crate::output::AddApplicationOutputOutput {
            crate::output::AddApplicationOutputOutput {}
        }
    }
}
impl AddApplicationOutputOutput {
    /// Creates a new builder-style object to manufacture [`AddApplicationOutputOutput`](crate::output::AddApplicationOutputOutput)
    pub fn builder() -> crate::output::add_application_output_output::Builder {
        crate::output::add_application_output_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AddApplicationInputProcessingConfigurationOutput {}
impl std::fmt::Debug for AddApplicationInputProcessingConfigurationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AddApplicationInputProcessingConfigurationOutput");
        formatter.finish()
    }
}
/// See [`AddApplicationInputProcessingConfigurationOutput`](crate::output::AddApplicationInputProcessingConfigurationOutput)
pub mod add_application_input_processing_configuration_output {
    /// A builder for [`AddApplicationInputProcessingConfigurationOutput`](crate::output::AddApplicationInputProcessingConfigurationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`AddApplicationInputProcessingConfigurationOutput`](crate::output::AddApplicationInputProcessingConfigurationOutput)
        pub fn build(self) -> crate::output::AddApplicationInputProcessingConfigurationOutput {
            crate::output::AddApplicationInputProcessingConfigurationOutput {}
        }
    }
}
impl AddApplicationInputProcessingConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`AddApplicationInputProcessingConfigurationOutput`](crate::output::AddApplicationInputProcessingConfigurationOutput)
    pub fn builder() -> crate::output::add_application_input_processing_configuration_output::Builder
    {
        crate::output::add_application_input_processing_configuration_output::Builder::default()
    }
}

/// <p></p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AddApplicationInputOutput {}
impl std::fmt::Debug for AddApplicationInputOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AddApplicationInputOutput");
        formatter.finish()
    }
}
/// See [`AddApplicationInputOutput`](crate::output::AddApplicationInputOutput)
pub mod add_application_input_output {
    /// A builder for [`AddApplicationInputOutput`](crate::output::AddApplicationInputOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`AddApplicationInputOutput`](crate::output::AddApplicationInputOutput)
        pub fn build(self) -> crate::output::AddApplicationInputOutput {
            crate::output::AddApplicationInputOutput {}
        }
    }
}
impl AddApplicationInputOutput {
    /// Creates a new builder-style object to manufacture [`AddApplicationInputOutput`](crate::output::AddApplicationInputOutput)
    pub fn builder() -> crate::output::add_application_input_output::Builder {
        crate::output::add_application_input_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AddApplicationCloudWatchLoggingOptionOutput {}
impl std::fmt::Debug for AddApplicationCloudWatchLoggingOptionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AddApplicationCloudWatchLoggingOptionOutput");
        formatter.finish()
    }
}
/// See [`AddApplicationCloudWatchLoggingOptionOutput`](crate::output::AddApplicationCloudWatchLoggingOptionOutput)
pub mod add_application_cloud_watch_logging_option_output {
    /// A builder for [`AddApplicationCloudWatchLoggingOptionOutput`](crate::output::AddApplicationCloudWatchLoggingOptionOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`AddApplicationCloudWatchLoggingOptionOutput`](crate::output::AddApplicationCloudWatchLoggingOptionOutput)
        pub fn build(self) -> crate::output::AddApplicationCloudWatchLoggingOptionOutput {
            crate::output::AddApplicationCloudWatchLoggingOptionOutput {}
        }
    }
}
impl AddApplicationCloudWatchLoggingOptionOutput {
    /// Creates a new builder-style object to manufacture [`AddApplicationCloudWatchLoggingOptionOutput`](crate::output::AddApplicationCloudWatchLoggingOptionOutput)
    pub fn builder() -> crate::output::add_application_cloud_watch_logging_option_output::Builder {
        crate::output::add_application_cloud_watch_logging_option_output::Builder::default()
    }
}
