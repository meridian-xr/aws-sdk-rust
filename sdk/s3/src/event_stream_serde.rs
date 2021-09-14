// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(Debug)]
pub struct SelectObjectContentEventStreamUnmarshaller;

impl SelectObjectContentEventStreamUnmarshaller {
    pub fn new() -> Self {
        SelectObjectContentEventStreamUnmarshaller
    }
}
impl smithy_eventstream::frame::UnmarshallMessage for SelectObjectContentEventStreamUnmarshaller {
    type Output = crate::model::SelectObjectContentEventStream;
    type Error = crate::error::SelectObjectContentError;
    fn unmarshall(
        &self,
        message: &smithy_eventstream::frame::Message,
    ) -> std::result::Result<
        smithy_eventstream::frame::UnmarshalledMessage<Self::Output, Self::Error>,
        smithy_eventstream::error::Error,
    > {
        let response_headers = smithy_eventstream::smithy::parse_response_headers(&message)?;
        match response_headers.message_type.as_str() {
            "event" => match response_headers.smithy_type.as_str() {
                "Records" => {
                    let mut builder = crate::model::RecordsEvent::builder();
                    if response_headers.content_type.as_str() != "application/octet-stream" {
                        return Err(smithy_eventstream::error::Error::Unmarshalling(format!(
                            "expected :content-type to be 'application/octet-stream', but was '{}'",
                            response_headers.content_type.as_str()
                        )));
                    }
                    builder = builder.payload(smithy_types::Blob::new(message.payload().as_ref()));
                    Ok(smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::model::SelectObjectContentEventStream::Records(builder.build()),
                    ))
                }
                "Stats" => {
                    let mut builder = crate::model::StatsEvent::builder();
                    builder = builder.details(
                        crate::xml_deser::deser_member_com_amazonaws_s3_stats_event_details(
                            &message.payload()[..],
                        )
                        .map_err(|err| {
                            smithy_eventstream::error::Error::Unmarshalling(format!(
                                "failed to unmarshall Details: {}",
                                err
                            ))
                        })?,
                    );
                    Ok(smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::model::SelectObjectContentEventStream::Stats(builder.build()),
                    ))
                }
                "Progress" => {
                    let mut builder = crate::model::ProgressEvent::builder();
                    builder = builder.details(
                        crate::xml_deser::deser_member_com_amazonaws_s3_progress_event_details(
                            &message.payload()[..],
                        )
                        .map_err(|err| {
                            smithy_eventstream::error::Error::Unmarshalling(format!(
                                "failed to unmarshall Details: {}",
                                err
                            ))
                        })?,
                    );
                    Ok(smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::model::SelectObjectContentEventStream::Progress(builder.build()),
                    ))
                }
                "Cont" => {
                    let parsed =
                            crate::xml_deser::deser_member_com_amazonaws_s3_select_object_content_event_stream_cont(&message.payload()[..])
                                            .map_err(|err| {
                                                smithy_eventstream::error::Error::Unmarshalling(format!("failed to unmarshall Cont: {}", err))
                                            })?
                        ;
                    Ok(smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::model::SelectObjectContentEventStream::Cont(parsed),
                    ))
                }
                "End" => {
                    let parsed =
                            crate::xml_deser::deser_member_com_amazonaws_s3_select_object_content_event_stream_end(&message.payload()[..])
                                            .map_err(|err| {
                                                smithy_eventstream::error::Error::Unmarshalling(format!("failed to unmarshall End: {}", err))
                                            })?
                        ;
                    Ok(smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::model::SelectObjectContentEventStream::End(parsed),
                    ))
                }
                smithy_type => {
                    return Err(smithy_eventstream::error::Error::Unmarshalling(format!(
                        "unrecognized :event-type: {}",
                        smithy_type
                    )));
                }
            },
            "exception" => {
                let generic =
                    match crate::xml_deser::parse_event_stream_generic_error(message.payload()) {
                        Ok(generic) => generic,
                        Err(err) => {
                            return Ok(smithy_eventstream::frame::UnmarshalledMessage::Error(
                                crate::error::SelectObjectContentError::unhandled(err),
                            ))
                        }
                    };
                Ok(smithy_eventstream::frame::UnmarshalledMessage::Error(
                    crate::error::SelectObjectContentError::generic(generic),
                ))
            }
            value => {
                return Err(smithy_eventstream::error::Error::Unmarshalling(format!(
                    "unrecognized :message-type: {}",
                    value
                )));
            }
        }
    }
}
