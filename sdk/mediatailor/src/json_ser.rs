// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_channel_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateChannelInput,
) {
    if let Some(var_1) = &input.filler_slate {
        let mut object_2 = object.key("FillerSlate").start_object();
        crate::json_ser::serialize_structure_crate_model_slate_source(&mut object_2, var_1);
        object_2.finish();
    }
    if let Some(var_3) = &input.outputs {
        let mut array_4 = object.key("Outputs").start_array();
        for item_5 in var_3 {
            {
                let mut object_6 = array_4.value().start_object();
                crate::json_ser::serialize_structure_crate_model_request_output_item(
                    &mut object_6,
                    item_5,
                );
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.playback_mode {
        object.key("PlaybackMode").string(var_7.as_str());
    }
    if let Some(var_8) = &input.tags {
        let mut object_9 = object.key("tags").start_object();
        for (key_10, value_11) in var_8 {
            {
                object_9.key(key_10).string(value_11);
            }
        }
        object_9.finish();
    }
}

pub fn serialize_structure_crate_input_create_program_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateProgramInput,
) {
    if let Some(var_12) = &input.ad_breaks {
        let mut array_13 = object.key("AdBreaks").start_array();
        for item_14 in var_12 {
            {
                let mut object_15 = array_13.value().start_object();
                crate::json_ser::serialize_structure_crate_model_ad_break(&mut object_15, item_14);
                object_15.finish();
            }
        }
        array_13.finish();
    }
    if let Some(var_16) = &input.schedule_configuration {
        let mut object_17 = object.key("ScheduleConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_schedule_configuration(
            &mut object_17,
            var_16,
        );
        object_17.finish();
    }
    if let Some(var_18) = &input.source_location_name {
        object.key("SourceLocationName").string(var_18);
    }
    if let Some(var_19) = &input.vod_source_name {
        object.key("VodSourceName").string(var_19);
    }
}

pub fn serialize_structure_crate_input_create_source_location_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSourceLocationInput,
) {
    if let Some(var_20) = &input.access_configuration {
        let mut object_21 = object.key("AccessConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_access_configuration(
            &mut object_21,
            var_20,
        );
        object_21.finish();
    }
    if let Some(var_22) = &input.default_segment_delivery_configuration {
        let mut object_23 = object
            .key("DefaultSegmentDeliveryConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_default_segment_delivery_configuration(
            &mut object_23,
            var_22,
        );
        object_23.finish();
    }
    if let Some(var_24) = &input.http_configuration {
        let mut object_25 = object.key("HttpConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_http_configuration(&mut object_25, var_24);
        object_25.finish();
    }
    if let Some(var_26) = &input.tags {
        let mut object_27 = object.key("tags").start_object();
        for (key_28, value_29) in var_26 {
            {
                object_27.key(key_28).string(value_29);
            }
        }
        object_27.finish();
    }
}

pub fn serialize_structure_crate_input_create_vod_source_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateVodSourceInput,
) {
    if let Some(var_30) = &input.http_package_configurations {
        let mut array_31 = object.key("HttpPackageConfigurations").start_array();
        for item_32 in var_30 {
            {
                let mut object_33 = array_31.value().start_object();
                crate::json_ser::serialize_structure_crate_model_http_package_configuration(
                    &mut object_33,
                    item_32,
                );
                object_33.finish();
            }
        }
        array_31.finish();
    }
    if let Some(var_34) = &input.tags {
        let mut object_35 = object.key("tags").start_object();
        for (key_36, value_37) in var_34 {
            {
                object_35.key(key_36).string(value_37);
            }
        }
        object_35.finish();
    }
}

pub fn serialize_structure_crate_input_put_channel_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutChannelPolicyInput,
) {
    if let Some(var_38) = &input.policy {
        object.key("Policy").string(var_38);
    }
}

pub fn serialize_structure_crate_input_put_playback_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutPlaybackConfigurationInput,
) {
    if let Some(var_39) = &input.ad_decision_server_url {
        object.key("AdDecisionServerUrl").string(var_39);
    }
    if let Some(var_40) = &input.avail_suppression {
        let mut object_41 = object.key("AvailSuppression").start_object();
        crate::json_ser::serialize_structure_crate_model_avail_suppression(&mut object_41, var_40);
        object_41.finish();
    }
    if let Some(var_42) = &input.bumper {
        let mut object_43 = object.key("Bumper").start_object();
        crate::json_ser::serialize_structure_crate_model_bumper(&mut object_43, var_42);
        object_43.finish();
    }
    if let Some(var_44) = &input.cdn_configuration {
        let mut object_45 = object.key("CdnConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_cdn_configuration(&mut object_45, var_44);
        object_45.finish();
    }
    if let Some(var_46) = &input.configuration_aliases {
        let mut object_47 = object.key("ConfigurationAliases").start_object();
        for (key_48, value_49) in var_46 {
            {
                let mut object_50 = object_47.key(key_48).start_object();
                for (key_51, value_52) in value_49 {
                    {
                        object_50.key(key_51).string(value_52);
                    }
                }
                object_50.finish();
            }
        }
        object_47.finish();
    }
    if let Some(var_53) = &input.dash_configuration {
        let mut object_54 = object.key("DashConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_configuration_for_put(
            &mut object_54,
            var_53,
        );
        object_54.finish();
    }
    if let Some(var_55) = &input.live_pre_roll_configuration {
        let mut object_56 = object.key("LivePreRollConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_live_pre_roll_configuration(
            &mut object_56,
            var_55,
        );
        object_56.finish();
    }
    if let Some(var_57) = &input.manifest_processing_rules {
        let mut object_58 = object.key("ManifestProcessingRules").start_object();
        crate::json_ser::serialize_structure_crate_model_manifest_processing_rules(
            &mut object_58,
            var_57,
        );
        object_58.finish();
    }
    if let Some(var_59) = &input.name {
        object.key("Name").string(var_59);
    }
    if input.personalization_threshold_seconds != 0 {
        object.key("PersonalizationThresholdSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.personalization_threshold_seconds).into()),
        );
    }
    if let Some(var_60) = &input.slate_ad_url {
        object.key("SlateAdUrl").string(var_60);
    }
    if let Some(var_61) = &input.tags {
        let mut object_62 = object.key("tags").start_object();
        for (key_63, value_64) in var_61 {
            {
                object_62.key(key_63).string(value_64);
            }
        }
        object_62.finish();
    }
    if let Some(var_65) = &input.transcode_profile_name {
        object.key("TranscodeProfileName").string(var_65);
    }
    if let Some(var_66) = &input.video_content_source_url {
        object.key("VideoContentSourceUrl").string(var_66);
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_67) = &input.tags {
        let mut object_68 = object.key("tags").start_object();
        for (key_69, value_70) in var_67 {
            {
                object_68.key(key_69).string(value_70);
            }
        }
        object_68.finish();
    }
}

pub fn serialize_structure_crate_input_update_channel_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateChannelInput,
) {
    if let Some(var_71) = &input.outputs {
        let mut array_72 = object.key("Outputs").start_array();
        for item_73 in var_71 {
            {
                let mut object_74 = array_72.value().start_object();
                crate::json_ser::serialize_structure_crate_model_request_output_item(
                    &mut object_74,
                    item_73,
                );
                object_74.finish();
            }
        }
        array_72.finish();
    }
}

pub fn serialize_structure_crate_input_update_source_location_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSourceLocationInput,
) {
    if let Some(var_75) = &input.access_configuration {
        let mut object_76 = object.key("AccessConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_access_configuration(
            &mut object_76,
            var_75,
        );
        object_76.finish();
    }
    if let Some(var_77) = &input.default_segment_delivery_configuration {
        let mut object_78 = object
            .key("DefaultSegmentDeliveryConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_default_segment_delivery_configuration(
            &mut object_78,
            var_77,
        );
        object_78.finish();
    }
    if let Some(var_79) = &input.http_configuration {
        let mut object_80 = object.key("HttpConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_http_configuration(&mut object_80, var_79);
        object_80.finish();
    }
}

pub fn serialize_structure_crate_input_update_vod_source_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateVodSourceInput,
) {
    if let Some(var_81) = &input.http_package_configurations {
        let mut array_82 = object.key("HttpPackageConfigurations").start_array();
        for item_83 in var_81 {
            {
                let mut object_84 = array_82.value().start_object();
                crate::json_ser::serialize_structure_crate_model_http_package_configuration(
                    &mut object_84,
                    item_83,
                );
                object_84.finish();
            }
        }
        array_82.finish();
    }
}

pub fn serialize_structure_crate_model_slate_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SlateSource,
) {
    if let Some(var_85) = &input.source_location_name {
        object.key("SourceLocationName").string(var_85);
    }
    if let Some(var_86) = &input.vod_source_name {
        object.key("VodSourceName").string(var_86);
    }
}

pub fn serialize_structure_crate_model_request_output_item(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RequestOutputItem,
) {
    if let Some(var_87) = &input.dash_playlist_settings {
        let mut object_88 = object.key("DashPlaylistSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_playlist_settings(
            &mut object_88,
            var_87,
        );
        object_88.finish();
    }
    if let Some(var_89) = &input.hls_playlist_settings {
        let mut object_90 = object.key("HlsPlaylistSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_hls_playlist_settings(
            &mut object_90,
            var_89,
        );
        object_90.finish();
    }
    if let Some(var_91) = &input.manifest_name {
        object.key("ManifestName").string(var_91);
    }
    if let Some(var_92) = &input.source_group {
        object.key("SourceGroup").string(var_92);
    }
}

pub fn serialize_structure_crate_model_ad_break(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdBreak,
) {
    if let Some(var_93) = &input.message_type {
        object.key("MessageType").string(var_93.as_str());
    }
    if input.offset_millis != 0 {
        object.key("OffsetMillis").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.offset_millis).into()),
        );
    }
    if let Some(var_94) = &input.slate {
        let mut object_95 = object.key("Slate").start_object();
        crate::json_ser::serialize_structure_crate_model_slate_source(&mut object_95, var_94);
        object_95.finish();
    }
    if let Some(var_96) = &input.splice_insert_message {
        let mut object_97 = object.key("SpliceInsertMessage").start_object();
        crate::json_ser::serialize_structure_crate_model_splice_insert_message(
            &mut object_97,
            var_96,
        );
        object_97.finish();
    }
}

pub fn serialize_structure_crate_model_schedule_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ScheduleConfiguration,
) {
    if let Some(var_98) = &input.transition {
        let mut object_99 = object.key("Transition").start_object();
        crate::json_ser::serialize_structure_crate_model_transition(&mut object_99, var_98);
        object_99.finish();
    }
}

pub fn serialize_structure_crate_model_access_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AccessConfiguration,
) {
    if let Some(var_100) = &input.access_type {
        object.key("AccessType").string(var_100.as_str());
    }
    if let Some(var_101) = &input.secrets_manager_access_token_configuration {
        let mut object_102 = object
            .key("SecretsManagerAccessTokenConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_secrets_manager_access_token_configuration(
            &mut object_102,
            var_101,
        );
        object_102.finish();
    }
}

pub fn serialize_structure_crate_model_default_segment_delivery_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DefaultSegmentDeliveryConfiguration,
) {
    if let Some(var_103) = &input.base_url {
        object.key("BaseUrl").string(var_103);
    }
}

pub fn serialize_structure_crate_model_http_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HttpConfiguration,
) {
    if let Some(var_104) = &input.base_url {
        object.key("BaseUrl").string(var_104);
    }
}

pub fn serialize_structure_crate_model_http_package_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HttpPackageConfiguration,
) {
    if let Some(var_105) = &input.path {
        object.key("Path").string(var_105);
    }
    if let Some(var_106) = &input.source_group {
        object.key("SourceGroup").string(var_106);
    }
    if let Some(var_107) = &input.r#type {
        object.key("Type").string(var_107.as_str());
    }
}

pub fn serialize_structure_crate_model_avail_suppression(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AvailSuppression,
) {
    if let Some(var_108) = &input.mode {
        object.key("Mode").string(var_108.as_str());
    }
    if let Some(var_109) = &input.value {
        object.key("Value").string(var_109);
    }
}

pub fn serialize_structure_crate_model_bumper(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Bumper,
) {
    if let Some(var_110) = &input.end_url {
        object.key("EndUrl").string(var_110);
    }
    if let Some(var_111) = &input.start_url {
        object.key("StartUrl").string(var_111);
    }
}

pub fn serialize_structure_crate_model_cdn_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CdnConfiguration,
) {
    if let Some(var_112) = &input.ad_segment_url_prefix {
        object.key("AdSegmentUrlPrefix").string(var_112);
    }
    if let Some(var_113) = &input.content_segment_url_prefix {
        object.key("ContentSegmentUrlPrefix").string(var_113);
    }
}

pub fn serialize_structure_crate_model_dash_configuration_for_put(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DashConfigurationForPut,
) {
    if let Some(var_114) = &input.mpd_location {
        object.key("MpdLocation").string(var_114);
    }
    if let Some(var_115) = &input.origin_manifest_type {
        object.key("OriginManifestType").string(var_115.as_str());
    }
}

pub fn serialize_structure_crate_model_live_pre_roll_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LivePreRollConfiguration,
) {
    if let Some(var_116) = &input.ad_decision_server_url {
        object.key("AdDecisionServerUrl").string(var_116);
    }
    if input.max_duration_seconds != 0 {
        object.key("MaxDurationSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_duration_seconds).into()),
        );
    }
}

pub fn serialize_structure_crate_model_manifest_processing_rules(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ManifestProcessingRules,
) {
    if let Some(var_117) = &input.ad_marker_passthrough {
        let mut object_118 = object.key("AdMarkerPassthrough").start_object();
        crate::json_ser::serialize_structure_crate_model_ad_marker_passthrough(
            &mut object_118,
            var_117,
        );
        object_118.finish();
    }
}

pub fn serialize_structure_crate_model_dash_playlist_settings(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DashPlaylistSettings,
) {
    if input.manifest_window_seconds != 0 {
        object.key("ManifestWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.manifest_window_seconds).into()),
        );
    }
    if input.min_buffer_time_seconds != 0 {
        object.key("MinBufferTimeSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.min_buffer_time_seconds).into()),
        );
    }
    if input.min_update_period_seconds != 0 {
        object.key("MinUpdatePeriodSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.min_update_period_seconds).into()),
        );
    }
    if input.suggested_presentation_delay_seconds != 0 {
        object.key("SuggestedPresentationDelaySeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.suggested_presentation_delay_seconds).into()),
        );
    }
}

pub fn serialize_structure_crate_model_hls_playlist_settings(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HlsPlaylistSettings,
) {
    if input.manifest_window_seconds != 0 {
        object.key("ManifestWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.manifest_window_seconds).into()),
        );
    }
}

pub fn serialize_structure_crate_model_splice_insert_message(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SpliceInsertMessage,
) {
    if input.avail_num != 0 {
        object.key("AvailNum").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.avail_num).into()),
        );
    }
    if input.avails_expected != 0 {
        object.key("AvailsExpected").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.avails_expected).into()),
        );
    }
    if input.splice_event_id != 0 {
        object.key("SpliceEventId").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.splice_event_id).into()),
        );
    }
    if input.unique_program_id != 0 {
        object.key("UniqueProgramId").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.unique_program_id).into()),
        );
    }
}

pub fn serialize_structure_crate_model_transition(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Transition,
) {
    if let Some(var_119) = &input.relative_position {
        object.key("RelativePosition").string(var_119.as_str());
    }
    if let Some(var_120) = &input.relative_program {
        object.key("RelativeProgram").string(var_120);
    }
    if input.scheduled_start_time_millis != 0 {
        object.key("ScheduledStartTimeMillis").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.scheduled_start_time_millis).into()),
        );
    }
    if let Some(var_121) = &input.r#type {
        object.key("Type").string(var_121);
    }
}

pub fn serialize_structure_crate_model_secrets_manager_access_token_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SecretsManagerAccessTokenConfiguration,
) {
    if let Some(var_122) = &input.header_name {
        object.key("HeaderName").string(var_122);
    }
    if let Some(var_123) = &input.secret_arn {
        object.key("SecretArn").string(var_123);
    }
    if let Some(var_124) = &input.secret_string_key {
        object.key("SecretStringKey").string(var_124);
    }
}

pub fn serialize_structure_crate_model_ad_marker_passthrough(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdMarkerPassthrough,
) {
    if input.enabled {
        object.key("Enabled").boolean(input.enabled);
    }
}
