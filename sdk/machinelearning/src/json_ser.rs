// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_tags_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddTagsInput,
) {
    if let Some(var_1) = &input.tags {
        let mut array_2 = object.key("Tags").start_array();
        for item_3 in var_1 {
            {
                let mut object_4 = array_2.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_4, item_3);
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.resource_id {
        object.key("ResourceId").string(var_5);
    }
    if let Some(var_6) = &input.resource_type {
        object.key("ResourceType").string(var_6.as_str());
    }
}

pub fn serialize_structure_crate_input_create_batch_prediction_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBatchPredictionInput,
) {
    if let Some(var_7) = &input.batch_prediction_id {
        object.key("BatchPredictionId").string(var_7);
    }
    if let Some(var_8) = &input.batch_prediction_name {
        object.key("BatchPredictionName").string(var_8);
    }
    if let Some(var_9) = &input.ml_model_id {
        object.key("MLModelId").string(var_9);
    }
    if let Some(var_10) = &input.batch_prediction_data_source_id {
        object.key("BatchPredictionDataSourceId").string(var_10);
    }
    if let Some(var_11) = &input.output_uri {
        object.key("OutputUri").string(var_11);
    }
}

pub fn serialize_structure_crate_input_create_data_source_from_rds_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDataSourceFromRdsInput,
) {
    if let Some(var_12) = &input.data_source_id {
        object.key("DataSourceId").string(var_12);
    }
    if let Some(var_13) = &input.data_source_name {
        object.key("DataSourceName").string(var_13);
    }
    if let Some(var_14) = &input.rds_data {
        let mut object_15 = object.key("RDSData").start_object();
        crate::json_ser::serialize_structure_crate_model_rds_data_spec(&mut object_15, var_14);
        object_15.finish();
    }
    if let Some(var_16) = &input.role_arn {
        object.key("RoleARN").string(var_16);
    }
    if input.compute_statistics {
        object
            .key("ComputeStatistics")
            .boolean(input.compute_statistics);
    }
}

pub fn serialize_structure_crate_input_create_data_source_from_redshift_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDataSourceFromRedshiftInput,
) {
    if let Some(var_17) = &input.data_source_id {
        object.key("DataSourceId").string(var_17);
    }
    if let Some(var_18) = &input.data_source_name {
        object.key("DataSourceName").string(var_18);
    }
    if let Some(var_19) = &input.data_spec {
        let mut object_20 = object.key("DataSpec").start_object();
        crate::json_ser::serialize_structure_crate_model_redshift_data_spec(&mut object_20, var_19);
        object_20.finish();
    }
    if let Some(var_21) = &input.role_arn {
        object.key("RoleARN").string(var_21);
    }
    if input.compute_statistics {
        object
            .key("ComputeStatistics")
            .boolean(input.compute_statistics);
    }
}

pub fn serialize_structure_crate_input_create_data_source_from_s3_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDataSourceFromS3Input,
) {
    if let Some(var_22) = &input.data_source_id {
        object.key("DataSourceId").string(var_22);
    }
    if let Some(var_23) = &input.data_source_name {
        object.key("DataSourceName").string(var_23);
    }
    if let Some(var_24) = &input.data_spec {
        let mut object_25 = object.key("DataSpec").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_data_spec(&mut object_25, var_24);
        object_25.finish();
    }
    if input.compute_statistics {
        object
            .key("ComputeStatistics")
            .boolean(input.compute_statistics);
    }
}

pub fn serialize_structure_crate_input_create_evaluation_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEvaluationInput,
) {
    if let Some(var_26) = &input.evaluation_id {
        object.key("EvaluationId").string(var_26);
    }
    if let Some(var_27) = &input.evaluation_name {
        object.key("EvaluationName").string(var_27);
    }
    if let Some(var_28) = &input.ml_model_id {
        object.key("MLModelId").string(var_28);
    }
    if let Some(var_29) = &input.evaluation_data_source_id {
        object.key("EvaluationDataSourceId").string(var_29);
    }
}

pub fn serialize_structure_crate_input_create_ml_model_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateMlModelInput,
) {
    if let Some(var_30) = &input.ml_model_id {
        object.key("MLModelId").string(var_30);
    }
    if let Some(var_31) = &input.ml_model_name {
        object.key("MLModelName").string(var_31);
    }
    if let Some(var_32) = &input.ml_model_type {
        object.key("MLModelType").string(var_32.as_str());
    }
    if let Some(var_33) = &input.parameters {
        let mut object_34 = object.key("Parameters").start_object();
        for (key_35, value_36) in var_33 {
            {
                object_34.key(key_35).string(value_36);
            }
        }
        object_34.finish();
    }
    if let Some(var_37) = &input.training_data_source_id {
        object.key("TrainingDataSourceId").string(var_37);
    }
    if let Some(var_38) = &input.recipe {
        object.key("Recipe").string(var_38);
    }
    if let Some(var_39) = &input.recipe_uri {
        object.key("RecipeUri").string(var_39);
    }
}

pub fn serialize_structure_crate_input_create_realtime_endpoint_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRealtimeEndpointInput,
) {
    if let Some(var_40) = &input.ml_model_id {
        object.key("MLModelId").string(var_40);
    }
}

pub fn serialize_structure_crate_input_delete_batch_prediction_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteBatchPredictionInput,
) {
    if let Some(var_41) = &input.batch_prediction_id {
        object.key("BatchPredictionId").string(var_41);
    }
}

pub fn serialize_structure_crate_input_delete_data_source_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteDataSourceInput,
) {
    if let Some(var_42) = &input.data_source_id {
        object.key("DataSourceId").string(var_42);
    }
}

pub fn serialize_structure_crate_input_delete_evaluation_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteEvaluationInput,
) {
    if let Some(var_43) = &input.evaluation_id {
        object.key("EvaluationId").string(var_43);
    }
}

pub fn serialize_structure_crate_input_delete_ml_model_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteMlModelInput,
) {
    if let Some(var_44) = &input.ml_model_id {
        object.key("MLModelId").string(var_44);
    }
}

pub fn serialize_structure_crate_input_delete_realtime_endpoint_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteRealtimeEndpointInput,
) {
    if let Some(var_45) = &input.ml_model_id {
        object.key("MLModelId").string(var_45);
    }
}

pub fn serialize_structure_crate_input_delete_tags_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteTagsInput,
) {
    if let Some(var_46) = &input.tag_keys {
        let mut array_47 = object.key("TagKeys").start_array();
        for item_48 in var_46 {
            {
                array_47.value().string(item_48);
            }
        }
        array_47.finish();
    }
    if let Some(var_49) = &input.resource_id {
        object.key("ResourceId").string(var_49);
    }
    if let Some(var_50) = &input.resource_type {
        object.key("ResourceType").string(var_50.as_str());
    }
}

pub fn serialize_structure_crate_input_describe_batch_predictions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBatchPredictionsInput,
) {
    if let Some(var_51) = &input.filter_variable {
        object.key("FilterVariable").string(var_51.as_str());
    }
    if let Some(var_52) = &input.eq {
        object.key("EQ").string(var_52);
    }
    if let Some(var_53) = &input.gt {
        object.key("GT").string(var_53);
    }
    if let Some(var_54) = &input.lt {
        object.key("LT").string(var_54);
    }
    if let Some(var_55) = &input.ge {
        object.key("GE").string(var_55);
    }
    if let Some(var_56) = &input.le {
        object.key("LE").string(var_56);
    }
    if let Some(var_57) = &input.ne {
        object.key("NE").string(var_57);
    }
    if let Some(var_58) = &input.prefix {
        object.key("Prefix").string(var_58);
    }
    if let Some(var_59) = &input.sort_order {
        object.key("SortOrder").string(var_59.as_str());
    }
    if let Some(var_60) = &input.next_token {
        object.key("NextToken").string(var_60);
    }
    if let Some(var_61) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_61).into()),
        );
    }
}

pub fn serialize_structure_crate_input_describe_data_sources_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDataSourcesInput,
) {
    if let Some(var_62) = &input.filter_variable {
        object.key("FilterVariable").string(var_62.as_str());
    }
    if let Some(var_63) = &input.eq {
        object.key("EQ").string(var_63);
    }
    if let Some(var_64) = &input.gt {
        object.key("GT").string(var_64);
    }
    if let Some(var_65) = &input.lt {
        object.key("LT").string(var_65);
    }
    if let Some(var_66) = &input.ge {
        object.key("GE").string(var_66);
    }
    if let Some(var_67) = &input.le {
        object.key("LE").string(var_67);
    }
    if let Some(var_68) = &input.ne {
        object.key("NE").string(var_68);
    }
    if let Some(var_69) = &input.prefix {
        object.key("Prefix").string(var_69);
    }
    if let Some(var_70) = &input.sort_order {
        object.key("SortOrder").string(var_70.as_str());
    }
    if let Some(var_71) = &input.next_token {
        object.key("NextToken").string(var_71);
    }
    if let Some(var_72) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_72).into()),
        );
    }
}

pub fn serialize_structure_crate_input_describe_evaluations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeEvaluationsInput,
) {
    if let Some(var_73) = &input.filter_variable {
        object.key("FilterVariable").string(var_73.as_str());
    }
    if let Some(var_74) = &input.eq {
        object.key("EQ").string(var_74);
    }
    if let Some(var_75) = &input.gt {
        object.key("GT").string(var_75);
    }
    if let Some(var_76) = &input.lt {
        object.key("LT").string(var_76);
    }
    if let Some(var_77) = &input.ge {
        object.key("GE").string(var_77);
    }
    if let Some(var_78) = &input.le {
        object.key("LE").string(var_78);
    }
    if let Some(var_79) = &input.ne {
        object.key("NE").string(var_79);
    }
    if let Some(var_80) = &input.prefix {
        object.key("Prefix").string(var_80);
    }
    if let Some(var_81) = &input.sort_order {
        object.key("SortOrder").string(var_81.as_str());
    }
    if let Some(var_82) = &input.next_token {
        object.key("NextToken").string(var_82);
    }
    if let Some(var_83) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_83).into()),
        );
    }
}

pub fn serialize_structure_crate_input_describe_ml_models_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeMlModelsInput,
) {
    if let Some(var_84) = &input.filter_variable {
        object.key("FilterVariable").string(var_84.as_str());
    }
    if let Some(var_85) = &input.eq {
        object.key("EQ").string(var_85);
    }
    if let Some(var_86) = &input.gt {
        object.key("GT").string(var_86);
    }
    if let Some(var_87) = &input.lt {
        object.key("LT").string(var_87);
    }
    if let Some(var_88) = &input.ge {
        object.key("GE").string(var_88);
    }
    if let Some(var_89) = &input.le {
        object.key("LE").string(var_89);
    }
    if let Some(var_90) = &input.ne {
        object.key("NE").string(var_90);
    }
    if let Some(var_91) = &input.prefix {
        object.key("Prefix").string(var_91);
    }
    if let Some(var_92) = &input.sort_order {
        object.key("SortOrder").string(var_92.as_str());
    }
    if let Some(var_93) = &input.next_token {
        object.key("NextToken").string(var_93);
    }
    if let Some(var_94) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_94).into()),
        );
    }
}

pub fn serialize_structure_crate_input_describe_tags_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeTagsInput,
) {
    if let Some(var_95) = &input.resource_id {
        object.key("ResourceId").string(var_95);
    }
    if let Some(var_96) = &input.resource_type {
        object.key("ResourceType").string(var_96.as_str());
    }
}

pub fn serialize_structure_crate_input_get_batch_prediction_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetBatchPredictionInput,
) {
    if let Some(var_97) = &input.batch_prediction_id {
        object.key("BatchPredictionId").string(var_97);
    }
}

pub fn serialize_structure_crate_input_get_data_source_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDataSourceInput,
) {
    if let Some(var_98) = &input.data_source_id {
        object.key("DataSourceId").string(var_98);
    }
    if input.verbose {
        object.key("Verbose").boolean(input.verbose);
    }
}

pub fn serialize_structure_crate_input_get_evaluation_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetEvaluationInput,
) {
    if let Some(var_99) = &input.evaluation_id {
        object.key("EvaluationId").string(var_99);
    }
}

pub fn serialize_structure_crate_input_get_ml_model_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetMlModelInput,
) {
    if let Some(var_100) = &input.ml_model_id {
        object.key("MLModelId").string(var_100);
    }
    if input.verbose {
        object.key("Verbose").boolean(input.verbose);
    }
}

pub fn serialize_structure_crate_input_predict_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PredictInput,
) {
    if let Some(var_101) = &input.ml_model_id {
        object.key("MLModelId").string(var_101);
    }
    if let Some(var_102) = &input.record {
        let mut object_103 = object.key("Record").start_object();
        for (key_104, value_105) in var_102 {
            {
                object_103.key(key_104).string(value_105);
            }
        }
        object_103.finish();
    }
    if let Some(var_106) = &input.predict_endpoint {
        object.key("PredictEndpoint").string(var_106);
    }
}

pub fn serialize_structure_crate_input_update_batch_prediction_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBatchPredictionInput,
) {
    if let Some(var_107) = &input.batch_prediction_id {
        object.key("BatchPredictionId").string(var_107);
    }
    if let Some(var_108) = &input.batch_prediction_name {
        object.key("BatchPredictionName").string(var_108);
    }
}

pub fn serialize_structure_crate_input_update_data_source_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDataSourceInput,
) {
    if let Some(var_109) = &input.data_source_id {
        object.key("DataSourceId").string(var_109);
    }
    if let Some(var_110) = &input.data_source_name {
        object.key("DataSourceName").string(var_110);
    }
}

pub fn serialize_structure_crate_input_update_evaluation_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEvaluationInput,
) {
    if let Some(var_111) = &input.evaluation_id {
        object.key("EvaluationId").string(var_111);
    }
    if let Some(var_112) = &input.evaluation_name {
        object.key("EvaluationName").string(var_112);
    }
}

pub fn serialize_structure_crate_input_update_ml_model_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateMlModelInput,
) {
    if let Some(var_113) = &input.ml_model_id {
        object.key("MLModelId").string(var_113);
    }
    if let Some(var_114) = &input.ml_model_name {
        object.key("MLModelName").string(var_114);
    }
    if let Some(var_115) = &input.score_threshold {
        object.key("ScoreThreshold").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_115).into()),
        );
    }
}

pub fn serialize_structure_crate_model_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_116) = &input.key {
        object.key("Key").string(var_116);
    }
    if let Some(var_117) = &input.value {
        object.key("Value").string(var_117);
    }
}

pub fn serialize_structure_crate_model_rds_data_spec(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RdsDataSpec,
) {
    if let Some(var_118) = &input.database_information {
        let mut object_119 = object.key("DatabaseInformation").start_object();
        crate::json_ser::serialize_structure_crate_model_rds_database(&mut object_119, var_118);
        object_119.finish();
    }
    if let Some(var_120) = &input.select_sql_query {
        object.key("SelectSqlQuery").string(var_120);
    }
    if let Some(var_121) = &input.database_credentials {
        let mut object_122 = object.key("DatabaseCredentials").start_object();
        crate::json_ser::serialize_structure_crate_model_rds_database_credentials(
            &mut object_122,
            var_121,
        );
        object_122.finish();
    }
    if let Some(var_123) = &input.s3_staging_location {
        object.key("S3StagingLocation").string(var_123);
    }
    if let Some(var_124) = &input.data_rearrangement {
        object.key("DataRearrangement").string(var_124);
    }
    if let Some(var_125) = &input.data_schema {
        object.key("DataSchema").string(var_125);
    }
    if let Some(var_126) = &input.data_schema_uri {
        object.key("DataSchemaUri").string(var_126);
    }
    if let Some(var_127) = &input.resource_role {
        object.key("ResourceRole").string(var_127);
    }
    if let Some(var_128) = &input.service_role {
        object.key("ServiceRole").string(var_128);
    }
    if let Some(var_129) = &input.subnet_id {
        object.key("SubnetId").string(var_129);
    }
    if let Some(var_130) = &input.security_group_ids {
        let mut array_131 = object.key("SecurityGroupIds").start_array();
        for item_132 in var_130 {
            {
                array_131.value().string(item_132);
            }
        }
        array_131.finish();
    }
}

pub fn serialize_structure_crate_model_redshift_data_spec(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RedshiftDataSpec,
) {
    if let Some(var_133) = &input.database_information {
        let mut object_134 = object.key("DatabaseInformation").start_object();
        crate::json_ser::serialize_structure_crate_model_redshift_database(
            &mut object_134,
            var_133,
        );
        object_134.finish();
    }
    if let Some(var_135) = &input.select_sql_query {
        object.key("SelectSqlQuery").string(var_135);
    }
    if let Some(var_136) = &input.database_credentials {
        let mut object_137 = object.key("DatabaseCredentials").start_object();
        crate::json_ser::serialize_structure_crate_model_redshift_database_credentials(
            &mut object_137,
            var_136,
        );
        object_137.finish();
    }
    if let Some(var_138) = &input.s3_staging_location {
        object.key("S3StagingLocation").string(var_138);
    }
    if let Some(var_139) = &input.data_rearrangement {
        object.key("DataRearrangement").string(var_139);
    }
    if let Some(var_140) = &input.data_schema {
        object.key("DataSchema").string(var_140);
    }
    if let Some(var_141) = &input.data_schema_uri {
        object.key("DataSchemaUri").string(var_141);
    }
}

pub fn serialize_structure_crate_model_s3_data_spec(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3DataSpec,
) {
    if let Some(var_142) = &input.data_location_s3 {
        object.key("DataLocationS3").string(var_142);
    }
    if let Some(var_143) = &input.data_rearrangement {
        object.key("DataRearrangement").string(var_143);
    }
    if let Some(var_144) = &input.data_schema {
        object.key("DataSchema").string(var_144);
    }
    if let Some(var_145) = &input.data_schema_location_s3 {
        object.key("DataSchemaLocationS3").string(var_145);
    }
}

pub fn serialize_structure_crate_model_rds_database(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RdsDatabase,
) {
    if let Some(var_146) = &input.instance_identifier {
        object.key("InstanceIdentifier").string(var_146);
    }
    if let Some(var_147) = &input.database_name {
        object.key("DatabaseName").string(var_147);
    }
}

pub fn serialize_structure_crate_model_rds_database_credentials(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RdsDatabaseCredentials,
) {
    if let Some(var_148) = &input.username {
        object.key("Username").string(var_148);
    }
    if let Some(var_149) = &input.password {
        object.key("Password").string(var_149);
    }
}

pub fn serialize_structure_crate_model_redshift_database(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RedshiftDatabase,
) {
    if let Some(var_150) = &input.database_name {
        object.key("DatabaseName").string(var_150);
    }
    if let Some(var_151) = &input.cluster_identifier {
        object.key("ClusterIdentifier").string(var_151);
    }
}

pub fn serialize_structure_crate_model_redshift_database_credentials(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RedshiftDatabaseCredentials,
) {
    if let Some(var_152) = &input.username {
        object.key("Username").string(var_152);
    }
    if let Some(var_153) = &input.password {
        object.key("Password").string(var_153);
    }
}
