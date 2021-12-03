use word::StaticWordType;
use yew::{html, Component, Properties};

pub struct WordComponent {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub word_type: &'static StaticWordType,
}

impl Component for WordComponent {
    type Message = ();

    type Properties = Props;

    fn create(props: Self::Properties, _link: yew::ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> yew::Html {
        let (word, conjugation) = match self.props.word_type {
            StaticWordType::Adjective { word, conjugation } => (word, Some(conjugation)),
            StaticWordType::Noun(word) => (word, None),
            StaticWordType::Verb { word, conjugation } => (word, Some(conjugation)),
        };
        html! {
            <>
                <div>{word.dictionary_form}</div>
                <div>{word.english_definition}</div>
                <div>{word.kanji_form.unwrap_or("")}</div>
                {conjugation.map(|conjugation|html!{
                    <table>
                        <tr>
                            <th></th>
                            <th>{"Affirmative"}</th>
                            <th>{"Negative"}</th>
                        </tr>
                        <tr>
                            <th>{"Present"}</th>
                            <td>{conjugation.present_affirmative}</td>
                            <td>{conjugation.present_negative}</td>
                        </tr>
                        <tr>
                            <th>{"Past"}</th>
                            <td>{conjugation.past_affirmative}</td>
                            <td>{conjugation.past_negative}</td>
                        </tr>
                    </table>
                }).unwrap_or(html!{<></>})}
            </>

        }
    }
}
