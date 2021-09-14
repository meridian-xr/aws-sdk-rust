// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_notification_channel_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddNotificationChannelInput,
) {
    if let Some(var_1) = &input.config {
        let mut object_2 = object.key("Config").start_object();
        crate::json_ser::serialize_structure_crate_model_notification_channel_config(
            &mut object_2,
            var_1,
        );
        object_2.finish();
    }
}

pub fn serialize_structure_crate_input_describe_account_overview_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAccountOverviewInput,
) {
    if let Some(var_3) = &input.from_time {
        object
            .key("FromTime")
            .instant(var_3, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_4) = &input.to_time {
        object
            .key("ToTime")
            .instant(var_4, smithy_types::instant::Format::EpochSeconds);
    }
}

pub fn serialize_structure_crate_input_describe_feedback_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeFeedbackInput,
) {
    if let Some(var_5) = &input.insight_id {
        object.key("InsightId").string(var_5);
    }
}

pub fn serialize_structure_crate_input_list_anomalies_for_insight_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAnomaliesForInsightInput,
) {
    if let Some(var_6) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.next_token {
        object.key("NextToken").string(var_7);
    }
    if let Some(var_8) = &input.start_time_range {
        let mut object_9 = object.key("StartTimeRange").start_object();
        crate::json_ser::serialize_structure_crate_model_start_time_range(&mut object_9, var_8);
        object_9.finish();
    }
}

pub fn serialize_structure_crate_input_list_events_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListEventsInput,
) {
    if let Some(var_10) = &input.filters {
        let mut object_11 = object.key("Filters").start_object();
        crate::json_ser::serialize_structure_crate_model_list_events_filters(
            &mut object_11,
            var_10,
        );
        object_11.finish();
    }
    if let Some(var_12) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    if let Some(var_13) = &input.next_token {
        object.key("NextToken").string(var_13);
    }
}

pub fn serialize_structure_crate_input_list_insights_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListInsightsInput,
) {
    if let Some(var_14) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    if let Some(var_15) = &input.next_token {
        object.key("NextToken").string(var_15);
    }
    if let Some(var_16) = &input.status_filter {
        let mut object_17 = object.key("StatusFilter").start_object();
        crate::json_ser::serialize_structure_crate_model_list_insights_status_filter(
            &mut object_17,
            var_16,
        );
        object_17.finish();
    }
}

pub fn serialize_structure_crate_input_list_notification_channels_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListNotificationChannelsInput,
) {
    if let Some(var_18) = &input.next_token {
        object.key("NextToken").string(var_18);
    }
}

pub fn serialize_structure_crate_input_list_recommendations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListRecommendationsInput,
) {
    if let Some(var_19) = &input.insight_id {
        object.key("InsightId").string(var_19);
    }
    if let Some(var_20) = &input.locale {
        object.key("Locale").string(var_20.as_str());
    }
    if let Some(var_21) = &input.next_token {
        object.key("NextToken").string(var_21);
    }
}

pub fn serialize_structure_crate_input_put_feedback_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutFeedbackInput,
) {
    if let Some(var_22) = &input.insight_feedback {
        let mut object_23 = object.key("InsightFeedback").start_object();
        crate::json_ser::serialize_structure_crate_model_insight_feedback(&mut object_23, var_22);
        object_23.finish();
    }
}

pub fn serialize_structure_crate_input_search_insights_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SearchInsightsInput,
) {
    if let Some(var_24) = &input.filters {
        let mut object_25 = object.key("Filters").start_object();
        crate::json_ser::serialize_structure_crate_model_search_insights_filters(
            &mut object_25,
            var_24,
        );
        object_25.finish();
    }
    if let Some(var_26) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_26).into()),
        );
    }
    if let Some(var_27) = &input.next_token {
        object.key("NextToken").string(var_27);
    }
    if let Some(var_28) = &input.start_time_range {
        let mut object_29 = object.key("StartTimeRange").start_object();
        crate::json_ser::serialize_structure_crate_model_start_time_range(&mut object_29, var_28);
        object_29.finish();
    }
    if let Some(var_30) = &input.r#type {
        object.key("Type").string(var_30.as_str());
    }
}

pub fn serialize_structure_crate_input_start_cost_estimation_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartCostEstimationInput,
) {
    if let Some(var_31) = &input.client_token {
        object.key("ClientToken").string(var_31);
    }
    if let Some(var_32) = &input.resource_collection {
        let mut object_33 = object.key("ResourceCollection").start_object();
        crate::json_ser::serialize_structure_crate_model_cost_estimation_resource_collection_filter(
            &mut object_33,
            var_32,
        );
        object_33.finish();
    }
}

pub fn serialize_structure_crate_input_update_resource_collection_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateResourceCollectionInput,
) {
    if let Some(var_34) = &input.action {
        object.key("Action").string(var_34.as_str());
    }
    if let Some(var_35) = &input.resource_collection {
        let mut object_36 = object.key("ResourceCollection").start_object();
        crate::json_ser::serialize_structure_crate_model_update_resource_collection_filter(
            &mut object_36,
            var_35,
        );
        object_36.finish();
    }
}

pub fn serialize_structure_crate_input_update_service_integration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateServiceIntegrationInput,
) {
    if let Some(var_37) = &input.service_integration {
        let mut object_38 = object.key("ServiceIntegration").start_object();
        crate::json_ser::serialize_structure_crate_model_update_service_integration_config(
            &mut object_38,
            var_37,
        );
        object_38.finish();
    }
}

pub fn serialize_structure_crate_model_notification_channel_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NotificationChannelConfig,
) {
    if let Some(var_39) = &input.sns {
        let mut object_40 = object.key("Sns").start_object();
        crate::json_ser::serialize_structure_crate_model_sns_channel_config(&mut object_40, var_39);
        object_40.finish();
    }
}

pub fn serialize_structure_crate_model_start_time_range(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StartTimeRange,
) {
    if let Some(var_41) = &input.from_time {
        object
            .key("FromTime")
            .instant(var_41, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_42) = &input.to_time {
        object
            .key("ToTime")
            .instant(var_42, smithy_types::instant::Format::EpochSeconds);
    }
}

pub fn serialize_structure_crate_model_list_events_filters(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ListEventsFilters,
) {
    if let Some(var_43) = &input.insight_id {
        object.key("InsightId").string(var_43);
    }
    if let Some(var_44) = &input.event_time_range {
        let mut object_45 = object.key("EventTimeRange").start_object();
        crate::json_ser::serialize_structure_crate_model_event_time_range(&mut object_45, var_44);
        object_45.finish();
    }
    if let Some(var_46) = &input.event_class {
        object.key("EventClass").string(var_46.as_str());
    }
    if let Some(var_47) = &input.event_source {
        object.key("EventSource").string(var_47);
    }
    if let Some(var_48) = &input.data_source {
        object.key("DataSource").string(var_48.as_str());
    }
    if let Some(var_49) = &input.resource_collection {
        let mut object_50 = object.key("ResourceCollection").start_object();
        crate::json_ser::serialize_structure_crate_model_resource_collection(
            &mut object_50,
            var_49,
        );
        object_50.finish();
    }
}

pub fn serialize_structure_crate_model_list_insights_status_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ListInsightsStatusFilter,
) {
    if let Some(var_51) = &input.ongoing {
        let mut object_52 = object.key("Ongoing").start_object();
        crate::json_ser::serialize_structure_crate_model_list_insights_ongoing_status_filter(
            &mut object_52,
            var_51,
        );
        object_52.finish();
    }
    if let Some(var_53) = &input.closed {
        let mut object_54 = object.key("Closed").start_object();
        crate::json_ser::serialize_structure_crate_model_list_insights_closed_status_filter(
            &mut object_54,
            var_53,
        );
        object_54.finish();
    }
    if let Some(var_55) = &input.any {
        let mut object_56 = object.key("Any").start_object();
        crate::json_ser::serialize_structure_crate_model_list_insights_any_status_filter(
            &mut object_56,
            var_55,
        );
        object_56.finish();
    }
}

pub fn serialize_structure_crate_model_insight_feedback(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InsightFeedback,
) {
    if let Some(var_57) = &input.id {
        object.key("Id").string(var_57);
    }
    if let Some(var_58) = &input.feedback {
        object.key("Feedback").string(var_58.as_str());
    }
}

pub fn serialize_structure_crate_model_search_insights_filters(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SearchInsightsFilters,
) {
    if let Some(var_59) = &input.severities {
        let mut array_60 = object.key("Severities").start_array();
        for item_61 in var_59 {
            {
                array_60.value().string(item_61.as_str());
            }
        }
        array_60.finish();
    }
    if let Some(var_62) = &input.statuses {
        let mut array_63 = object.key("Statuses").start_array();
        for item_64 in var_62 {
            {
                array_63.value().string(item_64.as_str());
            }
        }
        array_63.finish();
    }
    if let Some(var_65) = &input.resource_collection {
        let mut object_66 = object.key("ResourceCollection").start_object();
        crate::json_ser::serialize_structure_crate_model_resource_collection(
            &mut object_66,
            var_65,
        );
        object_66.finish();
    }
    if let Some(var_67) = &input.service_collection {
        let mut object_68 = object.key("ServiceCollection").start_object();
        crate::json_ser::serialize_structure_crate_model_service_collection(&mut object_68, var_67);
        object_68.finish();
    }
}

pub fn serialize_structure_crate_model_cost_estimation_resource_collection_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CostEstimationResourceCollectionFilter,
) {
    if let Some(var_69) = &input.cloud_formation {
        let mut object_70 = object.key("CloudFormation").start_object();
        crate::json_ser::serialize_structure_crate_model_cloud_formation_cost_estimation_resource_collection_filter(&mut object_70, var_69);
        object_70.finish();
    }
}

pub fn serialize_structure_crate_model_update_resource_collection_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateResourceCollectionFilter,
) {
    if let Some(var_71) = &input.cloud_formation {
        let mut object_72 = object.key("CloudFormation").start_object();
        crate::json_ser::serialize_structure_crate_model_update_cloud_formation_collection_filter(
            &mut object_72,
            var_71,
        );
        object_72.finish();
    }
}

pub fn serialize_structure_crate_model_update_service_integration_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateServiceIntegrationConfig,
) {
    if let Some(var_73) = &input.ops_center {
        let mut object_74 = object.key("OpsCenter").start_object();
        crate::json_ser::serialize_structure_crate_model_ops_center_integration_config(
            &mut object_74,
            var_73,
        );
        object_74.finish();
    }
}

pub fn serialize_structure_crate_model_sns_channel_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SnsChannelConfig,
) {
    if let Some(var_75) = &input.topic_arn {
        object.key("TopicArn").string(var_75);
    }
}

pub fn serialize_structure_crate_model_event_time_range(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EventTimeRange,
) {
    if let Some(var_76) = &input.from_time {
        object
            .key("FromTime")
            .instant(var_76, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_77) = &input.to_time {
        object
            .key("ToTime")
            .instant(var_77, smithy_types::instant::Format::EpochSeconds);
    }
}

pub fn serialize_structure_crate_model_resource_collection(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ResourceCollection,
) {
    if let Some(var_78) = &input.cloud_formation {
        let mut object_79 = object.key("CloudFormation").start_object();
        crate::json_ser::serialize_structure_crate_model_cloud_formation_collection(
            &mut object_79,
            var_78,
        );
        object_79.finish();
    }
}

pub fn serialize_structure_crate_model_list_insights_ongoing_status_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ListInsightsOngoingStatusFilter,
) {
    if let Some(var_80) = &input.r#type {
        object.key("Type").string(var_80.as_str());
    }
}

pub fn serialize_structure_crate_model_list_insights_closed_status_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ListInsightsClosedStatusFilter,
) {
    if let Some(var_81) = &input.r#type {
        object.key("Type").string(var_81.as_str());
    }
    if let Some(var_82) = &input.end_time_range {
        let mut object_83 = object.key("EndTimeRange").start_object();
        crate::json_ser::serialize_structure_crate_model_end_time_range(&mut object_83, var_82);
        object_83.finish();
    }
}

pub fn serialize_structure_crate_model_list_insights_any_status_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ListInsightsAnyStatusFilter,
) {
    if let Some(var_84) = &input.r#type {
        object.key("Type").string(var_84.as_str());
    }
    if let Some(var_85) = &input.start_time_range {
        let mut object_86 = object.key("StartTimeRange").start_object();
        crate::json_ser::serialize_structure_crate_model_start_time_range(&mut object_86, var_85);
        object_86.finish();
    }
}

pub fn serialize_structure_crate_model_service_collection(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ServiceCollection,
) {
    if let Some(var_87) = &input.service_names {
        let mut array_88 = object.key("ServiceNames").start_array();
        for item_89 in var_87 {
            {
                array_88.value().string(item_89.as_str());
            }
        }
        array_88.finish();
    }
}

pub fn serialize_structure_crate_model_cloud_formation_cost_estimation_resource_collection_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CloudFormationCostEstimationResourceCollectionFilter,
) {
    if let Some(var_90) = &input.stack_names {
        let mut array_91 = object.key("StackNames").start_array();
        for item_92 in var_90 {
            {
                array_91.value().string(item_92);
            }
        }
        array_91.finish();
    }
}

pub fn serialize_structure_crate_model_update_cloud_formation_collection_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateCloudFormationCollectionFilter,
) {
    if let Some(var_93) = &input.stack_names {
        let mut array_94 = object.key("StackNames").start_array();
        for item_95 in var_93 {
            {
                array_94.value().string(item_95);
            }
        }
        array_94.finish();
    }
}

pub fn serialize_structure_crate_model_ops_center_integration_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OpsCenterIntegrationConfig,
) {
    if let Some(var_96) = &input.opt_in_status {
        object.key("OptInStatus").string(var_96.as_str());
    }
}

pub fn serialize_structure_crate_model_cloud_formation_collection(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CloudFormationCollection,
) {
    if let Some(var_97) = &input.stack_names {
        let mut array_98 = object.key("StackNames").start_array();
        for item_99 in var_97 {
            {
                array_98.value().string(item_99);
            }
        }
        array_98.finish();
    }
}

pub fn serialize_structure_crate_model_end_time_range(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EndTimeRange,
) {
    if let Some(var_100) = &input.from_time {
        object
            .key("FromTime")
            .instant(var_100, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_101) = &input.to_time {
        object
            .key("ToTime")
            .instant(var_101, smithy_types::instant::Format::EpochSeconds);
    }
}
