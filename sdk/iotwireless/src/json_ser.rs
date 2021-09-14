// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_aws_account_with_partner_account_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateAwsAccountWithPartnerAccountInput,
) {
    if let Some(var_1) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_1);
    }
    if let Some(var_2) = &input.sidewalk {
        let mut object_3 = object.key("Sidewalk").start_object();
        crate::json_ser::serialize_structure_crate_model_sidewalk_account_info(
            &mut object_3,
            var_2,
        );
        object_3.finish();
    }
    if let Some(var_4) = &input.tags {
        let mut array_5 = object.key("Tags").start_array();
        for item_6 in var_4 {
            {
                let mut object_7 = array_5.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_7, item_6);
                object_7.finish();
            }
        }
        array_5.finish();
    }
}

pub fn serialize_structure_crate_input_associate_wireless_device_with_thing_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateWirelessDeviceWithThingInput,
) {
    if let Some(var_8) = &input.thing_arn {
        object.key("ThingArn").string(var_8);
    }
}

pub fn serialize_structure_crate_input_associate_wireless_gateway_with_certificate_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateWirelessGatewayWithCertificateInput,
) {
    if let Some(var_9) = &input.iot_certificate_id {
        object.key("IotCertificateId").string(var_9);
    }
}

pub fn serialize_structure_crate_input_associate_wireless_gateway_with_thing_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateWirelessGatewayWithThingInput,
) {
    if let Some(var_10) = &input.thing_arn {
        object.key("ThingArn").string(var_10);
    }
}

pub fn serialize_structure_crate_input_create_destination_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDestinationInput,
) {
    if let Some(var_11) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_11);
    }
    if let Some(var_12) = &input.description {
        object.key("Description").string(var_12);
    }
    if let Some(var_13) = &input.expression {
        object.key("Expression").string(var_13);
    }
    if let Some(var_14) = &input.expression_type {
        object.key("ExpressionType").string(var_14.as_str());
    }
    if let Some(var_15) = &input.name {
        object.key("Name").string(var_15);
    }
    if let Some(var_16) = &input.role_arn {
        object.key("RoleArn").string(var_16);
    }
    if let Some(var_17) = &input.tags {
        let mut array_18 = object.key("Tags").start_array();
        for item_19 in var_17 {
            {
                let mut object_20 = array_18.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_20, item_19);
                object_20.finish();
            }
        }
        array_18.finish();
    }
}

pub fn serialize_structure_crate_input_create_device_profile_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDeviceProfileInput,
) {
    if let Some(var_21) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_21);
    }
    if let Some(var_22) = &input.lo_ra_wan {
        let mut object_23 = object.key("LoRaWAN").start_object();
        crate::json_ser::serialize_structure_crate_model_lo_ra_wan_device_profile(
            &mut object_23,
            var_22,
        );
        object_23.finish();
    }
    if let Some(var_24) = &input.name {
        object.key("Name").string(var_24);
    }
    if let Some(var_25) = &input.tags {
        let mut array_26 = object.key("Tags").start_array();
        for item_27 in var_25 {
            {
                let mut object_28 = array_26.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_28, item_27);
                object_28.finish();
            }
        }
        array_26.finish();
    }
}

pub fn serialize_structure_crate_input_create_service_profile_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateServiceProfileInput,
) {
    if let Some(var_29) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_29);
    }
    if let Some(var_30) = &input.lo_ra_wan {
        let mut object_31 = object.key("LoRaWAN").start_object();
        crate::json_ser::serialize_structure_crate_model_lo_ra_wan_service_profile(
            &mut object_31,
            var_30,
        );
        object_31.finish();
    }
    if let Some(var_32) = &input.name {
        object.key("Name").string(var_32);
    }
    if let Some(var_33) = &input.tags {
        let mut array_34 = object.key("Tags").start_array();
        for item_35 in var_33 {
            {
                let mut object_36 = array_34.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_36, item_35);
                object_36.finish();
            }
        }
        array_34.finish();
    }
}

pub fn serialize_structure_crate_input_create_wireless_device_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateWirelessDeviceInput,
) {
    if let Some(var_37) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_37);
    }
    if let Some(var_38) = &input.description {
        object.key("Description").string(var_38);
    }
    if let Some(var_39) = &input.destination_name {
        object.key("DestinationName").string(var_39);
    }
    if let Some(var_40) = &input.lo_ra_wan {
        let mut object_41 = object.key("LoRaWAN").start_object();
        crate::json_ser::serialize_structure_crate_model_lo_ra_wan_device(&mut object_41, var_40);
        object_41.finish();
    }
    if let Some(var_42) = &input.name {
        object.key("Name").string(var_42);
    }
    if let Some(var_43) = &input.tags {
        let mut array_44 = object.key("Tags").start_array();
        for item_45 in var_43 {
            {
                let mut object_46 = array_44.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_46, item_45);
                object_46.finish();
            }
        }
        array_44.finish();
    }
    if let Some(var_47) = &input.r#type {
        object.key("Type").string(var_47.as_str());
    }
}

pub fn serialize_structure_crate_input_create_wireless_gateway_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateWirelessGatewayInput,
) {
    if let Some(var_48) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_48);
    }
    if let Some(var_49) = &input.description {
        object.key("Description").string(var_49);
    }
    if let Some(var_50) = &input.lo_ra_wan {
        let mut object_51 = object.key("LoRaWAN").start_object();
        crate::json_ser::serialize_structure_crate_model_lo_ra_wan_gateway(&mut object_51, var_50);
        object_51.finish();
    }
    if let Some(var_52) = &input.name {
        object.key("Name").string(var_52);
    }
    if let Some(var_53) = &input.tags {
        let mut array_54 = object.key("Tags").start_array();
        for item_55 in var_53 {
            {
                let mut object_56 = array_54.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_56, item_55);
                object_56.finish();
            }
        }
        array_54.finish();
    }
}

pub fn serialize_structure_crate_input_create_wireless_gateway_task_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateWirelessGatewayTaskInput,
) {
    if let Some(var_57) = &input.wireless_gateway_task_definition_id {
        object.key("WirelessGatewayTaskDefinitionId").string(var_57);
    }
}

pub fn serialize_structure_crate_input_create_wireless_gateway_task_definition_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateWirelessGatewayTaskDefinitionInput,
) {
    {
        object
            .key("AutoCreateTasks")
            .boolean(input.auto_create_tasks);
    }
    if let Some(var_58) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_58);
    }
    if let Some(var_59) = &input.name {
        object.key("Name").string(var_59);
    }
    if let Some(var_60) = &input.tags {
        let mut array_61 = object.key("Tags").start_array();
        for item_62 in var_60 {
            {
                let mut object_63 = array_61.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_63, item_62);
                object_63.finish();
            }
        }
        array_61.finish();
    }
    if let Some(var_64) = &input.update {
        let mut object_65 = object.key("Update").start_object();
        crate::json_ser::serialize_structure_crate_model_update_wireless_gateway_task_create(
            &mut object_65,
            var_64,
        );
        object_65.finish();
    }
}

pub fn serialize_structure_crate_input_put_resource_log_level_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutResourceLogLevelInput,
) {
    if let Some(var_66) = &input.log_level {
        object.key("LogLevel").string(var_66.as_str());
    }
}

pub fn serialize_structure_crate_input_send_data_to_wireless_device_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SendDataToWirelessDeviceInput,
) {
    if let Some(var_67) = &input.payload_data {
        object.key("PayloadData").string(var_67);
    }
    if let Some(var_68) = &input.transmit_mode {
        object.key("TransmitMode").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_68).into()),
        );
    }
    if let Some(var_69) = &input.wireless_metadata {
        let mut object_70 = object.key("WirelessMetadata").start_object();
        crate::json_ser::serialize_structure_crate_model_wireless_metadata(&mut object_70, var_69);
        object_70.finish();
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_71) = &input.tags {
        let mut array_72 = object.key("Tags").start_array();
        for item_73 in var_71 {
            {
                let mut object_74 = array_72.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_74, item_73);
                object_74.finish();
            }
        }
        array_72.finish();
    }
}

pub fn serialize_structure_crate_input_update_destination_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDestinationInput,
) {
    if let Some(var_75) = &input.description {
        object.key("Description").string(var_75);
    }
    if let Some(var_76) = &input.expression {
        object.key("Expression").string(var_76);
    }
    if let Some(var_77) = &input.expression_type {
        object.key("ExpressionType").string(var_77.as_str());
    }
    if let Some(var_78) = &input.role_arn {
        object.key("RoleArn").string(var_78);
    }
}

pub fn serialize_structure_crate_input_update_log_levels_by_resource_types_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLogLevelsByResourceTypesInput,
) {
    if let Some(var_79) = &input.default_log_level {
        object.key("DefaultLogLevel").string(var_79.as_str());
    }
    if let Some(var_80) = &input.wireless_device_log_options {
        let mut array_81 = object.key("WirelessDeviceLogOptions").start_array();
        for item_82 in var_80 {
            {
                let mut object_83 = array_81.value().start_object();
                crate::json_ser::serialize_structure_crate_model_wireless_device_log_option(
                    &mut object_83,
                    item_82,
                );
                object_83.finish();
            }
        }
        array_81.finish();
    }
    if let Some(var_84) = &input.wireless_gateway_log_options {
        let mut array_85 = object.key("WirelessGatewayLogOptions").start_array();
        for item_86 in var_84 {
            {
                let mut object_87 = array_85.value().start_object();
                crate::json_ser::serialize_structure_crate_model_wireless_gateway_log_option(
                    &mut object_87,
                    item_86,
                );
                object_87.finish();
            }
        }
        array_85.finish();
    }
}

pub fn serialize_structure_crate_input_update_partner_account_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePartnerAccountInput,
) {
    if let Some(var_88) = &input.sidewalk {
        let mut object_89 = object.key("Sidewalk").start_object();
        crate::json_ser::serialize_structure_crate_model_sidewalk_update_account(
            &mut object_89,
            var_88,
        );
        object_89.finish();
    }
}

pub fn serialize_structure_crate_input_update_wireless_device_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateWirelessDeviceInput,
) {
    if let Some(var_90) = &input.description {
        object.key("Description").string(var_90);
    }
    if let Some(var_91) = &input.destination_name {
        object.key("DestinationName").string(var_91);
    }
    if let Some(var_92) = &input.lo_ra_wan {
        let mut object_93 = object.key("LoRaWAN").start_object();
        crate::json_ser::serialize_structure_crate_model_lo_ra_wan_update_device(
            &mut object_93,
            var_92,
        );
        object_93.finish();
    }
    if let Some(var_94) = &input.name {
        object.key("Name").string(var_94);
    }
}

pub fn serialize_structure_crate_input_update_wireless_gateway_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateWirelessGatewayInput,
) {
    if let Some(var_95) = &input.description {
        object.key("Description").string(var_95);
    }
    if let Some(var_96) = &input.join_eui_filters {
        let mut array_97 = object.key("JoinEuiFilters").start_array();
        for item_98 in var_96 {
            {
                let mut array_99 = array_97.value().start_array();
                for item_100 in item_98 {
                    {
                        array_99.value().string(item_100);
                    }
                }
                array_99.finish();
            }
        }
        array_97.finish();
    }
    if let Some(var_101) = &input.name {
        object.key("Name").string(var_101);
    }
    if let Some(var_102) = &input.net_id_filters {
        let mut array_103 = object.key("NetIdFilters").start_array();
        for item_104 in var_102 {
            {
                array_103.value().string(item_104);
            }
        }
        array_103.finish();
    }
}

pub fn serialize_structure_crate_model_sidewalk_account_info(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SidewalkAccountInfo,
) {
    if let Some(var_105) = &input.amazon_id {
        object.key("AmazonId").string(var_105);
    }
    if let Some(var_106) = &input.app_server_private_key {
        object.key("AppServerPrivateKey").string(var_106);
    }
}

pub fn serialize_structure_crate_model_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_107) = &input.key {
        object.key("Key").string(var_107);
    }
    if let Some(var_108) = &input.value {
        object.key("Value").string(var_108);
    }
}

pub fn serialize_structure_crate_model_lo_ra_wan_device_profile(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LoRaWanDeviceProfile,
) {
    if input.supports_class_b {
        object.key("SupportsClassB").boolean(input.supports_class_b);
    }
    if let Some(var_109) = &input.class_b_timeout {
        object.key("ClassBTimeout").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_109).into()),
        );
    }
    if let Some(var_110) = &input.ping_slot_period {
        object.key("PingSlotPeriod").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_110).into()),
        );
    }
    if let Some(var_111) = &input.ping_slot_dr {
        object.key("PingSlotDr").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_111).into()),
        );
    }
    if let Some(var_112) = &input.ping_slot_freq {
        object.key("PingSlotFreq").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_112).into()),
        );
    }
    if input.supports_class_c {
        object.key("SupportsClassC").boolean(input.supports_class_c);
    }
    if let Some(var_113) = &input.class_c_timeout {
        object.key("ClassCTimeout").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_113).into()),
        );
    }
    if let Some(var_114) = &input.mac_version {
        object.key("MacVersion").string(var_114);
    }
    if let Some(var_115) = &input.reg_params_revision {
        object.key("RegParamsRevision").string(var_115);
    }
    if let Some(var_116) = &input.rx_delay1 {
        object.key("RxDelay1").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_116).into()),
        );
    }
    if let Some(var_117) = &input.rx_dr_offset1 {
        object.key("RxDrOffset1").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_117).into()),
        );
    }
    if let Some(var_118) = &input.rx_data_rate2 {
        object.key("RxDataRate2").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_118).into()),
        );
    }
    if let Some(var_119) = &input.rx_freq2 {
        object.key("RxFreq2").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_119).into()),
        );
    }
    if let Some(var_120) = &input.factory_preset_freqs_list {
        let mut array_121 = object.key("FactoryPresetFreqsList").start_array();
        for item_122 in var_120 {
            {
                array_121.value().number(
                    #[allow(clippy::useless_conversion)]
                    smithy_types::Number::NegInt((*item_122).into()),
                );
            }
        }
        array_121.finish();
    }
    if let Some(var_123) = &input.max_eirp {
        object.key("MaxEirp").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_123).into()),
        );
    }
    if let Some(var_124) = &input.max_duty_cycle {
        object.key("MaxDutyCycle").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_124).into()),
        );
    }
    if let Some(var_125) = &input.rf_region {
        object.key("RfRegion").string(var_125);
    }
    if let Some(var_126) = &input.supports_join {
        object.key("SupportsJoin").boolean(*var_126);
    }
    if input.supports32_bit_f_cnt {
        object
            .key("Supports32BitFCnt")
            .boolean(input.supports32_bit_f_cnt);
    }
}

pub fn serialize_structure_crate_model_lo_ra_wan_service_profile(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LoRaWanServiceProfile,
) {
    if input.add_gw_metadata {
        object.key("AddGwMetadata").boolean(input.add_gw_metadata);
    }
}

pub fn serialize_structure_crate_model_lo_ra_wan_device(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LoRaWanDevice,
) {
    if let Some(var_127) = &input.dev_eui {
        object.key("DevEui").string(var_127);
    }
    if let Some(var_128) = &input.device_profile_id {
        object.key("DeviceProfileId").string(var_128);
    }
    if let Some(var_129) = &input.service_profile_id {
        object.key("ServiceProfileId").string(var_129);
    }
    if let Some(var_130) = &input.otaa_v1_1 {
        let mut object_131 = object.key("OtaaV1_1").start_object();
        crate::json_ser::serialize_structure_crate_model_otaa_v11(&mut object_131, var_130);
        object_131.finish();
    }
    if let Some(var_132) = &input.otaa_v1_0_x {
        let mut object_133 = object.key("OtaaV1_0_x").start_object();
        crate::json_ser::serialize_structure_crate_model_otaa_v10_x(&mut object_133, var_132);
        object_133.finish();
    }
    if let Some(var_134) = &input.abp_v1_1 {
        let mut object_135 = object.key("AbpV1_1").start_object();
        crate::json_ser::serialize_structure_crate_model_abp_v11(&mut object_135, var_134);
        object_135.finish();
    }
    if let Some(var_136) = &input.abp_v1_0_x {
        let mut object_137 = object.key("AbpV1_0_x").start_object();
        crate::json_ser::serialize_structure_crate_model_abp_v10_x(&mut object_137, var_136);
        object_137.finish();
    }
}

pub fn serialize_structure_crate_model_lo_ra_wan_gateway(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LoRaWanGateway,
) {
    if let Some(var_138) = &input.gateway_eui {
        object.key("GatewayEui").string(var_138);
    }
    if let Some(var_139) = &input.rf_region {
        object.key("RfRegion").string(var_139);
    }
    if let Some(var_140) = &input.join_eui_filters {
        let mut array_141 = object.key("JoinEuiFilters").start_array();
        for item_142 in var_140 {
            {
                let mut array_143 = array_141.value().start_array();
                for item_144 in item_142 {
                    {
                        array_143.value().string(item_144);
                    }
                }
                array_143.finish();
            }
        }
        array_141.finish();
    }
    if let Some(var_145) = &input.net_id_filters {
        let mut array_146 = object.key("NetIdFilters").start_array();
        for item_147 in var_145 {
            {
                array_146.value().string(item_147);
            }
        }
        array_146.finish();
    }
    if let Some(var_148) = &input.sub_bands {
        let mut array_149 = object.key("SubBands").start_array();
        for item_150 in var_148 {
            {
                array_149.value().number(
                    #[allow(clippy::useless_conversion)]
                    smithy_types::Number::NegInt((*item_150).into()),
                );
            }
        }
        array_149.finish();
    }
}

pub fn serialize_structure_crate_model_update_wireless_gateway_task_create(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateWirelessGatewayTaskCreate,
) {
    if let Some(var_151) = &input.update_data_source {
        object.key("UpdateDataSource").string(var_151);
    }
    if let Some(var_152) = &input.update_data_role {
        object.key("UpdateDataRole").string(var_152);
    }
    if let Some(var_153) = &input.lo_ra_wan {
        let mut object_154 = object.key("LoRaWAN").start_object();
        crate::json_ser::serialize_structure_crate_model_lo_ra_wan_update_gateway_task_create(
            &mut object_154,
            var_153,
        );
        object_154.finish();
    }
}

pub fn serialize_structure_crate_model_wireless_metadata(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::WirelessMetadata,
) {
    if let Some(var_155) = &input.lo_ra_wan {
        let mut object_156 = object.key("LoRaWAN").start_object();
        crate::json_ser::serialize_structure_crate_model_lo_ra_wan_send_data_to_device(
            &mut object_156,
            var_155,
        );
        object_156.finish();
    }
    if let Some(var_157) = &input.sidewalk {
        let mut object_158 = object.key("Sidewalk").start_object();
        crate::json_ser::serialize_structure_crate_model_sidewalk_send_data_to_device(
            &mut object_158,
            var_157,
        );
        object_158.finish();
    }
}

pub fn serialize_structure_crate_model_wireless_device_log_option(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::WirelessDeviceLogOption,
) {
    if let Some(var_159) = &input.r#type {
        object.key("Type").string(var_159.as_str());
    }
    if let Some(var_160) = &input.log_level {
        object.key("LogLevel").string(var_160.as_str());
    }
    if let Some(var_161) = &input.events {
        let mut array_162 = object.key("Events").start_array();
        for item_163 in var_161 {
            {
                let mut object_164 = array_162.value().start_object();
                crate::json_ser::serialize_structure_crate_model_wireless_device_event_log_option(
                    &mut object_164,
                    item_163,
                );
                object_164.finish();
            }
        }
        array_162.finish();
    }
}

pub fn serialize_structure_crate_model_wireless_gateway_log_option(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::WirelessGatewayLogOption,
) {
    if let Some(var_165) = &input.r#type {
        object.key("Type").string(var_165.as_str());
    }
    if let Some(var_166) = &input.log_level {
        object.key("LogLevel").string(var_166.as_str());
    }
    if let Some(var_167) = &input.events {
        let mut array_168 = object.key("Events").start_array();
        for item_169 in var_167 {
            {
                let mut object_170 = array_168.value().start_object();
                crate::json_ser::serialize_structure_crate_model_wireless_gateway_event_log_option(
                    &mut object_170,
                    item_169,
                );
                object_170.finish();
            }
        }
        array_168.finish();
    }
}

pub fn serialize_structure_crate_model_sidewalk_update_account(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SidewalkUpdateAccount,
) {
    if let Some(var_171) = &input.app_server_private_key {
        object.key("AppServerPrivateKey").string(var_171);
    }
}

pub fn serialize_structure_crate_model_lo_ra_wan_update_device(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LoRaWanUpdateDevice,
) {
    if let Some(var_172) = &input.device_profile_id {
        object.key("DeviceProfileId").string(var_172);
    }
    if let Some(var_173) = &input.service_profile_id {
        object.key("ServiceProfileId").string(var_173);
    }
}

pub fn serialize_structure_crate_model_otaa_v11(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OtaaV11,
) {
    if let Some(var_174) = &input.app_key {
        object.key("AppKey").string(var_174);
    }
    if let Some(var_175) = &input.nwk_key {
        object.key("NwkKey").string(var_175);
    }
    if let Some(var_176) = &input.join_eui {
        object.key("JoinEui").string(var_176);
    }
}

pub fn serialize_structure_crate_model_otaa_v10_x(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OtaaV10X,
) {
    if let Some(var_177) = &input.app_key {
        object.key("AppKey").string(var_177);
    }
    if let Some(var_178) = &input.app_eui {
        object.key("AppEui").string(var_178);
    }
}

pub fn serialize_structure_crate_model_abp_v11(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AbpV11,
) {
    if let Some(var_179) = &input.dev_addr {
        object.key("DevAddr").string(var_179);
    }
    if let Some(var_180) = &input.session_keys {
        let mut object_181 = object.key("SessionKeys").start_object();
        crate::json_ser::serialize_structure_crate_model_session_keys_abp_v11(
            &mut object_181,
            var_180,
        );
        object_181.finish();
    }
}

pub fn serialize_structure_crate_model_abp_v10_x(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AbpV10X,
) {
    if let Some(var_182) = &input.dev_addr {
        object.key("DevAddr").string(var_182);
    }
    if let Some(var_183) = &input.session_keys {
        let mut object_184 = object.key("SessionKeys").start_object();
        crate::json_ser::serialize_structure_crate_model_session_keys_abp_v10_x(
            &mut object_184,
            var_183,
        );
        object_184.finish();
    }
}

pub fn serialize_structure_crate_model_lo_ra_wan_update_gateway_task_create(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LoRaWanUpdateGatewayTaskCreate,
) {
    if let Some(var_185) = &input.update_signature {
        object.key("UpdateSignature").string(var_185);
    }
    if let Some(var_186) = &input.sig_key_crc {
        object.key("SigKeyCrc").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_186).into()),
        );
    }
    if let Some(var_187) = &input.current_version {
        let mut object_188 = object.key("CurrentVersion").start_object();
        crate::json_ser::serialize_structure_crate_model_lo_ra_wan_gateway_version(
            &mut object_188,
            var_187,
        );
        object_188.finish();
    }
    if let Some(var_189) = &input.update_version {
        let mut object_190 = object.key("UpdateVersion").start_object();
        crate::json_ser::serialize_structure_crate_model_lo_ra_wan_gateway_version(
            &mut object_190,
            var_189,
        );
        object_190.finish();
    }
}

pub fn serialize_structure_crate_model_lo_ra_wan_send_data_to_device(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LoRaWanSendDataToDevice,
) {
    if let Some(var_191) = &input.f_port {
        object.key("FPort").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_191).into()),
        );
    }
}

pub fn serialize_structure_crate_model_sidewalk_send_data_to_device(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SidewalkSendDataToDevice,
) {
    if let Some(var_192) = &input.seq {
        object.key("Seq").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_192).into()),
        );
    }
    if let Some(var_193) = &input.message_type {
        object.key("MessageType").string(var_193.as_str());
    }
}

pub fn serialize_structure_crate_model_wireless_device_event_log_option(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::WirelessDeviceEventLogOption,
) {
    if let Some(var_194) = &input.event {
        object.key("Event").string(var_194.as_str());
    }
    if let Some(var_195) = &input.log_level {
        object.key("LogLevel").string(var_195.as_str());
    }
}

pub fn serialize_structure_crate_model_wireless_gateway_event_log_option(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::WirelessGatewayEventLogOption,
) {
    if let Some(var_196) = &input.event {
        object.key("Event").string(var_196.as_str());
    }
    if let Some(var_197) = &input.log_level {
        object.key("LogLevel").string(var_197.as_str());
    }
}

pub fn serialize_structure_crate_model_session_keys_abp_v11(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SessionKeysAbpV11,
) {
    if let Some(var_198) = &input.f_nwk_s_int_key {
        object.key("FNwkSIntKey").string(var_198);
    }
    if let Some(var_199) = &input.s_nwk_s_int_key {
        object.key("SNwkSIntKey").string(var_199);
    }
    if let Some(var_200) = &input.nwk_s_enc_key {
        object.key("NwkSEncKey").string(var_200);
    }
    if let Some(var_201) = &input.app_s_key {
        object.key("AppSKey").string(var_201);
    }
}

pub fn serialize_structure_crate_model_session_keys_abp_v10_x(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SessionKeysAbpV10X,
) {
    if let Some(var_202) = &input.nwk_s_key {
        object.key("NwkSKey").string(var_202);
    }
    if let Some(var_203) = &input.app_s_key {
        object.key("AppSKey").string(var_203);
    }
}

pub fn serialize_structure_crate_model_lo_ra_wan_gateway_version(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LoRaWanGatewayVersion,
) {
    if let Some(var_204) = &input.package_version {
        object.key("PackageVersion").string(var_204);
    }
    if let Some(var_205) = &input.model {
        object.key("Model").string(var_205);
    }
    if let Some(var_206) = &input.station {
        object.key("Station").string(var_206);
    }
}
