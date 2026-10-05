pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DebtRemindersPreviewPartnersResponse {
    #[serde(default)]
    pub rows: Vec<DebtRemindersPreviewPartnersResponseRowsItem>,
}

impl DebtRemindersPreviewPartnersResponse {
    pub fn builder() -> DebtRemindersPreviewPartnersResponseBuilder {
        <DebtRemindersPreviewPartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersPreviewPartnersResponseBuilder {
    rows: Option<Vec<DebtRemindersPreviewPartnersResponseRowsItem>>,
}

impl DebtRemindersPreviewPartnersResponseBuilder {
    pub fn rows(mut self, value: Vec<DebtRemindersPreviewPartnersResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DebtRemindersPreviewPartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](DebtRemindersPreviewPartnersResponseBuilder::rows)
    pub fn build(self) -> Result<DebtRemindersPreviewPartnersResponse, BuildError> {
        Ok(DebtRemindersPreviewPartnersResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
