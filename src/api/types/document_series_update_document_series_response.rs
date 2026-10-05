pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDocumentSeriesResponse {
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
    #[serde(rename = "allocatedFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_from: Option<i64>,
    #[serde(rename = "allocatedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_to: Option<i64>,
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
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl UpdateDocumentSeriesResponse {
    pub fn builder() -> UpdateDocumentSeriesResponseBuilder {
        <UpdateDocumentSeriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDocumentSeriesResponseBuilder {
    id: Option<String>,
    document_type: Option<String>,
    prefix: Option<String>,
    name: Option<String>,
    label: Option<String>,
    operation_type_id: Option<String>,
    number_length: Option<i64>,
    next_number: Option<i64>,
    allocated_from: Option<i64>,
    allocated_to: Option<i64>,
    warehouse_id: Option<String>,
    print_series: Option<bool>,
    is_default: Option<bool>,
    is_active: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl UpdateDocumentSeriesResponseBuilder {
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

    pub fn allocated_from(mut self, value: i64) -> Self {
        self.allocated_from = Some(value);
        self
    }

    pub fn allocated_to(mut self, value: i64) -> Self {
        self.allocated_to = Some(value);
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDocumentSeriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateDocumentSeriesResponseBuilder::id)
    /// - [`document_type`](UpdateDocumentSeriesResponseBuilder::document_type)
    /// - [`prefix`](UpdateDocumentSeriesResponseBuilder::prefix)
    /// - [`number_length`](UpdateDocumentSeriesResponseBuilder::number_length)
    /// - [`next_number`](UpdateDocumentSeriesResponseBuilder::next_number)
    /// - [`print_series`](UpdateDocumentSeriesResponseBuilder::print_series)
    /// - [`is_default`](UpdateDocumentSeriesResponseBuilder::is_default)
    /// - [`is_active`](UpdateDocumentSeriesResponseBuilder::is_active)
    /// - [`created_at`](UpdateDocumentSeriesResponseBuilder::created_at)
    /// - [`updated_at`](UpdateDocumentSeriesResponseBuilder::updated_at)
    pub fn build(self) -> Result<UpdateDocumentSeriesResponse, BuildError> {
        Ok(UpdateDocumentSeriesResponse {
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
            allocated_from: self.allocated_from,
            allocated_to: self.allocated_to,
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
