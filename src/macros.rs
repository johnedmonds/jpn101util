macro_rules! make_conjugation_macro {
    ($word_type:ident) => {
        #[macro_export]
        macro_rules! $word_type {
            ($chapter:expr, $dictionary_form:expr, $english_definition:expr) => {
                dictmacro::conjugate!($word_type,$chapter,$dictionary_form,$english_definition)
            };
            ($chapter:expr, $dictionary_form:expr, $english_definition:expr, $kanji_form:expr) => {
                dictmacro::conjugate!($word_type,$chapter,$dictionary_form,$english_definition)
            };
        }
    }
}

make_conjugation_macro!(ruverb);
make_conjugation_macro!(uverb);
make_conjugation_macro!(iadjective);
make_conjugation_macro!(naadjective);

#[macro_export]
macro_rules! noun {
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr) => {
        word::StaticWordType::Noun(word::StaticWord {
            chapter:$chapter,
            dictionary_form:$dictionary_form,
            english_definition:$english_definition,
            kanji_form:None,
        })
    };
    ($chapter:expr, $dictionary_form:expr, $english_definition:expr, $kanji_form:expr) => {
        word::StaticWordType::Noun(word::StaticWord {
            chapter:$chapter,
            dictionary_form:$dictionary_form.to_string(),
            english_definition:$english_definition.to_string(),
            kanji_form:Some($kanji_form.to_string()),
        })
    };
}
