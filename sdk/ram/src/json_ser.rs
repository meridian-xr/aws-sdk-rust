// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_accept_resource_share_invitation_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AcceptResourceShareInvitationInput,
) {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1);
    }
    if let Some(var_2) = &input.resource_share_invitation_arn {
        object.key("resourceShareInvitationArn").string(var_2);
    }
}

pub fn serialize_structure_crate_input_associate_resource_share_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateResourceShareInput,
) {
    if let Some(var_3) = &input.client_token {
        object.key("clientToken").string(var_3);
    }
    if let Some(var_4) = &input.principals {
        let mut array_5 = object.key("principals").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6);
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.resource_arns {
        let mut array_8 = object.key("resourceArns").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9);
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.resource_share_arn {
        object.key("resourceShareArn").string(var_10);
    }
}

pub fn serialize_structure_crate_input_associate_resource_share_permission_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateResourceSharePermissionInput,
) {
    if let Some(var_11) = &input.client_token {
        object.key("clientToken").string(var_11);
    }
    if let Some(var_12) = &input.permission_arn {
        object.key("permissionArn").string(var_12);
    }
    if let Some(var_13) = &input.permission_version {
        object.key("permissionVersion").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_13).into()),
        );
    }
    if let Some(var_14) = &input.replace {
        object.key("replace").boolean(*var_14);
    }
    if let Some(var_15) = &input.resource_share_arn {
        object.key("resourceShareArn").string(var_15);
    }
}

pub fn serialize_structure_crate_input_create_resource_share_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateResourceShareInput,
) {
    if let Some(var_16) = &input.allow_external_principals {
        object.key("allowExternalPrincipals").boolean(*var_16);
    }
    if let Some(var_17) = &input.client_token {
        object.key("clientToken").string(var_17);
    }
    if let Some(var_18) = &input.name {
        object.key("name").string(var_18);
    }
    if let Some(var_19) = &input.permission_arns {
        let mut array_20 = object.key("permissionArns").start_array();
        for item_21 in var_19 {
            {
                array_20.value().string(item_21);
            }
        }
        array_20.finish();
    }
    if let Some(var_22) = &input.principals {
        let mut array_23 = object.key("principals").start_array();
        for item_24 in var_22 {
            {
                array_23.value().string(item_24);
            }
        }
        array_23.finish();
    }
    if let Some(var_25) = &input.resource_arns {
        let mut array_26 = object.key("resourceArns").start_array();
        for item_27 in var_25 {
            {
                array_26.value().string(item_27);
            }
        }
        array_26.finish();
    }
    if let Some(var_28) = &input.tags {
        let mut array_29 = object.key("tags").start_array();
        for item_30 in var_28 {
            {
                let mut object_31 = array_29.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_31, item_30);
                object_31.finish();
            }
        }
        array_29.finish();
    }
}

pub fn serialize_structure_crate_input_disassociate_resource_share_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateResourceShareInput,
) {
    if let Some(var_32) = &input.client_token {
        object.key("clientToken").string(var_32);
    }
    if let Some(var_33) = &input.principals {
        let mut array_34 = object.key("principals").start_array();
        for item_35 in var_33 {
            {
                array_34.value().string(item_35);
            }
        }
        array_34.finish();
    }
    if let Some(var_36) = &input.resource_arns {
        let mut array_37 = object.key("resourceArns").start_array();
        for item_38 in var_36 {
            {
                array_37.value().string(item_38);
            }
        }
        array_37.finish();
    }
    if let Some(var_39) = &input.resource_share_arn {
        object.key("resourceShareArn").string(var_39);
    }
}

pub fn serialize_structure_crate_input_disassociate_resource_share_permission_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateResourceSharePermissionInput,
) {
    if let Some(var_40) = &input.client_token {
        object.key("clientToken").string(var_40);
    }
    if let Some(var_41) = &input.permission_arn {
        object.key("permissionArn").string(var_41);
    }
    if let Some(var_42) = &input.resource_share_arn {
        object.key("resourceShareArn").string(var_42);
    }
}

pub fn serialize_structure_crate_input_get_permission_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetPermissionInput,
) {
    if let Some(var_43) = &input.permission_arn {
        object.key("permissionArn").string(var_43);
    }
    if let Some(var_44) = &input.permission_version {
        object.key("permissionVersion").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_44).into()),
        );
    }
}

pub fn serialize_structure_crate_input_get_resource_policies_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetResourcePoliciesInput,
) {
    if let Some(var_45) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_45).into()),
        );
    }
    if let Some(var_46) = &input.next_token {
        object.key("nextToken").string(var_46);
    }
    if let Some(var_47) = &input.principal {
        object.key("principal").string(var_47);
    }
    if let Some(var_48) = &input.resource_arns {
        let mut array_49 = object.key("resourceArns").start_array();
        for item_50 in var_48 {
            {
                array_49.value().string(item_50);
            }
        }
        array_49.finish();
    }
}

pub fn serialize_structure_crate_input_get_resource_share_associations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetResourceShareAssociationsInput,
) {
    if let Some(var_51) = &input.association_status {
        object.key("associationStatus").string(var_51.as_str());
    }
    if let Some(var_52) = &input.association_type {
        object.key("associationType").string(var_52.as_str());
    }
    if let Some(var_53) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_53).into()),
        );
    }
    if let Some(var_54) = &input.next_token {
        object.key("nextToken").string(var_54);
    }
    if let Some(var_55) = &input.principal {
        object.key("principal").string(var_55);
    }
    if let Some(var_56) = &input.resource_arn {
        object.key("resourceArn").string(var_56);
    }
    if let Some(var_57) = &input.resource_share_arns {
        let mut array_58 = object.key("resourceShareArns").start_array();
        for item_59 in var_57 {
            {
                array_58.value().string(item_59);
            }
        }
        array_58.finish();
    }
}

pub fn serialize_structure_crate_input_get_resource_share_invitations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetResourceShareInvitationsInput,
) {
    if let Some(var_60) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_60).into()),
        );
    }
    if let Some(var_61) = &input.next_token {
        object.key("nextToken").string(var_61);
    }
    if let Some(var_62) = &input.resource_share_arns {
        let mut array_63 = object.key("resourceShareArns").start_array();
        for item_64 in var_62 {
            {
                array_63.value().string(item_64);
            }
        }
        array_63.finish();
    }
    if let Some(var_65) = &input.resource_share_invitation_arns {
        let mut array_66 = object.key("resourceShareInvitationArns").start_array();
        for item_67 in var_65 {
            {
                array_66.value().string(item_67);
            }
        }
        array_66.finish();
    }
}

pub fn serialize_structure_crate_input_get_resource_shares_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetResourceSharesInput,
) {
    if let Some(var_68) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_68).into()),
        );
    }
    if let Some(var_69) = &input.name {
        object.key("name").string(var_69);
    }
    if let Some(var_70) = &input.next_token {
        object.key("nextToken").string(var_70);
    }
    if let Some(var_71) = &input.permission_arn {
        object.key("permissionArn").string(var_71);
    }
    if let Some(var_72) = &input.resource_owner {
        object.key("resourceOwner").string(var_72.as_str());
    }
    if let Some(var_73) = &input.resource_share_arns {
        let mut array_74 = object.key("resourceShareArns").start_array();
        for item_75 in var_73 {
            {
                array_74.value().string(item_75);
            }
        }
        array_74.finish();
    }
    if let Some(var_76) = &input.resource_share_status {
        object.key("resourceShareStatus").string(var_76.as_str());
    }
    if let Some(var_77) = &input.tag_filters {
        let mut array_78 = object.key("tagFilters").start_array();
        for item_79 in var_77 {
            {
                let mut object_80 = array_78.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag_filter(
                    &mut object_80,
                    item_79,
                );
                object_80.finish();
            }
        }
        array_78.finish();
    }
}

pub fn serialize_structure_crate_input_list_pending_invitation_resources_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPendingInvitationResourcesInput,
) {
    if let Some(var_81) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_81).into()),
        );
    }
    if let Some(var_82) = &input.next_token {
        object.key("nextToken").string(var_82);
    }
    if let Some(var_83) = &input.resource_share_invitation_arn {
        object.key("resourceShareInvitationArn").string(var_83);
    }
}

pub fn serialize_structure_crate_input_list_permissions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPermissionsInput,
) {
    if let Some(var_84) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_84).into()),
        );
    }
    if let Some(var_85) = &input.next_token {
        object.key("nextToken").string(var_85);
    }
    if let Some(var_86) = &input.resource_type {
        object.key("resourceType").string(var_86);
    }
}

pub fn serialize_structure_crate_input_list_principals_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPrincipalsInput,
) {
    if let Some(var_87) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_87).into()),
        );
    }
    if let Some(var_88) = &input.next_token {
        object.key("nextToken").string(var_88);
    }
    if let Some(var_89) = &input.principals {
        let mut array_90 = object.key("principals").start_array();
        for item_91 in var_89 {
            {
                array_90.value().string(item_91);
            }
        }
        array_90.finish();
    }
    if let Some(var_92) = &input.resource_arn {
        object.key("resourceArn").string(var_92);
    }
    if let Some(var_93) = &input.resource_owner {
        object.key("resourceOwner").string(var_93.as_str());
    }
    if let Some(var_94) = &input.resource_share_arns {
        let mut array_95 = object.key("resourceShareArns").start_array();
        for item_96 in var_94 {
            {
                array_95.value().string(item_96);
            }
        }
        array_95.finish();
    }
    if let Some(var_97) = &input.resource_type {
        object.key("resourceType").string(var_97);
    }
}

pub fn serialize_structure_crate_input_list_resources_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListResourcesInput,
) {
    if let Some(var_98) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_98).into()),
        );
    }
    if let Some(var_99) = &input.next_token {
        object.key("nextToken").string(var_99);
    }
    if let Some(var_100) = &input.principal {
        object.key("principal").string(var_100);
    }
    if let Some(var_101) = &input.resource_arns {
        let mut array_102 = object.key("resourceArns").start_array();
        for item_103 in var_101 {
            {
                array_102.value().string(item_103);
            }
        }
        array_102.finish();
    }
    if let Some(var_104) = &input.resource_owner {
        object.key("resourceOwner").string(var_104.as_str());
    }
    if let Some(var_105) = &input.resource_share_arns {
        let mut array_106 = object.key("resourceShareArns").start_array();
        for item_107 in var_105 {
            {
                array_106.value().string(item_107);
            }
        }
        array_106.finish();
    }
    if let Some(var_108) = &input.resource_type {
        object.key("resourceType").string(var_108);
    }
}

pub fn serialize_structure_crate_input_list_resource_share_permissions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListResourceSharePermissionsInput,
) {
    if let Some(var_109) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_109).into()),
        );
    }
    if let Some(var_110) = &input.next_token {
        object.key("nextToken").string(var_110);
    }
    if let Some(var_111) = &input.resource_share_arn {
        object.key("resourceShareArn").string(var_111);
    }
}

pub fn serialize_structure_crate_input_list_resource_types_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListResourceTypesInput,
) {
    if let Some(var_112) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_112).into()),
        );
    }
    if let Some(var_113) = &input.next_token {
        object.key("nextToken").string(var_113);
    }
}

pub fn serialize_structure_crate_input_reject_resource_share_invitation_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RejectResourceShareInvitationInput,
) {
    if let Some(var_114) = &input.client_token {
        object.key("clientToken").string(var_114);
    }
    if let Some(var_115) = &input.resource_share_invitation_arn {
        object.key("resourceShareInvitationArn").string(var_115);
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_116) = &input.resource_share_arn {
        object.key("resourceShareArn").string(var_116);
    }
    if let Some(var_117) = &input.tags {
        let mut array_118 = object.key("tags").start_array();
        for item_119 in var_117 {
            {
                let mut object_120 = array_118.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_120, item_119);
                object_120.finish();
            }
        }
        array_118.finish();
    }
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_121) = &input.resource_share_arn {
        object.key("resourceShareArn").string(var_121);
    }
    if let Some(var_122) = &input.tag_keys {
        let mut array_123 = object.key("tagKeys").start_array();
        for item_124 in var_122 {
            {
                array_123.value().string(item_124);
            }
        }
        array_123.finish();
    }
}

pub fn serialize_structure_crate_input_update_resource_share_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateResourceShareInput,
) {
    if let Some(var_125) = &input.allow_external_principals {
        object.key("allowExternalPrincipals").boolean(*var_125);
    }
    if let Some(var_126) = &input.client_token {
        object.key("clientToken").string(var_126);
    }
    if let Some(var_127) = &input.name {
        object.key("name").string(var_127);
    }
    if let Some(var_128) = &input.resource_share_arn {
        object.key("resourceShareArn").string(var_128);
    }
}

pub fn serialize_structure_crate_model_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_129) = &input.key {
        object.key("key").string(var_129);
    }
    if let Some(var_130) = &input.value {
        object.key("value").string(var_130);
    }
}

pub fn serialize_structure_crate_model_tag_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TagFilter,
) {
    if let Some(var_131) = &input.tag_key {
        object.key("tagKey").string(var_131);
    }
    if let Some(var_132) = &input.tag_values {
        let mut array_133 = object.key("tagValues").start_array();
        for item_134 in var_132 {
            {
                array_133.value().string(item_134);
            }
        }
        array_133.finish();
    }
}
