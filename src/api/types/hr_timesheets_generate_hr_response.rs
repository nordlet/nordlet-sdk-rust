pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimesheetsGenerateHrResponse {
    #[serde(default)]
    pub generated: i64,
}

impl TimesheetsGenerateHrResponse {
    pub fn builder() -> TimesheetsGenerateHrResponseBuilder {
        <TimesheetsGenerateHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimesheetsGenerateHrResponseBuilder {
    generated: Option<i64>,
}

impl TimesheetsGenerateHrResponseBuilder {
    pub fn generated(mut self, value: i64) -> Self {
        self.generated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimesheetsGenerateHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`generated`](TimesheetsGenerateHrResponseBuilder::generated)
    pub fn build(self) -> Result<TimesheetsGenerateHrResponse, BuildError> {
        Ok(TimesheetsGenerateHrResponse {
            generated: self
                .generated
                .ok_or_else(|| BuildError::missing_field("generated"))?,
        })
    }
}
