// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct StartCodeFixJobInput {
    /// Indicates Range / Span in a Text Document
    pub snippet_range: ::std::option::Option<crate::types::Range>,
    /// Upload ID returned by CreateUploadUrl API
    pub upload_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub description: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub rule_id: ::std::option::Option<::std::string::String>,
    /// Code fix name
    pub code_fix_name: ::std::option::Option<::std::string::String>,
}
impl StartCodeFixJobInput {
    /// Indicates Range / Span in a Text Document
    pub fn snippet_range(&self) -> ::std::option::Option<&crate::types::Range> {
        self.snippet_range.as_ref()
    }

    /// Upload ID returned by CreateUploadUrl API
    pub fn upload_id(&self) -> ::std::option::Option<&str> {
        self.upload_id.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn rule_id(&self) -> ::std::option::Option<&str> {
        self.rule_id.as_deref()
    }

    /// Code fix name
    pub fn code_fix_name(&self) -> ::std::option::Option<&str> {
        self.code_fix_name.as_deref()
    }
}
impl ::std::fmt::Debug for StartCodeFixJobInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("StartCodeFixJobInput");
        formatter.field("snippet_range", &self.snippet_range);
        formatter.field("upload_id", &self.upload_id);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("rule_id", &self.rule_id);
        formatter.field("code_fix_name", &self.code_fix_name);
        formatter.finish()
    }
}
impl StartCodeFixJobInput {
    /// Creates a new builder-style object to manufacture
    /// [`StartCodeFixJobInput`](crate::operation::start_code_fix_job::StartCodeFixJobInput).
    pub fn builder() -> crate::operation::start_code_fix_job::builders::StartCodeFixJobInputBuilder {
        crate::operation::start_code_fix_job::builders::StartCodeFixJobInputBuilder::default()
    }
}

/// A builder for
/// [`StartCodeFixJobInput`](crate::operation::start_code_fix_job::StartCodeFixJobInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct StartCodeFixJobInputBuilder {
    pub(crate) snippet_range: ::std::option::Option<crate::types::Range>,
    pub(crate) upload_id: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) rule_id: ::std::option::Option<::std::string::String>,
    pub(crate) code_fix_name: ::std::option::Option<::std::string::String>,
}
impl StartCodeFixJobInputBuilder {
    /// Indicates Range / Span in a Text Document
    /// This field is required.
    pub fn snippet_range(mut self, input: crate::types::Range) -> Self {
        self.snippet_range = ::std::option::Option::Some(input);
        self
    }

    /// Indicates Range / Span in a Text Document
    pub fn set_snippet_range(mut self, input: ::std::option::Option<crate::types::Range>) -> Self {
        self.snippet_range = input;
        self
    }

    /// Indicates Range / Span in a Text Document
    pub fn get_snippet_range(&self) -> &::std::option::Option<crate::types::Range> {
        &self.snippet_range
    }

    /// Upload ID returned by CreateUploadUrl API
    /// This field is required.
    pub fn upload_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.upload_id = ::std::option::Option::Some(input.into());
        self
    }

    /// Upload ID returned by CreateUploadUrl API
    pub fn set_upload_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.upload_id = input;
        self
    }

    /// Upload ID returned by CreateUploadUrl API
    pub fn get_upload_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.upload_id
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn rule_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.rule_id = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_rule_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.rule_id = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_rule_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.rule_id
    }

    /// Code fix name
    pub fn code_fix_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.code_fix_name = ::std::option::Option::Some(input.into());
        self
    }

    /// Code fix name
    pub fn set_code_fix_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.code_fix_name = input;
        self
    }

    /// Code fix name
    pub fn get_code_fix_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.code_fix_name
    }

    /// Consumes the builder and constructs a
    /// [`StartCodeFixJobInput`](crate::operation::start_code_fix_job::StartCodeFixJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_code_fix_job::StartCodeFixJobInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::start_code_fix_job::StartCodeFixJobInput {
            snippet_range: self.snippet_range,
            upload_id: self.upload_id,
            description: self.description,
            rule_id: self.rule_id,
            code_fix_name: self.code_fix_name,
        })
    }
}
impl ::std::fmt::Debug for StartCodeFixJobInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("StartCodeFixJobInputBuilder");
        formatter.field("snippet_range", &self.snippet_range);
        formatter.field("upload_id", &self.upload_id);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("rule_id", &self.rule_id);
        formatter.field("code_fix_name", &self.code_fix_name);
        formatter.finish()
    }
}
