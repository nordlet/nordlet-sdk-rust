pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub documents: i64,
}

impl PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItem {
    pub fn builder() -> PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItemBuilder {
        <PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItemBuilder {
    country_code: Option<String>,
    amount: Option<String>,
    documents: Option<i64>,
}

impl PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItemBuilder::country_code)
    /// - [`amount`](PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItemBuilder::amount)
    /// - [`documents`](PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItemBuilder::documents)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItem, BuildError> {
        Ok(
            PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItem {
                country_code: self
                    .country_code
                    .ok_or_else(|| BuildError::missing_field("country_code"))?,
                amount: self
                    .amount
                    .ok_or_else(|| BuildError::missing_field("amount"))?,
                documents: self
                    .documents
                    .ok_or_else(|| BuildError::missing_field("documents"))?,
            },
        )
    }
}
