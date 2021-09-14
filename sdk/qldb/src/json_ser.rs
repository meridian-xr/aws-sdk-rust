// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_ledger_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLedgerInput,
) {
    if let Some(var_1) = &input.deletion_protection {
        object.key("DeletionProtection").boolean(*var_1);
    }
    if let Some(var_2) = &input.kms_key {
        object.key("KmsKey").string(var_2);
    }
    if let Some(var_3) = &input.name {
        object.key("Name").string(var_3);
    }
    if let Some(var_4) = &input.permissions_mode {
        object.key("PermissionsMode").string(var_4.as_str());
    }
    if let Some(var_5) = &input.tags {
        let mut object_6 = object.key("Tags").start_object();
        for (key_7, value_8) in var_5 {
            if let Some(var_9) = value_8 {
                object_6.key(key_7).string(var_9);
            } else {
                object_6.key(key_7).null();
            }
        }
        object_6.finish();
    }
}

pub fn serialize_structure_crate_input_export_journal_to_s3_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ExportJournalToS3Input,
) {
    if let Some(var_10) = &input.exclusive_end_time {
        object
            .key("ExclusiveEndTime")
            .instant(var_10, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_11) = &input.inclusive_start_time {
        object
            .key("InclusiveStartTime")
            .instant(var_11, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_12) = &input.role_arn {
        object.key("RoleArn").string(var_12);
    }
    if let Some(var_13) = &input.s3_export_configuration {
        let mut object_14 = object.key("S3ExportConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_export_configuration(
            &mut object_14,
            var_13,
        );
        object_14.finish();
    }
}

pub fn serialize_structure_crate_input_get_block_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetBlockInput,
) {
    if let Some(var_15) = &input.block_address {
        let mut object_16 = object.key("BlockAddress").start_object();
        crate::json_ser::serialize_structure_crate_model_value_holder(&mut object_16, var_15);
        object_16.finish();
    }
    if let Some(var_17) = &input.digest_tip_address {
        let mut object_18 = object.key("DigestTipAddress").start_object();
        crate::json_ser::serialize_structure_crate_model_value_holder(&mut object_18, var_17);
        object_18.finish();
    }
}

pub fn serialize_structure_crate_input_get_revision_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRevisionInput,
) {
    if let Some(var_19) = &input.block_address {
        let mut object_20 = object.key("BlockAddress").start_object();
        crate::json_ser::serialize_structure_crate_model_value_holder(&mut object_20, var_19);
        object_20.finish();
    }
    if let Some(var_21) = &input.digest_tip_address {
        let mut object_22 = object.key("DigestTipAddress").start_object();
        crate::json_ser::serialize_structure_crate_model_value_holder(&mut object_22, var_21);
        object_22.finish();
    }
    if let Some(var_23) = &input.document_id {
        object.key("DocumentId").string(var_23);
    }
}

pub fn serialize_structure_crate_input_stream_journal_to_kinesis_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StreamJournalToKinesisInput,
) {
    if let Some(var_24) = &input.exclusive_end_time {
        object
            .key("ExclusiveEndTime")
            .instant(var_24, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_25) = &input.inclusive_start_time {
        object
            .key("InclusiveStartTime")
            .instant(var_25, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_26) = &input.kinesis_configuration {
        let mut object_27 = object.key("KinesisConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_kinesis_configuration(
            &mut object_27,
            var_26,
        );
        object_27.finish();
    }
    if let Some(var_28) = &input.role_arn {
        object.key("RoleArn").string(var_28);
    }
    if let Some(var_29) = &input.stream_name {
        object.key("StreamName").string(var_29);
    }
    if let Some(var_30) = &input.tags {
        let mut object_31 = object.key("Tags").start_object();
        for (key_32, value_33) in var_30 {
            if let Some(var_34) = value_33 {
                object_31.key(key_32).string(var_34);
            } else {
                object_31.key(key_32).null();
            }
        }
        object_31.finish();
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_35) = &input.tags {
        let mut object_36 = object.key("Tags").start_object();
        for (key_37, value_38) in var_35 {
            if let Some(var_39) = value_38 {
                object_36.key(key_37).string(var_39);
            } else {
                object_36.key(key_37).null();
            }
        }
        object_36.finish();
    }
}

pub fn serialize_structure_crate_input_update_ledger_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLedgerInput,
) {
    if let Some(var_40) = &input.deletion_protection {
        object.key("DeletionProtection").boolean(*var_40);
    }
    if let Some(var_41) = &input.kms_key {
        object.key("KmsKey").string(var_41);
    }
}

pub fn serialize_structure_crate_input_update_ledger_permissions_mode_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLedgerPermissionsModeInput,
) {
    if let Some(var_42) = &input.permissions_mode {
        object.key("PermissionsMode").string(var_42.as_str());
    }
}

pub fn serialize_structure_crate_model_s3_export_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3ExportConfiguration,
) {
    if let Some(var_43) = &input.bucket {
        object.key("Bucket").string(var_43);
    }
    if let Some(var_44) = &input.prefix {
        object.key("Prefix").string(var_44);
    }
    if let Some(var_45) = &input.encryption_configuration {
        let mut object_46 = object.key("EncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_encryption_configuration(
            &mut object_46,
            var_45,
        );
        object_46.finish();
    }
}

pub fn serialize_structure_crate_model_value_holder(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ValueHolder,
) {
    if let Some(var_47) = &input.ion_text {
        object.key("IonText").string(var_47);
    }
}

pub fn serialize_structure_crate_model_kinesis_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KinesisConfiguration,
) {
    if let Some(var_48) = &input.stream_arn {
        object.key("StreamArn").string(var_48);
    }
    if let Some(var_49) = &input.aggregation_enabled {
        object.key("AggregationEnabled").boolean(*var_49);
    }
}

pub fn serialize_structure_crate_model_s3_encryption_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3EncryptionConfiguration,
) {
    if let Some(var_50) = &input.object_encryption_type {
        object.key("ObjectEncryptionType").string(var_50.as_str());
    }
    if let Some(var_51) = &input.kms_key_arn {
        object.key("KmsKeyArn").string(var_51);
    }
}
