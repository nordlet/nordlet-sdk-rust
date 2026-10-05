pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateOfficersResponseRole {
    Director,
    ManagingDirector,
    BoardMember,
    BoardChair,
    SupervisoryBoardMember,
    Secretary,
    Representative,
    Liquidator,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateOfficersResponseRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Director => serializer.serialize_str("director"),
            Self::ManagingDirector => serializer.serialize_str("managing_director"),
            Self::BoardMember => serializer.serialize_str("board_member"),
            Self::BoardChair => serializer.serialize_str("board_chair"),
            Self::SupervisoryBoardMember => serializer.serialize_str("supervisory_board_member"),
            Self::Secretary => serializer.serialize_str("secretary"),
            Self::Representative => serializer.serialize_str("representative"),
            Self::Liquidator => serializer.serialize_str("liquidator"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateOfficersResponseRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "director" => Ok(Self::Director),
            "managing_director" => Ok(Self::ManagingDirector),
            "board_member" => Ok(Self::BoardMember),
            "board_chair" => Ok(Self::BoardChair),
            "supervisory_board_member" => Ok(Self::SupervisoryBoardMember),
            "secretary" => Ok(Self::Secretary),
            "representative" => Ok(Self::Representative),
            "liquidator" => Ok(Self::Liquidator),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateOfficersResponseRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Director => write!(f, "director"),
            Self::ManagingDirector => write!(f, "managing_director"),
            Self::BoardMember => write!(f, "board_member"),
            Self::BoardChair => write!(f, "board_chair"),
            Self::SupervisoryBoardMember => write!(f, "supervisory_board_member"),
            Self::Secretary => write!(f, "secretary"),
            Self::Representative => write!(f, "representative"),
            Self::Liquidator => write!(f, "liquidator"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
