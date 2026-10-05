pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SeriesCreateReferenceResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "documentType")]
    #[serde(default)]
    pub document_type: String,
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "nextNumber")]
    #[serde(default)]
    pub next_number: i64,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl SeriesCreateReferenceResponse {
    pub fn builder() -> SeriesCreateReferenceResponseBuilder {
        <SeriesCreateReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SeriesCreateReferenceResponseBuilder {
    id: Option<String>,
    document_type: Option<String>,
    prefix: Option<String>,
    year: Option<i64>,
    next_number: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl SeriesCreateReferenceResponseBuilder {
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

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn next_number(mut self, value: i64) -> Self {
        self.next_number = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SeriesCreateReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SeriesCreateReferenceResponseBuilder::id)
    /// - [`document_type`](SeriesCreateReferenceResponseBuilder::document_type)
    /// - [`prefix`](SeriesCreateReferenceResponseBuilder::prefix)
    /// - [`year`](SeriesCreateReferenceResponseBuilder::year)
    /// - [`next_number`](SeriesCreateReferenceResponseBuilder::next_number)
    /// - [`created_at`](SeriesCreateReferenceResponseBuilder::created_at)
    pub fn build(self) -> Result<SeriesCreateReferenceResponse, BuildError> {
        Ok(SeriesCreateReferenceResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            document_type: self
                .document_type
                .ok_or_else(|| BuildError::missing_field("document_type"))?,
            prefix: self
                .prefix
                .ok_or_else(|| BuildError::missing_field("prefix"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            next_number: self
                .next_number
                .ok_or_else(|| BuildError::missing_field("next_number"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
