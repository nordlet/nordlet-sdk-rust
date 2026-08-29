pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureDocumentsConfirmRequestNewSupplier {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

impl PostV1CaptureDocumentsConfirmRequestNewSupplier {
    pub fn builder() -> PostV1CaptureDocumentsConfirmRequestNewSupplierBuilder {
        <PostV1CaptureDocumentsConfirmRequestNewSupplierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureDocumentsConfirmRequestNewSupplierBuilder {
    name: Option<String>,
    code: Option<String>,
    vat_code: Option<String>,
    country_code: Option<String>,
}

impl PostV1CaptureDocumentsConfirmRequestNewSupplierBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureDocumentsConfirmRequestNewSupplier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1CaptureDocumentsConfirmRequestNewSupplierBuilder::name)
    pub fn build(self) -> Result<PostV1CaptureDocumentsConfirmRequestNewSupplier, BuildError> {
        Ok(PostV1CaptureDocumentsConfirmRequestNewSupplier {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            vat_code: self.vat_code,
            country_code: self.country_code,
        })
    }
}
