// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateJobInput,
) {
    if let Some(var_1) = &input.input {
        let mut object_2 = object.key("Input").start_object();
        crate::json_ser::serialize_structure_crate_model_job_input(&mut object_2, var_1);
        object_2.finish();
    }
    if let Some(var_3) = &input.inputs {
        let mut array_4 = object.key("Inputs").start_array();
        for item_5 in var_3 {
            {
                let mut object_6 = array_4.value().start_object();
                crate::json_ser::serialize_structure_crate_model_job_input(&mut object_6, item_5);
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.output {
        let mut object_8 = object.key("Output").start_object();
        crate::json_ser::serialize_structure_crate_model_create_job_output(&mut object_8, var_7);
        object_8.finish();
    }
    if let Some(var_9) = &input.output_key_prefix {
        object.key("OutputKeyPrefix").string(var_9);
    }
    if let Some(var_10) = &input.outputs {
        let mut array_11 = object.key("Outputs").start_array();
        for item_12 in var_10 {
            {
                let mut object_13 = array_11.value().start_object();
                crate::json_ser::serialize_structure_crate_model_create_job_output(
                    &mut object_13,
                    item_12,
                );
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.pipeline_id {
        object.key("PipelineId").string(var_14);
    }
    if let Some(var_15) = &input.playlists {
        let mut array_16 = object.key("Playlists").start_array();
        for item_17 in var_15 {
            {
                let mut object_18 = array_16.value().start_object();
                crate::json_ser::serialize_structure_crate_model_create_job_playlist(
                    &mut object_18,
                    item_17,
                );
                object_18.finish();
            }
        }
        array_16.finish();
    }
    if let Some(var_19) = &input.user_metadata {
        let mut object_20 = object.key("UserMetadata").start_object();
        for (key_21, value_22) in var_19 {
            {
                object_20.key(key_21).string(value_22);
            }
        }
        object_20.finish();
    }
}

pub fn serialize_structure_crate_input_create_pipeline_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePipelineInput,
) {
    if let Some(var_23) = &input.aws_kms_key_arn {
        object.key("AwsKmsKeyArn").string(var_23);
    }
    if let Some(var_24) = &input.content_config {
        let mut object_25 = object.key("ContentConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_pipeline_output_config(
            &mut object_25,
            var_24,
        );
        object_25.finish();
    }
    if let Some(var_26) = &input.input_bucket {
        object.key("InputBucket").string(var_26);
    }
    if let Some(var_27) = &input.name {
        object.key("Name").string(var_27);
    }
    if let Some(var_28) = &input.notifications {
        let mut object_29 = object.key("Notifications").start_object();
        crate::json_ser::serialize_structure_crate_model_notifications(&mut object_29, var_28);
        object_29.finish();
    }
    if let Some(var_30) = &input.output_bucket {
        object.key("OutputBucket").string(var_30);
    }
    if let Some(var_31) = &input.role {
        object.key("Role").string(var_31);
    }
    if let Some(var_32) = &input.thumbnail_config {
        let mut object_33 = object.key("ThumbnailConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_pipeline_output_config(
            &mut object_33,
            var_32,
        );
        object_33.finish();
    }
}

pub fn serialize_structure_crate_input_create_preset_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePresetInput,
) {
    if let Some(var_34) = &input.audio {
        let mut object_35 = object.key("Audio").start_object();
        crate::json_ser::serialize_structure_crate_model_audio_parameters(&mut object_35, var_34);
        object_35.finish();
    }
    if let Some(var_36) = &input.container {
        object.key("Container").string(var_36);
    }
    if let Some(var_37) = &input.description {
        object.key("Description").string(var_37);
    }
    if let Some(var_38) = &input.name {
        object.key("Name").string(var_38);
    }
    if let Some(var_39) = &input.thumbnails {
        let mut object_40 = object.key("Thumbnails").start_object();
        crate::json_ser::serialize_structure_crate_model_thumbnails(&mut object_40, var_39);
        object_40.finish();
    }
    if let Some(var_41) = &input.video {
        let mut object_42 = object.key("Video").start_object();
        crate::json_ser::serialize_structure_crate_model_video_parameters(&mut object_42, var_41);
        object_42.finish();
    }
}

pub fn serialize_structure_crate_input_test_role_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TestRoleInput,
) {
    if let Some(var_43) = &input.input_bucket {
        object.key("InputBucket").string(var_43);
    }
    if let Some(var_44) = &input.output_bucket {
        object.key("OutputBucket").string(var_44);
    }
    if let Some(var_45) = &input.role {
        object.key("Role").string(var_45);
    }
    if let Some(var_46) = &input.topics {
        let mut array_47 = object.key("Topics").start_array();
        for item_48 in var_46 {
            {
                array_47.value().string(item_48);
            }
        }
        array_47.finish();
    }
}

pub fn serialize_structure_crate_input_update_pipeline_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePipelineInput,
) {
    if let Some(var_49) = &input.aws_kms_key_arn {
        object.key("AwsKmsKeyArn").string(var_49);
    }
    if let Some(var_50) = &input.content_config {
        let mut object_51 = object.key("ContentConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_pipeline_output_config(
            &mut object_51,
            var_50,
        );
        object_51.finish();
    }
    if let Some(var_52) = &input.input_bucket {
        object.key("InputBucket").string(var_52);
    }
    if let Some(var_53) = &input.name {
        object.key("Name").string(var_53);
    }
    if let Some(var_54) = &input.notifications {
        let mut object_55 = object.key("Notifications").start_object();
        crate::json_ser::serialize_structure_crate_model_notifications(&mut object_55, var_54);
        object_55.finish();
    }
    if let Some(var_56) = &input.role {
        object.key("Role").string(var_56);
    }
    if let Some(var_57) = &input.thumbnail_config {
        let mut object_58 = object.key("ThumbnailConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_pipeline_output_config(
            &mut object_58,
            var_57,
        );
        object_58.finish();
    }
}

pub fn serialize_structure_crate_input_update_pipeline_notifications_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePipelineNotificationsInput,
) {
    if let Some(var_59) = &input.notifications {
        let mut object_60 = object.key("Notifications").start_object();
        crate::json_ser::serialize_structure_crate_model_notifications(&mut object_60, var_59);
        object_60.finish();
    }
}

pub fn serialize_structure_crate_input_update_pipeline_status_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePipelineStatusInput,
) {
    if let Some(var_61) = &input.status {
        object.key("Status").string(var_61);
    }
}

pub fn serialize_structure_crate_model_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::JobInput,
) {
    if let Some(var_62) = &input.key {
        object.key("Key").string(var_62);
    }
    if let Some(var_63) = &input.frame_rate {
        object.key("FrameRate").string(var_63);
    }
    if let Some(var_64) = &input.resolution {
        object.key("Resolution").string(var_64);
    }
    if let Some(var_65) = &input.aspect_ratio {
        object.key("AspectRatio").string(var_65);
    }
    if let Some(var_66) = &input.interlaced {
        object.key("Interlaced").string(var_66);
    }
    if let Some(var_67) = &input.container {
        object.key("Container").string(var_67);
    }
    if let Some(var_68) = &input.encryption {
        let mut object_69 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption(&mut object_69, var_68);
        object_69.finish();
    }
    if let Some(var_70) = &input.time_span {
        let mut object_71 = object.key("TimeSpan").start_object();
        crate::json_ser::serialize_structure_crate_model_time_span(&mut object_71, var_70);
        object_71.finish();
    }
    if let Some(var_72) = &input.input_captions {
        let mut object_73 = object.key("InputCaptions").start_object();
        crate::json_ser::serialize_structure_crate_model_input_captions(&mut object_73, var_72);
        object_73.finish();
    }
    if let Some(var_74) = &input.detected_properties {
        let mut object_75 = object.key("DetectedProperties").start_object();
        crate::json_ser::serialize_structure_crate_model_detected_properties(
            &mut object_75,
            var_74,
        );
        object_75.finish();
    }
}

pub fn serialize_structure_crate_model_create_job_output(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateJobOutput,
) {
    if let Some(var_76) = &input.key {
        object.key("Key").string(var_76);
    }
    if let Some(var_77) = &input.thumbnail_pattern {
        object.key("ThumbnailPattern").string(var_77);
    }
    if let Some(var_78) = &input.thumbnail_encryption {
        let mut object_79 = object.key("ThumbnailEncryption").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption(&mut object_79, var_78);
        object_79.finish();
    }
    if let Some(var_80) = &input.rotate {
        object.key("Rotate").string(var_80);
    }
    if let Some(var_81) = &input.preset_id {
        object.key("PresetId").string(var_81);
    }
    if let Some(var_82) = &input.segment_duration {
        object.key("SegmentDuration").string(var_82);
    }
    if let Some(var_83) = &input.watermarks {
        let mut array_84 = object.key("Watermarks").start_array();
        for item_85 in var_83 {
            {
                let mut object_86 = array_84.value().start_object();
                crate::json_ser::serialize_structure_crate_model_job_watermark(
                    &mut object_86,
                    item_85,
                );
                object_86.finish();
            }
        }
        array_84.finish();
    }
    if let Some(var_87) = &input.album_art {
        let mut object_88 = object.key("AlbumArt").start_object();
        crate::json_ser::serialize_structure_crate_model_job_album_art(&mut object_88, var_87);
        object_88.finish();
    }
    if let Some(var_89) = &input.composition {
        let mut array_90 = object.key("Composition").start_array();
        for item_91 in var_89 {
            {
                let mut object_92 = array_90.value().start_object();
                crate::json_ser::serialize_structure_crate_model_clip(&mut object_92, item_91);
                object_92.finish();
            }
        }
        array_90.finish();
    }
    if let Some(var_93) = &input.captions {
        let mut object_94 = object.key("Captions").start_object();
        crate::json_ser::serialize_structure_crate_model_captions(&mut object_94, var_93);
        object_94.finish();
    }
    if let Some(var_95) = &input.encryption {
        let mut object_96 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption(&mut object_96, var_95);
        object_96.finish();
    }
}

pub fn serialize_structure_crate_model_create_job_playlist(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateJobPlaylist,
) {
    if let Some(var_97) = &input.name {
        object.key("Name").string(var_97);
    }
    if let Some(var_98) = &input.format {
        object.key("Format").string(var_98);
    }
    if let Some(var_99) = &input.output_keys {
        let mut array_100 = object.key("OutputKeys").start_array();
        for item_101 in var_99 {
            {
                array_100.value().string(item_101);
            }
        }
        array_100.finish();
    }
    if let Some(var_102) = &input.hls_content_protection {
        let mut object_103 = object.key("HlsContentProtection").start_object();
        crate::json_ser::serialize_structure_crate_model_hls_content_protection(
            &mut object_103,
            var_102,
        );
        object_103.finish();
    }
    if let Some(var_104) = &input.play_ready_drm {
        let mut object_105 = object.key("PlayReadyDrm").start_object();
        crate::json_ser::serialize_structure_crate_model_play_ready_drm(&mut object_105, var_104);
        object_105.finish();
    }
}

pub fn serialize_structure_crate_model_pipeline_output_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PipelineOutputConfig,
) {
    if let Some(var_106) = &input.bucket {
        object.key("Bucket").string(var_106);
    }
    if let Some(var_107) = &input.storage_class {
        object.key("StorageClass").string(var_107);
    }
    if let Some(var_108) = &input.permissions {
        let mut array_109 = object.key("Permissions").start_array();
        for item_110 in var_108 {
            {
                let mut object_111 = array_109.value().start_object();
                crate::json_ser::serialize_structure_crate_model_permission(
                    &mut object_111,
                    item_110,
                );
                object_111.finish();
            }
        }
        array_109.finish();
    }
}

pub fn serialize_structure_crate_model_notifications(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Notifications,
) {
    if let Some(var_112) = &input.progressing {
        object.key("Progressing").string(var_112);
    }
    if let Some(var_113) = &input.completed {
        object.key("Completed").string(var_113);
    }
    if let Some(var_114) = &input.warning {
        object.key("Warning").string(var_114);
    }
    if let Some(var_115) = &input.error {
        object.key("Error").string(var_115);
    }
}

pub fn serialize_structure_crate_model_audio_parameters(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AudioParameters,
) {
    if let Some(var_116) = &input.codec {
        object.key("Codec").string(var_116);
    }
    if let Some(var_117) = &input.sample_rate {
        object.key("SampleRate").string(var_117);
    }
    if let Some(var_118) = &input.bit_rate {
        object.key("BitRate").string(var_118);
    }
    if let Some(var_119) = &input.channels {
        object.key("Channels").string(var_119);
    }
    if let Some(var_120) = &input.audio_packing_mode {
        object.key("AudioPackingMode").string(var_120);
    }
    if let Some(var_121) = &input.codec_options {
        let mut object_122 = object.key("CodecOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_audio_codec_options(
            &mut object_122,
            var_121,
        );
        object_122.finish();
    }
}

pub fn serialize_structure_crate_model_thumbnails(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Thumbnails,
) {
    if let Some(var_123) = &input.format {
        object.key("Format").string(var_123);
    }
    if let Some(var_124) = &input.interval {
        object.key("Interval").string(var_124);
    }
    if let Some(var_125) = &input.resolution {
        object.key("Resolution").string(var_125);
    }
    if let Some(var_126) = &input.aspect_ratio {
        object.key("AspectRatio").string(var_126);
    }
    if let Some(var_127) = &input.max_width {
        object.key("MaxWidth").string(var_127);
    }
    if let Some(var_128) = &input.max_height {
        object.key("MaxHeight").string(var_128);
    }
    if let Some(var_129) = &input.sizing_policy {
        object.key("SizingPolicy").string(var_129);
    }
    if let Some(var_130) = &input.padding_policy {
        object.key("PaddingPolicy").string(var_130);
    }
}

pub fn serialize_structure_crate_model_video_parameters(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VideoParameters,
) {
    if let Some(var_131) = &input.codec {
        object.key("Codec").string(var_131);
    }
    if let Some(var_132) = &input.codec_options {
        let mut object_133 = object.key("CodecOptions").start_object();
        for (key_134, value_135) in var_132 {
            {
                object_133.key(key_134).string(value_135);
            }
        }
        object_133.finish();
    }
    if let Some(var_136) = &input.keyframes_max_dist {
        object.key("KeyframesMaxDist").string(var_136);
    }
    if let Some(var_137) = &input.fixed_gop {
        object.key("FixedGOP").string(var_137);
    }
    if let Some(var_138) = &input.bit_rate {
        object.key("BitRate").string(var_138);
    }
    if let Some(var_139) = &input.frame_rate {
        object.key("FrameRate").string(var_139);
    }
    if let Some(var_140) = &input.max_frame_rate {
        object.key("MaxFrameRate").string(var_140);
    }
    if let Some(var_141) = &input.resolution {
        object.key("Resolution").string(var_141);
    }
    if let Some(var_142) = &input.aspect_ratio {
        object.key("AspectRatio").string(var_142);
    }
    if let Some(var_143) = &input.max_width {
        object.key("MaxWidth").string(var_143);
    }
    if let Some(var_144) = &input.max_height {
        object.key("MaxHeight").string(var_144);
    }
    if let Some(var_145) = &input.display_aspect_ratio {
        object.key("DisplayAspectRatio").string(var_145);
    }
    if let Some(var_146) = &input.sizing_policy {
        object.key("SizingPolicy").string(var_146);
    }
    if let Some(var_147) = &input.padding_policy {
        object.key("PaddingPolicy").string(var_147);
    }
    if let Some(var_148) = &input.watermarks {
        let mut array_149 = object.key("Watermarks").start_array();
        for item_150 in var_148 {
            {
                let mut object_151 = array_149.value().start_object();
                crate::json_ser::serialize_structure_crate_model_preset_watermark(
                    &mut object_151,
                    item_150,
                );
                object_151.finish();
            }
        }
        array_149.finish();
    }
}

pub fn serialize_structure_crate_model_encryption(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Encryption,
) {
    if let Some(var_152) = &input.mode {
        object.key("Mode").string(var_152);
    }
    if let Some(var_153) = &input.key {
        object.key("Key").string(var_153);
    }
    if let Some(var_154) = &input.key_md5 {
        object.key("KeyMd5").string(var_154);
    }
    if let Some(var_155) = &input.initialization_vector {
        object.key("InitializationVector").string(var_155);
    }
}

pub fn serialize_structure_crate_model_time_span(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TimeSpan,
) {
    if let Some(var_156) = &input.start_time {
        object.key("StartTime").string(var_156);
    }
    if let Some(var_157) = &input.duration {
        object.key("Duration").string(var_157);
    }
}

pub fn serialize_structure_crate_model_input_captions(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputCaptions,
) {
    if let Some(var_158) = &input.merge_policy {
        object.key("MergePolicy").string(var_158);
    }
    if let Some(var_159) = &input.caption_sources {
        let mut array_160 = object.key("CaptionSources").start_array();
        for item_161 in var_159 {
            {
                let mut object_162 = array_160.value().start_object();
                crate::json_ser::serialize_structure_crate_model_caption_source(
                    &mut object_162,
                    item_161,
                );
                object_162.finish();
            }
        }
        array_160.finish();
    }
}

pub fn serialize_structure_crate_model_detected_properties(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DetectedProperties,
) {
    if let Some(var_163) = &input.width {
        object.key("Width").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_163).into()),
        );
    }
    if let Some(var_164) = &input.height {
        object.key("Height").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_164).into()),
        );
    }
    if let Some(var_165) = &input.frame_rate {
        object.key("FrameRate").string(var_165);
    }
    if let Some(var_166) = &input.file_size {
        object.key("FileSize").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_166).into()),
        );
    }
    if let Some(var_167) = &input.duration_millis {
        object.key("DurationMillis").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_167).into()),
        );
    }
}

pub fn serialize_structure_crate_model_job_watermark(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::JobWatermark,
) {
    if let Some(var_168) = &input.preset_watermark_id {
        object.key("PresetWatermarkId").string(var_168);
    }
    if let Some(var_169) = &input.input_key {
        object.key("InputKey").string(var_169);
    }
    if let Some(var_170) = &input.encryption {
        let mut object_171 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption(&mut object_171, var_170);
        object_171.finish();
    }
}

pub fn serialize_structure_crate_model_job_album_art(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::JobAlbumArt,
) {
    if let Some(var_172) = &input.merge_policy {
        object.key("MergePolicy").string(var_172);
    }
    if let Some(var_173) = &input.artwork {
        let mut array_174 = object.key("Artwork").start_array();
        for item_175 in var_173 {
            {
                let mut object_176 = array_174.value().start_object();
                crate::json_ser::serialize_structure_crate_model_artwork(&mut object_176, item_175);
                object_176.finish();
            }
        }
        array_174.finish();
    }
}

pub fn serialize_structure_crate_model_clip(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Clip,
) {
    if let Some(var_177) = &input.time_span {
        let mut object_178 = object.key("TimeSpan").start_object();
        crate::json_ser::serialize_structure_crate_model_time_span(&mut object_178, var_177);
        object_178.finish();
    }
}

pub fn serialize_structure_crate_model_captions(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Captions,
) {
    if let Some(var_179) = &input.merge_policy {
        object.key("MergePolicy").string(var_179);
    }
    if let Some(var_180) = &input.caption_sources {
        let mut array_181 = object.key("CaptionSources").start_array();
        for item_182 in var_180 {
            {
                let mut object_183 = array_181.value().start_object();
                crate::json_ser::serialize_structure_crate_model_caption_source(
                    &mut object_183,
                    item_182,
                );
                object_183.finish();
            }
        }
        array_181.finish();
    }
    if let Some(var_184) = &input.caption_formats {
        let mut array_185 = object.key("CaptionFormats").start_array();
        for item_186 in var_184 {
            {
                let mut object_187 = array_185.value().start_object();
                crate::json_ser::serialize_structure_crate_model_caption_format(
                    &mut object_187,
                    item_186,
                );
                object_187.finish();
            }
        }
        array_185.finish();
    }
}

pub fn serialize_structure_crate_model_hls_content_protection(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HlsContentProtection,
) {
    if let Some(var_188) = &input.method {
        object.key("Method").string(var_188);
    }
    if let Some(var_189) = &input.key {
        object.key("Key").string(var_189);
    }
    if let Some(var_190) = &input.key_md5 {
        object.key("KeyMd5").string(var_190);
    }
    if let Some(var_191) = &input.initialization_vector {
        object.key("InitializationVector").string(var_191);
    }
    if let Some(var_192) = &input.license_acquisition_url {
        object.key("LicenseAcquisitionUrl").string(var_192);
    }
    if let Some(var_193) = &input.key_storage_policy {
        object.key("KeyStoragePolicy").string(var_193);
    }
}

pub fn serialize_structure_crate_model_play_ready_drm(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PlayReadyDrm,
) {
    if let Some(var_194) = &input.format {
        object.key("Format").string(var_194);
    }
    if let Some(var_195) = &input.key {
        object.key("Key").string(var_195);
    }
    if let Some(var_196) = &input.key_md5 {
        object.key("KeyMd5").string(var_196);
    }
    if let Some(var_197) = &input.key_id {
        object.key("KeyId").string(var_197);
    }
    if let Some(var_198) = &input.initialization_vector {
        object.key("InitializationVector").string(var_198);
    }
    if let Some(var_199) = &input.license_acquisition_url {
        object.key("LicenseAcquisitionUrl").string(var_199);
    }
}

pub fn serialize_structure_crate_model_permission(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Permission,
) {
    if let Some(var_200) = &input.grantee_type {
        object.key("GranteeType").string(var_200);
    }
    if let Some(var_201) = &input.grantee {
        object.key("Grantee").string(var_201);
    }
    if let Some(var_202) = &input.access {
        let mut array_203 = object.key("Access").start_array();
        for item_204 in var_202 {
            {
                array_203.value().string(item_204);
            }
        }
        array_203.finish();
    }
}

pub fn serialize_structure_crate_model_audio_codec_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AudioCodecOptions,
) {
    if let Some(var_205) = &input.profile {
        object.key("Profile").string(var_205);
    }
    if let Some(var_206) = &input.bit_depth {
        object.key("BitDepth").string(var_206);
    }
    if let Some(var_207) = &input.bit_order {
        object.key("BitOrder").string(var_207);
    }
    if let Some(var_208) = &input.signed {
        object.key("Signed").string(var_208);
    }
}

pub fn serialize_structure_crate_model_preset_watermark(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PresetWatermark,
) {
    if let Some(var_209) = &input.id {
        object.key("Id").string(var_209);
    }
    if let Some(var_210) = &input.max_width {
        object.key("MaxWidth").string(var_210);
    }
    if let Some(var_211) = &input.max_height {
        object.key("MaxHeight").string(var_211);
    }
    if let Some(var_212) = &input.sizing_policy {
        object.key("SizingPolicy").string(var_212);
    }
    if let Some(var_213) = &input.horizontal_align {
        object.key("HorizontalAlign").string(var_213);
    }
    if let Some(var_214) = &input.horizontal_offset {
        object.key("HorizontalOffset").string(var_214);
    }
    if let Some(var_215) = &input.vertical_align {
        object.key("VerticalAlign").string(var_215);
    }
    if let Some(var_216) = &input.vertical_offset {
        object.key("VerticalOffset").string(var_216);
    }
    if let Some(var_217) = &input.opacity {
        object.key("Opacity").string(var_217);
    }
    if let Some(var_218) = &input.target {
        object.key("Target").string(var_218);
    }
}

pub fn serialize_structure_crate_model_caption_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CaptionSource,
) {
    if let Some(var_219) = &input.key {
        object.key("Key").string(var_219);
    }
    if let Some(var_220) = &input.language {
        object.key("Language").string(var_220);
    }
    if let Some(var_221) = &input.time_offset {
        object.key("TimeOffset").string(var_221);
    }
    if let Some(var_222) = &input.label {
        object.key("Label").string(var_222);
    }
    if let Some(var_223) = &input.encryption {
        let mut object_224 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption(&mut object_224, var_223);
        object_224.finish();
    }
}

pub fn serialize_structure_crate_model_artwork(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Artwork,
) {
    if let Some(var_225) = &input.input_key {
        object.key("InputKey").string(var_225);
    }
    if let Some(var_226) = &input.max_width {
        object.key("MaxWidth").string(var_226);
    }
    if let Some(var_227) = &input.max_height {
        object.key("MaxHeight").string(var_227);
    }
    if let Some(var_228) = &input.sizing_policy {
        object.key("SizingPolicy").string(var_228);
    }
    if let Some(var_229) = &input.padding_policy {
        object.key("PaddingPolicy").string(var_229);
    }
    if let Some(var_230) = &input.album_art_format {
        object.key("AlbumArtFormat").string(var_230);
    }
    if let Some(var_231) = &input.encryption {
        let mut object_232 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption(&mut object_232, var_231);
        object_232.finish();
    }
}

pub fn serialize_structure_crate_model_caption_format(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CaptionFormat,
) {
    if let Some(var_233) = &input.format {
        object.key("Format").string(var_233);
    }
    if let Some(var_234) = &input.pattern {
        object.key("Pattern").string(var_234);
    }
    if let Some(var_235) = &input.encryption {
        let mut object_236 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption(&mut object_236, var_235);
        object_236.finish();
    }
}
