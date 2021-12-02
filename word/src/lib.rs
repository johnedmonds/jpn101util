pub enum WordType {
    Adjective {
        word: Word,
        conjugation: Conjugation,
    },
    Verb {
        word: Word,
        conjugation: Conjugation,
    },
    Noun(Word),
}
pub enum StaticWordType {
    Adjective {
        word: StaticWord,
        conjugation: StaticConjugation,
    },
    Verb {
        word: StaticWord,
        conjugation: StaticConjugation,
    },
    Noun(StaticWord),
}
pub struct Conjugation {
    pub present_affirmative: String,
    pub present_negative: String,
    pub past_affirmative: String,
    pub past_negative: String,
}

pub struct StaticConjugation {
    pub present_affirmative: &'static str,
    pub present_negative: &'static str,
    pub past_affirmative: &'static str,
    pub past_negative: &'static str,
}

pub struct Word {
    pub dictionary_form: String,
    pub kanji_form: Option<String>,
    pub english_definition: String,
    pub chapter: usize,
}

pub struct StaticWord {
    pub dictionary_form: &'static str,
    pub kanji_form: Option<&'static str>,
    pub english_definition: &'static str,
    pub chapter: usize,
}