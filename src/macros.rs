#[macro_export]
macro_rules! noun {
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr) => {
        word::StaticWordType::Noun(word::StaticWord {
            chapter: $chapter,
            dictionary_form: $dictionary_form,
            english_definition: $english_definition,
            kanji_form: None,
        })
    };
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr, $kanji_form:expr) => {
        word::StaticWordType::Noun(word::StaticWord {
            chapter: $chapter,
            dictionary_form: $dictionary_form,
            english_definition: $english_definition,
            kanji_form: Some($kanji_form),
        })
    };
}

#[macro_export]
macro_rules! ruverb {
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr) => {
        dictmacro::conjugate!(ruverb, $chapter, $dictionary_form, $english_definition)
    };
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr, $kanji_form:expr) => {
        dictmacro::conjugate!(ruverb, $chapter, $dictionary_form, $english_definition)
    };
}

#[macro_export]
macro_rules! uverb {
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr) => {
        dictmacro::conjugate!(uverb, $chapter, $dictionary_form, $english_definition)
    };
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr, $kanji_form:expr) => {
        dictmacro::conjugate!(uverb, $chapter, $dictionary_form, $english_definition)
    };
}

#[macro_export]
macro_rules! iadjective {
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr) => {
        dictmacro::conjugate!(iadjective, $chapter, $dictionary_form, $english_definition)
    };
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr, $kanji_form:expr) => {
        dictmacro::conjugate!(iadjective, $chapter, $dictionary_form, $english_definition)
    };
}

#[macro_export]
macro_rules! naadjective {
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr) => {
        dictmacro::conjugate!(naadjective, $chapter, $dictionary_form, $english_definition)
    };
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr, $kanji_form:expr) => {
        dictmacro::conjugate!(naadjective, $chapter, $dictionary_form, $english_definition)
    };
}

#[macro_export]
macro_rules! adverb {
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr) => {
        word::StaticWordType::Adverb(static_word!(
            $chapter,
            $dictionary_form,
            $english_definition
        ))
    };
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr, $kanji_form:expr) => {
        word::StaticWordType::Adverb(static_word!(
            $chapter,
            $dictionary_form,
            $english_definition,
            $kanji_form
        ))
    };
}

#[macro_export]
macro_rules! static_word {
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr) => {
        word::StaticWord {
            chapter: $chapter,
            dictionary_form: $dictionary_form,
            english_definition: $english_definition,
            kanji_form: None,
        }
    };
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr, $kanji_form:expr) => {
        word::StaticWord {
            chapter: $chapter,
            dictionary_form: $dictionary_form,
            english_definition: $english_definition,
            kanji_form: Some($kanji_form),
        }
    };
}
