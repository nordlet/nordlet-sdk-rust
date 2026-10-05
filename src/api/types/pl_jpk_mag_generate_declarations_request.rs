pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlJpkMagGenerateDeclarationsRequest {
    #[serde(rename = "dateFrom")]
    #[serde(default)]
    pub date_from: NaiveDate,
    #[serde(rename = "dateTo")]
    #[serde(default)]
    pub date_to: NaiveDate,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl PlJpkMagGenerateDeclarationsRequest {
    pub fn builder() -> PlJpkMagGenerateDeclarationsRequestBuilder {
        <PlJpkMagGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlJpkMagGenerateDeclarationsRequestBuilder {
    date_from: Option<NaiveDate>,
    date_to: Option<NaiveDate>,
    warehouse_id: Option<String>,
}

impl PlJpkMagGenerateDeclarationsRequestBuilder {
    pub fn date_from(mut self, value: NaiveDate) -> Self {
        self.date_from = Some(value);
        self
    }

    pub fn date_to(mut self, value: NaiveDate) -> Self {
        self.date_to = Some(value);
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlJpkMagGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_from`](PlJpkMagGenerateDeclarationsRequestBuilder::date_from)
    /// - [`date_to`](PlJpkMagGenerateDeclarationsRequestBuilder::date_to)
    pub fn build(self) -> Result<PlJpkMagGenerateDeclarationsRequest, BuildError> {
        Ok(PlJpkMagGenerateDeclarationsRequest {
            date_from: self
                .date_from
                .ok_or_else(|| BuildError::missing_field("date_from"))?,
            date_to: self
                .date_to
                .ok_or_else(|| BuildError::missing_field("date_to"))?,
            warehouse_id: self.warehouse_id,
        })
    }
}
