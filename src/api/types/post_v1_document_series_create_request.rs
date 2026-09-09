pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DocumentSeriesCreateRequest {
    #[serde(rename = "documentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<PostV1DocumentSeriesCreateRequestDocumentType>,
    #[serde(default)]
    pub prefix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "operationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type_id: Option<String>,
    #[serde(rename = "numberLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_length: Option<i64>,
    #[serde(rename = "nextNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_number: Option<i64>,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(rename = "printSeries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_series: Option<bool>,
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl PostV1DocumentSeriesCreateRequest {
    pub fn builder() -> PostV1DocumentSeriesCreateRequestBuilder {
        <PostV1DocumentSeriesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DocumentSeriesCreateRequestBuilder {
    document_type: Option<PostV1DocumentSeriesCreateRequestDocumentType>,
    prefix: Option<String>,
    name: Option<String>,
    label: Option<String>,
    operation_type_id: Option<String>,
    number_length: Option<i64>,
    next_number: Option<i64>,
    warehouse_id: Option<String>,
    print_series: Option<bool>,
    is_default: Option<bool>,
    is_active: Option<bool>,
}

impl PostV1DocumentSeriesCreateRequestBuilder {
    pub fn document_type(mut self, value: PostV1DocumentSeriesCreateRequestDocumentType) -> Self {
        self.document_type = Some(value);
        self
    }

    pub fn prefix(mut self, value: impl Into<String>) -> Self {
        self.prefix = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn operation_type_id(mut self, value: impl Into<String>) -> Self {
        self.operation_type_id = Some(value.into());
        self
    }

    pub fn number_length(mut self, value: i64) -> Self {
        self.number_length = Some(value);
        self
    }

    pub fn next_number(mut self, value: i64) -> Self {
        self.next_number = Some(value);
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn print_series(mut self, value: bool) -> Self {
        self.print_series = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DocumentSeriesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prefix`](PostV1DocumentSeriesCreateRequestBuilder::prefix)
    pub fn build(self) -> Result<PostV1DocumentSeriesCreateRequest, BuildError> {
        Ok(PostV1DocumentSeriesCreateRequest {
            document_type: self.document_type,
            prefix: self
                .prefix
                .ok_or_else(|| BuildError::missing_field("prefix"))?,
            name: self.name,
            label: self.label,
            operation_type_id: self.operation_type_id,
            number_length: self.number_length,
            next_number: self.next_number,
            warehouse_id: self.warehouse_id,
            print_series: self.print_series,
            is_default: self.is_default,
            is_active: self.is_active,
        })
    }
}
