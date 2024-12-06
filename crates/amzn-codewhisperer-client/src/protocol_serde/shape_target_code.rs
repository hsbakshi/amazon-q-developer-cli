// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_target_code(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TargetCode,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object
            .key("relativeTargetPath")
            .string(input.relative_target_path.as_str());
    }
    if let Some(var_1) = &input.target_line_range_list {
        let mut array_2 = object.key("targetLineRangeList").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_range::ser_range(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}
