// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_customization_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_customization::CreateCustomizationInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.data_reference {
        #[allow(unused_mut)]
        let mut object_2 = object.key("dataReference").start_object();
        crate::protocol_serde::shape_data_reference::ser_data_reference(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.customization_name {
        object.key("customizationName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.description {
        object.key("description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.profile_arn {
        object.key("profileArn").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        let mut array_7 = object.key("tags").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.client_token {
        object.key("clientToken").string(var_10.as_str());
    }
    if let Some(var_11) = &input.include_repos {
        let mut array_12 = object.key("includeRepos").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    Ok(())
}
