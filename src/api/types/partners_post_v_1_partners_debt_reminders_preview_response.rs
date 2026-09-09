pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersDebtRemindersPreviewResponse {
    #[serde(default)]
    pub rows: Vec<PostV1PartnersDebtRemindersPreviewResponseRowsItem>,
}

impl PostV1PartnersDebtRemindersPreviewResponse {
    pub fn builder() -> PostV1PartnersDebtRemindersPreviewResponseBuilder {
        <PostV1PartnersDebtRemindersPreviewResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersDebtRemindersPreviewResponseBuilder {
    rows: Option<Vec<PostV1PartnersDebtRemindersPreviewResponseRowsItem>>,
}

impl PostV1PartnersDebtRemindersPreviewResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1PartnersDebtRemindersPreviewResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersDebtRemindersPreviewResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1PartnersDebtRemindersPreviewResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1PartnersDebtRemindersPreviewResponse, BuildError> {
        Ok(PostV1PartnersDebtRemindersPreviewResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
