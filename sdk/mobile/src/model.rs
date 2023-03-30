// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Detailed information about an AWS Mobile Hub project. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ProjectDetails  {
    /// <p> Name of the project. </p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p> Unique project identifier. </p>
    #[doc(hidden)]
    pub project_id: std::option::Option<std::string::String>,
    /// <p> Default region to use for AWS resource creation in the AWS Mobile Hub project. </p>
    #[doc(hidden)]
    pub region: std::option::Option<std::string::String>,
    /// <p> Synchronization state for a project. </p>
    #[doc(hidden)]
    pub state: std::option::Option<crate::model::ProjectState>,
    /// <p> Date the project was created. </p>
    #[doc(hidden)]
    pub created_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p> Date of the last modification of the project. </p>
    #[doc(hidden)]
    pub last_updated_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p> Website URL for this project in the AWS Mobile Hub console. </p>
    #[doc(hidden)]
    pub console_url: std::option::Option<std::string::String>,
    /// <p> List of AWS resources associated with a project. </p>
    #[doc(hidden)]
    pub resources: std::option::Option<std::vec::Vec<crate::model::Resource>>,
}
impl ProjectDetails {
    /// <p> Name of the project. </p>
    pub fn name(&self) -> std::option::Option<& str> {
        self.name.as_deref()
    }
    /// <p> Unique project identifier. </p>
    pub fn project_id(&self) -> std::option::Option<& str> {
        self.project_id.as_deref()
    }
    /// <p> Default region to use for AWS resource creation in the AWS Mobile Hub project. </p>
    pub fn region(&self) -> std::option::Option<& str> {
        self.region.as_deref()
    }
    /// <p> Synchronization state for a project. </p>
    pub fn state(&self) -> std::option::Option<& crate::model::ProjectState> {
        self.state.as_ref()
    }
    /// <p> Date the project was created. </p>
    pub fn created_date(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.created_date.as_ref()
    }
    /// <p> Date of the last modification of the project. </p>
    pub fn last_updated_date(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.last_updated_date.as_ref()
    }
    /// <p> Website URL for this project in the AWS Mobile Hub console. </p>
    pub fn console_url(&self) -> std::option::Option<& str> {
        self.console_url.as_deref()
    }
    /// <p> List of AWS resources associated with a project. </p>
    pub fn resources(&self) -> std::option::Option<& [crate::model::Resource]> {
        self.resources.as_deref()
    }
}
impl ProjectDetails {
    /// Creates a new builder-style object to manufacture [`ProjectDetails`](crate::model::ProjectDetails).
    pub fn builder() -> crate::model::project_details::Builder {
        crate::model::project_details::Builder::default()
    }
}

/// See [`ProjectDetails`](crate::model::ProjectDetails).
pub mod project_details {
    
    /// A builder for [`ProjectDetails`](crate::model::ProjectDetails).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) project_id: std::option::Option<std::string::String>,
        pub(crate) region: std::option::Option<std::string::String>,
        pub(crate) state: std::option::Option<crate::model::ProjectState>,
        pub(crate) created_date: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) last_updated_date: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) console_url: std::option::Option<std::string::String>,
        pub(crate) resources: std::option::Option<std::vec::Vec<crate::model::Resource>>,
    }
    impl Builder {
        /// <p> Name of the project. </p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p> Name of the project. </p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input; self
        }
        /// <p> Unique project identifier. </p>
        pub fn project_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.project_id = Some(input.into());
            self
        }
        /// <p> Unique project identifier. </p>
        pub fn set_project_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.project_id = input; self
        }
        /// <p> Default region to use for AWS resource creation in the AWS Mobile Hub project. </p>
        pub fn region(mut self, input: impl Into<std::string::String>) -> Self {
            self.region = Some(input.into());
            self
        }
        /// <p> Default region to use for AWS resource creation in the AWS Mobile Hub project. </p>
        pub fn set_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.region = input; self
        }
        /// <p> Synchronization state for a project. </p>
        pub fn state(mut self, input: crate::model::ProjectState) -> Self {
            self.state = Some(input);
            self
        }
        /// <p> Synchronization state for a project. </p>
        pub fn set_state(mut self, input: std::option::Option<crate::model::ProjectState>) -> Self {
            self.state = input; self
        }
        /// <p> Date the project was created. </p>
        pub fn created_date(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.created_date = Some(input);
            self
        }
        /// <p> Date the project was created. </p>
        pub fn set_created_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.created_date = input; self
        }
        /// <p> Date of the last modification of the project. </p>
        pub fn last_updated_date(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.last_updated_date = Some(input);
            self
        }
        /// <p> Date of the last modification of the project. </p>
        pub fn set_last_updated_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.last_updated_date = input; self
        }
        /// <p> Website URL for this project in the AWS Mobile Hub console. </p>
        pub fn console_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.console_url = Some(input.into());
            self
        }
        /// <p> Website URL for this project in the AWS Mobile Hub console. </p>
        pub fn set_console_url(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.console_url = input; self
        }
        /// Appends an item to `resources`.
        ///
        /// To override the contents of this collection use [`set_resources`](Self::set_resources).
        ///
        /// <p> List of AWS resources associated with a project. </p>
        pub fn resources(mut self, input: crate::model::Resource) -> Self {
            let mut v = self.resources.unwrap_or_default();
                            v.push(input);
                            self.resources = Some(v);
                            self
        }
        /// <p> List of AWS resources associated with a project. </p>
        pub fn set_resources(mut self, input: std::option::Option<std::vec::Vec<crate::model::Resource>>) -> Self {
            self.resources = input; self
        }
        /// Consumes the builder and constructs a [`ProjectDetails`](crate::model::ProjectDetails).
        pub fn build(self) -> crate::model::ProjectDetails {
            crate::model::ProjectDetails {
                name: self.name
                ,
                project_id: self.project_id
                ,
                region: self.region
                ,
                state: self.state
                ,
                created_date: self.created_date
                ,
                last_updated_date: self.last_updated_date
                ,
                console_url: self.console_url
                ,
                resources: self.resources
                ,
            }
        }
    }
    
    
}

/// <p> Information about an instance of an AWS resource associated with a project. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Resource  {
    /// <p> Simplified name for type of AWS resource (e.g., bucket is an Amazon S3 bucket). </p>
    #[doc(hidden)]
    pub r#type: std::option::Option<std::string::String>,
    /// <p> Name of the AWS resource (e.g., for an Amazon S3 bucket this is the name of the bucket). </p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p> AWS resource name which uniquely identifies the resource in AWS systems. </p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
    /// <p> Identifies which feature in AWS Mobile Hub is associated with this AWS resource. </p>
    #[doc(hidden)]
    pub feature: std::option::Option<std::string::String>,
    /// <p> Key-value attribute pairs. </p>
    #[doc(hidden)]
    pub attributes: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl Resource {
    /// <p> Simplified name for type of AWS resource (e.g., bucket is an Amazon S3 bucket). </p>
    pub fn r#type(&self) -> std::option::Option<& str> {
        self.r#type.as_deref()
    }
    /// <p> Name of the AWS resource (e.g., for an Amazon S3 bucket this is the name of the bucket). </p>
    pub fn name(&self) -> std::option::Option<& str> {
        self.name.as_deref()
    }
    /// <p> AWS resource name which uniquely identifies the resource in AWS systems. </p>
    pub fn arn(&self) -> std::option::Option<& str> {
        self.arn.as_deref()
    }
    /// <p> Identifies which feature in AWS Mobile Hub is associated with this AWS resource. </p>
    pub fn feature(&self) -> std::option::Option<& str> {
        self.feature.as_deref()
    }
    /// <p> Key-value attribute pairs. </p>
    pub fn attributes(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, std::string::String>> {
        self.attributes.as_ref()
    }
}
impl Resource {
    /// Creates a new builder-style object to manufacture [`Resource`](crate::model::Resource).
    pub fn builder() -> crate::model::resource::Builder {
        crate::model::resource::Builder::default()
    }
}

/// See [`Resource`](crate::model::Resource).
pub mod resource {
    
    /// A builder for [`Resource`](crate::model::Resource).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) r#type: std::option::Option<std::string::String>,
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) arn: std::option::Option<std::string::String>,
        pub(crate) feature: std::option::Option<std::string::String>,
        pub(crate) attributes: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    }
    impl Builder {
        /// <p> Simplified name for type of AWS resource (e.g., bucket is an Amazon S3 bucket). </p>
        pub fn r#type(mut self, input: impl Into<std::string::String>) -> Self {
            self.r#type = Some(input.into());
            self
        }
        /// <p> Simplified name for type of AWS resource (e.g., bucket is an Amazon S3 bucket). </p>
        pub fn set_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.r#type = input; self
        }
        /// <p> Name of the AWS resource (e.g., for an Amazon S3 bucket this is the name of the bucket). </p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p> Name of the AWS resource (e.g., for an Amazon S3 bucket this is the name of the bucket). </p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input; self
        }
        /// <p> AWS resource name which uniquely identifies the resource in AWS systems. </p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        /// <p> AWS resource name which uniquely identifies the resource in AWS systems. </p>
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input; self
        }
        /// <p> Identifies which feature in AWS Mobile Hub is associated with this AWS resource. </p>
        pub fn feature(mut self, input: impl Into<std::string::String>) -> Self {
            self.feature = Some(input.into());
            self
        }
        /// <p> Identifies which feature in AWS Mobile Hub is associated with this AWS resource. </p>
        pub fn set_feature(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.feature = input; self
        }
        /// Adds a key-value pair to `attributes`.
        ///
        /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
        ///
        /// <p> Key-value attribute pairs. </p>
        pub fn attributes(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            let mut hash_map = self.attributes.unwrap_or_default();
                            hash_map.insert(k.into(), v.into());
                            self.attributes = Some(hash_map);
                            self
        }
        /// <p> Key-value attribute pairs. </p>
        pub fn set_attributes(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.attributes = input; self
        }
        /// Consumes the builder and constructs a [`Resource`](crate::model::Resource).
        pub fn build(self) -> crate::model::Resource {
            crate::model::Resource {
                r#type: self.r#type
                ,
                name: self.name
                ,
                arn: self.arn
                ,
                feature: self.feature
                ,
                attributes: self.attributes
                ,
            }
        }
    }
    
    
}

/// When writing a match expression against `ProjectState`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let projectstate = unimplemented!();
/// match projectstate {
///     ProjectState::Importing => { /* ... */ },
///     ProjectState::Normal => { /* ... */ },
///     ProjectState::Syncing => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `projectstate` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ProjectState::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ProjectState::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ProjectState::NewFeature` is defined.
/// Specifically, when `projectstate` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ProjectState::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
/// <p>
/// Synchronization state for a project.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum ProjectState {
    #[allow(missing_docs)] // documentation missing in model
    Importing,
    #[allow(missing_docs)] // documentation missing in model
    Normal,
    #[allow(missing_docs)] // documentation missing in model
    Syncing,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for ProjectState {
                fn from(s: &str) -> Self {
                    match s {
                        "IMPORTING" => ProjectState::Importing,
"NORMAL" => ProjectState::Normal,
"SYNCING" => ProjectState::Syncing,
other => ProjectState::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
                    }
                }
            }
impl std::str::FromStr for ProjectState {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(ProjectState::from(s))
                }
            }
impl ProjectState {
                /// Returns the `&str` value of the enum member.
                pub fn as_str(&self) -> &str {
                    match self {
    ProjectState::Importing => "IMPORTING",
    ProjectState::Normal => "NORMAL",
    ProjectState::Syncing => "SYNCING",
    ProjectState::Unknown(value) => value.as_str()
}
                }
                /// Returns all the `&str` representations of the enum members.
                pub const fn values() -> &'static [&'static str] {
                    &["IMPORTING", "NORMAL", "SYNCING"]
                }
            }
impl AsRef<str> for ProjectState {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }

/// <p> Summary information about an AWS Mobile Hub project. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ProjectSummary  {
    /// <p> Name of the project. </p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p> Unique project identifier. </p>
    #[doc(hidden)]
    pub project_id: std::option::Option<std::string::String>,
}
impl ProjectSummary {
    /// <p> Name of the project. </p>
    pub fn name(&self) -> std::option::Option<& str> {
        self.name.as_deref()
    }
    /// <p> Unique project identifier. </p>
    pub fn project_id(&self) -> std::option::Option<& str> {
        self.project_id.as_deref()
    }
}
impl ProjectSummary {
    /// Creates a new builder-style object to manufacture [`ProjectSummary`](crate::model::ProjectSummary).
    pub fn builder() -> crate::model::project_summary::Builder {
        crate::model::project_summary::Builder::default()
    }
}

/// See [`ProjectSummary`](crate::model::ProjectSummary).
pub mod project_summary {
    
    /// A builder for [`ProjectSummary`](crate::model::ProjectSummary).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) project_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p> Name of the project. </p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p> Name of the project. </p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input; self
        }
        /// <p> Unique project identifier. </p>
        pub fn project_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.project_id = Some(input.into());
            self
        }
        /// <p> Unique project identifier. </p>
        pub fn set_project_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.project_id = input; self
        }
        /// Consumes the builder and constructs a [`ProjectSummary`](crate::model::ProjectSummary).
        pub fn build(self) -> crate::model::ProjectSummary {
            crate::model::ProjectSummary {
                name: self.name
                ,
                project_id: self.project_id
                ,
            }
        }
    }
    
    
}

/// <p> The details of the bundle. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BundleDetails  {
    /// <p> Unique bundle identifier. </p>
    #[doc(hidden)]
    pub bundle_id: std::option::Option<std::string::String>,
    /// <p> Title of the download bundle. </p>
    #[doc(hidden)]
    pub title: std::option::Option<std::string::String>,
    /// <p> Version of the download bundle. </p>
    #[doc(hidden)]
    pub version: std::option::Option<std::string::String>,
    /// <p> Description of the download bundle. </p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p> Icon for the download bundle. </p>
    #[doc(hidden)]
    pub icon_url: std::option::Option<std::string::String>,
    /// <p> Developer desktop or mobile app or website platforms. </p>
    #[doc(hidden)]
    pub available_platforms: std::option::Option<std::vec::Vec<crate::model::Platform>>,
}
impl BundleDetails {
    /// <p> Unique bundle identifier. </p>
    pub fn bundle_id(&self) -> std::option::Option<& str> {
        self.bundle_id.as_deref()
    }
    /// <p> Title of the download bundle. </p>
    pub fn title(&self) -> std::option::Option<& str> {
        self.title.as_deref()
    }
    /// <p> Version of the download bundle. </p>
    pub fn version(&self) -> std::option::Option<& str> {
        self.version.as_deref()
    }
    /// <p> Description of the download bundle. </p>
    pub fn description(&self) -> std::option::Option<& str> {
        self.description.as_deref()
    }
    /// <p> Icon for the download bundle. </p>
    pub fn icon_url(&self) -> std::option::Option<& str> {
        self.icon_url.as_deref()
    }
    /// <p> Developer desktop or mobile app or website platforms. </p>
    pub fn available_platforms(&self) -> std::option::Option<& [crate::model::Platform]> {
        self.available_platforms.as_deref()
    }
}
impl BundleDetails {
    /// Creates a new builder-style object to manufacture [`BundleDetails`](crate::model::BundleDetails).
    pub fn builder() -> crate::model::bundle_details::Builder {
        crate::model::bundle_details::Builder::default()
    }
}

/// See [`BundleDetails`](crate::model::BundleDetails).
pub mod bundle_details {
    
    /// A builder for [`BundleDetails`](crate::model::BundleDetails).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) bundle_id: std::option::Option<std::string::String>,
        pub(crate) title: std::option::Option<std::string::String>,
        pub(crate) version: std::option::Option<std::string::String>,
        pub(crate) description: std::option::Option<std::string::String>,
        pub(crate) icon_url: std::option::Option<std::string::String>,
        pub(crate) available_platforms: std::option::Option<std::vec::Vec<crate::model::Platform>>,
    }
    impl Builder {
        /// <p> Unique bundle identifier. </p>
        pub fn bundle_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.bundle_id = Some(input.into());
            self
        }
        /// <p> Unique bundle identifier. </p>
        pub fn set_bundle_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.bundle_id = input; self
        }
        /// <p> Title of the download bundle. </p>
        pub fn title(mut self, input: impl Into<std::string::String>) -> Self {
            self.title = Some(input.into());
            self
        }
        /// <p> Title of the download bundle. </p>
        pub fn set_title(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.title = input; self
        }
        /// <p> Version of the download bundle. </p>
        pub fn version(mut self, input: impl Into<std::string::String>) -> Self {
            self.version = Some(input.into());
            self
        }
        /// <p> Version of the download bundle. </p>
        pub fn set_version(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.version = input; self
        }
        /// <p> Description of the download bundle. </p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.description = Some(input.into());
            self
        }
        /// <p> Description of the download bundle. </p>
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.description = input; self
        }
        /// <p> Icon for the download bundle. </p>
        pub fn icon_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.icon_url = Some(input.into());
            self
        }
        /// <p> Icon for the download bundle. </p>
        pub fn set_icon_url(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.icon_url = input; self
        }
        /// Appends an item to `available_platforms`.
        ///
        /// To override the contents of this collection use [`set_available_platforms`](Self::set_available_platforms).
        ///
        /// <p> Developer desktop or mobile app or website platforms. </p>
        pub fn available_platforms(mut self, input: crate::model::Platform) -> Self {
            let mut v = self.available_platforms.unwrap_or_default();
                            v.push(input);
                            self.available_platforms = Some(v);
                            self
        }
        /// <p> Developer desktop or mobile app or website platforms. </p>
        pub fn set_available_platforms(mut self, input: std::option::Option<std::vec::Vec<crate::model::Platform>>) -> Self {
            self.available_platforms = input; self
        }
        /// Consumes the builder and constructs a [`BundleDetails`](crate::model::BundleDetails).
        pub fn build(self) -> crate::model::BundleDetails {
            crate::model::BundleDetails {
                bundle_id: self.bundle_id
                ,
                title: self.title
                ,
                version: self.version
                ,
                description: self.description
                ,
                icon_url: self.icon_url
                ,
                available_platforms: self.available_platforms
                ,
            }
        }
    }
    
    
}

/// When writing a match expression against `Platform`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let platform = unimplemented!();
/// match platform {
///     Platform::Android => { /* ... */ },
///     Platform::Javascript => { /* ... */ },
///     Platform::Linux => { /* ... */ },
///     Platform::Objc => { /* ... */ },
///     Platform::Osx => { /* ... */ },
///     Platform::Swift => { /* ... */ },
///     Platform::Windows => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `platform` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `Platform::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `Platform::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `Platform::NewFeature` is defined.
/// Specifically, when `platform` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `Platform::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
/// <p>
/// Developer desktop or target mobile app or website platform.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum Platform {
    #[allow(missing_docs)] // documentation missing in model
    Android,
    #[allow(missing_docs)] // documentation missing in model
    Javascript,
    #[allow(missing_docs)] // documentation missing in model
    Linux,
    #[allow(missing_docs)] // documentation missing in model
    Objc,
    #[allow(missing_docs)] // documentation missing in model
    Osx,
    #[allow(missing_docs)] // documentation missing in model
    Swift,
    #[allow(missing_docs)] // documentation missing in model
    Windows,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for Platform {
                fn from(s: &str) -> Self {
                    match s {
                        "ANDROID" => Platform::Android,
"JAVASCRIPT" => Platform::Javascript,
"LINUX" => Platform::Linux,
"OBJC" => Platform::Objc,
"OSX" => Platform::Osx,
"SWIFT" => Platform::Swift,
"WINDOWS" => Platform::Windows,
other => Platform::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
                    }
                }
            }
impl std::str::FromStr for Platform {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(Platform::from(s))
                }
            }
impl Platform {
                /// Returns the `&str` value of the enum member.
                pub fn as_str(&self) -> &str {
                    match self {
    Platform::Android => "ANDROID",
    Platform::Javascript => "JAVASCRIPT",
    Platform::Linux => "LINUX",
    Platform::Objc => "OBJC",
    Platform::Osx => "OSX",
    Platform::Swift => "SWIFT",
    Platform::Windows => "WINDOWS",
    Platform::Unknown(value) => value.as_str()
}
                }
                /// Returns all the `&str` representations of the enum members.
                pub const fn values() -> &'static [&'static str] {
                    &["ANDROID", "JAVASCRIPT", "LINUX", "OBJC", "OSX", "SWIFT", "WINDOWS"]
                }
            }
impl AsRef<str> for Platform {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }

