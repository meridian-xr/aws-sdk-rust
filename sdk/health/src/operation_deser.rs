// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_affected_accounts_for_organization_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeAffectedAccountsForOrganizationOutput,
    crate::error::DescribeAffectedAccountsForOrganizationError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeAffectedAccountsForOrganizationError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::error::DescribeAffectedAccountsForOrganizationError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidPaginationToken" => crate::error::DescribeAffectedAccountsForOrganizationError { meta: generic, kind: crate::error::DescribeAffectedAccountsForOrganizationErrorKind::InvalidPaginationToken({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::invalid_pagination_token::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_pagination_tokenjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeAffectedAccountsForOrganizationError::unhandled)?;
                    output.build()
                }
            ;
            if (&tmp.message).is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::DescribeAffectedAccountsForOrganizationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_affected_accounts_for_organization_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeAffectedAccountsForOrganizationOutput,
    crate::error::DescribeAffectedAccountsForOrganizationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::output::describe_affected_accounts_for_organization_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_affected_accounts_for_organization(response.body().as_ref(), output).map_err(crate::error::DescribeAffectedAccountsForOrganizationError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_affected_entities_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeAffectedEntitiesOutput,
    crate::error::DescribeAffectedEntitiesError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeAffectedEntitiesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::DescribeAffectedEntitiesError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidPaginationToken" => crate::error::DescribeAffectedEntitiesError {
            meta: generic,
            kind: crate::error::DescribeAffectedEntitiesErrorKind::InvalidPaginationToken({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_pagination_token::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_pagination_tokenjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeAffectedEntitiesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "UnsupportedLocale" => {
            crate::error::DescribeAffectedEntitiesError {
                meta: generic,
                kind: crate::error::DescribeAffectedEntitiesErrorKind::UnsupportedLocale({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output = crate::error::unsupported_locale::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_crate_error_unsupported_localejson_err(response.body().as_ref(), output).map_err(crate::error::DescribeAffectedEntitiesError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        _ => crate::error::DescribeAffectedEntitiesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_affected_entities_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeAffectedEntitiesOutput,
    crate::error::DescribeAffectedEntitiesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_affected_entities_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_affected_entities(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeAffectedEntitiesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_affected_entities_for_organization_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeAffectedEntitiesForOrganizationOutput,
    crate::error::DescribeAffectedEntitiesForOrganizationError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeAffectedEntitiesForOrganizationError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::error::DescribeAffectedEntitiesForOrganizationError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidPaginationToken" => crate::error::DescribeAffectedEntitiesForOrganizationError { meta: generic, kind: crate::error::DescribeAffectedEntitiesForOrganizationErrorKind::InvalidPaginationToken({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::invalid_pagination_token::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_pagination_tokenjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeAffectedEntitiesForOrganizationError::unhandled)?;
                    output.build()
                }
            ;
            if (&tmp.message).is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "UnsupportedLocale" => crate::error::DescribeAffectedEntitiesForOrganizationError { meta: generic, kind: crate::error::DescribeAffectedEntitiesForOrganizationErrorKind::UnsupportedLocale({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::unsupported_locale::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_unsupported_localejson_err(response.body().as_ref(), output).map_err(crate::error::DescribeAffectedEntitiesForOrganizationError::unhandled)?;
                    output.build()
                }
            ;
            if (&tmp.message).is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::DescribeAffectedEntitiesForOrganizationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_affected_entities_for_organization_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeAffectedEntitiesForOrganizationOutput,
    crate::error::DescribeAffectedEntitiesForOrganizationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::output::describe_affected_entities_for_organization_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_affected_entities_for_organization(response.body().as_ref(), output).map_err(crate::error::DescribeAffectedEntitiesForOrganizationError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_entity_aggregates_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeEntityAggregatesOutput,
    crate::error::DescribeEntityAggregatesError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeEntityAggregatesError::unhandled)?;
    Err(crate::error::DescribeEntityAggregatesError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_entity_aggregates_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeEntityAggregatesOutput,
    crate::error::DescribeEntityAggregatesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_entity_aggregates_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_entity_aggregates(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeEntityAggregatesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_event_aggregates_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeEventAggregatesOutput,
    crate::error::DescribeEventAggregatesError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeEventAggregatesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::DescribeEventAggregatesError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidPaginationToken" => crate::error::DescribeEventAggregatesError {
            meta: generic,
            kind: crate::error::DescribeEventAggregatesErrorKind::InvalidPaginationToken({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_pagination_token::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_pagination_tokenjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeEventAggregatesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeEventAggregatesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_event_aggregates_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeEventAggregatesOutput,
    crate::error::DescribeEventAggregatesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_event_aggregates_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_event_aggregates(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeEventAggregatesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_event_details_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeEventDetailsOutput,
    crate::error::DescribeEventDetailsError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeEventDetailsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DescribeEventDetailsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "UnsupportedLocale" => {
            crate::error::DescribeEventDetailsError {
                meta: generic,
                kind: crate::error::DescribeEventDetailsErrorKind::UnsupportedLocale({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output = crate::error::unsupported_locale::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_crate_error_unsupported_localejson_err(response.body().as_ref(), output).map_err(crate::error::DescribeEventDetailsError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        _ => crate::error::DescribeEventDetailsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_event_details_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeEventDetailsOutput,
    crate::error::DescribeEventDetailsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_event_details_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_event_details(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeEventDetailsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_event_details_for_organization_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeEventDetailsForOrganizationOutput,
    crate::error::DescribeEventDetailsForOrganizationError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeEventDetailsForOrganizationError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::DescribeEventDetailsForOrganizationError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "UnsupportedLocale" => {
            crate::error::DescribeEventDetailsForOrganizationError {
                meta: generic,
                kind: crate::error::DescribeEventDetailsForOrganizationErrorKind::UnsupportedLocale(
                    {
                        #[allow(unused_mut)]
                        let mut tmp = {
                            #[allow(unused_mut)]
                            let mut output = crate::error::unsupported_locale::Builder::default();
                            let _ = response;
                            output = crate::json_deser::deser_structure_crate_error_unsupported_localejson_err(response.body().as_ref(), output).map_err(crate::error::DescribeEventDetailsForOrganizationError::unhandled)?;
                            output.build()
                        };
                        if (&tmp.message).is_none() {
                            tmp.message = _error_message;
                        }
                        tmp
                    },
                ),
            }
        }
        _ => crate::error::DescribeEventDetailsForOrganizationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_event_details_for_organization_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeEventDetailsForOrganizationOutput,
    crate::error::DescribeEventDetailsForOrganizationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::output::describe_event_details_for_organization_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_event_details_for_organization(response.body().as_ref(), output).map_err(crate::error::DescribeEventDetailsForOrganizationError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_events_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeEventsOutput, crate::error::DescribeEventsError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeEventsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DescribeEventsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidPaginationToken" => crate::error::DescribeEventsError {
            meta: generic,
            kind: crate::error::DescribeEventsErrorKind::InvalidPaginationToken({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_pagination_token::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_pagination_tokenjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeEventsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "UnsupportedLocale" => {
            crate::error::DescribeEventsError {
                meta: generic,
                kind: crate::error::DescribeEventsErrorKind::UnsupportedLocale({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output = crate::error::unsupported_locale::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_crate_error_unsupported_localejson_err(response.body().as_ref(), output).map_err(crate::error::DescribeEventsError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        _ => crate::error::DescribeEventsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_events_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeEventsOutput, crate::error::DescribeEventsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_events_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_events(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeEventsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_events_for_organization_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeEventsForOrganizationOutput,
    crate::error::DescribeEventsForOrganizationError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeEventsForOrganizationError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::DescribeEventsForOrganizationError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidPaginationToken" => crate::error::DescribeEventsForOrganizationError {
            meta: generic,
            kind: crate::error::DescribeEventsForOrganizationErrorKind::InvalidPaginationToken({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_pagination_token::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_pagination_tokenjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeEventsForOrganizationError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "UnsupportedLocale" => {
            crate::error::DescribeEventsForOrganizationError {
                meta: generic,
                kind: crate::error::DescribeEventsForOrganizationErrorKind::UnsupportedLocale({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output = crate::error::unsupported_locale::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_crate_error_unsupported_localejson_err(response.body().as_ref(), output).map_err(crate::error::DescribeEventsForOrganizationError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        _ => crate::error::DescribeEventsForOrganizationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_events_for_organization_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeEventsForOrganizationOutput,
    crate::error::DescribeEventsForOrganizationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_events_for_organization_output::Builder::default();
        let _ = response;
        output =
            crate::json_deser::deser_operation_crate_operation_describe_events_for_organization(
                response.body().as_ref(),
                output,
            )
            .map_err(crate::error::DescribeEventsForOrganizationError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_event_types_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeEventTypesOutput,
    crate::error::DescribeEventTypesError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeEventTypesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DescribeEventTypesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidPaginationToken" => crate::error::DescribeEventTypesError {
            meta: generic,
            kind: crate::error::DescribeEventTypesErrorKind::InvalidPaginationToken({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_pagination_token::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_pagination_tokenjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeEventTypesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "UnsupportedLocale" => {
            crate::error::DescribeEventTypesError {
                meta: generic,
                kind: crate::error::DescribeEventTypesErrorKind::UnsupportedLocale({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output = crate::error::unsupported_locale::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_crate_error_unsupported_localejson_err(response.body().as_ref(), output).map_err(crate::error::DescribeEventTypesError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        _ => crate::error::DescribeEventTypesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_event_types_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeEventTypesOutput,
    crate::error::DescribeEventTypesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_event_types_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_event_types(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeEventTypesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_health_service_status_for_organization_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeHealthServiceStatusForOrganizationOutput,
    crate::error::DescribeHealthServiceStatusForOrganizationError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeHealthServiceStatusForOrganizationError::unhandled)?;
    Err(crate::error::DescribeHealthServiceStatusForOrganizationError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_health_service_status_for_organization_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeHealthServiceStatusForOrganizationOutput,
    crate::error::DescribeHealthServiceStatusForOrganizationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::output::describe_health_service_status_for_organization_output::Builder::default(
            );
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_health_service_status_for_organization(response.body().as_ref(), output).map_err(crate::error::DescribeHealthServiceStatusForOrganizationError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_disable_health_service_access_for_organization_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DisableHealthServiceAccessForOrganizationOutput,
    crate::error::DisableHealthServiceAccessForOrganizationError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DisableHealthServiceAccessForOrganizationError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::error::DisableHealthServiceAccessForOrganizationError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModificationException" => crate::error::DisableHealthServiceAccessForOrganizationError { meta: generic, kind: crate::error::DisableHealthServiceAccessForOrganizationErrorKind::ConcurrentModificationException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::concurrent_modification_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_concurrent_modification_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DisableHealthServiceAccessForOrganizationError::unhandled)?;
                    output.build()
                }
            ;
            if (&tmp.message).is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::DisableHealthServiceAccessForOrganizationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_disable_health_service_access_for_organization_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DisableHealthServiceAccessForOrganizationOutput,
    crate::error::DisableHealthServiceAccessForOrganizationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::output::disable_health_service_access_for_organization_output::Builder::default(
            );
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_enable_health_service_access_for_organization_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::EnableHealthServiceAccessForOrganizationOutput,
    crate::error::EnableHealthServiceAccessForOrganizationError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::EnableHealthServiceAccessForOrganizationError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::error::EnableHealthServiceAccessForOrganizationError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModificationException" => crate::error::EnableHealthServiceAccessForOrganizationError { meta: generic, kind: crate::error::EnableHealthServiceAccessForOrganizationErrorKind::ConcurrentModificationException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::concurrent_modification_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_concurrent_modification_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::EnableHealthServiceAccessForOrganizationError::unhandled)?;
                    output.build()
                }
            ;
            if (&tmp.message).is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::EnableHealthServiceAccessForOrganizationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_enable_health_service_access_for_organization_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::EnableHealthServiceAccessForOrganizationOutput,
    crate::error::EnableHealthServiceAccessForOrganizationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::output::enable_health_service_access_for_organization_output::Builder::default();
        let _ = response;
        output.build()
    })
}
