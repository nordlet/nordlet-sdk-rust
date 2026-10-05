pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PositionsCreateHrResponseTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PositionsCreateHrResponseTranslationsValue {
    pub fn builder() -> PositionsCreateHrResponseTranslationsValueBuilder {
        <PositionsCreateHrResponseTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PositionsCreateHrResponseTranslationsValueBuilder {
    name: Option<String>,
}

impl PositionsCreateHrResponseTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PositionsCreateHrResponseTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PositionsCreateHrResponseTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PositionsCreateHrResponseTranslationsValue, BuildError> {
        Ok(PositionsCreateHrResponseTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
