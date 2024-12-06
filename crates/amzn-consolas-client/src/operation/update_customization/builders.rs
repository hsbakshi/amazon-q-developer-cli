// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_customization::_update_customization_input::UpdateCustomizationInputBuilder;
pub use crate::operation::update_customization::_update_customization_output::UpdateCustomizationOutputBuilder;

impl crate::operation::update_customization::builders::UpdateCustomizationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_customization::UpdateCustomizationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_customization::UpdateCustomizationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_customization();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateCustomization`.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateCustomizationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_customization::builders::UpdateCustomizationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_customization::UpdateCustomizationOutput,
        crate::operation::update_customization::UpdateCustomizationError,
    > for UpdateCustomizationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_customization::UpdateCustomizationOutput,
            crate::operation::update_customization::UpdateCustomizationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateCustomizationFluentBuilder {
    /// Creates a new `UpdateCustomizationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }

    /// Access the UpdateCustomization as a reference.
    pub fn as_input(&self) -> &crate::operation::update_customization::builders::UpdateCustomizationInputBuilder {
        &self.inner
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_customization::UpdateCustomizationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_customization::UpdateCustomizationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_customization::UpdateCustomization::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_customization::UpdateCustomization::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_customization::UpdateCustomizationOutput,
        crate::operation::update_customization::UpdateCustomizationError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }

    pub(crate) fn config_override(
        mut self,
        config_override: impl ::std::convert::Into<crate::config::Builder>,
    ) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(
        &mut self,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> &mut Self {
        self.config_override = config_override;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identifier(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identifier(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identifier()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn operation(mut self, input: crate::types::UpdateOperation) -> Self {
        self.inner = self.inner.operation(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_operation(mut self, input: ::std::option::Option<crate::types::UpdateOperation>) -> Self {
        self.inner = self.inner.set_operation(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_operation(&self) -> &::std::option::Option<crate::types::UpdateOperation> {
        self.inner.get_operation()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn data_reference(mut self, input: crate::types::DataReference) -> Self {
        self.inner = self.inner.data_reference(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_data_reference(mut self, input: ::std::option::Option<crate::types::DataReference>) -> Self {
        self.inner = self.inner.set_data_reference(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_data_reference(&self) -> &::std::option::Option<crate::types::DataReference> {
        self.inner.get_data_reference()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn version(mut self, input: i64) -> Self {
        self.inner = self.inner.version(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_version(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_version(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_version(&self) -> &::std::option::Option<i64> {
        self.inner.get_version()
    }

    /// Appends an item to `includeRepos`.
    ///
    /// To override the contents of this collection use
    /// [`set_include_repos`](Self::set_include_repos).
    #[allow(missing_docs)] // documentation missing in model
    pub fn include_repos(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.include_repos(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_include_repos(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_include_repos(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_include_repos(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_include_repos()
    }
}
