// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_header_create_bucket_create_bucket_output_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_payload_get_public_access_block_get_public_access_block_output_public_access_block_configuration(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::model::PublicAccessBlockConfiguration>,
    crate::error::GetPublicAccessBlockError,
> {
    (!body.is_empty()).then(||{
        crate::xml_deser::deser_member_com_amazonaws_s3control_synthetic_get_public_access_block_output_public_access_block_configuration(body).map_err(crate::error::GetPublicAccessBlockError::unhandled)
    }).transpose()
}

pub fn deser_payload_get_storage_lens_configuration_get_storage_lens_configuration_output_storage_lens_configuration(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::model::StorageLensConfiguration>,
    crate::error::GetStorageLensConfigurationError,
> {
    (!body.is_empty()).then(||{
        crate::xml_deser::deser_member_com_amazonaws_s3control_synthetic_get_storage_lens_configuration_output_storage_lens_configuration(body).map_err(crate::error::GetStorageLensConfigurationError::unhandled)
    }).transpose()
}
