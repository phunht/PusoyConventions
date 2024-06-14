use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;
use std::rc::Rc;

#[derive(Debug)]
pub struct LanguageSet {
    games: Box<[(Game, Box<[Language]>)]>,
    languages: Box<[(Language, Box<[Game]>)]>,
}

pub type Language = Rc<str>;
pub type Game = Rc<str>;

impl LanguageSet {
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<LanguageSet> {
        let file = std::fs::File::open(path)?;
        let mut targets: BTreeMap<Box<str>, BTreeSet<Box<str>>> = serde_yaml::from_reader(file)?;
        let first_key = targets.first_key_value().map(|(k, _)| k.as_ref());
        let free_languages = match first_key {
            Some("") => {
                targets
                    .pop_first()
                    .unwrap() // The first key existence is already guaranteed.
                    .1
            }
            _ => BTreeSet::new(),
        };
        Ok(LanguageSet::new(targets, free_languages))
    }

    pub fn new<G, L>(
        game_to_languages: BTreeMap<G, BTreeSet<L>>,
        free_languages: BTreeSet<L>,
    ) -> Self
    where
        G: Into<Game>,
        L: Into<Language>,
    {
        let mut games = Vec::with_capacity(game_to_languages.len());
        let mut languages = HashMap::new();

        for (game, languages_of_game) in game_to_languages {
            let game = game.into();
            let languages_of_game: Box<[Language]> =
                languages_of_game.into_iter().map(|o| o.into()).collect();

            for lang in languages_of_game.iter() {
                languages
                    .entry(lang.clone())
                    .or_insert_with(Vec::new)
                    .push(game.clone());
            }

            games.push((game, languages_of_game));
        }

        let mut languages: Box<[_]> = languages
            .into_iter()
            .chain(free_languages.into_iter().map(|o| (o.into(), vec![])))
            .map(|(k, v)| (k, v.into()))
            .collect();
        languages.sort();
        let games = games.into();
        Self { games, languages }
    }
}

impl LanguageSet {
    pub fn games(&self) -> impl Iterator<Item = &Game> {
        self.games.iter().map(|(game, _)| game)
    }

    pub fn languages(&self) -> impl Iterator<Item = &Language> {
        self.languages.iter().map(|(lang, _)| lang)
    }

    pub fn games_by_language(&self, language: &str) -> impl Iterator<Item = &Game> {
        let vec: Vec<_> = match self
            .languages
            .binary_search_by(|o| o.0.as_ref().cmp(language))
        {
            Ok(idx) => match self.languages.get(idx) {
                None => self.games().collect(),
                Some(o) => o.1.iter().collect(),
            },
            Err(_) => self.games().collect(),
        };
        vec.into_iter()
    }

    pub fn language_by_games(&self, game: &str) -> impl Iterator<Item = &Game> {
        let vec: Vec<_> = match self.games.binary_search_by(|o| o.0.as_ref().cmp(game)) {
            Ok(idx) => match self.games.get(idx) {
                None => self.languages().collect(),
                Some(o) => o.1.iter().collect(),
            },
            Err(_) => self.languages().collect(),
        };
        vec.into_iter()
    }
}

impl LanguageSet {
    /// Associate a language with a game, creating the language or the game in the process if necessary.
    pub fn add_target(&mut self, language: &str, game: &str) {
        Self::insert_or_modify(
            &mut self.games,
            |element| element.0.as_ref().cmp(game),
            || (game.into(), Box::new([language.into()])),
            |_| (),
        );
        Self::insert_or_modify(
            &mut self.languages,
            |element| element.0.as_ref().cmp(language),
            || (language.into(), Box::new([game.into()])),
            |element| {
                Self::insert_or_modify(
                    &mut element.1,
                    |o| o.as_ref().cmp(game),
                    || game.into(),
                    |_| (),
                )
            },
        );
    }

    fn insert_or_modify<T>(
        slice: &mut Box<[T]>,
        search_by: impl FnMut(&T) -> Ordering,
        create_element: impl FnOnce() -> T,
        modify_element: impl FnOnce(&mut T),
    ) {
        let idx = match slice.binary_search_by(search_by) {
            Ok(idx) => return modify_element(&mut slice[idx]),
            Err(idx) => idx,
        };

        let vec = std::mem::replace(slice, Box::new([]));
        let mut vec = vec.into_vec();
        vec.insert(idx, create_element());
        *slice = vec.into_boxed_slice();
    }
}

impl LanguageSet {
    /// Remove the association between a language and a game.
    /// Does not remove any games or languages.
    pub fn remove_association(&mut self, _game: &str, _language: &str) -> Option<usize> {
        todo!()
    }

    /// Remove a game and all of its associations with any languages.
    /// Does not remove any languages.
    pub fn remove_game(&mut self, _game: &str) -> Vec<usize> {
        todo!()
    }

    /// Remove a language and all of its associations with any languages.
    /// Does not remove any games.
    pub fn remove_language(&mut self, _language: &str) -> Vec<usize> {
        todo!()
    }
}
