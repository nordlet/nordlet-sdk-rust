pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureDocumentsListResponseRowsItemExtractionSupplier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
}

impl PostV1CaptureDocumentsListResponseRowsItemExtractionSupplier {
    pub fn builder() -> PostV1CaptureDocumentsListResponseRowsItemExtractionSupplierBuilder {
        <PostV1CaptureDocumentsListResponseRowsItemExtractionSupplierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureDocumentsListResponseRowsItemExtractionSupplierBuilder {
    name: Option<String>,
    code: Option<String>,
    vat_code: Option<String>,
    country_code: Option<String>,
    iban: Option<String>,
}

impl PostV1CaptureDocumentsListResponseRowsItemExtractionSupplierBuilder {
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

    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureDocumentsListResponseRowsItemExtractionSupplier`].
    pub fn build(
        self,
    ) -> Result<PostV1CaptureDocumentsListResponseRowsItemExtractionSupplier, BuildError> {
        Ok(
            PostV1CaptureDocumentsListResponseRowsItemExtractionSupplier {
                name: self.name,
                code: self.code,
                vat_code: self.vat_code,
                country_code: self.country_code,
                iban: self.iban,
            },
        )
    }
}
