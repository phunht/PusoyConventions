use crate::game_metadata_parser::Game;
use crate::localized_bank_parser::text::Text;

#[derive(Debug)]
pub struct LanguageRecord {
    pub games: Box<[(Game, Text)]>,
    pub fallback: Text,
}
