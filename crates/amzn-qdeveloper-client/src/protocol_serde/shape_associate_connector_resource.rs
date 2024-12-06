// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_connector_resource_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_connector_resource::AssociateConnectorResourceOutput,
    crate::operation::associate_connector_resource::AssociateConnectorResourceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder =
        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
            .map_err(crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled(generic),
            );
        },
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => {
            crate::operation::associate_connector_resource::AssociateConnectorResourceError::ResourceNotFoundError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundErrorBuilder::default();
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::resource_not_found_exception_correct_errors(output)
                        .build()
                        .map_err(
                            crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled,
                        )?
                };
                tmp
            })
        },
        "InternalServerException" => {
            crate::operation::associate_connector_resource::AssociateConnectorResourceError::InternalServerError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                    output =
                        crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
                            _response_body,
                            output,
                        )
                        .map_err(
                            crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled,
                        )?;
                    let output = output.meta(generic);
                    crate::serde_util::internal_server_exception_correct_errors(output)
                        .build()
                        .map_err(
                            crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled,
                        )?
                };
                tmp
            })
        },
        "AccessDeniedException" => {
            crate::operation::associate_connector_resource::AssociateConnectorResourceError::AccessDeniedError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedErrorBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(
                        crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    crate::serde_util::access_denied_exception_correct_errors(output)
                        .build()
                        .map_err(
                            crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled,
                        )?
                };
                tmp
            })
        },
        "ConflictException" => {
            crate::operation::associate_connector_resource::AssociateConnectorResourceError::ConflictError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConflictErrorBuilder::default();
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(
                        crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    crate::serde_util::conflict_exception_correct_errors(output)
                        .build()
                        .map_err(
                            crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled,
                        )?
                };
                tmp
            })
        },
        "ValidationException" => {
            crate::operation::associate_connector_resource::AssociateConnectorResourceError::ValidationError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationErrorBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(
                        crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    crate::serde_util::validation_exception_correct_errors(output)
                        .build()
                        .map_err(
                            crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled,
                        )?
                };
                tmp
            })
        },
        "ThrottlingException" => {
            crate::operation::associate_connector_resource::AssociateConnectorResourceError::ThrottlingError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingErrorBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(
                        crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    crate::serde_util::throttling_exception_correct_errors(output)
                        .build()
                        .map_err(
                            crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled,
                        )?
                };
                tmp
            })
        },
        _ => crate::operation::associate_connector_resource::AssociateConnectorResourceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_connector_resource_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_connector_resource::AssociateConnectorResourceOutput,
    crate::operation::associate_connector_resource::AssociateConnectorResourceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::associate_connector_resource::builders::AssociateConnectorResourceOutputBuilder::default(
            );
        output = crate::protocol_serde::shape_associate_connector_resource::de_associate_connector_resource(
            _response_body,
            output,
        )
        .map_err(crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::associate_connector_resource_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::associate_connector_resource::AssociateConnectorResourceError::unhandled)?
    })
}

pub fn ser_associate_connector_resource_input(
    input: &crate::operation::associate_connector_resource::AssociateConnectorResourceInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_associate_connector_resource_input::ser_associate_connector_resource_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_associate_connector_resource(
    value: &[u8],
    mut builder: crate::operation::associate_connector_resource::builders::AssociateConnectorResourceOutputBuilder,
) -> Result<
    crate::operation::associate_connector_resource::builders::AssociateConnectorResourceOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned =
        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "connectorId" => {
                    builder = builder.set_connector_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                },
                "connectorName" => {
                    builder = builder.set_connector_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                },
                "connectorType" => {
                    builder = builder.set_connector_type(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                },
                "accountConnection" => {
                    builder = builder.set_account_connection(
                        crate::protocol_serde::shape_account_connection::de_account_connection(tokens)?,
                    );
                },
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                    format!("expected object key or end object, found: {:?}", other),
                ));
            },
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
