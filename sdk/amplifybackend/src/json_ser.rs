// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_clone_backend_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CloneBackendInput,
) {
    if let Some(var_1) = &input.target_environment_name {
        object.key("targetEnvironmentName").string(var_1);
    }
}

pub fn serialize_structure_crate_input_create_backend_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBackendInput,
) {
    if let Some(var_2) = &input.app_id {
        object.key("appId").string(var_2);
    }
    if let Some(var_3) = &input.app_name {
        object.key("appName").string(var_3);
    }
    if let Some(var_4) = &input.backend_environment_name {
        object.key("backendEnvironmentName").string(var_4);
    }
    if let Some(var_5) = &input.resource_config {
        let mut object_6 = object.key("resourceConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_resource_config(&mut object_6, var_5);
        object_6.finish();
    }
    if let Some(var_7) = &input.resource_name {
        object.key("resourceName").string(var_7);
    }
}

pub fn serialize_structure_crate_input_create_backend_api_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBackendApiInput,
) {
    if let Some(var_8) = &input.backend_environment_name {
        object.key("backendEnvironmentName").string(var_8);
    }
    if let Some(var_9) = &input.resource_config {
        let mut object_10 = object.key("resourceConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_backend_api_resource_config(
            &mut object_10,
            var_9,
        );
        object_10.finish();
    }
    if let Some(var_11) = &input.resource_name {
        object.key("resourceName").string(var_11);
    }
}

pub fn serialize_structure_crate_input_create_backend_auth_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBackendAuthInput,
) {
    if let Some(var_12) = &input.backend_environment_name {
        object.key("backendEnvironmentName").string(var_12);
    }
    if let Some(var_13) = &input.resource_config {
        let mut object_14 = object.key("resourceConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_create_backend_auth_resource_config(
            &mut object_14,
            var_13,
        );
        object_14.finish();
    }
    if let Some(var_15) = &input.resource_name {
        object.key("resourceName").string(var_15);
    }
}

pub fn serialize_structure_crate_input_create_backend_config_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBackendConfigInput,
) {
    if let Some(var_16) = &input.backend_manager_app_id {
        object.key("backendManagerAppId").string(var_16);
    }
}

pub fn serialize_structure_crate_input_delete_backend_api_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteBackendApiInput,
) {
    if let Some(var_17) = &input.resource_config {
        let mut object_18 = object.key("resourceConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_backend_api_resource_config(
            &mut object_18,
            var_17,
        );
        object_18.finish();
    }
    if let Some(var_19) = &input.resource_name {
        object.key("resourceName").string(var_19);
    }
}

pub fn serialize_structure_crate_input_delete_backend_auth_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteBackendAuthInput,
) {
    if let Some(var_20) = &input.resource_name {
        object.key("resourceName").string(var_20);
    }
}

pub fn serialize_structure_crate_input_generate_backend_api_models_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GenerateBackendApiModelsInput,
) {
    if let Some(var_21) = &input.resource_name {
        object.key("resourceName").string(var_21);
    }
}

pub fn serialize_structure_crate_input_get_backend_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetBackendInput,
) {
    if let Some(var_22) = &input.backend_environment_name {
        object.key("backendEnvironmentName").string(var_22);
    }
}

pub fn serialize_structure_crate_input_get_backend_api_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetBackendApiInput,
) {
    if let Some(var_23) = &input.resource_config {
        let mut object_24 = object.key("resourceConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_backend_api_resource_config(
            &mut object_24,
            var_23,
        );
        object_24.finish();
    }
    if let Some(var_25) = &input.resource_name {
        object.key("resourceName").string(var_25);
    }
}

pub fn serialize_structure_crate_input_get_backend_api_models_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetBackendApiModelsInput,
) {
    if let Some(var_26) = &input.resource_name {
        object.key("resourceName").string(var_26);
    }
}

pub fn serialize_structure_crate_input_get_backend_auth_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetBackendAuthInput,
) {
    if let Some(var_27) = &input.resource_name {
        object.key("resourceName").string(var_27);
    }
}

pub fn serialize_structure_crate_input_import_backend_auth_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ImportBackendAuthInput,
) {
    if let Some(var_28) = &input.identity_pool_id {
        object.key("identityPoolId").string(var_28);
    }
    if let Some(var_29) = &input.native_client_id {
        object.key("nativeClientId").string(var_29);
    }
    if let Some(var_30) = &input.user_pool_id {
        object.key("userPoolId").string(var_30);
    }
    if let Some(var_31) = &input.web_client_id {
        object.key("webClientId").string(var_31);
    }
}

pub fn serialize_structure_crate_input_list_backend_jobs_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListBackendJobsInput,
) {
    if let Some(var_32) = &input.job_id {
        object.key("jobId").string(var_32);
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_33) = &input.next_token {
        object.key("nextToken").string(var_33);
    }
    if let Some(var_34) = &input.operation {
        object.key("operation").string(var_34);
    }
    if let Some(var_35) = &input.status {
        object.key("status").string(var_35);
    }
}

pub fn serialize_structure_crate_input_remove_all_backends_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RemoveAllBackendsInput,
) {
    if input.clean_amplify_app {
        object
            .key("cleanAmplifyApp")
            .boolean(input.clean_amplify_app);
    }
}

pub fn serialize_structure_crate_input_update_backend_api_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBackendApiInput,
) {
    if let Some(var_36) = &input.resource_config {
        let mut object_37 = object.key("resourceConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_backend_api_resource_config(
            &mut object_37,
            var_36,
        );
        object_37.finish();
    }
    if let Some(var_38) = &input.resource_name {
        object.key("resourceName").string(var_38);
    }
}

pub fn serialize_structure_crate_input_update_backend_auth_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBackendAuthInput,
) {
    if let Some(var_39) = &input.resource_config {
        let mut object_40 = object.key("resourceConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_update_backend_auth_resource_config(
            &mut object_40,
            var_39,
        );
        object_40.finish();
    }
    if let Some(var_41) = &input.resource_name {
        object.key("resourceName").string(var_41);
    }
}

pub fn serialize_structure_crate_input_update_backend_config_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBackendConfigInput,
) {
    if let Some(var_42) = &input.login_auth_config {
        let mut object_43 = object.key("loginAuthConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_login_auth_config_req_obj(
            &mut object_43,
            var_42,
        );
        object_43.finish();
    }
}

pub fn serialize_structure_crate_input_update_backend_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBackendJobInput,
) {
    if let Some(var_44) = &input.operation {
        object.key("operation").string(var_44);
    }
    if let Some(var_45) = &input.status {
        object.key("status").string(var_45);
    }
}

pub fn serialize_structure_crate_model_resource_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ResourceConfig,
) {
    let (_, _) = (object, input);
}

pub fn serialize_structure_crate_model_backend_api_resource_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BackendApiResourceConfig,
) {
    if let Some(var_46) = &input.additional_auth_types {
        let mut array_47 = object.key("additionalAuthTypes").start_array();
        for item_48 in var_46 {
            {
                let mut object_49 = array_47.value().start_object();
                crate::json_ser::serialize_structure_crate_model_backend_api_auth_type(
                    &mut object_49,
                    item_48,
                );
                object_49.finish();
            }
        }
        array_47.finish();
    }
    if let Some(var_50) = &input.api_name {
        object.key("apiName").string(var_50);
    }
    if let Some(var_51) = &input.conflict_resolution {
        let mut object_52 = object.key("conflictResolution").start_object();
        crate::json_ser::serialize_structure_crate_model_backend_api_conflict_resolution(
            &mut object_52,
            var_51,
        );
        object_52.finish();
    }
    if let Some(var_53) = &input.default_auth_type {
        let mut object_54 = object.key("defaultAuthType").start_object();
        crate::json_ser::serialize_structure_crate_model_backend_api_auth_type(
            &mut object_54,
            var_53,
        );
        object_54.finish();
    }
    if let Some(var_55) = &input.service {
        object.key("service").string(var_55);
    }
    if let Some(var_56) = &input.transform_schema {
        object.key("transformSchema").string(var_56);
    }
}

pub fn serialize_structure_crate_model_create_backend_auth_resource_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateBackendAuthResourceConfig,
) {
    if let Some(var_57) = &input.auth_resources {
        object.key("authResources").string(var_57.as_str());
    }
    if let Some(var_58) = &input.identity_pool_configs {
        let mut object_59 = object.key("identityPoolConfigs").start_object();
        crate::json_ser::serialize_structure_crate_model_create_backend_auth_identity_pool_config(
            &mut object_59,
            var_58,
        );
        object_59.finish();
    }
    if let Some(var_60) = &input.service {
        object.key("service").string(var_60.as_str());
    }
    if let Some(var_61) = &input.user_pool_configs {
        let mut object_62 = object.key("userPoolConfigs").start_object();
        crate::json_ser::serialize_structure_crate_model_create_backend_auth_user_pool_config(
            &mut object_62,
            var_61,
        );
        object_62.finish();
    }
}

pub fn serialize_structure_crate_model_update_backend_auth_resource_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateBackendAuthResourceConfig,
) {
    if let Some(var_63) = &input.auth_resources {
        object.key("authResources").string(var_63.as_str());
    }
    if let Some(var_64) = &input.identity_pool_configs {
        let mut object_65 = object.key("identityPoolConfigs").start_object();
        crate::json_ser::serialize_structure_crate_model_update_backend_auth_identity_pool_config(
            &mut object_65,
            var_64,
        );
        object_65.finish();
    }
    if let Some(var_66) = &input.service {
        object.key("service").string(var_66.as_str());
    }
    if let Some(var_67) = &input.user_pool_configs {
        let mut object_68 = object.key("userPoolConfigs").start_object();
        crate::json_ser::serialize_structure_crate_model_update_backend_auth_user_pool_config(
            &mut object_68,
            var_67,
        );
        object_68.finish();
    }
}

pub fn serialize_structure_crate_model_login_auth_config_req_obj(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LoginAuthConfigReqObj,
) {
    if let Some(var_69) = &input.aws_cognito_identity_pool_id {
        object.key("aws_cognito_identity_pool_id").string(var_69);
    }
    if let Some(var_70) = &input.aws_cognito_region {
        object.key("aws_cognito_region").string(var_70);
    }
    if let Some(var_71) = &input.aws_user_pools_id {
        object.key("aws_user_pools_id").string(var_71);
    }
    if let Some(var_72) = &input.aws_user_pools_web_client_id {
        object.key("aws_user_pools_web_client_id").string(var_72);
    }
}

pub fn serialize_structure_crate_model_backend_api_auth_type(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BackendApiAuthType,
) {
    if let Some(var_73) = &input.mode {
        object.key("mode").string(var_73.as_str());
    }
    if let Some(var_74) = &input.settings {
        let mut object_75 = object.key("settings").start_object();
        crate::json_ser::serialize_structure_crate_model_backend_api_app_sync_auth_settings(
            &mut object_75,
            var_74,
        );
        object_75.finish();
    }
}

pub fn serialize_structure_crate_model_backend_api_conflict_resolution(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BackendApiConflictResolution,
) {
    if let Some(var_76) = &input.resolution_strategy {
        object.key("resolutionStrategy").string(var_76.as_str());
    }
}

pub fn serialize_structure_crate_model_create_backend_auth_identity_pool_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateBackendAuthIdentityPoolConfig,
) {
    if let Some(var_77) = &input.identity_pool_name {
        object.key("identityPoolName").string(var_77);
    }
    {
        object
            .key("unauthenticatedLogin")
            .boolean(input.unauthenticated_login);
    }
}

pub fn serialize_structure_crate_model_create_backend_auth_user_pool_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateBackendAuthUserPoolConfig,
) {
    if let Some(var_78) = &input.forgot_password {
        let mut object_79 = object.key("forgotPassword").start_object();
        crate::json_ser::serialize_structure_crate_model_create_backend_auth_forgot_password_config(
            &mut object_79,
            var_78,
        );
        object_79.finish();
    }
    if let Some(var_80) = &input.mfa {
        let mut object_81 = object.key("mfa").start_object();
        crate::json_ser::serialize_structure_crate_model_create_backend_auth_mfa_config(
            &mut object_81,
            var_80,
        );
        object_81.finish();
    }
    if let Some(var_82) = &input.o_auth {
        let mut object_83 = object.key("oAuth").start_object();
        crate::json_ser::serialize_structure_crate_model_create_backend_auth_o_auth_config(
            &mut object_83,
            var_82,
        );
        object_83.finish();
    }
    if let Some(var_84) = &input.password_policy {
        let mut object_85 = object.key("passwordPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_create_backend_auth_password_policy_config(
            &mut object_85,
            var_84,
        );
        object_85.finish();
    }
    if let Some(var_86) = &input.required_sign_up_attributes {
        let mut array_87 = object.key("requiredSignUpAttributes").start_array();
        for item_88 in var_86 {
            {
                array_87.value().string(item_88.as_str());
            }
        }
        array_87.finish();
    }
    if let Some(var_89) = &input.sign_in_method {
        object.key("signInMethod").string(var_89.as_str());
    }
    if let Some(var_90) = &input.user_pool_name {
        object.key("userPoolName").string(var_90);
    }
}

pub fn serialize_structure_crate_model_update_backend_auth_identity_pool_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateBackendAuthIdentityPoolConfig,
) {
    if input.unauthenticated_login {
        object
            .key("unauthenticatedLogin")
            .boolean(input.unauthenticated_login);
    }
}

pub fn serialize_structure_crate_model_update_backend_auth_user_pool_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateBackendAuthUserPoolConfig,
) {
    if let Some(var_91) = &input.forgot_password {
        let mut object_92 = object.key("forgotPassword").start_object();
        crate::json_ser::serialize_structure_crate_model_update_backend_auth_forgot_password_config(
            &mut object_92,
            var_91,
        );
        object_92.finish();
    }
    if let Some(var_93) = &input.mfa {
        let mut object_94 = object.key("mfa").start_object();
        crate::json_ser::serialize_structure_crate_model_update_backend_auth_mfa_config(
            &mut object_94,
            var_93,
        );
        object_94.finish();
    }
    if let Some(var_95) = &input.o_auth {
        let mut object_96 = object.key("oAuth").start_object();
        crate::json_ser::serialize_structure_crate_model_update_backend_auth_o_auth_config(
            &mut object_96,
            var_95,
        );
        object_96.finish();
    }
    if let Some(var_97) = &input.password_policy {
        let mut object_98 = object.key("passwordPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_update_backend_auth_password_policy_config(
            &mut object_98,
            var_97,
        );
        object_98.finish();
    }
}

pub fn serialize_structure_crate_model_backend_api_app_sync_auth_settings(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BackendApiAppSyncAuthSettings,
) {
    if let Some(var_99) = &input.cognito_user_pool_id {
        object.key("cognitoUserPoolId").string(var_99);
    }
    if let Some(var_100) = &input.description {
        object.key("description").string(var_100);
    }
    if input.expiration_time != 0.0 {
        object.key("expirationTime").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((input.expiration_time).into()),
        );
    }
    if let Some(var_101) = &input.open_id_auth_ttl {
        object.key("openIDAuthTTL").string(var_101);
    }
    if let Some(var_102) = &input.open_id_client_id {
        object.key("openIDClientId").string(var_102);
    }
    if let Some(var_103) = &input.open_id_iat_ttl {
        object.key("openIDIatTTL").string(var_103);
    }
    if let Some(var_104) = &input.open_id_issue_url {
        object.key("openIDIssueURL").string(var_104);
    }
    if let Some(var_105) = &input.open_id_provider_name {
        object.key("openIDProviderName").string(var_105);
    }
}

pub fn serialize_structure_crate_model_create_backend_auth_forgot_password_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateBackendAuthForgotPasswordConfig,
) {
    if let Some(var_106) = &input.delivery_method {
        object.key("deliveryMethod").string(var_106.as_str());
    }
    if let Some(var_107) = &input.email_settings {
        let mut object_108 = object.key("emailSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_email_settings(&mut object_108, var_107);
        object_108.finish();
    }
    if let Some(var_109) = &input.sms_settings {
        let mut object_110 = object.key("smsSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_sms_settings(&mut object_110, var_109);
        object_110.finish();
    }
}

pub fn serialize_structure_crate_model_create_backend_auth_mfa_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateBackendAuthMfaConfig,
) {
    if let Some(var_111) = &input.mfa_mode {
        object.key("MFAMode").string(var_111.as_str());
    }
    if let Some(var_112) = &input.settings {
        let mut object_113 = object.key("settings").start_object();
        crate::json_ser::serialize_structure_crate_model_settings(&mut object_113, var_112);
        object_113.finish();
    }
}

pub fn serialize_structure_crate_model_create_backend_auth_o_auth_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateBackendAuthOAuthConfig,
) {
    if let Some(var_114) = &input.domain_prefix {
        object.key("domainPrefix").string(var_114);
    }
    if let Some(var_115) = &input.o_auth_grant_type {
        object.key("oAuthGrantType").string(var_115.as_str());
    }
    if let Some(var_116) = &input.o_auth_scopes {
        let mut array_117 = object.key("oAuthScopes").start_array();
        for item_118 in var_116 {
            {
                array_117.value().string(item_118.as_str());
            }
        }
        array_117.finish();
    }
    if let Some(var_119) = &input.redirect_sign_in_ur_is {
        let mut array_120 = object.key("redirectSignInURIs").start_array();
        for item_121 in var_119 {
            {
                array_120.value().string(item_121);
            }
        }
        array_120.finish();
    }
    if let Some(var_122) = &input.redirect_sign_out_ur_is {
        let mut array_123 = object.key("redirectSignOutURIs").start_array();
        for item_124 in var_122 {
            {
                array_123.value().string(item_124);
            }
        }
        array_123.finish();
    }
    if let Some(var_125) = &input.social_provider_settings {
        let mut object_126 = object.key("socialProviderSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_social_provider_settings(
            &mut object_126,
            var_125,
        );
        object_126.finish();
    }
}

pub fn serialize_structure_crate_model_create_backend_auth_password_policy_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateBackendAuthPasswordPolicyConfig,
) {
    if let Some(var_127) = &input.additional_constraints {
        let mut array_128 = object.key("additionalConstraints").start_array();
        for item_129 in var_127 {
            {
                array_128.value().string(item_129.as_str());
            }
        }
        array_128.finish();
    }
    {
        object.key("minimumLength").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((input.minimum_length).into()),
        );
    }
}

pub fn serialize_structure_crate_model_update_backend_auth_forgot_password_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateBackendAuthForgotPasswordConfig,
) {
    if let Some(var_130) = &input.delivery_method {
        object.key("deliveryMethod").string(var_130.as_str());
    }
    if let Some(var_131) = &input.email_settings {
        let mut object_132 = object.key("emailSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_email_settings(&mut object_132, var_131);
        object_132.finish();
    }
    if let Some(var_133) = &input.sms_settings {
        let mut object_134 = object.key("smsSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_sms_settings(&mut object_134, var_133);
        object_134.finish();
    }
}

pub fn serialize_structure_crate_model_update_backend_auth_mfa_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateBackendAuthMfaConfig,
) {
    if let Some(var_135) = &input.mfa_mode {
        object.key("MFAMode").string(var_135.as_str());
    }
    if let Some(var_136) = &input.settings {
        let mut object_137 = object.key("settings").start_object();
        crate::json_ser::serialize_structure_crate_model_settings(&mut object_137, var_136);
        object_137.finish();
    }
}

pub fn serialize_structure_crate_model_update_backend_auth_o_auth_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateBackendAuthOAuthConfig,
) {
    if let Some(var_138) = &input.domain_prefix {
        object.key("domainPrefix").string(var_138);
    }
    if let Some(var_139) = &input.o_auth_grant_type {
        object.key("oAuthGrantType").string(var_139.as_str());
    }
    if let Some(var_140) = &input.o_auth_scopes {
        let mut array_141 = object.key("oAuthScopes").start_array();
        for item_142 in var_140 {
            {
                array_141.value().string(item_142.as_str());
            }
        }
        array_141.finish();
    }
    if let Some(var_143) = &input.redirect_sign_in_ur_is {
        let mut array_144 = object.key("redirectSignInURIs").start_array();
        for item_145 in var_143 {
            {
                array_144.value().string(item_145);
            }
        }
        array_144.finish();
    }
    if let Some(var_146) = &input.redirect_sign_out_ur_is {
        let mut array_147 = object.key("redirectSignOutURIs").start_array();
        for item_148 in var_146 {
            {
                array_147.value().string(item_148);
            }
        }
        array_147.finish();
    }
    if let Some(var_149) = &input.social_provider_settings {
        let mut object_150 = object.key("socialProviderSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_social_provider_settings(
            &mut object_150,
            var_149,
        );
        object_150.finish();
    }
}

pub fn serialize_structure_crate_model_update_backend_auth_password_policy_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateBackendAuthPasswordPolicyConfig,
) {
    if let Some(var_151) = &input.additional_constraints {
        let mut array_152 = object.key("additionalConstraints").start_array();
        for item_153 in var_151 {
            {
                array_152.value().string(item_153.as_str());
            }
        }
        array_152.finish();
    }
    if input.minimum_length != 0.0 {
        object.key("minimumLength").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((input.minimum_length).into()),
        );
    }
}

pub fn serialize_structure_crate_model_email_settings(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EmailSettings,
) {
    if let Some(var_154) = &input.email_message {
        object.key("emailMessage").string(var_154);
    }
    if let Some(var_155) = &input.email_subject {
        object.key("emailSubject").string(var_155);
    }
}

pub fn serialize_structure_crate_model_sms_settings(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SmsSettings,
) {
    if let Some(var_156) = &input.sms_message {
        object.key("smsMessage").string(var_156);
    }
}

pub fn serialize_structure_crate_model_settings(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Settings,
) {
    if let Some(var_157) = &input.mfa_types {
        let mut array_158 = object.key("mfaTypes").start_array();
        for item_159 in var_157 {
            {
                array_158.value().string(item_159.as_str());
            }
        }
        array_158.finish();
    }
    if let Some(var_160) = &input.sms_message {
        object.key("smsMessage").string(var_160);
    }
}

pub fn serialize_structure_crate_model_social_provider_settings(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SocialProviderSettings,
) {
    if let Some(var_161) = &input.facebook {
        let mut object_162 = object.key("Facebook").start_object();
        crate::json_ser::serialize_structure_crate_model_backend_auth_social_provider_config(
            &mut object_162,
            var_161,
        );
        object_162.finish();
    }
    if let Some(var_163) = &input.google {
        let mut object_164 = object.key("Google").start_object();
        crate::json_ser::serialize_structure_crate_model_backend_auth_social_provider_config(
            &mut object_164,
            var_163,
        );
        object_164.finish();
    }
    if let Some(var_165) = &input.login_with_amazon {
        let mut object_166 = object.key("LoginWithAmazon").start_object();
        crate::json_ser::serialize_structure_crate_model_backend_auth_social_provider_config(
            &mut object_166,
            var_165,
        );
        object_166.finish();
    }
    if let Some(var_167) = &input.sign_in_with_apple {
        let mut object_168 = object.key("SignInWithApple").start_object();
        crate::json_ser::serialize_structure_crate_model_backend_auth_apple_provider_config(
            &mut object_168,
            var_167,
        );
        object_168.finish();
    }
}

pub fn serialize_structure_crate_model_backend_auth_social_provider_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BackendAuthSocialProviderConfig,
) {
    if let Some(var_169) = &input.client_id {
        object.key("client_id").string(var_169);
    }
    if let Some(var_170) = &input.client_secret {
        object.key("client_secret").string(var_170);
    }
}

pub fn serialize_structure_crate_model_backend_auth_apple_provider_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BackendAuthAppleProviderConfig,
) {
    if let Some(var_171) = &input.client_id {
        object.key("client_id").string(var_171);
    }
    if let Some(var_172) = &input.key_id {
        object.key("key_id").string(var_172);
    }
    if let Some(var_173) = &input.private_key {
        object.key("private_key").string(var_173);
    }
    if let Some(var_174) = &input.team_id {
        object.key("team_id").string(var_174);
    }
}
