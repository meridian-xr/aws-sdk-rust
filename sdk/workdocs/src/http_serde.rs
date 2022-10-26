// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_abort_document_version_upload(
    input: &crate::input::AbortDocumentVersionUploadInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_1) = &input.authentication_token {
        let formatted_2 = AsRef::<str>::as_ref(inner_1);
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_activate_user(
    input: &crate::input::ActivateUserInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_3) = &input.authentication_token {
        let formatted_4 = AsRef::<str>::as_ref(inner_3);
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_add_resource_permissions(
    input: &crate::input::AddResourcePermissionsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_5) = &input.authentication_token {
        let formatted_6 = AsRef::<str>::as_ref(inner_5);
        if !formatted_6.is_empty() {
            let header_value = formatted_6;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_comment(
    input: &crate::input::CreateCommentInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_7) = &input.authentication_token {
        let formatted_8 = AsRef::<str>::as_ref(inner_7);
        if !formatted_8.is_empty() {
            let header_value = formatted_8;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_custom_metadata(
    input: &crate::input::CreateCustomMetadataInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_9) = &input.authentication_token {
        let formatted_10 = AsRef::<str>::as_ref(inner_9);
        if !formatted_10.is_empty() {
            let header_value = formatted_10;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_folder(
    input: &crate::input::CreateFolderInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_11) = &input.authentication_token {
        let formatted_12 = AsRef::<str>::as_ref(inner_11);
        if !formatted_12.is_empty() {
            let header_value = formatted_12;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_labels(
    input: &crate::input::CreateLabelsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_13) = &input.authentication_token {
        let formatted_14 = AsRef::<str>::as_ref(inner_13);
        if !formatted_14.is_empty() {
            let header_value = formatted_14;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_user(
    input: &crate::input::CreateUserInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_15) = &input.authentication_token {
        let formatted_16 = AsRef::<str>::as_ref(inner_15);
        if !formatted_16.is_empty() {
            let header_value = formatted_16;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_deactivate_user(
    input: &crate::input::DeactivateUserInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_17) = &input.authentication_token {
        let formatted_18 = AsRef::<str>::as_ref(inner_17);
        if !formatted_18.is_empty() {
            let header_value = formatted_18;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_comment(
    input: &crate::input::DeleteCommentInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_19) = &input.authentication_token {
        let formatted_20 = AsRef::<str>::as_ref(inner_19);
        if !formatted_20.is_empty() {
            let header_value = formatted_20;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_custom_metadata(
    input: &crate::input::DeleteCustomMetadataInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_21) = &input.authentication_token {
        let formatted_22 = AsRef::<str>::as_ref(inner_21);
        if !formatted_22.is_empty() {
            let header_value = formatted_22;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_document(
    input: &crate::input::DeleteDocumentInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_23) = &input.authentication_token {
        let formatted_24 = AsRef::<str>::as_ref(inner_23);
        if !formatted_24.is_empty() {
            let header_value = formatted_24;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_folder(
    input: &crate::input::DeleteFolderInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_25) = &input.authentication_token {
        let formatted_26 = AsRef::<str>::as_ref(inner_25);
        if !formatted_26.is_empty() {
            let header_value = formatted_26;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_folder_contents(
    input: &crate::input::DeleteFolderContentsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_27) = &input.authentication_token {
        let formatted_28 = AsRef::<str>::as_ref(inner_27);
        if !formatted_28.is_empty() {
            let header_value = formatted_28;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_labels(
    input: &crate::input::DeleteLabelsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_29) = &input.authentication_token {
        let formatted_30 = AsRef::<str>::as_ref(inner_29);
        if !formatted_30.is_empty() {
            let header_value = formatted_30;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_user(
    input: &crate::input::DeleteUserInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_31) = &input.authentication_token {
        let formatted_32 = AsRef::<str>::as_ref(inner_31);
        if !formatted_32.is_empty() {
            let header_value = formatted_32;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_activities(
    input: &crate::input::DescribeActivitiesInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_33) = &input.authentication_token {
        let formatted_34 = AsRef::<str>::as_ref(inner_33);
        if !formatted_34.is_empty() {
            let header_value = formatted_34;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_comments(
    input: &crate::input::DescribeCommentsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_35) = &input.authentication_token {
        let formatted_36 = AsRef::<str>::as_ref(inner_35);
        if !formatted_36.is_empty() {
            let header_value = formatted_36;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_document_versions(
    input: &crate::input::DescribeDocumentVersionsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_37) = &input.authentication_token {
        let formatted_38 = AsRef::<str>::as_ref(inner_37);
        if !formatted_38.is_empty() {
            let header_value = formatted_38;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_folder_contents(
    input: &crate::input::DescribeFolderContentsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_39) = &input.authentication_token {
        let formatted_40 = AsRef::<str>::as_ref(inner_39);
        if !formatted_40.is_empty() {
            let header_value = formatted_40;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_groups(
    input: &crate::input::DescribeGroupsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_41) = &input.authentication_token {
        let formatted_42 = AsRef::<str>::as_ref(inner_41);
        if !formatted_42.is_empty() {
            let header_value = formatted_42;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_resource_permissions(
    input: &crate::input::DescribeResourcePermissionsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_43) = &input.authentication_token {
        let formatted_44 = AsRef::<str>::as_ref(inner_43);
        if !formatted_44.is_empty() {
            let header_value = formatted_44;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_root_folders(
    input: &crate::input::DescribeRootFoldersInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_45) = &input.authentication_token {
        let formatted_46 = AsRef::<str>::as_ref(inner_45);
        if !formatted_46.is_empty() {
            let header_value = formatted_46;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_users(
    input: &crate::input::DescribeUsersInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_47) = &input.authentication_token {
        let formatted_48 = AsRef::<str>::as_ref(inner_47);
        if !formatted_48.is_empty() {
            let header_value = formatted_48;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_get_current_user(
    input: &crate::input::GetCurrentUserInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_49) = &input.authentication_token {
        let formatted_50 = AsRef::<str>::as_ref(inner_49);
        if !formatted_50.is_empty() {
            let header_value = formatted_50;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_get_document(
    input: &crate::input::GetDocumentInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_51) = &input.authentication_token {
        let formatted_52 = AsRef::<str>::as_ref(inner_51);
        if !formatted_52.is_empty() {
            let header_value = formatted_52;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_get_document_path(
    input: &crate::input::GetDocumentPathInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_53) = &input.authentication_token {
        let formatted_54 = AsRef::<str>::as_ref(inner_53);
        if !formatted_54.is_empty() {
            let header_value = formatted_54;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_get_document_version(
    input: &crate::input::GetDocumentVersionInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_55) = &input.authentication_token {
        let formatted_56 = AsRef::<str>::as_ref(inner_55);
        if !formatted_56.is_empty() {
            let header_value = formatted_56;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_get_folder(
    input: &crate::input::GetFolderInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_57) = &input.authentication_token {
        let formatted_58 = AsRef::<str>::as_ref(inner_57);
        if !formatted_58.is_empty() {
            let header_value = formatted_58;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_get_folder_path(
    input: &crate::input::GetFolderPathInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_59) = &input.authentication_token {
        let formatted_60 = AsRef::<str>::as_ref(inner_59);
        if !formatted_60.is_empty() {
            let header_value = formatted_60;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_get_resources(
    input: &crate::input::GetResourcesInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_61) = &input.authentication_token {
        let formatted_62 = AsRef::<str>::as_ref(inner_61);
        if !formatted_62.is_empty() {
            let header_value = formatted_62;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_initiate_document_version_upload(
    input: &crate::input::InitiateDocumentVersionUploadInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_63) = &input.authentication_token {
        let formatted_64 = AsRef::<str>::as_ref(inner_63);
        if !formatted_64.is_empty() {
            let header_value = formatted_64;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_remove_all_resource_permissions(
    input: &crate::input::RemoveAllResourcePermissionsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_65) = &input.authentication_token {
        let formatted_66 = AsRef::<str>::as_ref(inner_65);
        if !formatted_66.is_empty() {
            let header_value = formatted_66;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_remove_resource_permission(
    input: &crate::input::RemoveResourcePermissionInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_67) = &input.authentication_token {
        let formatted_68 = AsRef::<str>::as_ref(inner_67);
        if !formatted_68.is_empty() {
            let header_value = formatted_68;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_update_document(
    input: &crate::input::UpdateDocumentInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_69) = &input.authentication_token {
        let formatted_70 = AsRef::<str>::as_ref(inner_69);
        if !formatted_70.is_empty() {
            let header_value = formatted_70;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_update_document_version(
    input: &crate::input::UpdateDocumentVersionInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_71) = &input.authentication_token {
        let formatted_72 = AsRef::<str>::as_ref(inner_71);
        if !formatted_72.is_empty() {
            let header_value = formatted_72;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_update_folder(
    input: &crate::input::UpdateFolderInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_73) = &input.authentication_token {
        let formatted_74 = AsRef::<str>::as_ref(inner_73);
        if !formatted_74.is_empty() {
            let header_value = formatted_74;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_update_user(
    input: &crate::input::UpdateUserInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_75) = &input.authentication_token {
        let formatted_76 = AsRef::<str>::as_ref(inner_75);
        if !formatted_76.is_empty() {
            let header_value = formatted_76;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "authentication_token",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err,
                        ),
                    }
                })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}
