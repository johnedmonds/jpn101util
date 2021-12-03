use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Ident, LitInt, LitStr, Token};
use word::{Conjugation, Word, WordType};
struct Conjugatable {
    word_type: WordType,
}

impl Parse for Conjugatable {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kind_error = input.error("Invalid kind");
        let kind: Ident = input.parse()?;
        let kind = kind.to_string();
        let args_error = input.error("Need 3 or 4 args.");
        input.parse::<Token![,]>()?;
        let chapter: LitInt = input.parse()?;
        let chapter: usize = chapter.base10_parse()?;
        input.parse::<Token![,]>()?;
        let args: Punctuated<LitStr, Token![,]> =
            input.parse_terminated(|s| s.parse::<LitStr>())?;
        let args: Vec<String> = args.into_iter().map(|a| a.value()).collect();
        let word = match &args[..] {
            [dictionary_form, english_definition, kanji_form] => Word {
                dictionary_form: dictionary_form.to_string(),
                kanji_form: Some(kanji_form.to_string()),
                english_definition: english_definition.to_string(),
                chapter,
            },
            [dictionary_form, english_definition] => Word {
                dictionary_form: dictionary_form.to_string(),
                kanji_form: None,
                english_definition: english_definition.to_string(),
                chapter,
            },
            _ => {
                return Err(args_error);
            }
        };

        let base: String = word
            .dictionary_form
            .chars()
            .take(word.dictionary_form.chars().count() - 1)
            .collect();
        let word_type = match kind.as_str() {
            "ruverb" => WordType::Verb {
                conjugation: Conjugation {
                    present_affirmative: format!("{}ます", base),
                    present_negative: format!("{}ません", base),
                    past_affirmative: format!("{}ました", base),
                    past_negative: format!("{}ませんでした", base),
                },
                word,
            },
            "uverb" => {
                let suffix = word
                    .dictionary_form
                    .chars()
                    .skip(word.dictionary_form.chars().count() - 1)
                    .next()
                    .expect("Zero length string");
                let new_suffix = match suffix {
                    'む' => 'み',
                    'す' => 'し',
                    'く' => 'き',
                    'る' => 'り',
                    _ => panic!(
                        "unexpected trailing letter {} in {}",
                        suffix, word.dictionary_form
                    ),
                };
                let base = format!("{}{}", base, new_suffix);
                WordType::Verb {
                    word,
                    conjugation: Conjugation {
                        present_affirmative: format!("{}ます", base),
                        present_negative: format!("{}ません", base),
                        past_affirmative: format!("{}ました", base),
                        past_negative: format!("{}ませんでした", base),
                    },
                }
            }
            "iadjective" => WordType::Adjective {
                conjugation: Conjugation {
                    present_affirmative: format!("{}です", word.dictionary_form),
                    present_negative: format!("{}くないです", base),
                    past_affirmative: format!("{}かったです", base),
                    past_negative: format!("{}くなかったです", base),
                },
                word,
            },
            "naadjective" => WordType::Adjective {
                conjugation: Conjugation {
                    present_affirmative: format!("{}です", base),
                    present_negative: format!("{}じゃないです", base),
                    past_affirmative: format!("{}でした", base),
                    past_negative: format!("{}じゃなかったです", base),
                },
                word,
            },
            _ => {
                return Err(kind_error);
            }
        };
        Ok(Conjugatable { word_type })
    }
}

fn make_word(word: Word) -> proc_macro2::TokenStream {
    let chapter = Literal::usize_unsuffixed(word.chapter);
    let dictionary_form = Literal::string(&word.dictionary_form);
    let english_definition = Literal::string(&word.english_definition);
    let kanji_form = match word.kanji_form {
        Some(kanji_form) => {
            let kanji_form = Literal::string(&kanji_form);
            quote! {Some(#kanji_form.to_string())}
        }
        None => quote! {None},
    };
    quote! {
        word::StaticWord{
            chapter:#chapter,
            dictionary_form:#dictionary_form,
            english_definition:#english_definition,
            kanji_form:#kanji_form,
        }
    }
}

fn make_conjugation(conjugation: Conjugation) -> proc_macro2::TokenStream {
    let present_affirmative = Literal::string(&conjugation.present_affirmative);
    let present_negative = Literal::string(&conjugation.present_negative);
    let past_affirmative = Literal::string(&conjugation.past_affirmative);
    let past_negative = Literal::string(&conjugation.past_negative);
    quote! {
        word::StaticConjugation{
            present_affirmative:#present_affirmative,
            present_negative:#present_negative,
            past_affirmative:#past_affirmative,
            past_negative:#past_negative,
        }
    }
}

#[proc_macro]
pub fn conjugate(ts: TokenStream) -> TokenStream {
    let conjugatable = parse_macro_input!(ts as Conjugatable);
    let source = match conjugatable.word_type {
        WordType::Adjective { conjugation, word } => {
            let word = make_word(word);
            let conjugation = make_conjugation(conjugation);
            quote! {
                word::StaticWordType::Adjective{conjugation:#conjugation,word:#word}
            }
        }
        WordType::Verb { conjugation, word } => {
            let word = make_word(word);
            let conjugation = make_conjugation(conjugation);
            quote! {
                word::StaticWordType::Verb{conjugation:#conjugation,word:#word}
            }
        }
        WordType::Noun(word) => {
            let word = make_word(word);
            quote! {
                word::StaticWordType::Noun(#word);
            }
        }
    };
    TokenStream::from(source)
}
