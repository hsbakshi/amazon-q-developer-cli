// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_transformation_project_artifact_descriptor(
    object_7: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TransformationProjectArtifactDescriptor,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::TransformationProjectArtifactDescriptor::SourceCodeArtifact(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_7.key("sourceCodeArtifact").start_object();
            crate::protocol_serde::shape_transformation_source_code_artifact_descriptor::ser_transformation_source_code_artifact_descriptor(
                &mut object_1,
                inner,
            )?;
            object_1.finish();
        },
        crate::types::TransformationProjectArtifactDescriptor::Unknown => {
            return Err(
                ::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                    "TransformationProjectArtifactDescriptor",
                ),
            );
        },
    }
    Ok(())
}

pub(crate) fn de_transformation_project_artifact_descriptor<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::TransformationProjectArtifactDescriptor>,
    ::aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<
        Item = Result<
            ::aws_smithy_json::deserialize::Token<'a>,
            ::aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => loop {
            match tokens.next().transpose()? {
                Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                    if let ::std::option::Option::Some(::std::result::Result::Ok(
                        ::aws_smithy_json::deserialize::Token::ValueNull { .. },
                    )) = tokens.peek()
                    {
                        let _ = tokens.next().expect("peek returned a token")?;
                        continue;
                    }
                    let key = key.to_unescaped()?;
                    if key == "__type" {
                        ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                        continue;
                    }
                    if variant.is_some() {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            "encountered mixed variants in union",
                        ));
                    }
                    variant = match key.as_ref() {
                            "sourceCodeArtifact" => {
                                Some(crate::types::TransformationProjectArtifactDescriptor::SourceCodeArtifact(
                                    crate::protocol_serde::shape_transformation_source_code_artifact_descriptor::de_transformation_source_code_artifact_descriptor(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'sourceCodeArtifact' cannot be null"))?
                                ))
                            }
                            _ => {
                                                                              ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                              Some(crate::types::TransformationProjectArtifactDescriptor::Unknown)
                                                                            }
                        };
                },
                other => {
                    return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                        format!("expected object key or end object, found: {:?}", other),
                    ));
                },
            }
        },
        _ => {
            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ));
        },
    }
    if variant.is_none() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "Union did not contain a valid variant.",
        ));
    }
    Ok(variant)
}
