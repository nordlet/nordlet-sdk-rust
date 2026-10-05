pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PositionsUpdateHrResponseTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PositionsUpdateHrResponseTranslationsValue {
    pub fn builder() -> PositionsUpdateHrResponseTranslationsValueBuilder {
        <PositionsUpdateHrResponseTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PositionsUpdateHrResponseTranslationsValueBuilder {
    name: Option<String>,
}

impl PositionsUpdateHrResponseTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PositionsUpdateHrResponseTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PositionsUpdateHrResponseTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PositionsUpdateHrResponseTranslationsValue, BuildError> {
        Ok(PositionsUpdateHrResponseTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
