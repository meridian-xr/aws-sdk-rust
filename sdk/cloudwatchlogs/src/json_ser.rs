// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_kms_key_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateKmsKeyInput,
) {
    if let Some(var_1) = &input.log_group_name {
        object.key("logGroupName").string(var_1);
    }
    if let Some(var_2) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_2);
    }
}

pub fn serialize_structure_crate_input_cancel_export_task_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelExportTaskInput,
) {
    if let Some(var_3) = &input.task_id {
        object.key("taskId").string(var_3);
    }
}

pub fn serialize_structure_crate_input_create_export_task_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateExportTaskInput,
) {
    if let Some(var_4) = &input.task_name {
        object.key("taskName").string(var_4);
    }
    if let Some(var_5) = &input.log_group_name {
        object.key("logGroupName").string(var_5);
    }
    if let Some(var_6) = &input.log_stream_name_prefix {
        object.key("logStreamNamePrefix").string(var_6);
    }
    if let Some(var_7) = &input.from {
        object.key("from").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.to {
        object.key("to").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.destination {
        object.key("destination").string(var_9);
    }
    if let Some(var_10) = &input.destination_prefix {
        object.key("destinationPrefix").string(var_10);
    }
}

pub fn serialize_structure_crate_input_create_log_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLogGroupInput,
) {
    if let Some(var_11) = &input.log_group_name {
        object.key("logGroupName").string(var_11);
    }
    if let Some(var_12) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_12);
    }
    if let Some(var_13) = &input.tags {
        let mut object_14 = object.key("tags").start_object();
        for (key_15, value_16) in var_13 {
            {
                object_14.key(key_15).string(value_16);
            }
        }
        object_14.finish();
    }
}

pub fn serialize_structure_crate_input_create_log_stream_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLogStreamInput,
) {
    if let Some(var_17) = &input.log_group_name {
        object.key("logGroupName").string(var_17);
    }
    if let Some(var_18) = &input.log_stream_name {
        object.key("logStreamName").string(var_18);
    }
}

pub fn serialize_structure_crate_input_delete_destination_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteDestinationInput,
) {
    if let Some(var_19) = &input.destination_name {
        object.key("destinationName").string(var_19);
    }
}

pub fn serialize_structure_crate_input_delete_log_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteLogGroupInput,
) {
    if let Some(var_20) = &input.log_group_name {
        object.key("logGroupName").string(var_20);
    }
}

pub fn serialize_structure_crate_input_delete_log_stream_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteLogStreamInput,
) {
    if let Some(var_21) = &input.log_group_name {
        object.key("logGroupName").string(var_21);
    }
    if let Some(var_22) = &input.log_stream_name {
        object.key("logStreamName").string(var_22);
    }
}

pub fn serialize_structure_crate_input_delete_metric_filter_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteMetricFilterInput,
) {
    if let Some(var_23) = &input.log_group_name {
        object.key("logGroupName").string(var_23);
    }
    if let Some(var_24) = &input.filter_name {
        object.key("filterName").string(var_24);
    }
}

pub fn serialize_structure_crate_input_delete_query_definition_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteQueryDefinitionInput,
) {
    if let Some(var_25) = &input.query_definition_id {
        object.key("queryDefinitionId").string(var_25);
    }
}

pub fn serialize_structure_crate_input_delete_resource_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteResourcePolicyInput,
) {
    if let Some(var_26) = &input.policy_name {
        object.key("policyName").string(var_26);
    }
}

pub fn serialize_structure_crate_input_delete_retention_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteRetentionPolicyInput,
) {
    if let Some(var_27) = &input.log_group_name {
        object.key("logGroupName").string(var_27);
    }
}

pub fn serialize_structure_crate_input_delete_subscription_filter_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSubscriptionFilterInput,
) {
    if let Some(var_28) = &input.log_group_name {
        object.key("logGroupName").string(var_28);
    }
    if let Some(var_29) = &input.filter_name {
        object.key("filterName").string(var_29);
    }
}

pub fn serialize_structure_crate_input_describe_destinations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDestinationsInput,
) {
    if let Some(var_30) = &input.destination_name_prefix {
        object.key("DestinationNamePrefix").string(var_30);
    }
    if let Some(var_31) = &input.next_token {
        object.key("nextToken").string(var_31);
    }
    if let Some(var_32) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_32).into()),
        );
    }
}

pub fn serialize_structure_crate_input_describe_export_tasks_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeExportTasksInput,
) {
    if let Some(var_33) = &input.task_id {
        object.key("taskId").string(var_33);
    }
    if let Some(var_34) = &input.status_code {
        object.key("statusCode").string(var_34.as_str());
    }
    if let Some(var_35) = &input.next_token {
        object.key("nextToken").string(var_35);
    }
    if let Some(var_36) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_36).into()),
        );
    }
}

pub fn serialize_structure_crate_input_describe_log_groups_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLogGroupsInput,
) {
    if let Some(var_37) = &input.log_group_name_prefix {
        object.key("logGroupNamePrefix").string(var_37);
    }
    if let Some(var_38) = &input.next_token {
        object.key("nextToken").string(var_38);
    }
    if let Some(var_39) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_39).into()),
        );
    }
}

pub fn serialize_structure_crate_input_describe_log_streams_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLogStreamsInput,
) {
    if let Some(var_40) = &input.log_group_name {
        object.key("logGroupName").string(var_40);
    }
    if let Some(var_41) = &input.log_stream_name_prefix {
        object.key("logStreamNamePrefix").string(var_41);
    }
    if let Some(var_42) = &input.order_by {
        object.key("orderBy").string(var_42.as_str());
    }
    if let Some(var_43) = &input.descending {
        object.key("descending").boolean(*var_43);
    }
    if let Some(var_44) = &input.next_token {
        object.key("nextToken").string(var_44);
    }
    if let Some(var_45) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_45).into()),
        );
    }
}

pub fn serialize_structure_crate_input_describe_metric_filters_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeMetricFiltersInput,
) {
    if let Some(var_46) = &input.log_group_name {
        object.key("logGroupName").string(var_46);
    }
    if let Some(var_47) = &input.filter_name_prefix {
        object.key("filterNamePrefix").string(var_47);
    }
    if let Some(var_48) = &input.next_token {
        object.key("nextToken").string(var_48);
    }
    if let Some(var_49) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_49).into()),
        );
    }
    if let Some(var_50) = &input.metric_name {
        object.key("metricName").string(var_50);
    }
    if let Some(var_51) = &input.metric_namespace {
        object.key("metricNamespace").string(var_51);
    }
}

pub fn serialize_structure_crate_input_describe_queries_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeQueriesInput,
) {
    if let Some(var_52) = &input.log_group_name {
        object.key("logGroupName").string(var_52);
    }
    if let Some(var_53) = &input.status {
        object.key("status").string(var_53.as_str());
    }
    if let Some(var_54) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_54).into()),
        );
    }
    if let Some(var_55) = &input.next_token {
        object.key("nextToken").string(var_55);
    }
}

pub fn serialize_structure_crate_input_describe_query_definitions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeQueryDefinitionsInput,
) {
    if let Some(var_56) = &input.query_definition_name_prefix {
        object.key("queryDefinitionNamePrefix").string(var_56);
    }
    if let Some(var_57) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_57).into()),
        );
    }
    if let Some(var_58) = &input.next_token {
        object.key("nextToken").string(var_58);
    }
}

pub fn serialize_structure_crate_input_describe_resource_policies_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeResourcePoliciesInput,
) {
    if let Some(var_59) = &input.next_token {
        object.key("nextToken").string(var_59);
    }
    if let Some(var_60) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_60).into()),
        );
    }
}

pub fn serialize_structure_crate_input_describe_subscription_filters_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSubscriptionFiltersInput,
) {
    if let Some(var_61) = &input.log_group_name {
        object.key("logGroupName").string(var_61);
    }
    if let Some(var_62) = &input.filter_name_prefix {
        object.key("filterNamePrefix").string(var_62);
    }
    if let Some(var_63) = &input.next_token {
        object.key("nextToken").string(var_63);
    }
    if let Some(var_64) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_64).into()),
        );
    }
}

pub fn serialize_structure_crate_input_disassociate_kms_key_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateKmsKeyInput,
) {
    if let Some(var_65) = &input.log_group_name {
        object.key("logGroupName").string(var_65);
    }
}

pub fn serialize_structure_crate_input_filter_log_events_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::FilterLogEventsInput,
) {
    if let Some(var_66) = &input.log_group_name {
        object.key("logGroupName").string(var_66);
    }
    if let Some(var_67) = &input.log_stream_names {
        let mut array_68 = object.key("logStreamNames").start_array();
        for item_69 in var_67 {
            {
                array_68.value().string(item_69);
            }
        }
        array_68.finish();
    }
    if let Some(var_70) = &input.log_stream_name_prefix {
        object.key("logStreamNamePrefix").string(var_70);
    }
    if let Some(var_71) = &input.start_time {
        object.key("startTime").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_71).into()),
        );
    }
    if let Some(var_72) = &input.end_time {
        object.key("endTime").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_72).into()),
        );
    }
    if let Some(var_73) = &input.filter_pattern {
        object.key("filterPattern").string(var_73);
    }
    if let Some(var_74) = &input.next_token {
        object.key("nextToken").string(var_74);
    }
    if let Some(var_75) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_75).into()),
        );
    }
    if let Some(var_76) = &input.interleaved {
        object.key("interleaved").boolean(*var_76);
    }
}

pub fn serialize_structure_crate_input_get_log_events_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetLogEventsInput,
) {
    if let Some(var_77) = &input.log_group_name {
        object.key("logGroupName").string(var_77);
    }
    if let Some(var_78) = &input.log_stream_name {
        object.key("logStreamName").string(var_78);
    }
    if let Some(var_79) = &input.start_time {
        object.key("startTime").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_79).into()),
        );
    }
    if let Some(var_80) = &input.end_time {
        object.key("endTime").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_80).into()),
        );
    }
    if let Some(var_81) = &input.next_token {
        object.key("nextToken").string(var_81);
    }
    if let Some(var_82) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_82).into()),
        );
    }
    if let Some(var_83) = &input.start_from_head {
        object.key("startFromHead").boolean(*var_83);
    }
}

pub fn serialize_structure_crate_input_get_log_group_fields_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetLogGroupFieldsInput,
) {
    if let Some(var_84) = &input.log_group_name {
        object.key("logGroupName").string(var_84);
    }
    if let Some(var_85) = &input.time {
        object.key("time").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_85).into()),
        );
    }
}

pub fn serialize_structure_crate_input_get_log_record_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetLogRecordInput,
) {
    if let Some(var_86) = &input.log_record_pointer {
        object.key("logRecordPointer").string(var_86);
    }
}

pub fn serialize_structure_crate_input_get_query_results_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetQueryResultsInput,
) {
    if let Some(var_87) = &input.query_id {
        object.key("queryId").string(var_87);
    }
}

pub fn serialize_structure_crate_input_list_tags_log_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsLogGroupInput,
) {
    if let Some(var_88) = &input.log_group_name {
        object.key("logGroupName").string(var_88);
    }
}

pub fn serialize_structure_crate_input_put_destination_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutDestinationInput,
) {
    if let Some(var_89) = &input.destination_name {
        object.key("destinationName").string(var_89);
    }
    if let Some(var_90) = &input.target_arn {
        object.key("targetArn").string(var_90);
    }
    if let Some(var_91) = &input.role_arn {
        object.key("roleArn").string(var_91);
    }
}

pub fn serialize_structure_crate_input_put_destination_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutDestinationPolicyInput,
) {
    if let Some(var_92) = &input.destination_name {
        object.key("destinationName").string(var_92);
    }
    if let Some(var_93) = &input.access_policy {
        object.key("accessPolicy").string(var_93);
    }
}

pub fn serialize_structure_crate_input_put_log_events_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutLogEventsInput,
) {
    if let Some(var_94) = &input.log_group_name {
        object.key("logGroupName").string(var_94);
    }
    if let Some(var_95) = &input.log_stream_name {
        object.key("logStreamName").string(var_95);
    }
    if let Some(var_96) = &input.log_events {
        let mut array_97 = object.key("logEvents").start_array();
        for item_98 in var_96 {
            {
                let mut object_99 = array_97.value().start_object();
                crate::json_ser::serialize_structure_crate_model_input_log_event(
                    &mut object_99,
                    item_98,
                );
                object_99.finish();
            }
        }
        array_97.finish();
    }
    if let Some(var_100) = &input.sequence_token {
        object.key("sequenceToken").string(var_100);
    }
}

pub fn serialize_structure_crate_input_put_metric_filter_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutMetricFilterInput,
) {
    if let Some(var_101) = &input.log_group_name {
        object.key("logGroupName").string(var_101);
    }
    if let Some(var_102) = &input.filter_name {
        object.key("filterName").string(var_102);
    }
    if let Some(var_103) = &input.filter_pattern {
        object.key("filterPattern").string(var_103);
    }
    if let Some(var_104) = &input.metric_transformations {
        let mut array_105 = object.key("metricTransformations").start_array();
        for item_106 in var_104 {
            {
                let mut object_107 = array_105.value().start_object();
                crate::json_ser::serialize_structure_crate_model_metric_transformation(
                    &mut object_107,
                    item_106,
                );
                object_107.finish();
            }
        }
        array_105.finish();
    }
}

pub fn serialize_structure_crate_input_put_query_definition_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutQueryDefinitionInput,
) {
    if let Some(var_108) = &input.name {
        object.key("name").string(var_108);
    }
    if let Some(var_109) = &input.query_definition_id {
        object.key("queryDefinitionId").string(var_109);
    }
    if let Some(var_110) = &input.log_group_names {
        let mut array_111 = object.key("logGroupNames").start_array();
        for item_112 in var_110 {
            {
                array_111.value().string(item_112);
            }
        }
        array_111.finish();
    }
    if let Some(var_113) = &input.query_string {
        object.key("queryString").string(var_113);
    }
}

pub fn serialize_structure_crate_input_put_resource_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutResourcePolicyInput,
) {
    if let Some(var_114) = &input.policy_name {
        object.key("policyName").string(var_114);
    }
    if let Some(var_115) = &input.policy_document {
        object.key("policyDocument").string(var_115);
    }
}

pub fn serialize_structure_crate_input_put_retention_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutRetentionPolicyInput,
) {
    if let Some(var_116) = &input.log_group_name {
        object.key("logGroupName").string(var_116);
    }
    if let Some(var_117) = &input.retention_in_days {
        object.key("retentionInDays").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_117).into()),
        );
    }
}

pub fn serialize_structure_crate_input_put_subscription_filter_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutSubscriptionFilterInput,
) {
    if let Some(var_118) = &input.log_group_name {
        object.key("logGroupName").string(var_118);
    }
    if let Some(var_119) = &input.filter_name {
        object.key("filterName").string(var_119);
    }
    if let Some(var_120) = &input.filter_pattern {
        object.key("filterPattern").string(var_120);
    }
    if let Some(var_121) = &input.destination_arn {
        object.key("destinationArn").string(var_121);
    }
    if let Some(var_122) = &input.role_arn {
        object.key("roleArn").string(var_122);
    }
    if let Some(var_123) = &input.distribution {
        object.key("distribution").string(var_123.as_str());
    }
}

pub fn serialize_structure_crate_input_start_query_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartQueryInput,
) {
    if let Some(var_124) = &input.log_group_name {
        object.key("logGroupName").string(var_124);
    }
    if let Some(var_125) = &input.log_group_names {
        let mut array_126 = object.key("logGroupNames").start_array();
        for item_127 in var_125 {
            {
                array_126.value().string(item_127);
            }
        }
        array_126.finish();
    }
    if let Some(var_128) = &input.start_time {
        object.key("startTime").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_128).into()),
        );
    }
    if let Some(var_129) = &input.end_time {
        object.key("endTime").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_129).into()),
        );
    }
    if let Some(var_130) = &input.query_string {
        object.key("queryString").string(var_130);
    }
    if let Some(var_131) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_131).into()),
        );
    }
}

pub fn serialize_structure_crate_input_stop_query_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopQueryInput,
) {
    if let Some(var_132) = &input.query_id {
        object.key("queryId").string(var_132);
    }
}

pub fn serialize_structure_crate_input_tag_log_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagLogGroupInput,
) {
    if let Some(var_133) = &input.log_group_name {
        object.key("logGroupName").string(var_133);
    }
    if let Some(var_134) = &input.tags {
        let mut object_135 = object.key("tags").start_object();
        for (key_136, value_137) in var_134 {
            {
                object_135.key(key_136).string(value_137);
            }
        }
        object_135.finish();
    }
}

pub fn serialize_structure_crate_input_test_metric_filter_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TestMetricFilterInput,
) {
    if let Some(var_138) = &input.filter_pattern {
        object.key("filterPattern").string(var_138);
    }
    if let Some(var_139) = &input.log_event_messages {
        let mut array_140 = object.key("logEventMessages").start_array();
        for item_141 in var_139 {
            {
                array_140.value().string(item_141);
            }
        }
        array_140.finish();
    }
}

pub fn serialize_structure_crate_input_untag_log_group_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagLogGroupInput,
) {
    if let Some(var_142) = &input.log_group_name {
        object.key("logGroupName").string(var_142);
    }
    if let Some(var_143) = &input.tags {
        let mut array_144 = object.key("tags").start_array();
        for item_145 in var_143 {
            {
                array_144.value().string(item_145);
            }
        }
        array_144.finish();
    }
}

pub fn serialize_structure_crate_model_input_log_event(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputLogEvent,
) {
    if let Some(var_146) = &input.timestamp {
        object.key("timestamp").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_146).into()),
        );
    }
    if let Some(var_147) = &input.message {
        object.key("message").string(var_147);
    }
}

pub fn serialize_structure_crate_model_metric_transformation(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MetricTransformation,
) {
    if let Some(var_148) = &input.metric_name {
        object.key("metricName").string(var_148);
    }
    if let Some(var_149) = &input.metric_namespace {
        object.key("metricNamespace").string(var_149);
    }
    if let Some(var_150) = &input.metric_value {
        object.key("metricValue").string(var_150);
    }
    if let Some(var_151) = &input.default_value {
        object.key("defaultValue").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_151).into()),
        );
    }
    if let Some(var_152) = &input.dimensions {
        let mut object_153 = object.key("dimensions").start_object();
        for (key_154, value_155) in var_152 {
            {
                object_153.key(key_154).string(value_155);
            }
        }
        object_153.finish();
    }
    if let Some(var_156) = &input.unit {
        object.key("unit").string(var_156.as_str());
    }
}
