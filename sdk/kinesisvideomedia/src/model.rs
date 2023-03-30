// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Identifies the chunk on the Kinesis video stream where you want the <code>GetMedia</code> API to start returning media data. You have the following options to identify the starting chunk: </p> 
/// <ul> 
/// <li> <p>Choose the latest (or oldest) chunk.</p> </li> 
/// <li> <p>Identify a specific chunk. You can identify a specific chunk either by providing a fragment number or timestamp (server or producer). </p> </li> 
/// <li> <p>Each chunk's metadata includes a continuation token as a Matroska (MKV) tag (<code>AWS_KINESISVIDEO_CONTINUATION_TOKEN</code>). If your previous <code>GetMedia</code> request terminated, you can use this tag value in your next <code>GetMedia</code> request. The API then starts returning chunks starting where the last API ended.</p> </li> 
/// </ul>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct StartSelector  {
    /// <p>Identifies the fragment on the Kinesis video stream where you want to start getting the data from.</p> 
    /// <ul> 
    /// <li> <p>NOW - Start with the latest chunk on the stream.</p> </li> 
    /// <li> <p>EARLIEST - Start with earliest available chunk on the stream.</p> </li> 
    /// <li> <p>FRAGMENT_NUMBER - Start with the chunk after a specific fragment. You must also specify the <code>AfterFragmentNumber</code> parameter.</p> </li> 
    /// <li> <p>PRODUCER_TIMESTAMP or SERVER_TIMESTAMP - Start with the chunk containing a fragment with the specified producer or server timestamp. You specify the timestamp by adding <code>StartTimestamp</code>.</p> </li> 
    /// <li> <p> CONTINUATION_TOKEN - Read using the specified continuation token. </p> </li> 
    /// </ul> <note> 
    /// <p>If you choose the NOW, EARLIEST, or CONTINUATION_TOKEN as the <code>startSelectorType</code>, you don't provide any additional information in the <code>startSelector</code>.</p> 
    /// </note>
    #[doc(hidden)]
    pub start_selector_type: std::option::Option<crate::model::StartSelectorType>,
    /// <p>Specifies the fragment number from where you want the <code>GetMedia</code> API to start returning the fragments. </p>
    #[doc(hidden)]
    pub after_fragment_number: std::option::Option<std::string::String>,
    /// <p>A timestamp value. This value is required if you choose the PRODUCER_TIMESTAMP or the SERVER_TIMESTAMP as the <code>startSelectorType</code>. The <code>GetMedia</code> API then starts with the chunk containing the fragment that has the specified timestamp.</p>
    #[doc(hidden)]
    pub start_timestamp: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Continuation token that Kinesis Video Streams returned in the previous <code>GetMedia</code> response. The <code>GetMedia</code> API then starts with the chunk identified by the continuation token.</p>
    #[doc(hidden)]
    pub continuation_token: std::option::Option<std::string::String>,
}
impl StartSelector {
    /// <p>Identifies the fragment on the Kinesis video stream where you want to start getting the data from.</p> 
    /// <ul> 
    /// <li> <p>NOW - Start with the latest chunk on the stream.</p> </li> 
    /// <li> <p>EARLIEST - Start with earliest available chunk on the stream.</p> </li> 
    /// <li> <p>FRAGMENT_NUMBER - Start with the chunk after a specific fragment. You must also specify the <code>AfterFragmentNumber</code> parameter.</p> </li> 
    /// <li> <p>PRODUCER_TIMESTAMP or SERVER_TIMESTAMP - Start with the chunk containing a fragment with the specified producer or server timestamp. You specify the timestamp by adding <code>StartTimestamp</code>.</p> </li> 
    /// <li> <p> CONTINUATION_TOKEN - Read using the specified continuation token. </p> </li> 
    /// </ul> <note> 
    /// <p>If you choose the NOW, EARLIEST, or CONTINUATION_TOKEN as the <code>startSelectorType</code>, you don't provide any additional information in the <code>startSelector</code>.</p> 
    /// </note>
    pub fn start_selector_type(&self) -> std::option::Option<& crate::model::StartSelectorType> {
        self.start_selector_type.as_ref()
    }
    /// <p>Specifies the fragment number from where you want the <code>GetMedia</code> API to start returning the fragments. </p>
    pub fn after_fragment_number(&self) -> std::option::Option<& str> {
        self.after_fragment_number.as_deref()
    }
    /// <p>A timestamp value. This value is required if you choose the PRODUCER_TIMESTAMP or the SERVER_TIMESTAMP as the <code>startSelectorType</code>. The <code>GetMedia</code> API then starts with the chunk containing the fragment that has the specified timestamp.</p>
    pub fn start_timestamp(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.start_timestamp.as_ref()
    }
    /// <p>Continuation token that Kinesis Video Streams returned in the previous <code>GetMedia</code> response. The <code>GetMedia</code> API then starts with the chunk identified by the continuation token.</p>
    pub fn continuation_token(&self) -> std::option::Option<& str> {
        self.continuation_token.as_deref()
    }
}
impl StartSelector {
    /// Creates a new builder-style object to manufacture [`StartSelector`](crate::model::StartSelector).
    pub fn builder() -> crate::model::start_selector::Builder {
        crate::model::start_selector::Builder::default()
    }
}

/// See [`StartSelector`](crate::model::StartSelector).
pub mod start_selector {
    
    /// A builder for [`StartSelector`](crate::model::StartSelector).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) start_selector_type: std::option::Option<crate::model::StartSelectorType>,
        pub(crate) after_fragment_number: std::option::Option<std::string::String>,
        pub(crate) start_timestamp: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) continuation_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Identifies the fragment on the Kinesis video stream where you want to start getting the data from.</p> 
        /// <ul> 
        /// <li> <p>NOW - Start with the latest chunk on the stream.</p> </li> 
        /// <li> <p>EARLIEST - Start with earliest available chunk on the stream.</p> </li> 
        /// <li> <p>FRAGMENT_NUMBER - Start with the chunk after a specific fragment. You must also specify the <code>AfterFragmentNumber</code> parameter.</p> </li> 
        /// <li> <p>PRODUCER_TIMESTAMP or SERVER_TIMESTAMP - Start with the chunk containing a fragment with the specified producer or server timestamp. You specify the timestamp by adding <code>StartTimestamp</code>.</p> </li> 
        /// <li> <p> CONTINUATION_TOKEN - Read using the specified continuation token. </p> </li> 
        /// </ul> <note> 
        /// <p>If you choose the NOW, EARLIEST, or CONTINUATION_TOKEN as the <code>startSelectorType</code>, you don't provide any additional information in the <code>startSelector</code>.</p> 
        /// </note>
        pub fn start_selector_type(mut self, input: crate::model::StartSelectorType) -> Self {
            self.start_selector_type = Some(input);
            self
        }
        /// <p>Identifies the fragment on the Kinesis video stream where you want to start getting the data from.</p> 
        /// <ul> 
        /// <li> <p>NOW - Start with the latest chunk on the stream.</p> </li> 
        /// <li> <p>EARLIEST - Start with earliest available chunk on the stream.</p> </li> 
        /// <li> <p>FRAGMENT_NUMBER - Start with the chunk after a specific fragment. You must also specify the <code>AfterFragmentNumber</code> parameter.</p> </li> 
        /// <li> <p>PRODUCER_TIMESTAMP or SERVER_TIMESTAMP - Start with the chunk containing a fragment with the specified producer or server timestamp. You specify the timestamp by adding <code>StartTimestamp</code>.</p> </li> 
        /// <li> <p> CONTINUATION_TOKEN - Read using the specified continuation token. </p> </li> 
        /// </ul> <note> 
        /// <p>If you choose the NOW, EARLIEST, or CONTINUATION_TOKEN as the <code>startSelectorType</code>, you don't provide any additional information in the <code>startSelector</code>.</p> 
        /// </note>
        pub fn set_start_selector_type(mut self, input: std::option::Option<crate::model::StartSelectorType>) -> Self {
            self.start_selector_type = input; self
        }
        /// <p>Specifies the fragment number from where you want the <code>GetMedia</code> API to start returning the fragments. </p>
        pub fn after_fragment_number(mut self, input: impl Into<std::string::String>) -> Self {
            self.after_fragment_number = Some(input.into());
            self
        }
        /// <p>Specifies the fragment number from where you want the <code>GetMedia</code> API to start returning the fragments. </p>
        pub fn set_after_fragment_number(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.after_fragment_number = input; self
        }
        /// <p>A timestamp value. This value is required if you choose the PRODUCER_TIMESTAMP or the SERVER_TIMESTAMP as the <code>startSelectorType</code>. The <code>GetMedia</code> API then starts with the chunk containing the fragment that has the specified timestamp.</p>
        pub fn start_timestamp(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.start_timestamp = Some(input);
            self
        }
        /// <p>A timestamp value. This value is required if you choose the PRODUCER_TIMESTAMP or the SERVER_TIMESTAMP as the <code>startSelectorType</code>. The <code>GetMedia</code> API then starts with the chunk containing the fragment that has the specified timestamp.</p>
        pub fn set_start_timestamp(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.start_timestamp = input; self
        }
        /// <p>Continuation token that Kinesis Video Streams returned in the previous <code>GetMedia</code> response. The <code>GetMedia</code> API then starts with the chunk identified by the continuation token.</p>
        pub fn continuation_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.continuation_token = Some(input.into());
            self
        }
        /// <p>Continuation token that Kinesis Video Streams returned in the previous <code>GetMedia</code> response. The <code>GetMedia</code> API then starts with the chunk identified by the continuation token.</p>
        pub fn set_continuation_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.continuation_token = input; self
        }
        /// Consumes the builder and constructs a [`StartSelector`](crate::model::StartSelector).
        pub fn build(self) -> crate::model::StartSelector {
            crate::model::StartSelector {
                start_selector_type: self.start_selector_type
                ,
                after_fragment_number: self.after_fragment_number
                ,
                start_timestamp: self.start_timestamp
                ,
                continuation_token: self.continuation_token
                ,
            }
        }
    }
    
    
}

/// When writing a match expression against `StartSelectorType`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let startselectortype = unimplemented!();
/// match startselectortype {
///     StartSelectorType::ContinuationToken => { /* ... */ },
///     StartSelectorType::Earliest => { /* ... */ },
///     StartSelectorType::FragmentNumber => { /* ... */ },
///     StartSelectorType::Now => { /* ... */ },
///     StartSelectorType::ProducerTimestamp => { /* ... */ },
///     StartSelectorType::ServerTimestamp => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `startselectortype` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `StartSelectorType::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `StartSelectorType::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `StartSelectorType::NewFeature` is defined.
/// Specifically, when `startselectortype` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `StartSelectorType::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum StartSelectorType {
    #[allow(missing_docs)] // documentation missing in model
    ContinuationToken,
    #[allow(missing_docs)] // documentation missing in model
    Earliest,
    #[allow(missing_docs)] // documentation missing in model
    FragmentNumber,
    #[allow(missing_docs)] // documentation missing in model
    Now,
    #[allow(missing_docs)] // documentation missing in model
    ProducerTimestamp,
    #[allow(missing_docs)] // documentation missing in model
    ServerTimestamp,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for StartSelectorType {
                fn from(s: &str) -> Self {
                    match s {
                        "CONTINUATION_TOKEN" => StartSelectorType::ContinuationToken,
"EARLIEST" => StartSelectorType::Earliest,
"FRAGMENT_NUMBER" => StartSelectorType::FragmentNumber,
"NOW" => StartSelectorType::Now,
"PRODUCER_TIMESTAMP" => StartSelectorType::ProducerTimestamp,
"SERVER_TIMESTAMP" => StartSelectorType::ServerTimestamp,
other => StartSelectorType::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
                    }
                }
            }
impl std::str::FromStr for StartSelectorType {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(StartSelectorType::from(s))
                }
            }
impl StartSelectorType {
                /// Returns the `&str` value of the enum member.
                pub fn as_str(&self) -> &str {
                    match self {
    StartSelectorType::ContinuationToken => "CONTINUATION_TOKEN",
    StartSelectorType::Earliest => "EARLIEST",
    StartSelectorType::FragmentNumber => "FRAGMENT_NUMBER",
    StartSelectorType::Now => "NOW",
    StartSelectorType::ProducerTimestamp => "PRODUCER_TIMESTAMP",
    StartSelectorType::ServerTimestamp => "SERVER_TIMESTAMP",
    StartSelectorType::Unknown(value) => value.as_str()
}
                }
                /// Returns all the `&str` representations of the enum members.
                pub const fn values() -> &'static [&'static str] {
                    &["CONTINUATION_TOKEN", "EARLIEST", "FRAGMENT_NUMBER", "NOW", "PRODUCER_TIMESTAMP", "SERVER_TIMESTAMP"]
                }
            }
impl AsRef<str> for StartSelectorType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }

