// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn serialize_structure_crate_model_tag(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::Tag,
) {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Key");
    if let Some(var_2) = &input.key {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Value");
    if let Some(var_4) = &input.value {
        scope_3.string(var_4);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_log_delivery_configuration_request(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::LogDeliveryConfigurationRequest,
) {
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("LogType");
    if let Some(var_6) = &input.log_type {
        scope_5.string(var_6.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DestinationType");
    if let Some(var_8) = &input.destination_type {
        scope_7.string(var_8.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("DestinationDetails");
    if let Some(var_10) = &input.destination_details {
        crate::query_ser::serialize_structure_crate_model_destination_details(scope_9, var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("LogFormat");
    if let Some(var_12) = &input.log_format {
        scope_11.string(var_12.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("Enabled");
    if let Some(var_14) = &input.enabled {
        scope_13.boolean(*var_14);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_node_group_configuration(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::NodeGroupConfiguration,
) {
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("NodeGroupId");
    if let Some(var_16) = &input.node_group_id {
        scope_15.string(var_16);
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("Slots");
    if let Some(var_18) = &input.slots {
        scope_17.string(var_18);
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("ReplicaCount");
    if let Some(var_20) = &input.replica_count {
        scope_19.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_20).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("PrimaryAvailabilityZone");
    if let Some(var_22) = &input.primary_availability_zone {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("ReplicaAvailabilityZones");
    if let Some(var_24) = &input.replica_availability_zones {
        let mut list_26 = scope_23.start_list(false, Some("AvailabilityZone"));
        for item_25 in var_24 {
            #[allow(unused_mut)]
            let mut entry_27 = list_26.entry();
            entry_27.string(item_25);
        }
        list_26.finish();
    }
    #[allow(unused_mut)]
    let mut scope_28 = writer.prefix("PrimaryOutpostArn");
    if let Some(var_29) = &input.primary_outpost_arn {
        scope_28.string(var_29);
    }
    #[allow(unused_mut)]
    let mut scope_30 = writer.prefix("ReplicaOutpostArns");
    if let Some(var_31) = &input.replica_outpost_arns {
        let mut list_33 = scope_30.start_list(false, Some("OutpostArn"));
        for item_32 in var_31 {
            #[allow(unused_mut)]
            let mut entry_34 = list_33.entry();
            entry_34.string(item_32);
        }
        list_33.finish();
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_configure_shard(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::ConfigureShard,
) {
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("NodeGroupId");
    if let Some(var_36) = &input.node_group_id {
        scope_35.string(var_36);
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("NewReplicaCount");
    {
        scope_37.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.new_replica_count).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_38 = writer.prefix("PreferredAvailabilityZones");
    if let Some(var_39) = &input.preferred_availability_zones {
        let mut list_41 = scope_38.start_list(false, Some("PreferredAvailabilityZone"));
        for item_40 in var_39 {
            #[allow(unused_mut)]
            let mut entry_42 = list_41.entry();
            entry_42.string(item_40);
        }
        list_41.finish();
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("PreferredOutpostArns");
    if let Some(var_44) = &input.preferred_outpost_arns {
        let mut list_46 = scope_43.start_list(false, Some("PreferredOutpostArn"));
        for item_45 in var_44 {
            #[allow(unused_mut)]
            let mut entry_47 = list_46.entry();
            entry_47.string(item_45);
        }
        list_46.finish();
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_time_range_filter(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::TimeRangeFilter,
) {
    #[allow(unused_mut)]
    let mut scope_48 = writer.prefix("StartTime");
    if let Some(var_49) = &input.start_time {
        scope_48.instant(var_49, smithy_types::instant::Format::DateTime);
    }
    #[allow(unused_mut)]
    let mut scope_50 = writer.prefix("EndTime");
    if let Some(var_51) = &input.end_time {
        scope_50.instant(var_51, smithy_types::instant::Format::DateTime);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_filter(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::Filter,
) {
    #[allow(unused_mut)]
    let mut scope_52 = writer.prefix("Name");
    if let Some(var_53) = &input.name {
        scope_52.string(var_53);
    }
    #[allow(unused_mut)]
    let mut scope_54 = writer.prefix("Values");
    if let Some(var_55) = &input.values {
        let mut list_57 = scope_54.start_list(false, None);
        for item_56 in var_55 {
            #[allow(unused_mut)]
            let mut entry_58 = list_57.entry();
            entry_58.string(item_56);
        }
        list_57.finish();
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_regional_configuration(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::RegionalConfiguration,
) {
    #[allow(unused_mut)]
    let mut scope_59 = writer.prefix("ReplicationGroupId");
    if let Some(var_60) = &input.replication_group_id {
        scope_59.string(var_60);
    }
    #[allow(unused_mut)]
    let mut scope_61 = writer.prefix("ReplicationGroupRegion");
    if let Some(var_62) = &input.replication_group_region {
        scope_61.string(var_62);
    }
    #[allow(unused_mut)]
    let mut scope_63 = writer.prefix("ReshardingConfiguration");
    if let Some(var_64) = &input.resharding_configuration {
        let mut list_66 = scope_63.start_list(false, Some("ReshardingConfiguration"));
        for item_65 in var_64 {
            #[allow(unused_mut)]
            let mut entry_67 = list_66.entry();
            crate::query_ser::serialize_structure_crate_model_resharding_configuration(
                entry_67, item_65,
            );
        }
        list_66.finish();
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_parameter_name_value(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::ParameterNameValue,
) {
    #[allow(unused_mut)]
    let mut scope_68 = writer.prefix("ParameterName");
    if let Some(var_69) = &input.parameter_name {
        scope_68.string(var_69);
    }
    #[allow(unused_mut)]
    let mut scope_70 = writer.prefix("ParameterValue");
    if let Some(var_71) = &input.parameter_value {
        scope_70.string(var_71);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_resharding_configuration(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::ReshardingConfiguration,
) {
    #[allow(unused_mut)]
    let mut scope_72 = writer.prefix("NodeGroupId");
    if let Some(var_73) = &input.node_group_id {
        scope_72.string(var_73);
    }
    #[allow(unused_mut)]
    let mut scope_74 = writer.prefix("PreferredAvailabilityZones");
    if let Some(var_75) = &input.preferred_availability_zones {
        let mut list_77 = scope_74.start_list(false, Some("AvailabilityZone"));
        for item_76 in var_75 {
            #[allow(unused_mut)]
            let mut entry_78 = list_77.entry();
            entry_78.string(item_76);
        }
        list_77.finish();
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_customer_node_endpoint(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::CustomerNodeEndpoint,
) {
    #[allow(unused_mut)]
    let mut scope_79 = writer.prefix("Address");
    if let Some(var_80) = &input.address {
        scope_79.string(var_80);
    }
    #[allow(unused_mut)]
    let mut scope_81 = writer.prefix("Port");
    if let Some(var_82) = &input.port {
        scope_81.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_82).into()),
        );
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_destination_details(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::DestinationDetails,
) {
    #[allow(unused_mut)]
    let mut scope_83 = writer.prefix("CloudWatchLogsDetails");
    if let Some(var_84) = &input.cloud_watch_logs_details {
        crate::query_ser::serialize_structure_crate_model_cloud_watch_logs_destination_details(
            scope_83, var_84,
        );
    }
    #[allow(unused_mut)]
    let mut scope_85 = writer.prefix("KinesisFirehoseDetails");
    if let Some(var_86) = &input.kinesis_firehose_details {
        crate::query_ser::serialize_structure_crate_model_kinesis_firehose_destination_details(
            scope_85, var_86,
        );
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_cloud_watch_logs_destination_details(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::CloudWatchLogsDestinationDetails,
) {
    #[allow(unused_mut)]
    let mut scope_87 = writer.prefix("LogGroup");
    if let Some(var_88) = &input.log_group {
        scope_87.string(var_88);
    }
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_kinesis_firehose_destination_details(
    mut writer: smithy_query::QueryValueWriter,
    input: &crate::model::KinesisFirehoseDestinationDetails,
) {
    #[allow(unused_mut)]
    let mut scope_89 = writer.prefix("DeliveryStream");
    if let Some(var_90) = &input.delivery_stream {
        scope_89.string(var_90);
    }
}
