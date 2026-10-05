pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PositionsListHrResponseRowsItemTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PositionsListHrResponseRowsItemTranslationsValue {
    pub fn builder() -> PositionsListHrResponseRowsItemTranslationsValueBuilder {
        <PositionsListHrResponseRowsItemTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PositionsListHrResponseRowsItemTranslationsValueBuilder {
    name: Option<String>,
}

impl PositionsListHrResponseRowsItemTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PositionsListHrResponseRowsItemTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PositionsListHrResponseRowsItemTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PositionsListHrResponseRowsItemTranslationsValue, BuildError> {
        Ok(PositionsListHrResponseRowsItemTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
