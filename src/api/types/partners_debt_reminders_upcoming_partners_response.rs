pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DebtRemindersUpcomingPartnersResponse {
    #[serde(default)]
    pub rows: Vec<DebtRemindersUpcomingPartnersResponseRowsItem>,
}

impl DebtRemindersUpcomingPartnersResponse {
    pub fn builder() -> DebtRemindersUpcomingPartnersResponseBuilder {
        <DebtRemindersUpcomingPartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersUpcomingPartnersResponseBuilder {
    rows: Option<Vec<DebtRemindersUpcomingPartnersResponseRowsItem>>,
}

impl DebtRemindersUpcomingPartnersResponseBuilder {
    pub fn rows(mut self, value: Vec<DebtRemindersUpcomingPartnersResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DebtRemindersUpcomingPartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](DebtRemindersUpcomingPartnersResponseBuilder::rows)
    pub fn build(self) -> Result<DebtRemindersUpcomingPartnersResponse, BuildError> {
        Ok(DebtRemindersUpcomingPartnersResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
