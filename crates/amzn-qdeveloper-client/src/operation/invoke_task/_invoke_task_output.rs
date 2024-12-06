// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InvokeTaskOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub task_id: ::std::string::String,
    _request_id: Option<String>,
}
impl InvokeTaskOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn task_id(&self) -> &str {
        use std::ops::Deref;
        self.task_id.deref()
    }
}
impl ::aws_types::request_id::RequestId for InvokeTaskOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl InvokeTaskOutput {
    /// Creates a new builder-style object to manufacture
    /// [`InvokeTaskOutput`](crate::operation::invoke_task::InvokeTaskOutput).
    pub fn builder() -> crate::operation::invoke_task::builders::InvokeTaskOutputBuilder {
        crate::operation::invoke_task::builders::InvokeTaskOutputBuilder::default()
    }
}

/// A builder for [`InvokeTaskOutput`](crate::operation::invoke_task::InvokeTaskOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct InvokeTaskOutputBuilder {
    pub(crate) task_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl InvokeTaskOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn task_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.task_id = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_task_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.task_id = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_task_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.task_id
    }

    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }

    /// Consumes the builder and constructs a
    /// [`InvokeTaskOutput`](crate::operation::invoke_task::InvokeTaskOutput). This method will
    /// fail if any of the following fields are not set:
    /// - [`task_id`](crate::operation::invoke_task::builders::InvokeTaskOutputBuilder::task_id)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::invoke_task::InvokeTaskOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::invoke_task::InvokeTaskOutput {
            task_id: self.task_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "task_id",
                    "task_id was not specified but it is required when building InvokeTaskOutput",
                )
            })?,
            _request_id: self._request_id,
        })
    }
}
