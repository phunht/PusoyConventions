use std::borrow::Cow;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError<'a> {
    #[error("Mismatch type for attribute '{attribute}', expected '{r#type}'")]
    MismatchType {
        attribute: Cow<'a, str>,
        r#type: Cow<'a, str>,
    },

    #[error("Missing attribute '{attribute}' for \"{reason}\"")]
    MissingAttribute {
        attribute: Cow<'a, str>,
        reason: Cow<'a, str>,
    },
}
