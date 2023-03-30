// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UntagResourceOutput  {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for UntagResourceOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput).
pub mod untag_resource_output {
    
    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput).
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
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput).
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct TagResourceOutput  {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for TagResourceOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput).
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

/// See [`TagResourceOutput`](crate::output::TagResourceOutput).
pub mod tag_resource_output {
    
    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput).
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
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput).
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListTagsForResourceOutput  {
    /// <p>The tags associated with the specified media pipeline.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    _request_id: Option<String>,
}
impl ListTagsForResourceOutput {
    /// <p>The tags associated with the specified media pipeline.</p>
    pub fn tags(&self) -> std::option::Option<& [crate::model::Tag]> {
        self.tags.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListTagsForResourceOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
pub mod list_tags_for_resource_output {
    
    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// Appends an item to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The tags associated with the specified media pipeline.</p>
        pub fn tags(mut self, input: crate::model::Tag) -> Self {
            let mut v = self.tags.unwrap_or_default();
                            v.push(input);
                            self.tags = Some(v);
                            self
        }
        /// <p>The tags associated with the specified media pipeline.</p>
        pub fn set_tags(mut self, input: std::option::Option<std::vec::Vec<crate::model::Tag>>) -> Self {
            self.tags = input; self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput {
                tags: self.tags
                ,
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListMediaPipelinesOutput  {
    /// <p>The media pipeline objects in the list.</p>
    #[doc(hidden)]
    pub media_pipelines: std::option::Option<std::vec::Vec<crate::model::MediaPipelineSummary>>,
    /// <p>The token used to retrieve the next page of results. </p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListMediaPipelinesOutput {
    /// <p>The media pipeline objects in the list.</p>
    pub fn media_pipelines(&self) -> std::option::Option<& [crate::model::MediaPipelineSummary]> {
        self.media_pipelines.as_deref()
    }
    /// <p>The token used to retrieve the next page of results. </p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListMediaPipelinesOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl ListMediaPipelinesOutput {
    /// Creates a new builder-style object to manufacture [`ListMediaPipelinesOutput`](crate::output::ListMediaPipelinesOutput).
    pub fn builder() -> crate::output::list_media_pipelines_output::Builder {
        crate::output::list_media_pipelines_output::Builder::default()
    }
}

/// See [`ListMediaPipelinesOutput`](crate::output::ListMediaPipelinesOutput).
pub mod list_media_pipelines_output {
    
    /// A builder for [`ListMediaPipelinesOutput`](crate::output::ListMediaPipelinesOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) media_pipelines: std::option::Option<std::vec::Vec<crate::model::MediaPipelineSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// Appends an item to `media_pipelines`.
        ///
        /// To override the contents of this collection use [`set_media_pipelines`](Self::set_media_pipelines).
        ///
        /// <p>The media pipeline objects in the list.</p>
        pub fn media_pipelines(mut self, input: crate::model::MediaPipelineSummary) -> Self {
            let mut v = self.media_pipelines.unwrap_or_default();
                            v.push(input);
                            self.media_pipelines = Some(v);
                            self
        }
        /// <p>The media pipeline objects in the list.</p>
        pub fn set_media_pipelines(mut self, input: std::option::Option<std::vec::Vec<crate::model::MediaPipelineSummary>>) -> Self {
            self.media_pipelines = input; self
        }
        /// <p>The token used to retrieve the next page of results. </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token used to retrieve the next page of results. </p>
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
        /// Consumes the builder and constructs a [`ListMediaPipelinesOutput`](crate::output::ListMediaPipelinesOutput).
        pub fn build(self) -> crate::output::ListMediaPipelinesOutput {
            crate::output::ListMediaPipelinesOutput {
                media_pipelines: self.media_pipelines
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
pub struct ListMediaCapturePipelinesOutput  {
    /// <p>The media pipeline objects in the list.</p>
    #[doc(hidden)]
    pub media_capture_pipelines: std::option::Option<std::vec::Vec<crate::model::MediaCapturePipelineSummary>>,
    /// <p>The token used to retrieve the next page of results. </p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListMediaCapturePipelinesOutput {
    /// <p>The media pipeline objects in the list.</p>
    pub fn media_capture_pipelines(&self) -> std::option::Option<& [crate::model::MediaCapturePipelineSummary]> {
        self.media_capture_pipelines.as_deref()
    }
    /// <p>The token used to retrieve the next page of results. </p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListMediaCapturePipelinesOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl ListMediaCapturePipelinesOutput {
    /// Creates a new builder-style object to manufacture [`ListMediaCapturePipelinesOutput`](crate::output::ListMediaCapturePipelinesOutput).
    pub fn builder() -> crate::output::list_media_capture_pipelines_output::Builder {
        crate::output::list_media_capture_pipelines_output::Builder::default()
    }
}

/// See [`ListMediaCapturePipelinesOutput`](crate::output::ListMediaCapturePipelinesOutput).
pub mod list_media_capture_pipelines_output {
    
    /// A builder for [`ListMediaCapturePipelinesOutput`](crate::output::ListMediaCapturePipelinesOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) media_capture_pipelines: std::option::Option<std::vec::Vec<crate::model::MediaCapturePipelineSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// Appends an item to `media_capture_pipelines`.
        ///
        /// To override the contents of this collection use [`set_media_capture_pipelines`](Self::set_media_capture_pipelines).
        ///
        /// <p>The media pipeline objects in the list.</p>
        pub fn media_capture_pipelines(mut self, input: crate::model::MediaCapturePipelineSummary) -> Self {
            let mut v = self.media_capture_pipelines.unwrap_or_default();
                            v.push(input);
                            self.media_capture_pipelines = Some(v);
                            self
        }
        /// <p>The media pipeline objects in the list.</p>
        pub fn set_media_capture_pipelines(mut self, input: std::option::Option<std::vec::Vec<crate::model::MediaCapturePipelineSummary>>) -> Self {
            self.media_capture_pipelines = input; self
        }
        /// <p>The token used to retrieve the next page of results. </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token used to retrieve the next page of results. </p>
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
        /// Consumes the builder and constructs a [`ListMediaCapturePipelinesOutput`](crate::output::ListMediaCapturePipelinesOutput).
        pub fn build(self) -> crate::output::ListMediaCapturePipelinesOutput {
            crate::output::ListMediaCapturePipelinesOutput {
                media_capture_pipelines: self.media_capture_pipelines
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
pub struct GetMediaPipelineOutput  {
    /// <p>The media pipeline object.</p>
    #[doc(hidden)]
    pub media_pipeline: std::option::Option<crate::model::MediaPipeline>,
    _request_id: Option<String>,
}
impl GetMediaPipelineOutput {
    /// <p>The media pipeline object.</p>
    pub fn media_pipeline(&self) -> std::option::Option<& crate::model::MediaPipeline> {
        self.media_pipeline.as_ref()
    }
}
impl aws_http::request_id::RequestId for GetMediaPipelineOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl GetMediaPipelineOutput {
    /// Creates a new builder-style object to manufacture [`GetMediaPipelineOutput`](crate::output::GetMediaPipelineOutput).
    pub fn builder() -> crate::output::get_media_pipeline_output::Builder {
        crate::output::get_media_pipeline_output::Builder::default()
    }
}

/// See [`GetMediaPipelineOutput`](crate::output::GetMediaPipelineOutput).
pub mod get_media_pipeline_output {
    
    /// A builder for [`GetMediaPipelineOutput`](crate::output::GetMediaPipelineOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) media_pipeline: std::option::Option<crate::model::MediaPipeline>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// <p>The media pipeline object.</p>
        pub fn media_pipeline(mut self, input: crate::model::MediaPipeline) -> Self {
            self.media_pipeline = Some(input);
            self
        }
        /// <p>The media pipeline object.</p>
        pub fn set_media_pipeline(mut self, input: std::option::Option<crate::model::MediaPipeline>) -> Self {
            self.media_pipeline = input; self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`GetMediaPipelineOutput`](crate::output::GetMediaPipelineOutput).
        pub fn build(self) -> crate::output::GetMediaPipelineOutput {
            crate::output::GetMediaPipelineOutput {
                media_pipeline: self.media_pipeline
                ,
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetMediaCapturePipelineOutput  {
    /// <p>The media pipeline object.</p>
    #[doc(hidden)]
    pub media_capture_pipeline: std::option::Option<crate::model::MediaCapturePipeline>,
    _request_id: Option<String>,
}
impl GetMediaCapturePipelineOutput {
    /// <p>The media pipeline object.</p>
    pub fn media_capture_pipeline(&self) -> std::option::Option<& crate::model::MediaCapturePipeline> {
        self.media_capture_pipeline.as_ref()
    }
}
impl aws_http::request_id::RequestId for GetMediaCapturePipelineOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl GetMediaCapturePipelineOutput {
    /// Creates a new builder-style object to manufacture [`GetMediaCapturePipelineOutput`](crate::output::GetMediaCapturePipelineOutput).
    pub fn builder() -> crate::output::get_media_capture_pipeline_output::Builder {
        crate::output::get_media_capture_pipeline_output::Builder::default()
    }
}

/// See [`GetMediaCapturePipelineOutput`](crate::output::GetMediaCapturePipelineOutput).
pub mod get_media_capture_pipeline_output {
    
    /// A builder for [`GetMediaCapturePipelineOutput`](crate::output::GetMediaCapturePipelineOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) media_capture_pipeline: std::option::Option<crate::model::MediaCapturePipeline>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// <p>The media pipeline object.</p>
        pub fn media_capture_pipeline(mut self, input: crate::model::MediaCapturePipeline) -> Self {
            self.media_capture_pipeline = Some(input);
            self
        }
        /// <p>The media pipeline object.</p>
        pub fn set_media_capture_pipeline(mut self, input: std::option::Option<crate::model::MediaCapturePipeline>) -> Self {
            self.media_capture_pipeline = input; self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`GetMediaCapturePipelineOutput`](crate::output::GetMediaCapturePipelineOutput).
        pub fn build(self) -> crate::output::GetMediaCapturePipelineOutput {
            crate::output::GetMediaCapturePipelineOutput {
                media_capture_pipeline: self.media_capture_pipeline
                ,
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteMediaPipelineOutput  {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for DeleteMediaPipelineOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl DeleteMediaPipelineOutput {
    /// Creates a new builder-style object to manufacture [`DeleteMediaPipelineOutput`](crate::output::DeleteMediaPipelineOutput).
    pub fn builder() -> crate::output::delete_media_pipeline_output::Builder {
        crate::output::delete_media_pipeline_output::Builder::default()
    }
}

/// See [`DeleteMediaPipelineOutput`](crate::output::DeleteMediaPipelineOutput).
pub mod delete_media_pipeline_output {
    
    /// A builder for [`DeleteMediaPipelineOutput`](crate::output::DeleteMediaPipelineOutput).
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
        /// Consumes the builder and constructs a [`DeleteMediaPipelineOutput`](crate::output::DeleteMediaPipelineOutput).
        pub fn build(self) -> crate::output::DeleteMediaPipelineOutput {
            crate::output::DeleteMediaPipelineOutput {
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteMediaCapturePipelineOutput  {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for DeleteMediaCapturePipelineOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl DeleteMediaCapturePipelineOutput {
    /// Creates a new builder-style object to manufacture [`DeleteMediaCapturePipelineOutput`](crate::output::DeleteMediaCapturePipelineOutput).
    pub fn builder() -> crate::output::delete_media_capture_pipeline_output::Builder {
        crate::output::delete_media_capture_pipeline_output::Builder::default()
    }
}

/// See [`DeleteMediaCapturePipelineOutput`](crate::output::DeleteMediaCapturePipelineOutput).
pub mod delete_media_capture_pipeline_output {
    
    /// A builder for [`DeleteMediaCapturePipelineOutput`](crate::output::DeleteMediaCapturePipelineOutput).
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
        /// Consumes the builder and constructs a [`DeleteMediaCapturePipelineOutput`](crate::output::DeleteMediaCapturePipelineOutput).
        pub fn build(self) -> crate::output::DeleteMediaCapturePipelineOutput {
            crate::output::DeleteMediaCapturePipelineOutput {
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateMediaLiveConnectorPipelineOutput  {
    /// <p>The new media pipeline.</p>
    #[doc(hidden)]
    pub media_live_connector_pipeline: std::option::Option<crate::model::MediaLiveConnectorPipeline>,
    _request_id: Option<String>,
}
impl CreateMediaLiveConnectorPipelineOutput {
    /// <p>The new media pipeline.</p>
    pub fn media_live_connector_pipeline(&self) -> std::option::Option<& crate::model::MediaLiveConnectorPipeline> {
        self.media_live_connector_pipeline.as_ref()
    }
}
impl aws_http::request_id::RequestId for CreateMediaLiveConnectorPipelineOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl CreateMediaLiveConnectorPipelineOutput {
    /// Creates a new builder-style object to manufacture [`CreateMediaLiveConnectorPipelineOutput`](crate::output::CreateMediaLiveConnectorPipelineOutput).
    pub fn builder() -> crate::output::create_media_live_connector_pipeline_output::Builder {
        crate::output::create_media_live_connector_pipeline_output::Builder::default()
    }
}

/// See [`CreateMediaLiveConnectorPipelineOutput`](crate::output::CreateMediaLiveConnectorPipelineOutput).
pub mod create_media_live_connector_pipeline_output {
    
    /// A builder for [`CreateMediaLiveConnectorPipelineOutput`](crate::output::CreateMediaLiveConnectorPipelineOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) media_live_connector_pipeline: std::option::Option<crate::model::MediaLiveConnectorPipeline>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// <p>The new media pipeline.</p>
        pub fn media_live_connector_pipeline(mut self, input: crate::model::MediaLiveConnectorPipeline) -> Self {
            self.media_live_connector_pipeline = Some(input);
            self
        }
        /// <p>The new media pipeline.</p>
        pub fn set_media_live_connector_pipeline(mut self, input: std::option::Option<crate::model::MediaLiveConnectorPipeline>) -> Self {
            self.media_live_connector_pipeline = input; self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`CreateMediaLiveConnectorPipelineOutput`](crate::output::CreateMediaLiveConnectorPipelineOutput).
        pub fn build(self) -> crate::output::CreateMediaLiveConnectorPipelineOutput {
            crate::output::CreateMediaLiveConnectorPipelineOutput {
                media_live_connector_pipeline: self.media_live_connector_pipeline
                ,
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateMediaConcatenationPipelineOutput  {
    /// <p>A media concatenation pipeline object, the ID, source type, <code>MediaPipelineARN</code>, and sink of a media concatenation pipeline object.</p>
    #[doc(hidden)]
    pub media_concatenation_pipeline: std::option::Option<crate::model::MediaConcatenationPipeline>,
    _request_id: Option<String>,
}
impl CreateMediaConcatenationPipelineOutput {
    /// <p>A media concatenation pipeline object, the ID, source type, <code>MediaPipelineARN</code>, and sink of a media concatenation pipeline object.</p>
    pub fn media_concatenation_pipeline(&self) -> std::option::Option<& crate::model::MediaConcatenationPipeline> {
        self.media_concatenation_pipeline.as_ref()
    }
}
impl aws_http::request_id::RequestId for CreateMediaConcatenationPipelineOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl CreateMediaConcatenationPipelineOutput {
    /// Creates a new builder-style object to manufacture [`CreateMediaConcatenationPipelineOutput`](crate::output::CreateMediaConcatenationPipelineOutput).
    pub fn builder() -> crate::output::create_media_concatenation_pipeline_output::Builder {
        crate::output::create_media_concatenation_pipeline_output::Builder::default()
    }
}

/// See [`CreateMediaConcatenationPipelineOutput`](crate::output::CreateMediaConcatenationPipelineOutput).
pub mod create_media_concatenation_pipeline_output {
    
    /// A builder for [`CreateMediaConcatenationPipelineOutput`](crate::output::CreateMediaConcatenationPipelineOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) media_concatenation_pipeline: std::option::Option<crate::model::MediaConcatenationPipeline>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// <p>A media concatenation pipeline object, the ID, source type, <code>MediaPipelineARN</code>, and sink of a media concatenation pipeline object.</p>
        pub fn media_concatenation_pipeline(mut self, input: crate::model::MediaConcatenationPipeline) -> Self {
            self.media_concatenation_pipeline = Some(input);
            self
        }
        /// <p>A media concatenation pipeline object, the ID, source type, <code>MediaPipelineARN</code>, and sink of a media concatenation pipeline object.</p>
        pub fn set_media_concatenation_pipeline(mut self, input: std::option::Option<crate::model::MediaConcatenationPipeline>) -> Self {
            self.media_concatenation_pipeline = input; self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`CreateMediaConcatenationPipelineOutput`](crate::output::CreateMediaConcatenationPipelineOutput).
        pub fn build(self) -> crate::output::CreateMediaConcatenationPipelineOutput {
            crate::output::CreateMediaConcatenationPipelineOutput {
                media_concatenation_pipeline: self.media_concatenation_pipeline
                ,
                _request_id: self._request_id,
            }
        }
    }
    
    
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateMediaCapturePipelineOutput  {
    /// <p>A media pipeline object, the ID, source type, source ARN, sink type, and sink ARN of a media pipeline object.</p>
    #[doc(hidden)]
    pub media_capture_pipeline: std::option::Option<crate::model::MediaCapturePipeline>,
    _request_id: Option<String>,
}
impl CreateMediaCapturePipelineOutput {
    /// <p>A media pipeline object, the ID, source type, source ARN, sink type, and sink ARN of a media pipeline object.</p>
    pub fn media_capture_pipeline(&self) -> std::option::Option<& crate::model::MediaCapturePipeline> {
        self.media_capture_pipeline.as_ref()
    }
}
impl aws_http::request_id::RequestId for CreateMediaCapturePipelineOutput {
                                fn request_id(&self) -> Option<&str> {
                                    self._request_id.as_deref()
                                }
                            }
impl CreateMediaCapturePipelineOutput {
    /// Creates a new builder-style object to manufacture [`CreateMediaCapturePipelineOutput`](crate::output::CreateMediaCapturePipelineOutput).
    pub fn builder() -> crate::output::create_media_capture_pipeline_output::Builder {
        crate::output::create_media_capture_pipeline_output::Builder::default()
    }
}

/// See [`CreateMediaCapturePipelineOutput`](crate::output::CreateMediaCapturePipelineOutput).
pub mod create_media_capture_pipeline_output {
    
    /// A builder for [`CreateMediaCapturePipelineOutput`](crate::output::CreateMediaCapturePipelineOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) media_capture_pipeline: std::option::Option<crate::model::MediaCapturePipeline>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// <p>A media pipeline object, the ID, source type, source ARN, sink type, and sink ARN of a media pipeline object.</p>
        pub fn media_capture_pipeline(mut self, input: crate::model::MediaCapturePipeline) -> Self {
            self.media_capture_pipeline = Some(input);
            self
        }
        /// <p>A media pipeline object, the ID, source type, source ARN, sink type, and sink ARN of a media pipeline object.</p>
        pub fn set_media_capture_pipeline(mut self, input: std::option::Option<crate::model::MediaCapturePipeline>) -> Self {
            self.media_capture_pipeline = input; self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
                                        self._request_id = Some(request_id.into());
                                        self
                                    }
        
                                    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
                                        self._request_id = request_id;
                                        self
                                    }
        /// Consumes the builder and constructs a [`CreateMediaCapturePipelineOutput`](crate::output::CreateMediaCapturePipelineOutput).
        pub fn build(self) -> crate::output::CreateMediaCapturePipelineOutput {
            crate::output::CreateMediaCapturePipelineOutput {
                media_capture_pipeline: self.media_capture_pipeline
                ,
                _request_id: self._request_id,
            }
        }
    }
    
    
}

