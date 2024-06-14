use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::path::Path;

use crate::game_metadata_parser::{Game, Language};
use crate::localized_bank_parser::localized_record::LanguageRecord;
use crate::localized_bank_parser::raw_data::RawRecord;
use crate::localized_bank_parser::text::Text;
use crate::parsing_error::ParsingError;

mod localized_record;
mod piece;
mod raw_data;
mod text;

pub struct LocalizedBank(BTreeMap<BankKey, Record>);

pub type BankKey = Box<str>;
pub type BankTag = Box<str>;

#[derive(Debug)]
pub struct Record {
    pub text: Box<[(Language, LanguageRecord)]>,
    pub tag: Option<Box<[BankTag]>>,
}

impl LocalizedBank {
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<LocalizedBank> {
        let d = RawRecord::load(path)?;
        let mut vec = BTreeMap::new();
        for (k, v) in d {
            vec.insert(k, v.try_into()?);
        }
        Ok(LocalizedBank(vec))
    }

    pub fn records<'a>(
        &'a self,
        filter_language: impl Fn(&'a Language) -> bool,
        filter_game: Option<&'a mut [Game]>,
        mut filter_tag: impl FnMut(Option<&'a [BankTag]>) -> bool,
    ) -> Vec<(&'a str, &'a str, &'a str, &'a Text)> {
        let it = self
            .0
            .iter()
            .filter(|(_, record)| filter_tag(record.tag.as_ref().map(|o| o.as_ref())))
            .flat_map(|(key, record)| {
                record
                    .text
                    .iter()
                    .filter(|(lang, _)| filter_language(lang))
                    .map(move |(lang, o)| (key.as_ref(), lang.as_ref(), o))
            });
        if let Some(concerned_games) = filter_game {
            concerned_games.sort();
            it.flat_map(|(key, lang, game_specifics)| {
                let mut ret = Vec::with_capacity(concerned_games.len());
                let fallback = &game_specifics.fallback;
                let specifics = &game_specifics.games;
                for game in concerned_games.iter() {
                    let (game, texts) = match specifics.binary_search_by(|o| o.0.cmp(game)) {
                        Err(_) => (game.as_ref(), fallback),
                        Ok(idx) => {
                            let o = &specifics[idx];
                            (o.0.as_ref(), &o.1)
                        }
                    };
                    ret.push((game, texts));
                }
                ret.into_iter()
                    .map(move |(game, texts)| (key, lang, game, texts))
            })
            .collect()
        } else {
            it.flat_map(|(key, lang, game_specifics)| {
                std::iter::once((key, lang, "", &game_specifics.fallback)).chain(
                    game_specifics
                        .games
                        .iter()
                        .map(move |(game, texts)| (key, lang, game.as_ref(), texts)),
                )
            })
            .collect()
        }
    }
}

impl TryFrom<RawRecord> for Record {
    type Error = ParsingError<'static>;

    fn try_from(value: RawRecord) -> Result<Self, Self::Error> {
        let RawRecord { text, tag } = value;

        let mut vec = Vec::with_capacity(text.len());
        for (lang, games) in text {
            let mut fallback = None;
            let mut game_specifics = Vec::with_capacity(games.len());
            for (game, texts) in games {
                let mut components = Vec::with_capacity(texts.len());
                for s in texts.into_iter() {
                    components.push(s.try_into()?);
                }

                let components = Text::new(components.into_boxed_slice());
                if "".cmp(&game) == Ordering::Equal {
                    fallback = Some(components);
                } else {
                    game_specifics.push((game.into(), components));
                }
            }
            let fallback = fallback.ok_or_else(|| ParsingError::MissingAttribute {
                attribute: "".into(),
                reason: "fallback".into(),
            })?;
            vec.push((
                lang.into(),
                LanguageRecord {
                    fallback,
                    games: game_specifics.into(),
                },
            ));
        }
        Ok(Self {
            text: vec.into(),
            tag,
        })
    }
}
