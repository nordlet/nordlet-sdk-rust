pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsModernizeRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub amount: String,
    #[serde(rename = "addedLifeMonths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_life_months: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1AssetsAssetsModernizeRequest {
    pub fn builder() -> PostV1AssetsAssetsModernizeRequestBuilder {
        <PostV1AssetsAssetsModernizeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsModernizeRequestBuilder {
    id: Option<String>,
    date: Option<String>,
    amount: Option<String>,
    added_life_months: Option<i64>,
    notes: Option<String>,
}

impl PostV1AssetsAssetsModernizeRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn added_life_months(mut self, value: i64) -> Self {
        self.added_life_months = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsModernizeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AssetsAssetsModernizeRequestBuilder::id)
    /// - [`date`](PostV1AssetsAssetsModernizeRequestBuilder::date)
    /// - [`amount`](PostV1AssetsAssetsModernizeRequestBuilder::amount)
    pub fn build(self) -> Result<PostV1AssetsAssetsModernizeRequest, BuildError> {
        Ok(PostV1AssetsAssetsModernizeRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            added_life_months: self.added_life_months,
            notes: self.notes,
        })
    }
}
