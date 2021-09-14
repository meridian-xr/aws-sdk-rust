// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_get_ice_server_config_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetIceServerConfigInput,
) {
    if let Some(var_1) = &input.channel_arn {
        object.key("ChannelARN").string(var_1);
    }
    if let Some(var_2) = &input.client_id {
        object.key("ClientId").string(var_2);
    }
    if let Some(var_3) = &input.service {
        object.key("Service").string(var_3.as_str());
    }
    if let Some(var_4) = &input.username {
        object.key("Username").string(var_4);
    }
}

pub fn serialize_structure_crate_input_send_alexa_offer_to_master_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SendAlexaOfferToMasterInput,
) {
    if let Some(var_5) = &input.channel_arn {
        object.key("ChannelARN").string(var_5);
    }
    if let Some(var_6) = &input.message_payload {
        object.key("MessagePayload").string(var_6);
    }
    if let Some(var_7) = &input.sender_client_id {
        object.key("SenderClientId").string(var_7);
    }
}
