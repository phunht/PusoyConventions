use std::convert::TryFrom;

use crate::parsing_error::ParsingError;

/// The smallest logical unit of localized text in the localization bank.
/// Each triple of target game, language & bank key will result in a list of these pieces concatenated.
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Piece {
    /// The raw string as is.
    Raw(PieceInner),

    /// The raw string used as key to reference another record.
    Ref(PieceInner),
}

/// The raw string stored in a piece.
pub type PieceInner = Box<str>;

impl TryFrom<serde_yaml::Value> for Piece {
    type Error = ParsingError<'static>;

    fn try_from(value: serde_yaml::Value) -> Result<Self, Self::Error> {
        use serde_yaml::Value;
        match value {
            Value::String(o) => Ok(Piece::Raw(o.to_string().into())),
            Value::Tagged(o) => match o.value {
                Value::String(o) => Ok(Piece::Ref(o.to_string().into())),
                _ => Err(ParsingError::MismatchType {
                    attribute: "tagged value".into(),
                    r#type: "string".into(),
                }),
            },
            _ => Err(ParsingError::MismatchType {
                attribute: "string value".into(),
                r#type: "string".into(),
            }),
        }
    }
}
