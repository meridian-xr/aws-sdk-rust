// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_cancel_quantum_task(
    input: &crate::input::CancelQuantumTaskInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_cancel_quantum_task_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_quantum_task(
    input: &crate::input::CreateQuantumTaskInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_create_quantum_task_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_search_devices(
    input: &crate::input::SearchDevicesInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_search_devices_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_search_quantum_tasks(
    input: &crate::input::SearchQuantumTasksInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_search_quantum_tasks_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_tag_resource(
    input: &crate::input::TagResourceInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_tag_resource_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}
