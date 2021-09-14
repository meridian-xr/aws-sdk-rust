// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_certificate_authority_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCertificateAuthorityInput,
) {
    if let Some(var_1) = &input.certificate_authority_configuration {
        let mut object_2 = object
            .key("CertificateAuthorityConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_certificate_authority_configuration(
            &mut object_2,
            var_1,
        );
        object_2.finish();
    }
    if let Some(var_3) = &input.revocation_configuration {
        let mut object_4 = object.key("RevocationConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_revocation_configuration(
            &mut object_4,
            var_3,
        );
        object_4.finish();
    }
    if let Some(var_5) = &input.certificate_authority_type {
        object
            .key("CertificateAuthorityType")
            .string(var_5.as_str());
    }
    if let Some(var_6) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_6);
    }
    if let Some(var_7) = &input.key_storage_security_standard {
        object
            .key("KeyStorageSecurityStandard")
            .string(var_7.as_str());
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("Tags").start_array();
        for item_10 in var_8 {
            {
                let mut object_11 = array_9.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_11, item_10);
                object_11.finish();
            }
        }
        array_9.finish();
    }
}

pub fn serialize_structure_crate_input_create_certificate_authority_audit_report_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCertificateAuthorityAuditReportInput,
) {
    if let Some(var_12) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_12);
    }
    if let Some(var_13) = &input.s3_bucket_name {
        object.key("S3BucketName").string(var_13);
    }
    if let Some(var_14) = &input.audit_report_response_format {
        object
            .key("AuditReportResponseFormat")
            .string(var_14.as_str());
    }
}

pub fn serialize_structure_crate_input_create_permission_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePermissionInput,
) {
    if let Some(var_15) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_15);
    }
    if let Some(var_16) = &input.principal {
        object.key("Principal").string(var_16);
    }
    if let Some(var_17) = &input.source_account {
        object.key("SourceAccount").string(var_17);
    }
    if let Some(var_18) = &input.actions {
        let mut array_19 = object.key("Actions").start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20.as_str());
            }
        }
        array_19.finish();
    }
}

pub fn serialize_structure_crate_input_delete_certificate_authority_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteCertificateAuthorityInput,
) {
    if let Some(var_21) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_21);
    }
    if let Some(var_22) = &input.permanent_deletion_time_in_days {
        object.key("PermanentDeletionTimeInDays").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_22).into()),
        );
    }
}

pub fn serialize_structure_crate_input_delete_permission_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeletePermissionInput,
) {
    if let Some(var_23) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_23);
    }
    if let Some(var_24) = &input.principal {
        object.key("Principal").string(var_24);
    }
    if let Some(var_25) = &input.source_account {
        object.key("SourceAccount").string(var_25);
    }
}

pub fn serialize_structure_crate_input_delete_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeletePolicyInput,
) {
    if let Some(var_26) = &input.resource_arn {
        object.key("ResourceArn").string(var_26);
    }
}

pub fn serialize_structure_crate_input_describe_certificate_authority_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeCertificateAuthorityInput,
) {
    if let Some(var_27) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_27);
    }
}

pub fn serialize_structure_crate_input_describe_certificate_authority_audit_report_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeCertificateAuthorityAuditReportInput,
) {
    if let Some(var_28) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_28);
    }
    if let Some(var_29) = &input.audit_report_id {
        object.key("AuditReportId").string(var_29);
    }
}

pub fn serialize_structure_crate_input_get_certificate_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetCertificateInput,
) {
    if let Some(var_30) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_30);
    }
    if let Some(var_31) = &input.certificate_arn {
        object.key("CertificateArn").string(var_31);
    }
}

pub fn serialize_structure_crate_input_get_certificate_authority_certificate_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetCertificateAuthorityCertificateInput,
) {
    if let Some(var_32) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_32);
    }
}

pub fn serialize_structure_crate_input_get_certificate_authority_csr_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetCertificateAuthorityCsrInput,
) {
    if let Some(var_33) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_33);
    }
}

pub fn serialize_structure_crate_input_get_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetPolicyInput,
) {
    if let Some(var_34) = &input.resource_arn {
        object.key("ResourceArn").string(var_34);
    }
}

pub fn serialize_structure_crate_input_import_certificate_authority_certificate_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ImportCertificateAuthorityCertificateInput,
) {
    if let Some(var_35) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_35);
    }
    if let Some(var_36) = &input.certificate {
        object
            .key("Certificate")
            .string_unchecked(&smithy_types::base64::encode(var_36));
    }
    if let Some(var_37) = &input.certificate_chain {
        object
            .key("CertificateChain")
            .string_unchecked(&smithy_types::base64::encode(var_37));
    }
}

pub fn serialize_structure_crate_input_issue_certificate_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::IssueCertificateInput,
) {
    if let Some(var_38) = &input.api_passthrough {
        let mut object_39 = object.key("ApiPassthrough").start_object();
        crate::json_ser::serialize_structure_crate_model_api_passthrough(&mut object_39, var_38);
        object_39.finish();
    }
    if let Some(var_40) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_40);
    }
    if let Some(var_41) = &input.csr {
        object
            .key("Csr")
            .string_unchecked(&smithy_types::base64::encode(var_41));
    }
    if let Some(var_42) = &input.signing_algorithm {
        object.key("SigningAlgorithm").string(var_42.as_str());
    }
    if let Some(var_43) = &input.template_arn {
        object.key("TemplateArn").string(var_43);
    }
    if let Some(var_44) = &input.validity {
        let mut object_45 = object.key("Validity").start_object();
        crate::json_ser::serialize_structure_crate_model_validity(&mut object_45, var_44);
        object_45.finish();
    }
    if let Some(var_46) = &input.validity_not_before {
        let mut object_47 = object.key("ValidityNotBefore").start_object();
        crate::json_ser::serialize_structure_crate_model_validity(&mut object_47, var_46);
        object_47.finish();
    }
    if let Some(var_48) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_48);
    }
}

pub fn serialize_structure_crate_input_list_certificate_authorities_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListCertificateAuthoritiesInput,
) {
    if let Some(var_49) = &input.next_token {
        object.key("NextToken").string(var_49);
    }
    if let Some(var_50) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_50).into()),
        );
    }
    if let Some(var_51) = &input.resource_owner {
        object.key("ResourceOwner").string(var_51.as_str());
    }
}

pub fn serialize_structure_crate_input_list_permissions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPermissionsInput,
) {
    if let Some(var_52) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_52);
    }
    if let Some(var_53) = &input.next_token {
        object.key("NextToken").string(var_53);
    }
    if let Some(var_54) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_54).into()),
        );
    }
}

pub fn serialize_structure_crate_input_list_tags_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsInput,
) {
    if let Some(var_55) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_55);
    }
    if let Some(var_56) = &input.next_token {
        object.key("NextToken").string(var_56);
    }
    if let Some(var_57) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_57).into()),
        );
    }
}

pub fn serialize_structure_crate_input_put_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutPolicyInput,
) {
    if let Some(var_58) = &input.resource_arn {
        object.key("ResourceArn").string(var_58);
    }
    if let Some(var_59) = &input.policy {
        object.key("Policy").string(var_59);
    }
}

pub fn serialize_structure_crate_input_restore_certificate_authority_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RestoreCertificateAuthorityInput,
) {
    if let Some(var_60) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_60);
    }
}

pub fn serialize_structure_crate_input_revoke_certificate_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RevokeCertificateInput,
) {
    if let Some(var_61) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_61);
    }
    if let Some(var_62) = &input.certificate_serial {
        object.key("CertificateSerial").string(var_62);
    }
    if let Some(var_63) = &input.revocation_reason {
        object.key("RevocationReason").string(var_63.as_str());
    }
}

pub fn serialize_structure_crate_input_tag_certificate_authority_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagCertificateAuthorityInput,
) {
    if let Some(var_64) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_64);
    }
    if let Some(var_65) = &input.tags {
        let mut array_66 = object.key("Tags").start_array();
        for item_67 in var_65 {
            {
                let mut object_68 = array_66.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_68, item_67);
                object_68.finish();
            }
        }
        array_66.finish();
    }
}

pub fn serialize_structure_crate_input_untag_certificate_authority_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagCertificateAuthorityInput,
) {
    if let Some(var_69) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_69);
    }
    if let Some(var_70) = &input.tags {
        let mut array_71 = object.key("Tags").start_array();
        for item_72 in var_70 {
            {
                let mut object_73 = array_71.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_73, item_72);
                object_73.finish();
            }
        }
        array_71.finish();
    }
}

pub fn serialize_structure_crate_input_update_certificate_authority_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCertificateAuthorityInput,
) {
    if let Some(var_74) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_74);
    }
    if let Some(var_75) = &input.revocation_configuration {
        let mut object_76 = object.key("RevocationConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_revocation_configuration(
            &mut object_76,
            var_75,
        );
        object_76.finish();
    }
    if let Some(var_77) = &input.status {
        object.key("Status").string(var_77.as_str());
    }
}

pub fn serialize_structure_crate_model_certificate_authority_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CertificateAuthorityConfiguration,
) {
    if let Some(var_78) = &input.key_algorithm {
        object.key("KeyAlgorithm").string(var_78.as_str());
    }
    if let Some(var_79) = &input.signing_algorithm {
        object.key("SigningAlgorithm").string(var_79.as_str());
    }
    if let Some(var_80) = &input.subject {
        let mut object_81 = object.key("Subject").start_object();
        crate::json_ser::serialize_structure_crate_model_asn1_subject(&mut object_81, var_80);
        object_81.finish();
    }
    if let Some(var_82) = &input.csr_extensions {
        let mut object_83 = object.key("CsrExtensions").start_object();
        crate::json_ser::serialize_structure_crate_model_csr_extensions(&mut object_83, var_82);
        object_83.finish();
    }
}

pub fn serialize_structure_crate_model_revocation_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RevocationConfiguration,
) {
    if let Some(var_84) = &input.crl_configuration {
        let mut object_85 = object.key("CrlConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_crl_configuration(&mut object_85, var_84);
        object_85.finish();
    }
    if let Some(var_86) = &input.ocsp_configuration {
        let mut object_87 = object.key("OcspConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_ocsp_configuration(&mut object_87, var_86);
        object_87.finish();
    }
}

pub fn serialize_structure_crate_model_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_88) = &input.key {
        object.key("Key").string(var_88);
    }
    if let Some(var_89) = &input.value {
        object.key("Value").string(var_89);
    }
}

pub fn serialize_structure_crate_model_api_passthrough(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ApiPassthrough,
) {
    if let Some(var_90) = &input.extensions {
        let mut object_91 = object.key("Extensions").start_object();
        crate::json_ser::serialize_structure_crate_model_extensions(&mut object_91, var_90);
        object_91.finish();
    }
    if let Some(var_92) = &input.subject {
        let mut object_93 = object.key("Subject").start_object();
        crate::json_ser::serialize_structure_crate_model_asn1_subject(&mut object_93, var_92);
        object_93.finish();
    }
}

pub fn serialize_structure_crate_model_validity(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Validity,
) {
    if let Some(var_94) = &input.value {
        object.key("Value").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_94).into()),
        );
    }
    if let Some(var_95) = &input.r#type {
        object.key("Type").string(var_95.as_str());
    }
}

pub fn serialize_structure_crate_model_asn1_subject(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Asn1Subject,
) {
    if let Some(var_96) = &input.country {
        object.key("Country").string(var_96);
    }
    if let Some(var_97) = &input.organization {
        object.key("Organization").string(var_97);
    }
    if let Some(var_98) = &input.organizational_unit {
        object.key("OrganizationalUnit").string(var_98);
    }
    if let Some(var_99) = &input.distinguished_name_qualifier {
        object.key("DistinguishedNameQualifier").string(var_99);
    }
    if let Some(var_100) = &input.state {
        object.key("State").string(var_100);
    }
    if let Some(var_101) = &input.common_name {
        object.key("CommonName").string(var_101);
    }
    if let Some(var_102) = &input.serial_number {
        object.key("SerialNumber").string(var_102);
    }
    if let Some(var_103) = &input.locality {
        object.key("Locality").string(var_103);
    }
    if let Some(var_104) = &input.title {
        object.key("Title").string(var_104);
    }
    if let Some(var_105) = &input.surname {
        object.key("Surname").string(var_105);
    }
    if let Some(var_106) = &input.given_name {
        object.key("GivenName").string(var_106);
    }
    if let Some(var_107) = &input.initials {
        object.key("Initials").string(var_107);
    }
    if let Some(var_108) = &input.pseudonym {
        object.key("Pseudonym").string(var_108);
    }
    if let Some(var_109) = &input.generation_qualifier {
        object.key("GenerationQualifier").string(var_109);
    }
}

pub fn serialize_structure_crate_model_csr_extensions(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CsrExtensions,
) {
    if let Some(var_110) = &input.key_usage {
        let mut object_111 = object.key("KeyUsage").start_object();
        crate::json_ser::serialize_structure_crate_model_key_usage(&mut object_111, var_110);
        object_111.finish();
    }
    if let Some(var_112) = &input.subject_information_access {
        let mut array_113 = object.key("SubjectInformationAccess").start_array();
        for item_114 in var_112 {
            {
                let mut object_115 = array_113.value().start_object();
                crate::json_ser::serialize_structure_crate_model_access_description(
                    &mut object_115,
                    item_114,
                );
                object_115.finish();
            }
        }
        array_113.finish();
    }
}

pub fn serialize_structure_crate_model_crl_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CrlConfiguration,
) {
    if let Some(var_116) = &input.enabled {
        object.key("Enabled").boolean(*var_116);
    }
    if let Some(var_117) = &input.expiration_in_days {
        object.key("ExpirationInDays").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_117).into()),
        );
    }
    if let Some(var_118) = &input.custom_cname {
        object.key("CustomCname").string(var_118);
    }
    if let Some(var_119) = &input.s3_bucket_name {
        object.key("S3BucketName").string(var_119);
    }
    if let Some(var_120) = &input.s3_object_acl {
        object.key("S3ObjectAcl").string(var_120.as_str());
    }
}

pub fn serialize_structure_crate_model_ocsp_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OcspConfiguration,
) {
    if let Some(var_121) = &input.enabled {
        object.key("Enabled").boolean(*var_121);
    }
    if let Some(var_122) = &input.ocsp_custom_cname {
        object.key("OcspCustomCname").string(var_122);
    }
}

pub fn serialize_structure_crate_model_extensions(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Extensions,
) {
    if let Some(var_123) = &input.certificate_policies {
        let mut array_124 = object.key("CertificatePolicies").start_array();
        for item_125 in var_123 {
            {
                let mut object_126 = array_124.value().start_object();
                crate::json_ser::serialize_structure_crate_model_policy_information(
                    &mut object_126,
                    item_125,
                );
                object_126.finish();
            }
        }
        array_124.finish();
    }
    if let Some(var_127) = &input.extended_key_usage {
        let mut array_128 = object.key("ExtendedKeyUsage").start_array();
        for item_129 in var_127 {
            {
                let mut object_130 = array_128.value().start_object();
                crate::json_ser::serialize_structure_crate_model_extended_key_usage(
                    &mut object_130,
                    item_129,
                );
                object_130.finish();
            }
        }
        array_128.finish();
    }
    if let Some(var_131) = &input.key_usage {
        let mut object_132 = object.key("KeyUsage").start_object();
        crate::json_ser::serialize_structure_crate_model_key_usage(&mut object_132, var_131);
        object_132.finish();
    }
    if let Some(var_133) = &input.subject_alternative_names {
        let mut array_134 = object.key("SubjectAlternativeNames").start_array();
        for item_135 in var_133 {
            {
                let mut object_136 = array_134.value().start_object();
                crate::json_ser::serialize_structure_crate_model_general_name(
                    &mut object_136,
                    item_135,
                );
                object_136.finish();
            }
        }
        array_134.finish();
    }
}

pub fn serialize_structure_crate_model_key_usage(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KeyUsage,
) {
    if input.digital_signature {
        object
            .key("DigitalSignature")
            .boolean(input.digital_signature);
    }
    if input.non_repudiation {
        object.key("NonRepudiation").boolean(input.non_repudiation);
    }
    if input.key_encipherment {
        object
            .key("KeyEncipherment")
            .boolean(input.key_encipherment);
    }
    if input.data_encipherment {
        object
            .key("DataEncipherment")
            .boolean(input.data_encipherment);
    }
    if input.key_agreement {
        object.key("KeyAgreement").boolean(input.key_agreement);
    }
    if input.key_cert_sign {
        object.key("KeyCertSign").boolean(input.key_cert_sign);
    }
    if input.crl_sign {
        object.key("CRLSign").boolean(input.crl_sign);
    }
    if input.encipher_only {
        object.key("EncipherOnly").boolean(input.encipher_only);
    }
    if input.decipher_only {
        object.key("DecipherOnly").boolean(input.decipher_only);
    }
}

pub fn serialize_structure_crate_model_access_description(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AccessDescription,
) {
    if let Some(var_137) = &input.access_method {
        let mut object_138 = object.key("AccessMethod").start_object();
        crate::json_ser::serialize_structure_crate_model_access_method(&mut object_138, var_137);
        object_138.finish();
    }
    if let Some(var_139) = &input.access_location {
        let mut object_140 = object.key("AccessLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_general_name(&mut object_140, var_139);
        object_140.finish();
    }
}

pub fn serialize_structure_crate_model_policy_information(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PolicyInformation,
) {
    if let Some(var_141) = &input.cert_policy_id {
        object.key("CertPolicyId").string(var_141);
    }
    if let Some(var_142) = &input.policy_qualifiers {
        let mut array_143 = object.key("PolicyQualifiers").start_array();
        for item_144 in var_142 {
            {
                let mut object_145 = array_143.value().start_object();
                crate::json_ser::serialize_structure_crate_model_policy_qualifier_info(
                    &mut object_145,
                    item_144,
                );
                object_145.finish();
            }
        }
        array_143.finish();
    }
}

pub fn serialize_structure_crate_model_extended_key_usage(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ExtendedKeyUsage,
) {
    if let Some(var_146) = &input.extended_key_usage_type {
        object.key("ExtendedKeyUsageType").string(var_146.as_str());
    }
    if let Some(var_147) = &input.extended_key_usage_object_identifier {
        object
            .key("ExtendedKeyUsageObjectIdentifier")
            .string(var_147);
    }
}

pub fn serialize_structure_crate_model_general_name(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::GeneralName,
) {
    if let Some(var_148) = &input.other_name {
        let mut object_149 = object.key("OtherName").start_object();
        crate::json_ser::serialize_structure_crate_model_other_name(&mut object_149, var_148);
        object_149.finish();
    }
    if let Some(var_150) = &input.rfc822_name {
        object.key("Rfc822Name").string(var_150);
    }
    if let Some(var_151) = &input.dns_name {
        object.key("DnsName").string(var_151);
    }
    if let Some(var_152) = &input.directory_name {
        let mut object_153 = object.key("DirectoryName").start_object();
        crate::json_ser::serialize_structure_crate_model_asn1_subject(&mut object_153, var_152);
        object_153.finish();
    }
    if let Some(var_154) = &input.edi_party_name {
        let mut object_155 = object.key("EdiPartyName").start_object();
        crate::json_ser::serialize_structure_crate_model_edi_party_name(&mut object_155, var_154);
        object_155.finish();
    }
    if let Some(var_156) = &input.uniform_resource_identifier {
        object.key("UniformResourceIdentifier").string(var_156);
    }
    if let Some(var_157) = &input.ip_address {
        object.key("IpAddress").string(var_157);
    }
    if let Some(var_158) = &input.registered_id {
        object.key("RegisteredId").string(var_158);
    }
}

pub fn serialize_structure_crate_model_access_method(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AccessMethod,
) {
    if let Some(var_159) = &input.custom_object_identifier {
        object.key("CustomObjectIdentifier").string(var_159);
    }
    if let Some(var_160) = &input.access_method_type {
        object.key("AccessMethodType").string(var_160.as_str());
    }
}

pub fn serialize_structure_crate_model_policy_qualifier_info(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PolicyQualifierInfo,
) {
    if let Some(var_161) = &input.policy_qualifier_id {
        object.key("PolicyQualifierId").string(var_161.as_str());
    }
    if let Some(var_162) = &input.qualifier {
        let mut object_163 = object.key("Qualifier").start_object();
        crate::json_ser::serialize_structure_crate_model_qualifier(&mut object_163, var_162);
        object_163.finish();
    }
}

pub fn serialize_structure_crate_model_other_name(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OtherName,
) {
    if let Some(var_164) = &input.type_id {
        object.key("TypeId").string(var_164);
    }
    if let Some(var_165) = &input.value {
        object.key("Value").string(var_165);
    }
}

pub fn serialize_structure_crate_model_edi_party_name(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EdiPartyName,
) {
    if let Some(var_166) = &input.party_name {
        object.key("PartyName").string(var_166);
    }
    if let Some(var_167) = &input.name_assigner {
        object.key("NameAssigner").string(var_167);
    }
}

pub fn serialize_structure_crate_model_qualifier(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Qualifier,
) {
    if let Some(var_168) = &input.cps_uri {
        object.key("CpsUri").string(var_168);
    }
}
