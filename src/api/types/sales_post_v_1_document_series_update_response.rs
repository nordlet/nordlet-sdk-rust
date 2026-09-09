pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DocumentSeriesUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "documentType")]
    #[serde(default)]
    pub document_type: String,
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
    #[serde(default)]
    pub number_length: i64,
    #[serde(rename = "nextNumber")]
    #[serde(default)]
    pub next_number: i64,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(rename = "printSeries")]
    #[serde(default)]
    pub print_series: bool,
    #[serde(rename = "isDefault")]
    #[serde(default)]
    pub is_default: bool,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1DocumentSeriesUpdateResponse {
    pub fn builder() -> PostV1DocumentSeriesUpdateResponseBuilder {
        <PostV1DocumentSeriesUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DocumentSeriesUpdateResponseBuilder {
    id: Option<String>,
    document_type: Option<String>,
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
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1DocumentSeriesUpdateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn document_type(mut self, value: impl Into<String>) -> Self {
        self.document_type = Some(value.into());
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DocumentSeriesUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DocumentSeriesUpdateResponseBuilder::id)
    /// - [`document_type`](PostV1DocumentSeriesUpdateResponseBuilder::document_type)
    /// - [`prefix`](PostV1DocumentSeriesUpdateResponseBuilder::prefix)
    /// - [`number_length`](PostV1DocumentSeriesUpdateResponseBuilder::number_length)
    /// - [`next_number`](PostV1DocumentSeriesUpdateResponseBuilder::next_number)
    /// - [`print_series`](PostV1DocumentSeriesUpdateResponseBuilder::print_series)
    /// - [`is_default`](PostV1DocumentSeriesUpdateResponseBuilder::is_default)
    /// - [`is_active`](PostV1DocumentSeriesUpdateResponseBuilder::is_active)
    /// - [`created_at`](PostV1DocumentSeriesUpdateResponseBuilder::created_at)
    /// - [`updated_at`](PostV1DocumentSeriesUpdateResponseBuilder::updated_at)
    pub fn build(self) -> Result<PostV1DocumentSeriesUpdateResponse, BuildError> {
        Ok(PostV1DocumentSeriesUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            document_type: self
                .document_type
                .ok_or_else(|| BuildError::missing_field("document_type"))?,
            prefix: self
                .prefix
                .ok_or_else(|| BuildError::missing_field("prefix"))?,
            name: self.name,
            label: self.label,
            operation_type_id: self.operation_type_id,
            number_length: self
                .number_length
                .ok_or_else(|| BuildError::missing_field("number_length"))?,
            next_number: self
                .next_number
                .ok_or_else(|| BuildError::missing_field("next_number"))?,
            warehouse_id: self.warehouse_id,
            print_series: self
                .print_series
                .ok_or_else(|| BuildError::missing_field("print_series"))?,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
