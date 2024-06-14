use std::collections::HashMap;
use std::rc::Rc;

use crate::localized_bank_parser::piece::{Piece, PieceInner};

/// The smallest unit of exported localization.
/// Each triple of target game, language & bank key will result in a single one of these texts.
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Text {
    /// The pieces comprising this text.
    pieces: Box<[Piece]>,

    /// The cached text actualized from the pieces.
    actualized: Option<Rc<str>>,
}

impl Text {
    /// Create text from its pieces without actualizing the display text.
    pub fn new(pieces: impl Into<Box<[Piece]>>) -> Self {
        Self {
            pieces: pieces.into(),
            actualized: None,
        }
    }
}

impl Text {
    /// Get all foreign referential keys among the pieces.
    pub fn get_references(&self) -> impl Iterator<Item = &PieceInner> {
        self.pieces.iter().flat_map(|o| match o {
            Piece::Ref(o) => Some(o),
            _ => None,
        })
    }

    pub fn pieces(&self) -> &[Piece] {
        &self.pieces
    }
}

impl Text {
    pub fn cache(&mut self, bank: HashMap<PieceInner, &str>) {
        if self.actualized.is_some() {
            return;
        }

        let mut string = String::new();
        for piece in self.pieces.iter() {
            let t = match piece {
                Piece::Raw(o) => o.as_ref(),
                Piece::Ref(o) => bank.get(o).copied().unwrap_or(o.as_ref()),
            };
            string.push_str(t);
        }
        self.actualized = Some(string.into_boxed_str().into());
    }

    pub fn uncache(&mut self) {
        self.actualized = None;
    }
}
