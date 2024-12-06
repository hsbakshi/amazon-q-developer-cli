// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the
    /// [`CreateUploadUrl`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder)
    /// operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`content_md5(impl Into<String>)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::content_md5) / [`set_content_md5(Option<String>)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::set_content_md5):<br>required: **false**<br>(undocumented)<br>
    ///   - [`content_checksum(impl Into<String>)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::content_checksum) / [`set_content_checksum(Option<String>)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::set_content_checksum):<br>required: **false**<br>(undocumented)<br>
    ///   - [`content_checksum_type(ContentChecksumType)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::content_checksum_type) / [`set_content_checksum_type(Option<ContentChecksumType>)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::set_content_checksum_type):<br>required: **false**<br>(undocumented)<br>
    ///   - [`content_length(i64)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::content_length) / [`set_content_length(Option<i64>)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::set_content_length):<br>required: **false**<br>(undocumented)<br>
    ///   - [`artifact_type(ArtifactType)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::artifact_type) / [`set_artifact_type(Option<ArtifactType>)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::set_artifact_type):<br>required: **false**<br>(undocumented)<br>
    ///   - [`upload_intent(UploadIntent)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::upload_intent) / [`set_upload_intent(Option<UploadIntent>)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::set_upload_intent):<br>required: **false**<br>Upload Intent<br>
    ///   - [`upload_context(UploadContext)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::upload_context) / [`set_upload_context(Option<UploadContext>)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::set_upload_context):<br>required: **false**<br>(undocumented)<br>
    ///   - [`upload_id(impl Into<String>)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::upload_id) / [`set_upload_id(Option<String>)`](crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::set_upload_id):<br>required: **false**<br>Upload ID returned by CreateUploadUrl API<br>
    /// - On success, responds with
    ///   [`CreateUploadUrlOutput`](crate::operation::create_upload_url::CreateUploadUrlOutput) with
    ///   field(s):
    ///   - [`upload_id(String)`](crate::operation::create_upload_url::CreateUploadUrlOutput::upload_id): Upload ID returned by CreateUploadUrl API
    ///   - [`upload_url(String)`](crate::operation::create_upload_url::CreateUploadUrlOutput::upload_url): (undocumented)
    ///   - [`kms_key_arn(Option<String>)`](crate::operation::create_upload_url::CreateUploadUrlOutput::kms_key_arn): (undocumented)
    ///   - [`request_headers(Option<HashMap::<String,
    ///     String>>)`](crate::operation::create_upload_url::CreateUploadUrlOutput::request_headers):
    ///     (undocumented)
    /// - On failure, responds with
    ///   [`SdkError<CreateUploadUrlError>`](crate::operation::create_upload_url::CreateUploadUrlError)
    pub fn create_upload_url(&self) -> crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder {
        crate::operation::create_upload_url::builders::CreateUploadUrlFluentBuilder::new(self.handle.clone())
    }
}
