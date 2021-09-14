// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_cancel_rotate_secret(
    input: &crate::input::CancelRotateSecretInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_cancel_rotate_secret_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_secret(
    input: &crate::input::CreateSecretInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_secret_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_resource_policy(
    input: &crate::input::DeleteResourcePolicyInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_delete_resource_policy_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_secret(
    input: &crate::input::DeleteSecretInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_delete_secret_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_secret(
    input: &crate::input::DescribeSecretInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_secret_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_get_random_password(
    input: &crate::input::GetRandomPasswordInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_get_random_password_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_get_resource_policy(
    input: &crate::input::GetResourcePolicyInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_get_resource_policy_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_get_secret_value(
    input: &crate::input::GetSecretValueInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_get_secret_value_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_secrets(
    input: &crate::input::ListSecretsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_secrets_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_secret_version_ids(
    input: &crate::input::ListSecretVersionIdsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_secret_version_ids_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_put_resource_policy(
    input: &crate::input::PutResourcePolicyInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_put_resource_policy_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_put_secret_value(
    input: &crate::input::PutSecretValueInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_put_secret_value_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_remove_regions_from_replication(
    input: &crate::input::RemoveRegionsFromReplicationInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_remove_regions_from_replication_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_replicate_secret_to_regions(
    input: &crate::input::ReplicateSecretToRegionsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_replicate_secret_to_regions_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_restore_secret(
    input: &crate::input::RestoreSecretInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_restore_secret_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_rotate_secret(
    input: &crate::input::RotateSecretInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_rotate_secret_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_stop_replication_to_replica(
    input: &crate::input::StopReplicationToReplicaInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_stop_replication_to_replica_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_tag_resource(
    input: &crate::input::TagResourceInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_tag_resource_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_untag_resource(
    input: &crate::input::UntagResourceInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_untag_resource_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_update_secret(
    input: &crate::input::UpdateSecretInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_update_secret_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_update_secret_version_stage(
    input: &crate::input::UpdateSecretVersionStageInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_update_secret_version_stage_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_validate_resource_policy(
    input: &crate::input::ValidateResourcePolicyInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_validate_resource_policy_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}
